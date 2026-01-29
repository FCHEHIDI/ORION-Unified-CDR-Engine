use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use crate::service::model::EnrichedCDR;
use std::time::Duration;
use tracing::{info, error};

pub struct KafkaProducerService {
    producer: FutureProducer,
    output_topic: String,
}

impl KafkaProducerService {
    pub fn new(brokers: &str, output_topic: String) -> anyhow::Result<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()?;

        Ok(Self {
            producer,
            output_topic,
        })
    }

    pub async fn send(&self, cdr: &EnrichedCDR) -> anyhow::Result<()> {
        let payload = serde_json::to_string(cdr)?;
        let key = cdr.unified.cdr_id.as_bytes();

        let record = FutureRecord::to(&self.output_topic)
            .key(key)
            .payload(&payload);

        match self.producer.send(record, Timeout::After(Duration::from_secs(5))).await {
            Ok((partition, offset)) => {
                info!(
                    "Sent enriched CDR {} to {} (partition: {}, offset: {})",
                    cdr.unified.cdr_id, self.output_topic, partition, offset
                );
                Ok(())
            }
            Err((e, _)) => {
                error!("Failed to send enriched CDR: {:?}", e);
                Err(anyhow::anyhow!("Kafka send error: {:?}", e))
            }
        }
    }
}
