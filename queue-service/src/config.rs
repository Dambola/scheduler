use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Server {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Database {
    pub host: String,
    pub port: i32,
    pub username: String,
    pub password: String,
    pub database: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Configuration {
    pub server: Server,
    pub database: Database,
}

impl Configuration {
    pub fn new() -> Result<Self, ConfigError> {
        let builder = Config::builder()
            .add_source(File::with_name("config/default"));

        builder
            .build()?
            // Deserialize (and thus freeze) the entire configuration.
            .try_deserialize()
    }
}