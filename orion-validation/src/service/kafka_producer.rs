use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use crate::service::model::{ValidatedCDR, ValidationError};
use std::time::Duration;
use tracing::{info, error};

pub struct KafkaProducerService {
    producer: FutureProducer,
    output_topic: String,
    rejected_topic: String,
}

impl KafkaProducerService {
    pub fn new(brokers: &str, output_topic: String, rejected_topic: String) -> anyhow::Result<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()?;

        Ok(Self {
            producer,
            output_topic,
            rejected_topic,
        })
    }

    pub async fn send_valid(&self, cdr: &ValidatedCDR) -> anyhow::Result<()> {
        let payload = serde_json::to_string(cdr)?;
        let key = cdr.cdr_id.as_bytes();

        let record = FutureRecord::to(&self.output_topic)
            .key(key)
            .payload(&payload);

        match self.producer.send(record, Timeout::After(Duration::from_secs(5))).await {
            Ok((partition, offset)) => {
                info!(
                    "Sent valid CDR {} to {} (partition: {}, offset: {})",
                    cdr.cdr_id, self.output_topic, partition, offset
                );
                Ok(())
            }
            Err((e, _)) => {
                error!("Failed to send valid CDR: {:?}", e);
                Err(anyhow::anyhow!("Kafka send error: {:?}", e))
            }
        }
    }

    pub async fn send_rejected(&self, error: &ValidationError) -> anyhow::Result<()> {
        let payload = serde_json::to_string(error)?;
        let key = error.timestamp.as_bytes();

        let record = FutureRecord::to(&self.rejected_topic)
            .key(key)
            .payload(&payload);

        match self.producer.send(record, Timeout::After(Duration::from_secs(5))).await {
            Ok((partition, offset)) => {
                info!(
                    "Sent rejected CDR to {} (partition: {}, offset: {})",
                    self.rejected_topic, partition, offset
                );
                Ok(())
            }
            Err((e, _)) => {
                error!("Failed to send rejected CDR: {:?}", e);
                Err(anyhow::anyhow!("Kafka send error: {:?}", e))
            }
        }
    }
}
