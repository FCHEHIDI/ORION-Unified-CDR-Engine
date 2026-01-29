use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub kafka: KafkaConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Clone)]
pub struct KafkaConfig {
    pub brokers: String,
    pub input_topic: String,
    pub output_topic: String,
    pub rejected_topic: String,
    pub consumer_group: String,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let kafka_brokers = env::var("KAFKA_BROKERS")
            .unwrap_or_else(|_| "localhost:9092".to_string());
        
        let input_topic = env::var("KAFKA_INPUT_TOPIC")
            .unwrap_or_else(|_| "cdr.validated".to_string());
        
        let output_topic = env::var("KAFKA_OUTPUT_TOPIC")
            .unwrap_or_else(|_| "cdr.normalized".to_string());
        
        let rejected_topic = env::var("KAFKA_REJECTED_TOPIC")
            .unwrap_or_else(|_| "cdr.rejected".to_string());
        
        let consumer_group = env::var("KAFKA_CONSUMER_GROUP")
            .unwrap_or_else(|_| "orion-validation".to_string());
        
        let server_host = env::var("SERVER_HOST")
            .unwrap_or_else(|_| "0.0.0.0".to_string());
        
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8082".to_string())
            .parse::<u16>()?;
        
        Ok(Config {
            kafka: KafkaConfig {
                brokers: kafka_brokers,
                input_topic,
                output_topic,
                rejected_topic,
                consumer_group,
            },
            server: ServerConfig {
                host: server_host,
                port: server_port,
            },
        })
    }
}
