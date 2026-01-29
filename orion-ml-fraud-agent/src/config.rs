use anyhow::Result;
use serde::Deserialize;
use std::env;

/// Main configuration structure for the ML Fraud Agent service
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub model: ModelConfig,
}

/// HTTP server configuration
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

/// ML model configuration
#[derive(Debug, Clone, Deserialize)]
pub struct ModelConfig {
    pub path: String,
    pub threshold: f32,
    pub batch_size: usize,
    pub enable_cuda: bool,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        let server = ServerConfig {
            host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8090".to_string())
                .parse()?,
        };

        let model = ModelConfig {
            path: env::var("MODEL_PATH")
                .unwrap_or_else(|_| "./models/fraud_detector.onnx".to_string()),
            threshold: env::var("FRAUD_THRESHOLD")
                .unwrap_or_else(|_| "0.5".to_string())
                .parse()?,
            batch_size: env::var("MODEL_BATCH_SIZE")
                .unwrap_or_else(|_| "32".to_string())
                .parse()?,
            enable_cuda: env::var("ENABLE_CUDA")
                .unwrap_or_else(|_| "false".to_string())
                .parse()?,
        };

        Ok(Config { server, model })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_env_defaults() {
        // Test avec variables par défaut
        let config = Config::from_env().unwrap();
        
        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.server.port, 8090);
        assert_eq!(config.model.threshold, 0.5);
        assert_eq!(config.model.batch_size, 32);
        assert!(!config.model.enable_cuda);
    }
}
