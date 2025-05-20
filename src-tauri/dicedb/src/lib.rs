mod constant;
mod packages;
pub mod wire;
use crate::{
    constant::{DEFAULT_BACKOFF_DURATION_SECS, DEFAULT_MAX_RETRIES, MAX_RESPONSE_SIZE},
    packages::{
        client_builder::ClientBuilder,
        client_wire::ClientWire,
        error::{ClientWireError, DiceDbError},
        retrier::Retrier,
        utils::compute_fingerprint,
    },
    wire::{
        cmd::Command,
        res::{Result as WireResult, Status as WireStatus},
    },
};
use log::{error, info, warn};
use std::{sync::Arc, time::Duration};
use tokio::sync::{Mutex, mpsc};
use uuid::Uuid;

// Client struct - main public API
pub struct Client {
    id: String,
    main_wire: Arc<Mutex<ClientWire>>,
    main_retrier: Retrier,
    watch_wire: Option<Arc<Mutex<ClientWire>>>,
    watch_retrier: Option<Retrier>,
    watch_tx: Option<mpsc::Sender<WireResult>>,
    host: String,
    port: u16,
}

impl Client {
    pub fn builder(host: &str, port: u16) -> ClientBuilder {
        ClientBuilder::new(host, port)
    }

    pub async fn new(host: &str, port: u16) -> Result<Self, DiceDbError> {
        Self::new_with_options(
            host,
            port,
            None,
            DEFAULT_MAX_RETRIES,
            Duration::from_secs(DEFAULT_BACKOFF_DURATION_SECS),
        )
        .await
    }

    pub async fn new_with_options(
        host: &str,
        port: u16,
        id: Option<&str>,
        max_retries: u32,
        backoff_duration: Duration,
    ) -> Result<Self, DiceDbError> {
        let main_retrier = Retrier::new(max_retries, backoff_duration);

        // Create and connect the main wire
        let client_wire = main_retrier
            .execute_async(
                || async { ClientWire::new_async(MAX_RESPONSE_SIZE, host, port).await },
                || async { Ok(()) },
            )
            .await
            .map_err(|err| {
                DiceDbError::ConnectionError(format!(
                    "Could not connect to dicedb server after {} retries: {}",
                    main_retrier.max_retries, err
                ))
            })?;

        let client_id = id
            .map(|s| s.to_string())
            .unwrap_or_else(|| Uuid::now_v7().to_string());

        let client = Self {
            id: client_id.clone(),
            main_wire: Arc::new(Mutex::new(client_wire)),
            main_retrier,
            watch_wire: None,
            watch_retrier: None,
            watch_tx: None,
            host: host.to_string(),
            port,
        };

        let handshake_resp = client
            .fire(Command {
                cmd: "HANDSHAKE".to_string(),
                args: vec![client_id, "command".to_string()],
            })
            .await;

        if handshake_resp.status == WireStatus::Err as i32 {
            return Err(DiceDbError::HandshakeError(handshake_resp.message));
        }

        Ok(client)
    }

    async fn fire(&self, cmd: Command) -> WireResult {
        self.fire_with_wire(cmd, &self.main_wire).await
    }

    async fn fire_with_wire(
        &self,
        cmd: Command,
        client_wire: &Arc<Mutex<ClientWire>>,
    ) -> WireResult {
        let send_result = self
            .main_retrier
            .execute_async_void(
                move || {
                    let wire = Arc::clone(client_wire);
                    let cmd = cmd.clone();
                    async move {
                        let mut guard = wire.lock().await;
                        guard.send(&cmd).await
                    }
                },
                || {
                    let self_clone = self.clone();
                    async move { self_clone.restore_main_wire().await }
                },
            )
            .await;

        if let Err(err) = send_result {
            let message = format!("Failed to send command: {}", err);
            return WireResult {
                fingerprint64: compute_fingerprint(&message),
                status: WireStatus::Err as i32,
                message,
                response: None,
            };
        }

        let wire_clone = Arc::clone(client_wire);
        match self
            .main_retrier
            .execute_async(
                move || {
                    let wire = wire_clone.clone();
                    async move {
                        let mut guard = wire.lock().await;
                        guard.receive().await
                    }
                },
                || {
                    let self_clone = self.clone();
                    async move { self_clone.restore_main_wire().await }
                },
            )
            .await
        {
            Ok(resp) => resp,
            Err(err) => {
                let message = format!("Failed to receive response: {}", err);
                WireResult {
                    fingerprint64: compute_fingerprint(&message),
                    status: WireStatus::Err as i32,
                    message,
                    response: None,
                }
            }
        }
    }

