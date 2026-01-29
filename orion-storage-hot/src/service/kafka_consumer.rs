use crate::config::KafkaConfig;
use crate::metrics;
use crate::service::{model::EnrichedCDR, ScyllaRepository};
use anyhow::Result;
use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    ClientConfig, Message,
};
use std::sync::Arc;
use std::time::Instant;
use tracing;

pub struct KafkaConsumerService {
    consumer: StreamConsumer,
    repository: Arc<ScyllaRepository>,
}

impl KafkaConsumerService {
    pub fn new(kafka_config: &KafkaConfig, repository: Arc<ScyllaRepository>) -> Result<Self> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", kafka_config.brokers.join(","))
            .set("group.id", &kafka_config.consumer_group)
            .set("enable.auto.commit", "true")
            .set("auto.offset.reset", "earliest")
            .create()?;

        consumer.subscribe(&[&kafka_config.input_topic])?;

        tracing::info!(
            "Kafka consumer subscribed to topic: {}",
            kafka_config.input_topic
        );

        Ok(Self {
            consumer,
            repository,
        })
    }

    pub async fn run(&self) -> Result<()> {
        tracing::info!("Starting Kafka consumer loop");

        loop {
            match self.consumer.recv().await {
                Ok(message) => {
                    let start = Instant::now();
                    metrics::increment_messages_total();

                    if let Err(e) = self.process_message(&message).await {
                        tracing::error!("Failed to process message: {}", e);
                        metrics::increment_errors_total();
                    } else {
                        metrics::record_latency(start.elapsed().as_secs_f64());
                    }
                }
                Err(e) => {
                    tracing::error!("Kafka consumer error: {}", e);
                    metrics::increment_errors_total();
                }
            }
        }
    }

    async fn process_message(&self, message: &rdkafka::message::BorrowedMessage<'_>) -> Result<()> {
        let payload = message
            .payload()
            .ok_or_else(|| anyhow::anyhow!("Empty message payload"))?;

        let enriched: EnrichedCDR = serde_json::from_slice(payload)?;

        tracing::debug!(
            "Processing CDR: {} (event_type: {})",
            enriched.unified.cdr_id,
            enriched.unified.event_type
        );

        // Insert into ScyllaDB
        self.repository.insert_cdr(&enriched).await?;
        metrics::increment_inserted_total();

        tracing::info!(
            "CDR {} stored successfully in ScyllaDB",
            enriched.unified.cdr_id
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enriched_cdr_deserialization() {
        let json = r#"{
            "unified": {
                "cdr_id": "123",
                "event_type": "voice",
                "imsi": "208150123456789",
                "msisdn": "+33612345678",
                "country": "FR",
                "start_timestamp": "2024-01-15T10:30:00Z",
                "is_roaming": false,
                "hash": "abc123",
                "ingestion_timestamp": "2024-01-15T10:30:00Z",
                "normalization_timestamp": "2024-01-15T10:30:05Z"
            },
            "fraud_info": null,
            "network_info": null,
            "client_info": null,
            "enrichment_timestamp": "2024-01-15T10:30:10Z",
            "enrichment_version": "v1.0.0"
        }"#;

        let enriched: Result<EnrichedCDR, _> = serde_json::from_str(json);
        assert!(enriched.is_ok());

        let cdr = enriched.unwrap();
        assert_eq!(cdr.unified.cdr_id, "123");
        assert_eq!(cdr.unified.event_type, "voice");
    }
}
