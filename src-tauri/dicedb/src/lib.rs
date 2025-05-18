mod constant;
mod wire;
use bytes::BytesMut;
use futures::{SinkExt, StreamExt};
use prost::Message;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use uuid::Uuid;
use wire::cmd::Command;

pub async fn connect_to_dicedb(
    addr: &str,
) -> Result<Framed<TcpStream, LengthDelimitedCodec>, Box<dyn std::error::Error>> {
    println!("Connecting to dicedb server...");
    let stream = TcpStream::connect(addr).await?;
    println!("Connected to dicedb server!");
    let mut framed = Framed::new(stream, LengthDelimitedCodec::new());
    println!("Performing handshake...");
    perform_handshake(&mut framed).await?;
    Ok(framed)
}

pub async fn perform_handshake(
    framed: &mut Framed<TcpStream, LengthDelimitedCodec>,
) -> Result<(), Box<dyn std::error::Error>> {
    let handshake_cmd = Command {
        cmd: "HANDSHAKE".to_string(),
        args: vec![Uuid::now_v7().to_string(), "command".to_string()],
    };

    println!("Sending handshake command...");
    let mut buf = BytesMut::new();
    handshake_cmd.encode(&mut buf)?;

    framed.send(buf.freeze()).await?;

    println!("Waiting for handshake response...");
    if let Some(response) = framed.next().await {
        let res_bytes = response?;
        let res = wire::res::Result::decode(res_bytes.as_ref())?;

        match res.response {
            Some(wire::res::result::Response::HandshakeRes(_)) => {
                println!("Handshake successful!");
            }
            other => {
                println!("Unexpected response: {:?}", other);
            }
        }
    }

    Ok(())
}