    pub async fn fire_string(&self, cmd_str: &str) -> WireResult {
        let cmd_str = cmd_str.trim();
        let tokens: Vec<&str> = cmd_str.split_whitespace().collect();

        if tokens.is_empty() {
            let message = "Empty command".to_string();
            return WireResult {
                fingerprint64: compute_fingerprint(&message),
                status: WireStatus::Err as i32,
                message,
                response: None,
            };
        }

        let cmd = tokens[0].to_string();
        let args = tokens[1..].iter().map(|s| s.to_string()).collect();

        self.fire(Command { cmd, args }).await
    }

    pub async fn set(&self, key: &str, value: &[u8]) -> WireResult {
        self.fire(Command {
            cmd: "SET".to_string(),
            args: vec![key.to_string(), String::from_utf8_lossy(value).to_string()],
        })
        .await
    }

    pub async fn get(&self, key: &str) -> WireResult {
        self.fire(Command {
            cmd: "GET".to_string(),
            args: vec![key.to_string()],
        })
        .await
    }

    pub async fn del(&self, key: &str) -> WireResult {
        self.fire(Command {
            cmd: "DEL".to_string(),
            args: vec![key.to_string()],
        })
        .await
    }

    pub async fn watch(&mut self) -> Result<mpsc::Receiver<WireResult>, DiceDbError> {
        if let Some(tx) = &self.watch_tx {
            let (_tx, mut rx) = mpsc::channel(100);
            let existing_tx = tx.clone();

            tokio::spawn(async move {
                while let Some(result) = rx.recv().await {
                    if existing_tx.send(result).await.is_err() {
                        break;
                    }
                }
            });
        }

        // Create new channel for watch events
        let (tx, rx) = mpsc::channel(100);
        self.watch_tx = Some(tx.clone());

        // Set up watch connection and retrier
        self.watch_retrier = Some(Retrier::new(5, Duration::from_secs(5)));

        let watch_wire = ClientWire::new_async(MAX_RESPONSE_SIZE, &self.host, self.port)
            .await
            .map_err(|e| {
                DiceDbError::ConnectionError(format!(
                    "Failed to establish watch connection with server: {}",
                    e
                ))
            })?;

        self.watch_wire = Some(Arc::new(Mutex::new(watch_wire)));

        // Perform watch handshake
        if let Some(watch_wire) = &self.watch_wire {
            let handshake_resp = self
                .fire_with_wire(
                    Command {
                        cmd: "HANDSHAKE".to_string(),
                        args: vec![self.id.clone(), "watch".to_string()],
                    },
                    watch_wire,
                )
                .await;

            if handshake_resp.status == WireStatus::Err as i32 {
                return Err(DiceDbError::HandshakeError(handshake_resp.message));
            }
        }

        // Start the watch loop in a background task
        let watch_wire_clone = self.watch_wire.as_ref().unwrap().clone();
        let watch_retrier_clone = self.watch_retrier.as_ref().unwrap().clone();
        let tx_clone = tx.clone();
        let client_host = self.host.clone();
        let client_port = self.port;
        let client_id = self.id.clone();

        tokio::spawn(async move {
            Self::watch_loop(
                watch_wire_clone,
                watch_retrier_clone,
                tx_clone,
                client_host,
                client_port,
                client_id,
            )
            .await;
        });

        Ok(rx)
    }

