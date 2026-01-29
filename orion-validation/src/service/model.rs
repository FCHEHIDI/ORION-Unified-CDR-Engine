use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// CDR processed by ingestion service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedCDR {
    pub raw_payload: String,
    pub source_topic: String,
    pub country: String,
    pub ingestion_timestamp: String,
}

/// Validated CDR ready for normalization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedCDR {
    pub cdr_id: String,
    pub event_type: EventType,
    pub imsi: String,
    pub msisdn: String,
    pub timestamp: DateTime<Utc>,
    pub country: String,
    pub raw_data: serde_json::Value,
    pub validation_timestamp: String,
}

/// Event type classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EventType {
    Voice,
    Data,
    Sms,
    Unknown,
}

/// Validation result
#[derive(Debug)]
pub enum ValidationResult {
    Valid(ValidatedCDR),
    Invalid(ValidationError),
}

/// Validation error with details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub error_type: String,
    pub message: String,
    pub field: Option<String>,
    pub original_cdr: String,
    pub timestamp: String,
}

impl ValidationError {
    pub fn new(error_type: &str, message: String, field: Option<String>, original_cdr: String) -> Self {
        Self {
            error_type: error_type.to_string(),
            message,
            field,
            original_cdr,
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_type_serialization() {
        let event = EventType::Voice;
        let json = serde_json::to_string(&event).unwrap();
        assert_eq!(json, "\"voice\"");
    }

    #[test]
    fn test_validation_error_creation() {
        let error = ValidationError::new(
            "missing_field",
            "IMSI is required".to_string(),
            Some("imsi".to_string()),
            "{}".to_string(),
        );
        
        assert_eq!(error.error_type, "missing_field");
        assert_eq!(error.message, "IMSI is required");
        assert_eq!(error.field, Some("imsi".to_string()));
    }
}
