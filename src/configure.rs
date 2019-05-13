use crate::protocol::Protocol;
use failure::Error as FailError;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::Read;

/// # Push Stream Config.
#[derive(Deserialize, Debug)]
struct Listener {
    protocol: Protocol,
    genre: String,
    code: String,
    host: String,
    port: u32,
}

/// # Live Pool
#[derive(Deserialize, Debug)]
struct Pool {
    bytes: u8,
}

/// # Project Config.
#[derive(Deserialize, Debug)]
pub struct Config {
    server: Vec<Listener>,
    pool: Pool,
}

impl Config {
    /// Read configure file.
    ///
    /// ## example
    /// ```
    /// use quasipaa::configure::Config;
    /// let configure: Config = Config::from("./configure.toml").unwrap();
    /// 
    /// ```
    pub fn from_file(path: &'static str) -> Result<Config, FailError> {
        let mut file = File::open(path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let value: Config = toml::from_str(buffer.as_str())?;
        Ok(value)
    }

    pub fn from(config: &'static str) -> Result<Config, FailError> {
        let result: Config = toml::from_str(config)?;
        Ok(result)
    }
}
