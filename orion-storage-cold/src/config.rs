use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub kafka: KafkaConfig,
    pub s3: S3Config,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)] // Used with rdkafka on Linux/production
pub struct KafkaConfig {
    pub brokers: String,
    pub topic: String,
    pub group_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct S3Config {
    pub endpoint: String,
    pub region: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
    pub path_style: bool, // true for MinIO/Ceph, false for AWS
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            server: ServerConfig {
                host: env::var("ORION_STORAGE_COLD_HOST")
                    .unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("ORION_STORAGE_COLD_PORT")
                    .unwrap_or_else(|_| "9400".to_string())
                    .parse()
                    .expect("Invalid port"),
            },
            kafka: KafkaConfig {
                brokers: env::var("KAFKA_BROKERS")
                    .unwrap_or_else(|_| "localhost:9092".to_string()),
                topic: env::var("KAFKA_TOPIC_ENRICHED")
                    .unwrap_or_else(|_| "cdr-enriched".to_string()),
                group_id: env::var("KAFKA_GROUP_ID")
                    .unwrap_or_else(|_| "orion-storage-cold".to_string()),
            },
            s3: S3Config {
                endpoint: env::var("S3_ENDPOINT")
                    .unwrap_or_else(|_| "http://localhost:9000".to_string()),
                region: env::var("S3_REGION")
                    .unwrap_or_else(|_| "us-east-1".to_string()),
                bucket: env::var("S3_BUCKET")
                    .unwrap_or_else(|_| "orion-cdr-archive".to_string()),
                access_key: env::var("S3_ACCESS_KEY")
                    .unwrap_or_else(|_| "minioadmin".to_string()),
                secret_key: env::var("S3_SECRET_KEY")
                    .unwrap_or_else(|_| "minioadmin".to_string()),
                path_style: env::var("S3_PATH_STYLE")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()
                    .unwrap_or(true),
            },
        }
    }
}
