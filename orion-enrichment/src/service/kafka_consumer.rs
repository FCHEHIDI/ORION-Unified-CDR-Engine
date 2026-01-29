use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use crate::config::{KafkaConfig, EnrichmentConfig};
use crate::service::model::UnifiedCDR;
use crate::service::enricher::Enricher;
use crate::service::kafka_producer::KafkaProducerService;
use tracing::{info, error};

pub struct KafkaConsumerService {
    consumer: StreamConsumer,
    enricher: Enricher,
    producer: KafkaProducerService,
}

impl KafkaConsumerService {
    pub fn new(kafka_config: &KafkaConfig, enrichment_config: EnrichmentConfig) -> anyhow::Result<Self> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", &kafka_config.brokers)
            .set("group.id", &kafka_config.consumer_group)
            .set("enable.auto.commit", "true")
            .set("auto.offset.reset", "earliest")
            .create()?;

        consumer.subscribe(&[&kafka_config.input_topic])?;

        let producer = KafkaProducerService::new(
            &kafka_config.brokers,
            kafka_config.output_topic.clone(),
        )?;

        Ok(Self {
            consumer,
            enricher: Enricher::new(enrichment_config),
            producer,
        })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        info!("Kafka consumer service started");

        loop {
            match self.consumer.recv().await {
                Ok(message) => {
                    if let Some(payload) = message.payload() {
                        match serde_json::from_slice::<UnifiedCDR>(payload) {
                            Ok(unified_cdr) => {
                                info!(
                                    "Received CDR {} (event: {}, country: {})",
                                    unified_cdr.cdr_id,
                                    unified_cdr.event_type,
                                    unified_cdr.country_code
                                );
                                
                                match self.enricher.enrich(unified_cdr).await {
                                    Ok(enriched_cdr) => {
                                        if let Some(ref fraud) = enriched_cdr.fraud_info {
                                            info!(
                                                "Fraud check: score={:.2}, risk={}",
                                                fraud.fraud_score,
                                                fraud.risk_level
                                            );
                                        }
                                        
                                        if let Err(e) = self.producer.send(&enriched_cdr).await {
                                            error!("Failed to send enriched CDR: {}", e);
                                        }
                                    }
                                    Err(e) => {
                                        error!("Failed to enrich CDR: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Failed to deserialize UnifiedCDR: {}", e);
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
