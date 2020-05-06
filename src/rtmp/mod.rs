pub mod handshake;
pub mod session;

use handshake::Handshake;
use session::Session;
use bytes::{Bytes, BytesMut, BufMut};

/// 处理结果.
pub enum PorcessResult {
    /// 有未处理完成的数据块.
    Overflow(Bytes),

    /// 有需要回复给对等端的数据块.
    Callback(Bytes),

    /// 清空缓冲区
    /// 用于握手到会话之间的传递
    Empty
}

/// RTMP 协议处理.
///
/// 输入输出TCP数据，整个过程自动完成.
/// 同时返回一些关键性的RTMP消息.
pub struct Rtmp {
    handshake: Handshake,
    session: Session,
}

impl Rtmp {
    pub fn new() -> Self {
        Self {
            handshake: Handshake::new(),
            session: Session::new(),
        }
    }

    pub fn process(&mut self, message: Bytes) -> Bytes {
        let mut output = BytesMut::new();
        let mut chunk = message.clone();

        if !&self.handshake.completed {
            if let Some(results) = self.handshake.process(chunk.clone()) {
                for value in results {
                    match value {
                        PorcessResult::Callback(data) => {
                            &output.put(data);
                        },
                        PorcessResult::Overflow(data) => {
                            chunk = data;
                        },
                        PorcessResult::Empty => {
                            chunk.clear();
                        }
                    }
                }
            }
        }

        if self.handshake.completed && !&chunk.is_empty() {
            if let Some(results) = self.session.process(chunk) {
                for value in results {
                    if let PorcessResult::Callback(data) = value {
                        &output.put(data);
                    }
                }
            }
        }

        output.freeze()
    }
}