    async fn watch_loop(
        watch_wire: Arc<Mutex<ClientWire>>,
        retrier: Retrier,
        tx: mpsc::Sender<WireResult>,
        host: String,
        port: u16,
        client_id: String,
    ) {
        loop {
            let wire_clone = Arc::clone(&watch_wire);
            let result = retrier
                .execute_async(
                    move || {
                        let wire = wire_clone.clone();
                        async move {
                            let mut guard = wire.lock().await;
                            guard.receive().await
                        }
                    },
                    || {
                        let watch_wire = Arc::clone(&watch_wire);
                        let host = host.clone();
                        let client_id = client_id.clone();
                        async move {
                            warn!("Trying to restore watch connection with server...");

                            match ClientWire::new_async(MAX_RESPONSE_SIZE, &host, port).await {
                                Ok(new_wire) => {
                                    let mut guard = watch_wire.lock().await;
                                    *guard = new_wire;

                                    // Re-perform handshake after reconnection
                                    let handshake_cmd = Command {
                                        cmd: "HANDSHAKE".to_string(),
                                        args: vec![client_id.clone(), "watch".to_string()],
                                    };

                                    if let Err(e) = guard.send(&handshake_cmd).await {
                                        warn!("Failed to send handshake after reconnection: {}", e);
                                        return Err(e);
                                    }

                                    match guard.receive().await {
                                        Ok(result) => {
                                            if result.status == WireStatus::Ok as i32 {
                                                info!("Watch connection restored successfully");
                                                Ok(())
                                            } else {
                                                warn!(
                                                    "Handshake failed after reconnection: {}",
                                                    result.message
                                                );
                                                Err(ClientWireError::HandshakeError(result.message))
                                            }
                                        }
                                        Err(e) => {
                                            warn!("Failed to receive handshake response: {}", e);
                                            Err(e)
                                        }
                                    }
                                }
                                Err(err) => {
                                    warn!(
                                        "Failed to restore watch connection with server: {}",
                                        err
                                    );
                                    Err(err)
                                }
                            }
                        }
                    },
                )
                .await;

            match result {
                Ok(resp) => {
                    if tx.send(resp).await.is_err() {
                        // Channel is closed, exit the watch loop
                        break;
                    }
                }
                Err(err) => {
                    error!(
                        "Watch connection has been terminated due to an error: {}",
                        err
                    );
                    // Close the wire connection before exiting
                    let mut guard = watch_wire.lock().await;
                    guard.close().await;
                    break;
                }
            }
        }
    }

    async fn restore_main_wire(&self) -> Result<(), ClientWireError> {
        warn!("Trying to restore connection with server...");

        match ClientWire::new(MAX_RESPONSE_SIZE, &self.host, self.port) {
            Ok(new_wire) => {
                let mut guard = self.main_wire.lock().await;
                *guard = new_wire;

                // Re-perform handshake after reconnection
                let handshake_cmd = Command {
                    cmd: "HANDSHAKE".to_string(),
                    args: vec![self.id.clone(), "command".to_string()],
                };

                if let Err(e) = guard.send(&handshake_cmd).await {
                    warn!("Failed to send handshake after reconnection: {}", e);
                    return Err(e);
                }

                match guard.receive().await {
                    Ok(result) => {
                        if result.status == WireStatus::Ok as i32 {
                            info!("Connection restored successfully");
                            Ok(())
                        } else {
                            warn!("Handshake failed after reconnection: {}", result.message);
                            Err(ClientWireError::HandshakeError(result.message))
                        }
                    }
                    Err(e) => {
                        warn!("Failed to receive handshake response: {}", e);
                        Err(e)
                    }
                }
            }
            Err(err) => {
                warn!("Failed to restore connection with server: {}", err);
                Err(err)
            }
        }
    }

    pub async fn close(&mut self) {
        // Close main wire
        let mut main_guard = self.main_wire.lock().await;
        main_guard.close().await;

        // Close watch wire if exists
        if let Some(watch_wire) = &self.watch_wire {
            let mut watch_guard = watch_wire.lock().await;
            watch_guard.close().await;
            self.watch_tx = None;
        }
    }
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            main_wire: Arc::clone(&self.main_wire),
            main_retrier: self.main_retrier.clone(),
            watch_wire: self.watch_wire.as_ref().map(Arc::clone),
            watch_retrier: self.watch_retrier.clone(),
            watch_tx: self.watch_tx.clone(),
            host: self.host.clone(),
            port: self.port,
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        // We can't use async functions in Drop, so we need to spawn a task to run close
        if let Ok(runtime) = tokio::runtime::Handle::try_current() {
            let mut client_clone = self.clone();
            runtime.spawn(async move {
                client_clone.close().await;
            });
        }
    }
}
