use anyhow::Result;
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub scylla: ScyllaConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScyllaConfig {
    pub nodes: Vec<String>,
    pub keyspace: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let server = ServerConfig {
            host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string()).parse()?,
        };

        let nodes_str = env::var("SCYLLA_NODES").unwrap_or_else(|_| "scylladb:9042".to_string());
        let nodes = nodes_str.split(',').map(|s| s.to_string()).collect();

        let scylla = ScyllaConfig {
            nodes,
            keyspace: env::var("SCYLLA_KEYSPACE").unwrap_or_else(|_| "orion_cdr".to_string()),
        };

        Ok(Config { server, scylla })
    }
}
