//! This is a completely contrived example - we continually send some fixed date to a CMI.
//!
//! This example shows how to create payloads and use the date convenience functions.

use std::error::Error;
use coe::{to_month_of_year, Packet, ParseCOEError, Payload};
use tokio::net::UdpSocket;

#[derive(Debug)]
pub enum SenderError {
    UDP(std::io::Error),
    COE(ParseCOEError),
}
impl From<std::io::Error> for SenderError {
    fn from(value: std::io::Error) -> Self {
        Self::UDP(value)
    }
}
impl From<ParseCOEError> for SenderError {
    fn from(value: ParseCOEError) -> Self {
        Self::COE(value)
    }
}
impl std::fmt::Display for SenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UDP(e) => write!(f, "UDP Error: {e}"),
            Self::COE(e) => write!(f, "COE Error: {e}")
        }
    }
}
impl std::error::Error for SenderError {}

async fn sender() -> Result<(), SenderError> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(120));
    interval.tick().await;
    loop {
        // create the packet
        let mut packet = Packet::new();
        packet.try_push(Payload::new(13, 1, to_month_of_year(4, 2021).expect("statically good date").into()));

        // send the packet
        socket.connect("192.168.1.123:5442").await?;
        let buf = packet.serialize_into_vec();
        socket.send(&buf).await?;

        interval.tick().await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listen_handle = tokio::spawn(async { sender().await });
    let (res,) = tokio::join!(listen_handle);
    res??;
    Ok(())
}
