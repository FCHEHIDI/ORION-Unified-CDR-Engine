use metrics::{describe_counter, describe_histogram, counter, histogram};

pub fn init_metrics() {
    describe_counter!(
        "orion_validation_messages_total",
        "Total number of CDR messages processed"
    );
    
    describe_counter!(
        "orion_validation_valid_total",
        "Total number of valid CDR messages"
    );
    
    describe_counter!(
        "orion_validation_invalid_total",
        "Total number of invalid CDR messages"
    );
    
    describe_counter!(
        "orion_validation_errors_total",
        "Total number of validation errors"
    );
    
    describe_histogram!(
        "orion_validation_latency_seconds",
        "Validation latency in seconds"
    );
}

pub fn increment_messages_total() {
    counter!("orion_validation_messages_total").increment(1);
}

pub fn increment_valid_total() {
    counter!("orion_validation_valid_total").increment(1);
}

pub fn increment_invalid_total() {
    counter!("orion_validation_invalid_total").increment(1);
}

pub fn record_latency(duration: f64) {
    histogram!("orion_validation_latency_seconds").record(duration);
}
