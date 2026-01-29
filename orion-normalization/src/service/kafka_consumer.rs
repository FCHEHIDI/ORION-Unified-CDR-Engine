use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use crate::config::KafkaConfig;
use crate::service::model::ValidatedCDR;
use crate::service::normalizer::Normalizer;
use crate::service::kafka_producer::KafkaProducerService;
use tracing::{info, error};

pub struct KafkaConsumerService {
    consumer: StreamConsumer,
    normalizer: Normalizer,
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
        )?;

        Ok(Self {
            consumer,
            normalizer: Normalizer::new(),
            producer,
        })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        info!("Kafka consumer service started");

        loop {
            match self.consumer.recv().await {
                Ok(message) => {
                    if let Some(payload) = message.payload() {
                        match serde_json::from_slice::<ValidatedCDR>(payload) {
                            Ok(validated_cdr) => {
                                info!(
                                    "Received validated CDR {} (event: {:?}, country: {})",
                                    validated_cdr.cdr_id,
                                    validated_cdr.event_type,
                                    validated_cdr.country
                                );
                                
                                match self.normalizer.normalize(&validated_cdr).await {
                                    Ok(unified_cdr) => {
                                        if let Err(e) = self.producer.send(&unified_cdr).await {
                                            error!("Failed to send normalized CDR: {}", e);
                                        }
                                    }
                                    Err(e) => {
                                        error!("Failed to normalize CDR {}: {}", validated_cdr.cdr_id, e);
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Failed to deserialize ValidatedCDR: {}", e);
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
