mod rtmp;
mod server;

use std::error::Error;
use server::Server;
use tokio::main;
use tokio::spawn;
use std::net::SocketAddr;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use bytes::{Bytes, BytesMut};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:1935".parse().unwrap();
    let mut server = Server::new(addr).await?;
    server.run().await;
    Ok(())
}

// #[tokio::main]
// async fn main() {
//     let mut listener = TcpListener::bind("127.0.0.1:1935".parse::<SocketAddr>().unwrap()).await.unwrap();

//     loop {
//         let (mut stream, _) = listener.accept().await.unwrap();

//         spawn(async move {
//             let mut buf = [0; 1024];
//             let mut rtmp = rtmp::Rtmp::new();

//             loop {
//                 let n = stream.read(&mut buf).await.unwrap();

//                 if n == 0 {
//                     break;
//                 }

//                 let data = rtmp.process(Bytes::copy_from_slice(&buf[0..n]));
//                 stream.write_all(&data).await.unwrap();
//             }
//         });
//     }
// }