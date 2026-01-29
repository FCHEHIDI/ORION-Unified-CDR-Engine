use metrics::{counter, histogram};

pub fn init_metrics() {
    let _ = counter!("traffic_generator_cdr_generated_total");
    let _ = counter!("traffic_generator_cdr_sent_total");
    let _ = counter!("traffic_generator_errors_total");
    let _ = counter!("traffic_generator_retries_total");
    let _ = counter!("traffic_generator_malformed_total");
    let _ = counter!("traffic_generator_fraud_total");
    let _ = histogram!("traffic_generator_latency_seconds");
    let _ = histogram!("traffic_generator_kafka_latency_seconds");
}

pub fn increment_cdr_generated() {
    counter!("traffic_generator_cdr_generated_total").increment(1);
}

pub fn increment_cdr_sent() {
    counter!("traffic_generator_cdr_sent_total").increment(1);
}

pub fn increment_errors() {
    counter!("traffic_generator_errors_total").increment(1);
}

pub fn increment_retries() {
    counter!("traffic_generator_retries_total").increment(1);
}

pub fn increment_malformed() {
    counter!("traffic_generator_malformed_total").increment(1);
}

pub fn increment_fraud() {
    counter!("traffic_generator_fraud_total").increment(1);
}

pub fn record_latency(duration: f64) {
    histogram!("traffic_generator_latency_seconds").record(duration);
}

pub fn record_kafka_latency(duration: f64) {
    histogram!("traffic_generator_kafka_latency_seconds").record(duration);
}
