use anyhow::Result;
use serde::Deserialize;
use std::env;

/// Configuration for the observability service
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub services: ServicesConfig,
}

/// HTTP server configuration
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

/// Monitored services configuration
#[derive(Debug, Clone, Deserialize)]
pub struct ServicesConfig {
    pub traffic_generator_url: String,
    pub ingestion_url: String,
    pub validation_url: String,
    pub normalization_url: String,
    pub enrichment_url: String,
    pub ml_fraud_agent_url: String,
    pub storage_hot_url: String,
    pub health_check_interval_secs: u64,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        let server = ServerConfig {
            host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "9100".to_string())
                .parse()?,
        };

        let services = ServicesConfig {
            traffic_generator_url: env::var("TRAFFIC_GENERATOR_URL")
                .unwrap_or_else(|_| "http://orion-traffic-generator:9200".to_string()),
            ingestion_url: env::var("INGESTION_URL")
                .unwrap_or_else(|_| "http://orion-ingestion:8081".to_string()),
            validation_url: env::var("VALIDATION_URL")
                .unwrap_or_else(|_| "http://orion-validation:8082".to_string()),
            normalization_url: env::var("NORMALIZATION_URL")
                .unwrap_or_else(|_| "http://orion-normalization:8083".to_string()),
            enrichment_url: env::var("ENRICHMENT_URL")
                .unwrap_or_else(|_| "http://orion-enrichment:8084".to_string()),
            ml_fraud_agent_url: env::var("ML_FRAUD_AGENT_URL")
                .unwrap_or_else(|_| "http://orion-ml-fraud-agent:8090".to_string()),
            storage_hot_url: env::var("STORAGE_HOT_URL")
                .unwrap_or_else(|_| "http://orion-storage-hot:8085".to_string()),
            health_check_interval_secs: env::var("HEALTH_CHECK_INTERVAL_SECS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()?,
        };

        Ok(Config { server, services })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_env_defaults() {
        let config = Config::from_env().unwrap();
        
        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.server.port, 9100);
        assert_eq!(config.services.health_check_interval_secs, 30);
        assert!(config.services.ingestion_url.contains("orion-ingestion"));
    }
}
