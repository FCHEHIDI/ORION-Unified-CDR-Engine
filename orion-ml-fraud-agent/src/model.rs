use crate::config::ModelConfig;
use crate::features::{FraudFeatures, FraudPrediction};
use crate::metrics;
use crate::simple_ml::LogisticRegressionModel;
use anyhow::Result;
use std::sync::Arc;
use tracing::{info, warn};

/// Fraud detection model
pub struct FraudDetector {
    model: Option<Arc<LogisticRegressionModel>>,
    threshold: f32,
    batch_size: usize,
    model_type: String,
    use_fallback: bool,
}

impl FraudDetector {
    /// Create a new fraud detector
    pub async fn new(config: &ModelConfig) -> Result<Self> {
        info!("Initializing fraud detector");
        info!("Model path: {}", config.path);
        info!("Fraud threshold: {}", config.threshold);
        
        // Try to load simple ML model from JSON weights
        let (model, use_fallback) = match LogisticRegressionModel::from_json(&config.path) {
            Ok(model) => {
                info!("✅ ML model loaded successfully from {}", config.path);
                metrics::record_model_load(true);
                (Some(Arc::new(model)), false)
            }
            Err(e) => {
                warn!("⚠️  Failed to load ML model: {}. Using rule-based fallback.", e);
                warn!("Expected model weights at: {}", config.path);
                metrics::record_model_load(false);
                (None, true)
            }
        };
        
        let model_type = if use_fallback {
            "Rule-based fallback".to_string()
        } else {
            "Logistic Regression (native Rust)".to_string()
        };
        
        Ok(Self {
            model,
            threshold: config.threshold,
            batch_size: config.batch_size,
            model_type,
            use_fallback,
        })
    }

    /// Predict fraud for a single CDR
    pub async fn predict(&self, features: &FraudFeatures) -> Result<FraudPrediction> {
        let start = std::time::Instant::now();
        
        let fraud_score = if self.use_fallback {
            // Fallback to rule-based scoring
            self.calculate_fallback_score(features)
        } else {
            // Use simple ML model inference
            self.predict_with_ml(features)?
        };
        
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
    
    /// Perform ML inference for a single sample using Burn
    fn predict_with_ml(&self, features: &FraudFeatures) -> Result<f32> {
        let model = self.model.as_ref()
            .ok_or_else(|| anyhow::anyhow!("No ML model available"))?;
        
        // Convert features to array
        let feature_array = features.to_array();
        
        // Run inference using Burn
        let fraud_score = model.predict(&feature_array);
        
        Ok(fraud_score.max(0.0).min(1.0))
    }

    /// Predict fraud for a batch of CDRs
    pub async fn predict_batch(&self, features_batch: &[FraudFeatures]) -> Result<Vec<FraudPrediction>> {
        let start = std::time::Instant::now();
        
        if features_batch.is_empty() {
            return Ok(Vec::new());
        }
        
        let mut predictions = Vec::with_capacity(features_batch.len());
        
        if self.use_fallback {
            // Fallback: predict each individually
            for features in features_batch {
                let prediction = self.predict(features).await?;
                predictions.push(prediction);
            }
        } else {
            // Burn ML batch inference
            let model = self.model.as_ref()
                .ok_or_else(|| anyhow::anyhow!("No ML model available"))?;
            
            // Convert features to batch array
            let feature_arrays: Vec<Vec<f32>> = features_batch
                .iter()
                .map(|f| f.to_array())
                .collect();
            
            // Run batch inference
            let scores = model.predict_batch(&feature_arrays);
            
            // Process each prediction
            for (features, fraud_score) in features_batch.iter().zip(scores.iter()) {
                let fraud_score = fraud_score.max(0.0).min(1.0);
                let is_fraud = fraud_score > self.threshold;
                let confidence = if is_fraud { fraud_score } else { 1.0 - fraud_score };
                
                predictions.push(FraudPrediction {
                    cdr_id: features.cdr_id.clone(),
                    fraud_score,
                    is_fraud,
                    confidence,
                    inference_time_ms: 0.0, // Set below
                });
            }
        }
        
        let total_time_ms = start.elapsed().as_secs_f32() * 1000.0;
        let avg_time_ms = total_time_ms / features_batch.len() as f32;
        
        // Update inference time for all predictions
        for pred in &mut predictions {
            pred.inference_time_ms = avg_time_ms;
        }
        
        info!(
            "Batch prediction completed: {} samples in {:.2}ms ({:.2}ms avg per sample)",
            features_batch.len(),
            total_time_ms,
            avg_time_ms
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
