use std::{env, fs::File};

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]

pub struct ServerConfig {
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub database: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let config = match (
            File::open("user_stat.yml"),
            env::var("NOTIFY_SERVER_CONFIG"),
        ) {
            (Ok(file), _) => serde_yaml::from_reader(file),
            (_, Ok(config)) => serde_yaml::from_reader(File::open(config)?),
            _ => bail!("config not found"),
        };

        Ok(config?)
    }
}

impl DatabaseConfig {
    pub fn get_url_with_database(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
    pub fn get_url_without_database(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}
