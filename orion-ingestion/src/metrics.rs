use metrics::{counter, histogram, describe_counter, describe_histogram};

pub fn init_metrics() {
    describe_counter!(
        "orion_ingestion_messages_total",
        "Total number of messages consumed from Kafka"
    );
    describe_counter!(
        "orion_ingestion_errors_total",
        "Total number of ingestion errors"
    );
    describe_counter!(
        "orion_ingestion_bytes_total",
        "Total bytes ingested"
    );
    describe_histogram!(
        "orion_ingestion_latency_seconds",
        "Ingestion processing latency in seconds"
    );
}

pub fn record_message_consumed() {
    counter!("orion_ingestion_messages_total").increment(1);
}

pub fn record_error() {
    counter!("orion_ingestion_errors_total").increment(1);
}

pub fn record_bytes(bytes: u64) {
    counter!("orion_ingestion_bytes_total").increment(bytes);
}

pub fn record_latency(duration_secs: f64) {
    histogram!("orion_ingestion_latency_seconds").record(duration_secs);
}
