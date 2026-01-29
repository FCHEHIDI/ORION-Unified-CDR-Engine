use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use crate::config::KafkaConfig;
use crate::service::model::{ProcessedCDR, ValidationResult};
use crate::service::validator::Validator;
use crate::service::kafka_producer::KafkaProducerService;
use tracing::{info, error, warn};

pub struct KafkaConsumerService {
    consumer: StreamConsumer,
    validator: Validator,
    producer: KafkaProducerService,
}

impl KafkaConsumerService {
    pub fn new(config: &KafkaConfig) -> anyhow::Result<Self> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", &config.brokers)
            .set("group.id", &config.consumer_group)
            .set("enable.auto.commit", "true")
            .set("auto.offset.reset", "earliest")
            .create()?;

        consumer.subscribe(&[&config.input_topic])?;

        let producer = KafkaProducerService::new(
            &config.brokers,
            config.output_topic.clone(),
            config.rejected_topic.clone(),
        )?;

        Ok(Self {
            consumer,
            validator: Validator::new(),
            producer,
        })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        info!("Kafka consumer service started");

        loop {
            match self.consumer.recv().await {
                Ok(message) => {
                    if let Some(payload) = message.payload() {
                        match serde_json::from_slice::<ProcessedCDR>(payload) {
                            Ok(cdr) => {
                                info!("Received CDR from {} (country: {})", cdr.source_topic, cdr.country);
                                
                                match self.validator.validate(&cdr).await {
                                    ValidationResult::Valid(valid_cdr) => {
                                        if let Err(e) = self.producer.send_valid(&valid_cdr).await {
                                            error!("Failed to send valid CDR: {}", e);
                                        }
                                    }
                                    ValidationResult::Invalid(error) => {
                                        warn!("CDR validation failed: {} - {}", error.error_type, error.message);
                                        if let Err(e) = self.producer.send_rejected(&error).await {
                                            error!("Failed to send rejected CDR: {}", e);
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Failed to deserialize ProcessedCDR: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Kafka consumer error: {:?}", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }
        }
    }
}
