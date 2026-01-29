use anyhow::{Context, Result};
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub kafka: KafkaConfig,
    pub scylla: ScyllaConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KafkaConfig {
    pub brokers: Vec<String>,
    pub input_topic: String,
    pub consumer_group: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScyllaConfig {
    pub nodes: Vec<String>,
    pub keyspace: String,
    pub replication_factor: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok();

        let kafka = KafkaConfig {
            brokers: env::var("KAFKA_BROKERS")
                .unwrap_or_else(|_| "localhost:9092".to_string())
                .split(',')
                .map(|s| s.to_string())
                .collect(),
            input_topic: env::var("KAFKA_INPUT_TOPIC")
                .unwrap_or_else(|_| "cdr.stored".to_string()),
            consumer_group: env::var("KAFKA_CONSUMER_GROUP")
                .unwrap_or_else(|_| "orion-storage-hot".to_string()),
        };

        let scylla = ScyllaConfig {
            nodes: env::var("SCYLLA_NODES")
                .unwrap_or_else(|_| "localhost:9042".to_string())
                .split(',')
                .map(|s| s.to_string())
                .collect(),
            keyspace: env::var("SCYLLA_KEYSPACE")
                .unwrap_or_else(|_| "orion".to_string()),
            replication_factor: env::var("SCYLLA_REPLICATION_FACTOR")
                .unwrap_or_else(|_| "1".to_string())
                .parse()
                .context("Invalid SCYLLA_REPLICATION_FACTOR")?,
        };

        let server = ServerConfig {
            host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8085".to_string())
                .parse()
                .context("Invalid SERVER_PORT")?,
        };

        Ok(Config {
            kafka,
            scylla,
            server,
        })
    }
}
