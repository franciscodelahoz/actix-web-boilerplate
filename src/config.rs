use std::env;

use crate::libraries::constants::config::{DEFAULT_LOG_LEVEL, DEFAULT_PORT};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub port: u16,
    pub log_level: String,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let _ = dotenvy::dotenv();

        let application_port = env::var("PORT")
            .unwrap_or_else(|_| DEFAULT_PORT.into())
            .parse()
            .expect("PORT value must be a valid number");

        let log_level = env::var("LOG_LEVEL")
            .unwrap_or_else(|_| DEFAULT_LOG_LEVEL.to_string());

        Ok(Config {
            port: application_port,
            log_level,
        })
    }
}
