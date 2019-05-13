use serde::{Deserialize, Deserializer};
use serde_derive::{Deserialize};
use std::string::{ToString};

#[derive(Debug, Deserialize)]
pub(crate) enum Protocol {
    websocket,
    rtmp,
}
