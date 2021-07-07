use config::{ConfigError, Config, File};
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Slam {
    pub camera_config: String,
    pub vocab: String,
    pub video: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub grpc_port: u16,
    pub rover_address: String,
    pub slam: Slam,
    // TODO add slam options
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();
        s.merge(File::with_name("config/default"))?;
        s.merge(File::with_name("config/local").required(false))?;
        s.try_into()
    }
}