use crate::config::ModelConfig;
use crate::features::{FraudFeatures, FraudPrediction};
use crate::metrics;
use anyhow::Result;
use tracing::info;

/// Rule-based fraud detection model (ONNX integration planned)
pub struct FraudDetector {
    threshold: f32,
    batch_size: usize,
    model_type: String,
}

impl FraudDetector {
    /// Create a new fraud detector
    /// For now uses rule-based scoring, will integrate ONNX model later
    pub async fn new(config: &ModelConfig) -> Result<Self> {
        info!("Initializing fraud detector (rule-based mode)");
        info!("Model path configured: {} (not loaded yet - using rules)", config.path);
        info!("Fraud threshold: {}", config.threshold);
        
        metrics::record_model_load(true);
        
        Ok(Self {
            threshold: config.threshold,
            batch_size: config.batch_size,
            model_type: "Rule-based (ONNX integration planned)".to_string(),
        })
    }

    /// Predict fraud for a single CDR
    pub async fn predict(&self, features: &FraudFeatures) -> Result<FraudPrediction> {
        let start = std::time::Instant::now();
        
        // For now, use a simple rule-based score as fallback
        // This will be replaced by actual ONNX inference once we have the model
        let fraud_score = self.calculate_fallback_score(features);
        
        let is_fraud = fraud_score > self.threshold;
        let confidence = if is_fraud {
            fraud_score
        } else {
            1.0 - fraud_score
        };
        
        let inference_time_ms = start.elapsed().as_secs_f32() * 1000.0;
        
        // Record metrics
        metrics::record_prediction(is_fraud, fraud_score, start.elapsed().as_secs_f64());
        
        Ok(FraudPrediction {
            cdr_id: features.cdr_id.clone(),
            fraud_score,
            is_fraud,
            confidence,
            inference_time_ms,
        })
    }

    /// Predict fraud for a batch of CDRs
    pub async fn predict_batch(&self, features_batch: &[FraudFeatures]) -> Result<Vec<FraudPrediction>> {
        let start = std::time::Instant::now();
        
        if features_batch.is_empty() {
            return Ok(Vec::new());
        }
        
        // For batch processing, predict each individually for now
        // TODO: Implement true batch inference with ONNX
        let mut predictions = Vec::with_capacity(features_batch.len());
        
        for features in features_batch {
            let prediction = self.predict(features).await?;
            predictions.push(prediction);
        }
        
        info!(
            "Batch prediction completed: {} samples in {:.2}ms",
            features_batch.len(),
            start.elapsed().as_secs_f32() * 1000.0
        );
        
        Ok(predictions)
    }

    /// Calculate fallback fraud score using rule-based approach
    /// This is temporary until we have a trained ONNX model
    fn calculate_fallback_score(&self, features: &FraudFeatures) -> f32 {
        let mut score: f32 = 0.0;
        
        // High risk: International + roaming + premium
        if features.is_international > 0.5 && features.is_roaming > 0.5 {
            score += 0.3;
        }
        
        // High risk: Night calls with high frequency
        if features.is_night_call > 0.5 && features.call_frequency_per_hour > 2.0 {
            score += 0.25;
        }
        
        // High risk: Abnormal duration
        if features.duration_zscore.abs() > 2.0 {
            score += 0.2;
        }
        
        // High risk: Abnormal cost
        if features.cost_zscore.abs() > 2.5 {
            score += 0.25;
        }
        
        // High risk: Many cell tower changes
        if features.cell_tower_changes > 5.0 {
            score += 0.15;
        }
        
        // High risk: Low signal strength + international
        if features.signal_strength < 0.3 && features.is_international > 0.5 {
            score += 0.1;
        }
        
        // Clamp to [0, 1]
        score.min(1.0).max(0.0)
    }

    /// Get model information
    pub async fn model_info(&self) -> ModelInfo {
        ModelInfo {
            threshold: self.threshold,
            batch_size: self.batch_size,
            feature_count: FraudFeatures::FEATURE_COUNT,
            model_type: self.model_type.clone(),
        }
    }
}

/// Model metadata
#[derive(Debug, serde::Serialize)]
pub struct ModelInfo {
    pub threshold: f32,
    pub batch_size: usize,
    pub feature_count: usize,
    pub model_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fallback_score_high_risk() {
        let config = ModelConfig {
            path: "./models/test.onnx".to_string(),
            threshold: 0.5,
            batch_size: 32,
            enable_cuda: false,
        };
        
        let detector = FraudDetector::new(&config).await.unwrap();
        
        let features = FraudFeatures {
            cdr_id: "test-high-risk".to_string(),
            duration_seconds: 3600.0,
            is_international: 1.0,
            is_premium: 1.0,
            is_roaming: 1.0,
            hour_of_day: 2.0,
            day_of_week: 6.0,
            is_weekend: 1.0,
            is_night_call: 1.0,
            daily_call_count: 50.0,
            daily_call_duration: 5000.0,
            unique_destinations_count: 30.0,
            call_frequency_per_hour: 5.0,
            cell_tower_changes: 10.0,
            signal_strength: 0.1,
            duration_zscore: 3.5,
            cost_zscore: 4.0,
        };
        
        let score = detector.calculate_fallback_score(&features);
        assert!(score > 0.7, "High risk CDR should have high fraud score");
    }

    #[tokio::test]
    async fn test_fallback_score_low_risk() {
        let config = ModelConfig {
            path: "./models/test.onnx".to_string(),
            threshold: 0.5,
            batch_size: 32,
            enable_cuda: false,
        };
        
        let detector = FraudDetector::new(&config).await.unwrap();
        
        let features = FraudFeatures {
            cdr_id: "test-low-risk".to_string(),
            duration_seconds: 120.0,
            is_international: 0.0,
            is_premium: 0.0,
            is_roaming: 0.0,
            hour_of_day: 14.0,
            day_of_week: 2.0,
            is_weekend: 0.0,
            is_night_call: 0.0,
            daily_call_count: 5.0,
            daily_call_duration: 600.0,
            unique_destinations_count: 3.0,
            call_frequency_per_hour: 0.5,
            cell_tower_changes: 1.0,
            signal_strength: 0.8,
            duration_zscore: 0.2,
            cost_zscore: 0.1,
        };
        
        let score = detector.calculate_fallback_score(&features);
        assert!(score < 0.3, "Low risk CDR should have low fraud score");
    }
}
