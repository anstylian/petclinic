use argh::FromArgs;
use config::FileFormat;
use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::env;
use std::path::Path;

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct Database {
    pub path: String,
    pub connections: usize,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct Redis {
    pub server: String,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct Session {
    pub timeout: usize,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct Settings {
    pub config_name: String,
    pub service_port: u32,
    pub database: Database,
    pub redis: Redis,
    pub tera_templates: String,
    pub session: Session,
}

/// Available Arguments
#[derive(Debug, Deserialize, Default, FromArgs)]
#[allow(unused)]
struct Args {
    /// config file setup
    #[argh(option)]
    pub config_file: Option<String>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let args: Args = argh::from_env();

        let run_mode = if let Some(config_file) = args.config_file {
            let root = Path::new("/etc/petclinic");
            env::set_current_dir(root).unwrap();
            config_file
        } else {
            env::var("RUN_MODE").unwrap_or(String::from("development"))
        };

        let s = Config::builder()
            .add_source(
                File::new(&format!("petclinic_config/{}", run_mode), FileFormat::Toml)
                    .required(false),
            )
            .add_source(File::new(&run_mode.to_string(), FileFormat::Toml).required(false))
            // Local configuration file
            // This file shouldn't be checked in to git
            // .add_source(File::with_name("src/config/local").required(false))
            // .add_source(Environment::with_prefix("petclinic-"))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}
