use serde::{Deserialize, Serialize};

/// Input features for fraud detection model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudFeatures {
    // CDR identifier
    pub cdr_id: String,
    
    // Call characteristics
    pub duration_seconds: f32,
    pub is_international: f32,  // 0.0 or 1.0
    pub is_premium: f32,        // 0.0 or 1.0
    pub is_roaming: f32,        // 0.0 or 1.0
    
    // Time features
    pub hour_of_day: f32,       // 0-23
    pub day_of_week: f32,       // 0-6
    pub is_weekend: f32,        // 0.0 or 1.0
    pub is_night_call: f32,     // 0.0 or 1.0 (22h-6h)
    
    // User behavior features
    pub daily_call_count: f32,
    pub daily_call_duration: f32,
    pub unique_destinations_count: f32,
    pub call_frequency_per_hour: f32,
    
    // Network features
    pub cell_tower_changes: f32,
    pub signal_strength: f32,   // -120 to -20 dBm normalized
    
    // Statistical features
    pub duration_zscore: f32,   // Z-score of duration
    pub cost_zscore: f32,       // Z-score of cost
}

impl FraudFeatures {
    /// Convert features to array for ONNX input (currently unused - for future ONNX integration)
    #[allow(dead_code)]
    pub fn to_array(&self) -> Vec<f32> {
        vec![
            self.duration_seconds,
            self.is_international,
            self.is_premium,
            self.is_roaming,
            self.hour_of_day,
            self.day_of_week,
            self.is_weekend,
            self.is_night_call,
            self.daily_call_count,
            self.daily_call_duration,
            self.unique_destinations_count,
            self.call_frequency_per_hour,
            self.cell_tower_changes,
            self.signal_strength,
            self.duration_zscore,
            self.cost_zscore,
        ]
    }
    
    /// Number of features (for model validation)
    pub const FEATURE_COUNT: usize = 16;
}

/// Fraud prediction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudPrediction {
    pub cdr_id: String,
    pub fraud_score: f32,
    pub is_fraud: bool,
    pub confidence: f32,
    pub inference_time_ms: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_features_to_array() {
        let features = FraudFeatures {
            cdr_id: "test-123".to_string(),
            duration_seconds: 120.0,
            is_international: 1.0,
            is_premium: 0.0,
            is_roaming: 1.0,
            hour_of_day: 14.0,
            day_of_week: 3.0,
            is_weekend: 0.0,
            is_night_call: 0.0,
            daily_call_count: 5.0,
            daily_call_duration: 600.0,
            unique_destinations_count: 3.0,
            call_frequency_per_hour: 0.5,
            cell_tower_changes: 2.0,
            signal_strength: 0.7,
            duration_zscore: 1.5,
            cost_zscore: 2.0,
        };

        let array = features.to_array();
        assert_eq!(array.len(), FraudFeatures::FEATURE_COUNT);
        assert_eq!(array[0], 120.0);
        assert_eq!(array[1], 1.0);
    }

    #[test]
    fn test_feature_count_constant() {
        assert_eq!(FraudFeatures::FEATURE_COUNT, 16);
    }
}
