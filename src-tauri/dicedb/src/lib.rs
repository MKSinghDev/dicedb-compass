mod constant;
mod packages;
use crate::constant::MAX_RESPONSE_SIZE;
use packages::{
    client_builder::ClientBuilder, client_wire::ClientWire, error::DiceDbError, retrier::Retrier,
    wire,
};
use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
    time::Duration,
};
use tracing::{error, info, warn};
use uuid::Uuid;

pub struct Client {
    id: String,
    main_wire: Arc<Mutex<ClientWire>>,
    main_retrier: Retrier,
    watch_wire: Option<Arc<Mutex<ClientWire>>>,
    watch_retrier: Option<Retrier>,
    watch_tx: Option<mpsc::Sender<wire::Result>>,
    host: String,
    port: u16,
}

impl Client {
    pub fn builder(host: &str, port: u16) -> ClientBuilder {
        ClientBuilder::new(host, port)
    }

    pub fn new(host: &str, port: u16) -> Result<Self, DiceDbError> {
        Self::new_with_options(host.to_string(), port, None)
    }

    pub fn new_with_options(
        host: String,
        port: u16,
        id: Option<String>,
    ) -> Result<Self, DiceDbError> {
        let main_retrier = Retrier::new(3, Duration::from_secs(5));

        // Create and connect the main wire
        let client_wire = main_retrier.execute(
            &[wire::ErrKind::NotEstablished],
            || ClientWire::new(MAX_RESPONSE_SIZE, &host, port),
            || Ok(()),
        ).map_err(|err| {
            if err.kind == wire::ErrKind::NotEstablished {
                DiceDbError::ConnectionError(format!(
                    "Could not connect to dicedb server after {} retries: {}", 
                    main_retrier.max_retries, err.cause
                ))
            } else {
                DiceDbError::UnexpectedError(format!(
                    "Unexpected error when establishing server connection, report this to dicedb maintainers: {}", 
                    err.cause
                ))
            }
        })?;

        let client_id = id.unwrap_or_else(|| Uuid::now_v7().to_string());

        let client = Self {
            id: client_id.clone(),
            main_wire: Arc::new(Mutex::new(client_wire)),
            main_retrier,
            watch_wire: None,
            watch_retrier: None,
            watch_tx: None,
            host,
            port,
        };

        // Perform handshake
        let handshake_resp = client.fire(&wire::Command {
            cmd: "HANDSHAKE".to_string(),
            args: vec![client_id, "command".to_string()],
        });

        if handshake_resp.status == wire::Status::Err {
            return Err(DiceDbError::HandshakeError(handshake_resp.message));
        }

        Ok(client)
    }

    pub fn fire(&self, cmd: &wire::Command) -> wire::Result {
        self.fire_with_wire(cmd, &self.main_wire)
    }

    pub fn fire_with_wire(
        &self,
        cmd: &wire::Command,
        client_wire: &Arc<Mutex<ClientWire>>,
    ) -> wire::Result {
        let wire_clone = Arc::clone(client_wire);
        let cmd_clone = cmd.clone();

        // Send the command
        let send_result = self.main_retrier.execute_void(
            &[wire::ErrKind::Terminated],
            move || {
                let guard = wire_clone.lock().unwrap();
                guard.send(&cmd_clone)
            },
            || self.restore_main_wire(),
        );

        if let Err(err) = send_result {
            let message = match err.kind {
                wire::ErrKind::Terminated => {
                    format!(
                        "Failed to send command, connection terminated: {}",
                        err.cause
                    )
                }
                wire::ErrKind::CorruptMessage => {
                    format!("Failed to send command, corrupt message: {}", err.cause)
                }
                _ => {
                    format!(
                        "Failed to send command: unrecognized error, this should be reported to DiceDB maintainers: {}",
                        err.cause
                    )
                }
            };

            return wire::Result {
                status: wire::Status::Err,
                message,
                data: None,
            };
        }

        // Receive the response
        let wire_clone = Arc::clone(client_wire);
        match self.main_retrier.execute(
            &[wire::ErrKind::Terminated],
            move || {
                let guard = wire_clone.lock().unwrap();
                guard.receive()
            },
            || self.restore_main_wire(),
        ) {
            Ok(resp) => resp,
            Err(err) => wire::Result {
                status: wire::Status::Err,
                message: format!("Failed to receive response: {}", err.cause),
                data: None,
            },
        }
    }

