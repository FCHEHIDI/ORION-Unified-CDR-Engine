use crate::config::KafkaConfig;
use crate::service::{MessageProcessor, ProcessedCDR};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use rdkafka::error::KafkaError;
use tokio::sync::mpsc;
use tracing::{info, error, warn};
use std::time::Duration;

pub struct KafkaConsumerService {
    consumer: StreamConsumer,
    processor: MessageProcessor,
    output_tx: mpsc::Sender<ProcessedCDR>,
}

impl KafkaConsumerService {
    pub fn new(
        config: &KafkaConfig,
        output_tx: mpsc::Sender<ProcessedCDR>,
    ) -> Result<Self, KafkaError> {
        info!("Initializing Kafka consumer with brokers: {}", config.brokers);
        info!("Topics: {:?}", config.topics);

        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", &config.brokers)
            .set("group.id", &config.group_id)
            .set("enable.auto.commit", "true")
            .set("auto.offset.reset", &config.auto_offset_reset)
            .set("session.timeout.ms", "6000")
            .set("enable.partition.eof", "false")
            .create()?;

        // Subscribe to topics
        let topics: Vec<&str> = config.topics.iter().map(|s| s.as_str()).collect();
        consumer.subscribe(&topics)?;

        info!("Successfully subscribed to topics: {:?}", topics);

        Ok(Self {
            consumer,
            processor: MessageProcessor::new(),
            output_tx,
        })
    }

    /// Run the consumer loop
    /// This is the main processing loop that consumes messages from Kafka
    pub async fn run(&self) -> anyhow::Result<()> {
        info!("Starting Kafka consumer loop...");

        loop {
            match self.consumer.recv().await {
                Ok(message) => {
                    let topic = message.topic();
                    let partition = message.partition();
                    let offset = message.offset();

                    info!(
                        "Received message from topic: {}, partition: {}, offset: {}",
                        topic, partition, offset
                    );

                    if let Some(payload) = message.payload() {
                        // Process the message
                        if let Some(processed) = self.processor.process(payload, topic).await {
                            // Send to output channel (for next service: validation)
                            if let Err(e) = self.output_tx.send(processed).await {
                                error!("Failed to send processed CDR to output channel: {:?}", e);
                            }
                        } else {
                            warn!("Failed to process message from topic: {}", topic);
                        }
                    } else {
                        warn!("Received message with no payload");
                    }
                }
                Err(e) => {
                    error!("Kafka error: {:?}", e);
                    // Don't exit on error, just log and continue
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }
}
