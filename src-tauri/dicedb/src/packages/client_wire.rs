use bytes::BytesMut;
use futures::{SinkExt, StreamExt};
use log::{debug, error};
use prost::Message;
use std::time::Duration;
use tokio::time::timeout;
use tokio::{io::AsyncWriteExt, net::TcpStream};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use crate::wire::{cmd::Command, res::Result as WireResult};

use super::error::ClientWireError;

pub struct ClientWire {
    framed: Framed<TcpStream, LengthDelimitedCodec>,
    max_response_size: usize,
}

impl ClientWire {
    pub fn new(max_response_size: usize, host: &str, port: u16) -> Result<Self, ClientWireError> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            ClientWireError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e))
        })?;

        rt.block_on(async { Self::new_async(max_response_size, host, port).await })
    }

    pub async fn new_async(
        max_response_size: usize,
        host: &str,
        port: u16,
    ) -> Result<Self, ClientWireError> {
        let addr = format!("{}:{}", host, port);

        debug!("Connecting to {}", addr);

        let stream = match timeout(Duration::from_secs(5), TcpStream::connect(&addr)).await {
            Ok(result) => result.map_err(|e| {
                ClientWireError::NotEstablished(format!("Failed to connect: {}", e))
            })?,
            Err(_) => {
                return Err(ClientWireError::TimeoutError(
                    "Connection timeout".to_string(),
                ));
            }
        };

        stream.set_nodelay(true).map_err(|e| {
            ClientWireError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to set nodelay: {}", e),
            ))
        })?;

        let framed = Framed::new(stream, LengthDelimitedCodec::new());

        Ok(Self {
            framed,
            max_response_size,
        })
    }

    pub async fn send(&mut self, cmd: &Command) -> Result<(), ClientWireError> {
        debug!("Sending command: {:?}", cmd);

        let mut buf = BytesMut::new();
        cmd.encode(&mut buf)?;

        self.framed
            .send(buf.freeze())
            .await
            .map_err(|e| ClientWireError::Terminated(format!("Failed to send command: {}", e)))?;

        Ok(())
    }

    pub async fn receive(&mut self) -> Result<WireResult, ClientWireError> {
        debug!("Waiting for response");

        let response = match timeout(Duration::from_secs(30), self.framed.next()).await {
            Ok(Some(Ok(bytes))) => {
                if bytes.len() > self.max_response_size {
                    return Err(ClientWireError::CorruptMessage(format!(
                        "Response too large: {} bytes, max is {} bytes",
                        bytes.len(),
                        self.max_response_size
                    )));
                }

                match WireResult::decode(bytes) {
                    Ok(result) => result,
                    Err(e) => return Err(ClientWireError::DecodingError(e)),
                }
            }
            Ok(Some(Err(e))) => {
                return Err(ClientWireError::Terminated(format!("Socket error: {}", e)));
            }
            Ok(None) => {
                return Err(ClientWireError::Terminated("Connection closed".to_string()));
            }
            Err(_) => {
                return Err(ClientWireError::TimeoutError(
                    "Response timeout".to_string(),
                ));
            }
        };

        debug!("Received response: status={}", response.status);
        Ok(response)
    }

    pub async fn close(&mut self) {
        debug!("Closing connection");
        if (self.framed.get_mut().shutdown().await).is_ok() {
            debug!("Socket shutdown successful");
        } else {
            error!("Failed to properly shutdown socket");
        }
    }
}
