use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub kafka: KafkaConfig,
    pub generation: GenerationConfig,
    pub simulation: SimulationConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Clone)]
pub struct KafkaConfig {
    pub brokers: Vec<String>,
    pub base_topic: String,
}

#[derive(Debug, Clone)]
pub struct GenerationConfig {
    pub rate_per_second: u32,
    pub burst_enabled: bool,
    pub burst_multiplier: u32,
    pub burst_duration_secs: u64,
    pub fraud_rate_percent: u8,
    pub malformed_rate_percent: u8,
}

#[derive(Debug, Clone)]
pub struct SimulationConfig {
    pub enable_latency: bool,
    pub min_latency_ms: u64,
    pub max_latency_ms: u64,
    pub enable_errors: bool,
    pub error_rate_percent: u8,
    pub enable_retry: bool,
    pub max_retries: u32,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    #[allow(dead_code)]
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
            base_topic: env::var("KAFKA_BASE_TOPIC")
                .unwrap_or_else(|_| "cdr.raw".to_string()),
        };

        let generation = GenerationConfig {
            rate_per_second: env::var("GENERATION_RATE")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .context("Invalid GENERATION_RATE")?,
            burst_enabled: env::var("BURST_ENABLED")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .context("Invalid BURST_ENABLED")?,
            burst_multiplier: env::var("BURST_MULTIPLIER")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .context("Invalid BURST_MULTIPLIER")?,
            burst_duration_secs: env::var("BURST_DURATION_SECS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .context("Invalid BURST_DURATION_SECS")?,
            fraud_rate_percent: env::var("FRAUD_RATE_PERCENT")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .context("Invalid FRAUD_RATE_PERCENT")?,
            malformed_rate_percent: env::var("MALFORMED_RATE_PERCENT")
                .unwrap_or_else(|_| "2".to_string())
                .parse()
                .context("Invalid MALFORMED_RATE_PERCENT")?,
        };

        let simulation = SimulationConfig {
            enable_latency: env::var("SIMULATE_LATENCY")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .context("Invalid SIMULATE_LATENCY")?,
            min_latency_ms: env::var("MIN_LATENCY_MS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .context("Invalid MIN_LATENCY_MS")?,
            max_latency_ms: env::var("MAX_LATENCY_MS")
                .unwrap_or_else(|_| "500".to_string())
                .parse()
                .context("Invalid MAX_LATENCY_MS")?,
            enable_errors: env::var("SIMULATE_ERRORS")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .context("Invalid SIMULATE_ERRORS")?,
            error_rate_percent: env::var("ERROR_RATE_PERCENT")
                .unwrap_or_else(|_| "3".to_string())
                .parse()
                .context("Invalid ERROR_RATE_PERCENT")?,
            enable_retry: env::var("ENABLE_RETRY")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .context("Invalid ENABLE_RETRY")?,
            max_retries: env::var("MAX_RETRIES")
                .unwrap_or_else(|_| "3".to_string())
                .parse()
                .context("Invalid MAX_RETRIES")?,
        };

        let server = ServerConfig {
            host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "9200".to_string())
                .parse()
                .context("Invalid SERVER_PORT")?,
        };

        Ok(Config {
            kafka,
            generation,
            simulation,
            server,
        })
    }
}
