pub mod socket;

use futures::prelude::*;
use socket::Socket;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener};
use tokio::spawn;
use bytes::BytesMut;

pub struct Server(TcpListener);

impl Server {
    pub async fn new(addr: SocketAddr) -> Result<Self, Box<dyn Error>> {
        Ok(Self(TcpListener::bind(&addr).await?))
    }

    pub async fn run(mut self) {
        loop {
            if let Ok((stream, _)) = self.0.accept().await {
                let mut socket = Socket::new(stream);

                spawn(
                    socket.run()
                );
            }
        }
    }
}
