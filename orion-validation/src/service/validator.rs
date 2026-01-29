use crate::metrics;
use crate::service::model::{ProcessedCDR, ValidatedCDR, ValidationResult, ValidationError, EventType};
use regex::Regex;
use chrono::Utc;
use std::time::Instant;
use uuid::Uuid;

pub struct Validator {
    imsi_regex: Regex,
    msisdn_regex: Regex,
}

impl Validator {
    pub fn new() -> Self {
        Self {
            // IMSI: 14-15 digits
            imsi_regex: Regex::new(r"^\d{14,15}$").unwrap(),
            // MSISDN: + followed by 10-15 digits
            msisdn_regex: Regex::new(r"^\+?\d{10,15}$").unwrap(),
        }
    }

    pub async fn validate(&self, cdr: &ProcessedCDR) -> ValidationResult {
        let start = Instant::now();
        metrics::increment_messages_total();

        // Parse JSON from raw_payload
        let json_data: serde_json::Value = match serde_json::from_str(&cdr.raw_payload) {
            Ok(data) => data,
            Err(e) => {
                metrics::increment_invalid_total();
                metrics::record_latency(start.elapsed().as_secs_f64());
                return ValidationResult::Invalid(ValidationError::new(
                    "json_parse_error",
                    format!("Failed to parse JSON: {}", e),
                    None,
                    cdr.raw_payload.clone(),
                ));
            }
        };

        // Extract and validate IMSI
        let imsi = match json_data.get("imsi").and_then(|v| v.as_str()) {
            Some(val) => {
                if !self.imsi_regex.is_match(val) {
                    metrics::increment_invalid_total();
                    metrics::record_latency(start.elapsed().as_secs_f64());
                    return ValidationResult::Invalid(ValidationError::new(
                        "invalid_imsi",
                        format!("IMSI must be 14-15 digits, got: {}", val),
                        Some("imsi".to_string()),
                        cdr.raw_payload.clone(),
                    ));
                }
                val.to_string()
            }
            None => {
                metrics::increment_invalid_total();
                metrics::record_latency(start.elapsed().as_secs_f64());
                return ValidationResult::Invalid(ValidationError::new(
                    "missing_field",
                    "IMSI field is required".to_string(),
                    Some("imsi".to_string()),
                    cdr.raw_payload.clone(),
                ));
            }
        };

        // Extract and validate MSISDN
        let msisdn = match json_data.get("msisdn").and_then(|v| v.as_str()) {
            Some(val) => {
                if !self.msisdn_regex.is_match(val) {
                    metrics::increment_invalid_total();
                    metrics::record_latency(start.elapsed().as_secs_f64());
                    return ValidationResult::Invalid(ValidationError::new(
                        "invalid_msisdn",
                        format!("MSISDN must be 10-15 digits with optional +, got: {}", val),
                        Some("msisdn".to_string()),
                        cdr.raw_payload.clone(),
                    ));
                }
                val.to_string()
            }
            None => {
                metrics::increment_invalid_total();
                metrics::record_latency(start.elapsed().as_secs_f64());
                return ValidationResult::Invalid(ValidationError::new(
                    "missing_field",
                    "MSISDN field is required".to_string(),
                    Some("msisdn".to_string()),
                    cdr.raw_payload.clone(),
                ));
            }
        };

        // Extract event type
        let event_type = json_data
            .get("event_type")
            .and_then(|v| v.as_str())
            .map(|v| match v.to_lowercase().as_str() {
                "voice" => EventType::Voice,
                "data" => EventType::Data,
                "sms" => EventType::Sms,
                _ => EventType::Unknown,
            })
            .unwrap_or(EventType::Unknown);

        // Create validated CDR
        let validated = ValidatedCDR {
            cdr_id: Uuid::new_v4().to_string(),
            event_type,
            imsi,
            msisdn,
            timestamp: Utc::now(),
            country: cdr.country.clone(),
            raw_data: json_data,
            validation_timestamp: Utc::now().to_rfc3339(),
        };

        metrics::increment_valid_total();
        metrics::record_latency(start.elapsed().as_secs_f64());

        ValidationResult::Valid(validated)
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_validate_valid_cdr() {
        let validator = Validator::new();
        let cdr = ProcessedCDR {
            raw_payload: r#"{"imsi":"208150123456789","msisdn":"+33612345678","event_type":"voice"}"#.to_string(),
            source_topic: "cdr.raw.FR".to_string(),
            country: "FR".to_string(),
            ingestion_timestamp: Utc::now().to_rfc3339(),
        };

        let result = validator.validate(&cdr).await;
        assert!(matches!(result, ValidationResult::Valid(_)));
    }

    #[tokio::test]
    async fn test_validate_missing_imsi() {
        let validator = Validator::new();
        let cdr = ProcessedCDR {
            raw_payload: r#"{"msisdn":"+33612345678","event_type":"voice"}"#.to_string(),
            source_topic: "cdr.raw.FR".to_string(),
            country: "FR".to_string(),
            ingestion_timestamp: Utc::now().to_rfc3339(),
        };

        let result = validator.validate(&cdr).await;
        if let ValidationResult::Invalid(err) = result {
            assert_eq!(err.error_type, "missing_field");
            assert_eq!(err.field, Some("imsi".to_string()));
        } else {
            panic!("Expected Invalid result");
        }
    }

    #[tokio::test]
    async fn test_validate_invalid_imsi_format() {
        let validator = Validator::new();
        let cdr = ProcessedCDR {
            raw_payload: r#"{"imsi":"123","msisdn":"+33612345678","event_type":"voice"}"#.to_string(),
            source_topic: "cdr.raw.FR".to_string(),
            country: "FR".to_string(),
            ingestion_timestamp: Utc::now().to_rfc3339(),
        };

        let result = validator.validate(&cdr).await;
        if let ValidationResult::Invalid(err) = result {
            assert_eq!(err.error_type, "invalid_imsi");
        } else {
            panic!("Expected Invalid result");
        }
    }
}
