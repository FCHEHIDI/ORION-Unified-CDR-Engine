use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub kafka: KafkaConfig,
    pub server: ServerConfig,
    pub enrichment: EnrichmentConfig,
}

#[derive(Debug, Clone)]
pub struct KafkaConfig {
    pub brokers: String,
    pub input_topic: String,
    pub output_topic: String,
    pub consumer_group: String,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct EnrichmentConfig {
    pub enable_fraud_detection: bool,
    pub fraud_agent_url: String,
    pub enable_network_data: bool,
    pub enable_client_data: bool,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let kafka_brokers = env::var("KAFKA_BROKERS")
            .unwrap_or_else(|_| "localhost:9092".to_string());
        
        let input_topic = env::var("KAFKA_INPUT_TOPIC")
            .unwrap_or_else(|_| "cdr.enriched".to_string());
        
        let output_topic = env::var("KAFKA_OUTPUT_TOPIC")
            .unwrap_or_else(|_| "cdr.stored".to_string());
        
        let consumer_group = env::var("KAFKA_CONSUMER_GROUP")
            .unwrap_or_else(|_| "orion-enrichment".to_string());
        
        let server_host = env::var("SERVER_HOST")
            .unwrap_or_else(|_| "0.0.0.0".to_string());
        
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8084".to_string())
            .parse::<u16>()?;
        
        let enable_fraud_detection = env::var("ENABLE_FRAUD_DETECTION")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);
        
        let fraud_agent_url = env::var("FRAUD_AGENT_URL")
            .unwrap_or_else(|_| "http://localhost:50051".to_string());
        
        let enable_network_data = env::var("ENABLE_NETWORK_DATA")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .unwrap_or(true);
        
        let enable_client_data = env::var("ENABLE_CLIENT_DATA")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .unwrap_or(true);
        
        Ok(Config {
            kafka: KafkaConfig {
                brokers: kafka_brokers,
                input_topic,
                output_topic,
                consumer_group,
            },
            server: ServerConfig {
                host: server_host,
                port: server_port,
            },
            enrichment: EnrichmentConfig {
                enable_fraud_detection,
                fraud_agent_url,
                enable_network_data,
                enable_client_data,
            },
        })
    }
}
