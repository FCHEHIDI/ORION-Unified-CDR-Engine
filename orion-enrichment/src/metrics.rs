use metrics::{describe_counter, describe_histogram, counter, histogram};

pub fn init_metrics() {
    describe_counter!(
        "orion_enrichment_messages_total",
        "Total number of CDR messages enriched"
    );
    
    describe_counter!(
        "orion_enrichment_errors_total",
        "Total number of enrichment errors"
    );
    
    describe_counter!(
        "orion_enrichment_fraud_detected_total",
        "Total number of fraud cases detected"
    );
    
    describe_histogram!(
        "orion_enrichment_latency_seconds",
        "Enrichment latency in seconds"
    );
}

pub fn increment_messages_total() {
    counter!("orion_enrichment_messages_total").increment(1);
}

pub fn increment_fraud_detected_total() {
    counter!("orion_enrichment_fraud_detected_total").increment(1);
}

pub fn record_latency(duration: f64) {
    histogram!("orion_enrichment_latency_seconds").record(duration);
}
