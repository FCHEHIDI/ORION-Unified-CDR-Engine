use metrics::{counter, histogram};

pub fn init_metrics() {
    // Counters
    counter!("orion_storage_messages_total");
    counter!("orion_storage_errors_total");
    counter!("orion_storage_inserted_total");

    // Histograms
    histogram!("orion_storage_latency_seconds");
}

pub fn increment_messages_total() {
    counter!("orion_storage_messages_total").increment(1);
}

pub fn increment_errors_total() {
    counter!("orion_storage_errors_total").increment(1);
}

pub fn increment_inserted_total() {
    counter!("orion_storage_inserted_total").increment(1);
}

pub fn record_latency(duration: f64) {
    histogram!("orion_storage_latency_seconds").record(duration);
}
