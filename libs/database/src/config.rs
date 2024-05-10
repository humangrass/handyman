use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub max_open_cons: u32,
    pub min_idle_cons: u32,
    pub conn_max_lifetime: Duration,
    pub connection_timeout: Duration,
    pub idle_timeout: Duration,
}
