use metrics::{describe_counter, describe_histogram, counter, histogram};

pub fn init_metrics() {
    describe_counter!(
        "orion_normalization_messages_total",
        "Total number of CDR messages normalized"
    );
    
    describe_counter!(
        "orion_normalization_errors_total",
        "Total number of normalization errors"
    );
    
    describe_counter!(
        "orion_normalization_voice_total",
        "Total voice CDRs normalized"
    );
    
    describe_counter!(
        "orion_normalization_data_total",
        "Total data CDRs normalized"
    );
    
    describe_counter!(
        "orion_normalization_sms_total",
        "Total SMS CDRs normalized"
    );
    
    describe_histogram!(
        "orion_normalization_latency_seconds",
        "Normalization latency in seconds"
    );
}

pub fn increment_messages_total() {
    counter!("orion_normalization_messages_total").increment(1);
}

pub fn increment_voice_total() {
    counter!("orion_normalization_voice_total").increment(1);
}

pub fn increment_data_total() {
    counter!("orion_normalization_data_total").increment(1);
}

pub fn increment_sms_total() {
    counter!("orion_normalization_sms_total").increment(1);
}

pub fn record_latency(duration: f64) {
    histogram!("orion_normalization_latency_seconds").record(duration);
}
