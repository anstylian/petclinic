use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Database {
    pub path: String,
    pub connections: usize,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Redis {
    pub server: String,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Session {
    pub timeout: usize,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub config_name: String,
    pub database: Database,
    pub redis: Redis,
    pub session: Session,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or(String::from("development"));

        let s = Config::builder()
            .add_source(File::with_name(&format!("petclinic_config/{}", run_mode)).required(false))
            // Local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name("src/config/local").required(false))
            .add_source(Environment::with_prefix("app"))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}
