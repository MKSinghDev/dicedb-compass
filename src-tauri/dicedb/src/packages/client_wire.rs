use super::{connector::TcpConnection, wire};

pub struct ClientWire {
    pub connection: TcpConnection,
    pub max_response_size: usize,
}

impl ClientWire {
    pub fn new(max_response_size: usize, host: &str, port: u16) -> Result<Self, wire::WireError> {
        let connection = TcpConnection::new(host, port)?;

        Ok(Self {
            connection,
            max_response_size,
        })
    }

    pub fn send(&self, cmd: &wire::Command) -> Result<(), wire::WireError> {
        if !self.connection.connected {
            return Err(wire::WireError {
                kind: wire::ErrKind::Terminated,
                cause: "Connection terminated".to_string(),
            });
        }

        // In a real implementation, this would serialize and send the command

        Ok(())
    }

    pub fn receive(&self) -> Result<wire::Result, wire::WireError> {
        if !self.connection.connected {
            return Err(wire::WireError {
                kind: wire::ErrKind::Terminated,
                cause: "Connection terminated".to_string(),
            });
        }

        // In a real implementation, this would receive and deserialize the response

        Ok(wire::Result {
            status: wire::Status::Ok,
            message: "OK".to_string(),
            data: None,
        })
    }

    pub fn close(&mut self) {
        self.connection.close();
    }
}