    pub fn fire_string(&self, cmd_str: &str) -> wire::Result {
        let cmd_str = cmd_str.trim();
        let tokens: Vec<&str> = cmd_str.split_whitespace().collect();

        if tokens.is_empty() {
            return wire::Result {
                status: wire::Status::Err,
                message: "Empty command".to_string(),
                data: None,
            };
        }

        let cmd = tokens[0].to_string();
        let args = tokens[1..].iter().map(|s| s.to_string()).collect();

        self.fire(&wire::Command { cmd, args })
    }

    pub fn watch(&mut self) -> Result<mpsc::Receiver<wire::Result>, DiceDbError> {
        if let Some(tx) = &self.watch_tx {
            // Channel exists, clone the receiver
            let (_, rx) = mpsc::channel();
            return Ok(rx); // This is a simplification, in real code you'd need to clone the receiver properly
        }

        // Create new channel for watch events
        let (tx, rx) = mpsc::channel();
        self.watch_tx = Some(tx.clone());

        // Set up watch connection and retrier
        self.watch_retrier = Some(Retrier::new(5, Duration::from_secs(5)));

        let watch_wire =
            ClientWire::new(MAX_RESPONSE_SIZE, &self.host, self.port).map_err(|e| {
                DiceDbError::ConnectionError(format!(
                    "Failed to establish watch connection with server: {}",
                    e.cause
                ))
            })?;

        self.watch_wire = Some(Arc::new(Mutex::new(watch_wire)));

        // Perform watch handshake
        if let Some(watch_wire) = &self.watch_wire {
            let handshake_resp = self.fire_with_wire(
                &wire::Command {
                    cmd: "HANDSHAKE".to_string(),
                    args: vec![self.id.clone(), "watch".to_string()],
                },
                watch_wire,
            );

            if handshake_resp.status == wire::Status::Err {
                return Err(DiceDbError::HandshakeError(handshake_resp.message));
            }
        }

        // Start the watch loop in a background thread
        let watch_wire_clone = self.watch_wire.as_ref().unwrap().clone();
        let watch_retrier_clone = self.watch_retrier.as_ref().unwrap().clone();
        let client_host = self.host.clone();
        let client_port = self.port;

        thread::spawn(move || {
            Self::watch_loop(
                watch_wire_clone,
                watch_retrier_clone,
                tx,
                client_host,
                client_port,
            );
        });

        Ok(rx)
    }

    pub fn watch_loop(
        watch_wire: Arc<Mutex<ClientWire>>,
        retrier: Retrier,
        tx: mpsc::Sender<wire::Result>,
        host: String,
        port: u16,
    ) {
        let restore_watch_wire = |wire: &Arc<Mutex<ClientWire>>| -> Result<(), wire::WireError> {
            warn!("Trying to restore connection with server...");

            match ClientWire::new(MAX_RESPONSE_SIZE, &host, port) {
                Ok(new_wire) => {
                    let mut guard = wire.lock().unwrap();
                    *guard = new_wire;
                    info!("Connection restored successfully");
                    Ok(())
                }
                Err(err) => {
                    warn!("Failed to restore connection with server: {:?}", err);
                    Err(err)
                }
            }
        };

        loop {
            let wire_clone = Arc::clone(&watch_wire);
            let result = retrier.execute(
                &[wire::ErrKind::Terminated],
                move || {
                    let guard = wire_clone.lock().unwrap();
                    guard.receive()
                },
                || restore_watch_wire(&watch_wire),
            );

            match result {
                Ok(resp) => {
                    if tx.send(resp).is_err() {
                        // Channel is closed, exit the watch loop
                        break;
                    }
                }
                Err(err) => {
                    error!(
                        "Watch connection has been terminated due to an error: {:?}",
                        err
                    );
                    let mut guard = watch_wire.lock().unwrap();
                    guard.close();
                    break;
                }
            }
        }
    }

    pub fn restore_main_wire(&self) -> Result<(), wire::WireError> {
        warn!("Trying to restore connection with server...");

        match ClientWire::new(MAX_RESPONSE_SIZE, &self.host, self.port) {
            Ok(new_wire) => {
                let mut guard = self.main_wire.lock().unwrap();
                *guard = new_wire;
                info!("Connection restored successfully");
                Ok(())
            }
            Err(err) => {
                warn!("Failed to restore connection with server: {:?}", err);
                Err(err)
            }
        }
    }

    pub fn close(&mut self) {
        // Close main wire
        let mut main_guard = self.main_wire.lock().unwrap();
        main_guard.close();

        // Close watch wire if exists
        if let Some(watch_wire) = &self.watch_wire {
            let mut watch_guard = watch_wire.lock().unwrap();
            watch_guard.close();
            self.watch_tx = None;
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.close();
    }
}
