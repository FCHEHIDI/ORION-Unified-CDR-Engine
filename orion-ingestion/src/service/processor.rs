use crate::service::model::{RawCDR, ProcessedCDR};
use crate::metrics;
use tracing::{info, error};
use std::time::Instant;

pub struct MessageProcessor;

impl MessageProcessor {
    pub fn new() -> Self {
        Self
    }

    /// Process a raw message from Kafka
    /// Returns ProcessedCDR or None if parsing fails
    pub async fn process(&self, payload: &[u8], topic: &str) -> Option<ProcessedCDR> {
        let start = Instant::now();
        
        // Record bytes ingested
        metrics::record_bytes(payload.len() as u64);
        
        // Try to parse as JSON first
        let raw_cdr = match serde_json::from_slice::<serde_json::Value>(payload) {
            Ok(json) => {
                info!("Parsed JSON CDR from topic: {}", topic);
                RawCDR::Json(json)
            }
            Err(_) => {
                // Fallback to text
                match String::from_utf8(payload.to_vec()) {
                    Ok(text) => {
                        info!("Parsed text CDR from topic: {}", topic);
                        RawCDR::Text(text)
                    }
                    Err(e) => {
                        error!("Failed to parse message as UTF-8: {:?}", e);
                        metrics::record_error();
                        return None;
                    }
                }
            }
        };

        let processed = ProcessedCDR::new(raw_cdr, topic);
        
        // Record latency
        let duration = start.elapsed();
        metrics::record_latency(duration.as_secs_f64());
        metrics::record_message_consumed();
        
        info!(
            "Processed CDR from {} (country: {}) in {:?}",
            topic, processed.country, duration
        );
        
        Some(processed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_process_json_message() {
        let processor = MessageProcessor::new();
        let json_payload = r#"{"imsi": "208150123456789", "msisdn": "+33612345678"}"#;
        
        let result = processor.process(json_payload.as_bytes(), "cdr.raw.FR").await;
        
        assert!(result.is_some());
        let processed = result.unwrap();
        assert_eq!(processed.country, "FR");
        assert!(processed.raw_payload.contains("imsi"));
    }

    #[tokio::test]
    async fn test_process_text_message() {
        let processor = MessageProcessor::new();
        let text_payload = "208150123456789;+33612345678;data";
        
        let result = processor.process(text_payload.as_bytes(), "cdr.raw.TN").await;
        
        assert!(result.is_some());
        let processed = result.unwrap();
        assert_eq!(processed.country, "TN");
    }

    #[tokio::test]
    async fn test_process_invalid_message() {
        let processor = MessageProcessor::new();
        let invalid_payload = vec![0xFF, 0xFE, 0xFD]; // Invalid UTF-8
        
        let result = processor.process(&invalid_payload, "cdr.raw.FN").await;
        
        assert!(result.is_none());
    }
}
