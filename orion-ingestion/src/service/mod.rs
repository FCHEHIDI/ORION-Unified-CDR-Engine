mod kafka_consumer;
mod processor;
mod model;

pub use kafka_consumer::KafkaConsumerService;
pub use processor::MessageProcessor;
pub use model::ProcessedCDR;
