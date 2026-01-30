use serde::{Deserialize, Serialize};

/// Simple logistic regression model for fraud detection
/// 100% Rust native - no external ML dependencies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogisticRegressionModel {
    pub weights: Vec<f32>,  // [16] coefficients
    pub intercept: f32,      // bias term
}

impl LogisticRegressionModel {
    /// Load model weights from JSON file
    pub fn from_json(path: &str) -> anyhow::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let model: LogisticRegressionModel = serde_json::from_str(&contents)?;
        Ok(model)
    }
    
    /// Predict fraud probability for a single sample
    /// Returns value in [0, 1]
    pub fn predict(&self, features: &[f32]) -> f32 {
        if features.len() != self.weights.len() {
            return 0.5; // Default to uncertain
        }
        
        // Compute dot product: w · x + b
        let linear_score: f32 = features
            .iter()
            .zip(&self.weights)
            .map(|(x, w)| x * w)
            .sum::<f32>() + self.intercept;
        
        // Apply sigmoid: 1 / (1 + exp(-score))
        sigmoid(linear_score)
    }
    
    /// Batch prediction
    pub fn predict_batch(&self, features_batch: &[Vec<f32>]) -> Vec<f32> {
        features_batch
            .iter()
            .map(|features| self.predict(features))
            .collect()
    }
}

/// Sigmoid activation function
#[inline]
fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sigmoid() {
        assert!((sigmoid(0.0) - 0.5).abs() < 0.001);
        assert!(sigmoid(10.0) > 0.99);
        assert!(sigmoid(-10.0) < 0.01);
    }
    
    #[test]
    fn test_prediction() {
        let model = LogisticRegressionModel {
            weights: vec![1.0, -1.0, 0.5, -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            intercept: 0.0,
        };
        
        let features = vec![1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let prob = model.predict(&features);
        
        // Should be 0.5 since weights cancel out
        assert!((prob - 0.5).abs() < 0.1);
    }
    
    #[test]
    fn test_batch_prediction() {
        let model = LogisticRegressionModel {
            weights: vec![1.0; 16],
            intercept: 0.0,
        };
        
        let batch = vec![
            vec![0.1; 16],
            vec![0.5; 16],
            vec![0.9; 16],
        ];
        
        let predictions = model.predict_batch(&batch);
        assert_eq!(predictions.len(), 3);
        
        // Higher input should give higher probability
        assert!(predictions[0] < predictions[1]);
        assert!(predictions[1] < predictions[2]);
    }
}
