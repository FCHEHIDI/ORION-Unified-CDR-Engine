pub mod kafka_consumer;
pub mod processor;
pub mod model;

pub use kafka_consumer::KafkaConsumerService;
pub use processor::MessageProcessor;
pub use model::{RawCDR, ProcessedCDR};
