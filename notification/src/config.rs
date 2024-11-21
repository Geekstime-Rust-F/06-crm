use std::{env, fs::File};

use anyhow::{bail, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize)]

pub struct ServerConfig {
    pub port: u16,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let config = match (
            File::open("notification.yml"),
            env::var("NOTIFY_SERVER_CONFIG"),
        ) {
            (Ok(file), _) => serde_yaml::from_reader(file),
            (_, Ok(config)) => serde_yaml::from_reader(File::open(config)?),
            _ => bail!("config not found"),
        };

        Ok(config?)
    }
}
