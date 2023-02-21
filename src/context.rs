use anyhow::Result;
use deadpool_diesel::{sqlite::Pool, Manager, Runtime};
use redis::Connection;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::settings::Settings;

pub struct Context {
    pub db_connection_pool: Pool,
    pub redis_connection: Mutex<Connection>,
    pub settings: Arc<Settings>,
}

impl Context {
    pub fn new(settings: Arc<Settings>) -> Result<Self> {
        let redis_url = match &settings.redis.password {
            Some(password) => format!("redis://:{}@{}", password, settings.redis.server),
            None => format!("redis://{}", settings.redis.server),
        };

        let client = redis::Client::open(redis_url)?;
        let redis_connection = client.get_connection()?;

        let manager = Manager::new(&settings.database.path, Runtime::Tokio1);

        Ok(Self {
            db_connection_pool: Pool::builder(manager)
                .max_size(settings.database.connections)
                .build()?,
            redis_connection: Mutex::new(redis_connection),
            settings,
        })
    }
}
