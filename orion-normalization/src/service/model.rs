use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Validated CDR from validation service
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

/// Unified CDR schema (normalized)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedCDR {
    // Identifiers
    pub cdr_id: String,
    pub session_id: Option<String>,
    
    // Subscriber info
    pub imsi: String,
    pub msisdn: String,
    pub imei: Option<String>,
    
    // Event classification
    pub event_type: EventType,
    pub service_type: ServiceType,
    
    // Temporal
    pub start_timestamp: DateTime<Utc>,
    pub end_timestamp: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i64>,
    
    // Location
    pub country_code: String,
    pub mcc: Option<String>, // Mobile Country Code
    pub mnc: Option<String>, // Mobile Network Code
    pub lac: Option<String>, // Location Area Code
    pub cell_id: Option<String>,
    
    // Voice specific
    pub calling_number: Option<String>,
    pub called_number: Option<String>,
    pub call_type: Option<CallType>,
    
    // Data specific
    pub bytes_uploaded: Option<i64>,
    pub bytes_downloaded: Option<i64>,
    pub apn: Option<String>,
    
    // SMS specific
    pub sms_type: Option<SmsType>,
    pub message_length: Option<i32>,
    
    // Roaming
    pub is_roaming: bool,
    pub visited_country: Option<String>,
    pub visited_network: Option<String>,
    
    // Charging
    pub charging_id: Option<String>,
    pub rated_amount: Option<f64>,
    pub currency: Option<String>,
    
    // Metadata
    pub normalization_timestamp: String,
    pub source_system: String,
    pub raw_data_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ServiceType {
    Standard,
    Premium,
    Roaming,
    Emergency,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CallType {
    Mobile,
    Landline,
    International,
    Emergency,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SmsType {
    MtSms, // Mobile Terminated
    MoSms, // Mobile Originated
    Unknown,
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
    fn test_unified_cdr_structure() {
        let cdr = UnifiedCDR {
            cdr_id: "test-123".to_string(),
            session_id: None,
            imsi: "208150123456789".to_string(),
            msisdn: "+33612345678".to_string(),
            imei: None,
            event_type: EventType::Voice,
            service_type: ServiceType::Standard,
            start_timestamp: Utc::now(),
            end_timestamp: None,
            duration_seconds: Some(120),
            country_code: "FR".to_string(),
            mcc: Some("208".to_string()),
            mnc: Some("15".to_string()),
            lac: None,
            cell_id: None,
            calling_number: Some("+33612345678".to_string()),
            called_number: Some("+33698765432".to_string()),
            call_type: Some(CallType::Mobile),
            bytes_uploaded: None,
            bytes_downloaded: None,
            apn: None,
            sms_type: None,
            message_length: None,
            is_roaming: false,
            visited_country: None,
            visited_network: None,
            charging_id: None,
            rated_amount: None,
            currency: None,
            normalization_timestamp: Utc::now().to_rfc3339(),
            source_system: "orion-ingestion".to_string(),
            raw_data_hash: "abc123".to_string(),
        };

        let json = serde_json::to_string(&cdr).unwrap();
        assert!(json.contains("\"cdr_id\":\"test-123\""));
    }
}
