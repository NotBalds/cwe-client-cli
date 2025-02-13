use crate::sys::cfg;
use notblib::{fmt::toml, fs};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub net: NetConfig,
}

#[derive(Serialize, Deserialize)]
pub struct NetConfig {
    pub host: String,
    pub port: String,
    pub proto: String,
}

pub fn exists() -> bool {
    cfg::CONFIG_PATH.exists()
}

pub fn get() -> Result<Config, crate::AnyError> {
    let config = fs::read::string(&cfg::CONFIG_PATH)?;
    let config: Config = toml::from(&config)?;
    Ok(config)
}

impl Config {
    pub fn upd() -> Result<Self, crate::AnyError> {
        get()
    }

    pub fn set(&self) -> Result<(), crate::AnyError> {
        let config: String = toml::to(self)?;
        fs::write::str(&cfg::CONFIG_PATH, config)?;
        Ok(())
    }
}
