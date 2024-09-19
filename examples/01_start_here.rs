//! Simple Example: listens for COE packets, changes the CAN-ID and echoes them back.
//!
//! NOTE: this example requires at least the `alloc` feature, because it requires [ParseCOEError]
//! to `impl`Display`.
use std::error::Error;
use coe::{ParseCOEError, Payload};
use tokio::net::UdpSocket;

#[derive(Debug)]
pub enum ListenerError {
    UDP(std::io::Error),
    COE(ParseCOEError),
}
impl From<std::io::Error> for ListenerError {
    fn from(value: std::io::Error) -> Self {
        Self::UDP(value)
    }
}
impl From<ParseCOEError> for ListenerError {
    fn from(value: ParseCOEError) -> Self {
        Self::COE(value)
    }
}
impl std::fmt::Display for ListenerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UDP(e) => write!(f, "UDP Error: {e}"),
            Self::COE(e) => write!(f, "COE Error: {e}")
        }
    }
}
impl std::error::Error for ListenerError {}

async fn listener() -> Result<(), ListenerError> {
    let socket = UdpSocket::bind("0.0.0.0:5442").await?;

    // the largest possible COE packet is 256 byte long
    let mut buf = [0_u8; 256];
    loop {
        let (length, sender) = socket.recv_from(&mut buf).await?;
        let mut packet = TryInto::<coe::Packet>::try_into(&buf[0..length])?;
        for payload in packet.iter_mut() {
            // update all payloads and let them point to another CAN-ID
            *payload = Payload::new(23, payload.pdo_index(), payload.value());
        }

        // now forward the result back
        socket.connect(sender).await?;
        let buf = packet.serialize_into_vec();
        socket.send(&buf).await?;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listen_handle = tokio::spawn(async { listener().await });
    let (res,) = tokio::join!(listen_handle);
    res??;
    Ok(())
}
