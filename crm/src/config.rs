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
    pub sender_email: String,
    pub user_stat: String,
    pub metadata: String,
    pub notification: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let config = match (File::open("crm.yml"), env::var("CRM_SERVER_CONFIG")) {
            (Ok(file), _) => serde_yaml::from_reader(file),
            (_, Ok(config)) => serde_yaml::from_reader(File::open(config)?),
            _ => bail!("config not found"),
        };

        Ok(config?)
    }
}
