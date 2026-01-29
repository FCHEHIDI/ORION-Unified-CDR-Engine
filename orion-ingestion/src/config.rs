use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub kafka: KafkaConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KafkaConfig {
    pub brokers: String,
    pub group_id: String,
    pub topics: Vec<String>,
    pub auto_offset_reset: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let kafka_brokers = env::var("KAFKA_BROKERS")
            .unwrap_or_else(|_| "localhost:9092".to_string());
        
        let kafka_topics = env::var("KAFKA_TOPICS")
            .unwrap_or_else(|_| "cdr.raw.FR,cdr.raw.TN,cdr.raw.FN,cdr.raw.CH".to_string());
        
        let server_host = env::var("SERVER_HOST")
            .unwrap_or_else(|_| "0.0.0.0".to_string());
        
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8081".to_string())
            .parse()
            .unwrap_or(8081);

        Ok(Config {
            kafka: KafkaConfig {
                brokers: kafka_brokers,
                group_id: "orion-ingestion-group".to_string(),
                topics: kafka_topics.split(',').map(|s| s.trim().to_string()).collect(),
                auto_offset_reset: "earliest".to_string(),
            },
            server: ServerConfig {
                host: server_host,
                port: server_port,
            },
        })
    }
}
