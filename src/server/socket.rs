use crate::rtmp::Rtmp;
use futures::prelude::*;
use bytes::{Bytes, BytesMut, BufMut};
use tokio::{net::TcpStream, io::AsyncRead, io::AsyncWrite};
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::split;
use tokio::sync::mpsc::{channel, Sender, Receiver};
use tokio::select;
use tokio::spawn;
use std::time::Duration;

enum State {
    Data(Bytes),
    NotData,
    Close
}

pub struct Socket {
    stream: TcpStream,
    codec: Rtmp
}

impl Socket {
    pub fn new(stream: TcpStream) -> Self {
        Self { 
            stream,
            codec: Rtmp::new()
        }
    }

    #[rustfmt::skip]
    pub async fn send(&mut self, data: &[u8]) {
        println!("data send size {:?}", &data.len());
        let mut offset: usize = 0;
        loop {
            match self.stream.write(&data[offset..]).await {
                Ok(size) => {
                    self.flush().await;
                    
                    offset += size;
                    if offset >= data.len() {
                        break;
                    }
                },
                Err(e) => {
                    dbg!(&e);
                }
            }
        }
    }

    // #[rustfmt::skip]
    // fn read(&mut self) -> State {
    //     println!("read data");
    //     let mut receiver = [0; 4096];
    //     match self.stream.poll_read(&mut receiver) {
    //         Ok(Async::Ready(size)) if size > 0 => State::Data(BytesMut::from(&receiver[0..size]).freeze()),
    //         Ok(Async::Ready(size)) if size == 0 => State::Close,
    //         _ => {
    //             println!("no data");
    //             State::NotData
    //         }
    //     }
    // }

    pub async fn flush(&mut self) {
        self.stream.flush().await.unwrap();
        // loop {
        //     match self.stream.poll_flush() {
        //         Ok(Async::Ready(_)) => { break; },
        //         _ => (),
        //     }
        // }
    }

    pub async fn run(mut self) {
        let mut buff = [0u8; 4096];
        // self.stream.set_nodelay(false).unwrap();
        // self.stream.set_send_buffer_size(16).unwrap();
        // self.stream.set_recv_buffer_size(16).unwrap();
        // self.stream.set_linger(Some(Duration::from_nanos(100))).unwrap();

        let (mut read_stream, mut write_stream) = split(self.stream);
        let (mut sender, mut receiver) = channel(16);

        loop {
            select! {
                result = read_stream.read(&mut buff) => {
                    match result {
                        Ok(size) => {
                            if size == 0 {
                                println!("break");
                                break;
                            } else {
                                sender.send(buff[0..size].to_vec()).await.unwrap();
                            }
                        },
                        Err(e) => {
                            println!("{:?}", e);
                        }
                    }
                }
                buff = receiver.recv() => {
                    if let Some(buff) = buff {
                        let mut buffer = BytesMut::new();
                        buffer.put_slice(&buff);
                        let receiver = self.codec.process(buffer.freeze());
                        dbg!(&receiver);

                        
                        write_stream.write_all(&receiver[..]).await.unwrap();
                        write_stream.flush().await.unwrap();
                    
                        println!("send finish");
                    }
                }
            }
        }
    }
}


// impl Future for Socket {
//     type Item = ();
//     type Error = ();

//     #[rustfmt::skip]
//     fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
//         // while let State::Data(buffer) = self.read() {
//         //     let receiver = self.codec.process(buffer);
//         //     dbg!(&receiver);
//         //     self.send(&receiver[..]);
//         //     // self.flush();
//         // }
        
            
//         // loop {
//         //     match self.read() {
//         //         State::Data(buffer) => {
//         //             let receiver = self.codec.process(buffer);
//         //             dbg!(&receiver);
//         //             self.send(&receiver[..]);
//         //             // self.flush();
//         //         },
//         //         State::NotData => {
//         //             println!("no data");
//         //         },
//         //         _ => {
//         //             println!("fuck");
//         //         }
//         //     }
//         // }

//         Ok(Async::NotReady)
//     }
// }
