use metrics::{counter, describe_counter, describe_histogram, histogram};

/// Initialize all metrics with descriptions
pub fn init_metrics() {
    // Prediction metrics
    describe_counter!(
        "ml_fraud_predictions_total",
        "Total number of fraud predictions made"
    );
    
    describe_counter!(
        "ml_fraud_detections_total",
        "Total number of fraud cases detected (score > threshold)"
    );
    
    describe_histogram!(
        "ml_fraud_prediction_duration_seconds",
        "Duration of fraud prediction in seconds"
    );
    
    describe_histogram!(
        "ml_fraud_score_distribution",
        "Distribution of fraud scores"
    );
    
    // Model metrics
    describe_counter!(
        "ml_fraud_model_loads_total",
        "Total number of model load attempts"
    );
    
    describe_counter!(
        "ml_fraud_model_load_errors_total",
        "Total number of model load errors"
    );
    
    // Feature extraction metrics
    describe_counter!(
        "ml_fraud_feature_extraction_errors_total",
        "Total number of feature extraction errors"
    );
    
    describe_histogram!(
        "ml_fraud_feature_extraction_duration_seconds",
        "Duration of feature extraction in seconds"
    );
}

/// Record a prediction
pub fn record_prediction(fraud_detected: bool, score: f32, duration_secs: f64) {
    counter!("ml_fraud_predictions_total").increment(1);
    
    if fraud_detected {
        counter!("ml_fraud_detections_total").increment(1);
    }
    
    histogram!("ml_fraud_prediction_duration_seconds").record(duration_secs);
    histogram!("ml_fraud_score_distribution").record(score as f64);
}

/// Record model load attempt
pub fn record_model_load(success: bool) {
    counter!("ml_fraud_model_loads_total").increment(1);
    
    if !success {
        counter!("ml_fraud_model_load_errors_total").increment(1);
    }
}

/// Record feature extraction error
#[allow(dead_code)]
pub fn record_feature_extraction_error() {
    counter!("ml_fraud_feature_extraction_errors_total").increment(1);
}

/// Record feature extraction duration
#[allow(dead_code)]
pub fn record_feature_extraction_duration(duration_secs: f64) {
    histogram!("ml_fraud_feature_extraction_duration_seconds").record(duration_secs);
}
