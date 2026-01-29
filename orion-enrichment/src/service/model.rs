use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Unified CDR from normalization service (re-export structure)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedCDR {
    pub cdr_id: String,
    pub session_id: Option<String>,
    pub imsi: String,
    pub msisdn: String,
    pub imei: Option<String>,
    pub event_type: String,
    pub service_type: String,
    pub start_timestamp: DateTime<Utc>,
    pub end_timestamp: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i64>,
    pub country_code: String,
    pub mcc: Option<String>,
    pub mnc: Option<String>,
    pub lac: Option<String>,
    pub cell_id: Option<String>,
    pub calling_number: Option<String>,
    pub called_number: Option<String>,
    pub call_type: Option<String>,
    pub bytes_uploaded: Option<i64>,
    pub bytes_downloaded: Option<i64>,
    pub apn: Option<String>,
    pub sms_type: Option<String>,
    pub message_length: Option<i32>,
    pub is_roaming: bool,
    pub visited_country: Option<String>,
    pub visited_network: Option<String>,
    pub charging_id: Option<String>,
    pub rated_amount: Option<f64>,
    pub currency: Option<String>,
    pub normalization_timestamp: String,
    pub source_system: String,
    pub raw_data_hash: String,
}

/// Enriched CDR with additional data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrichedCDR {
    // All fields from UnifiedCDR
    #[serde(flatten)]
    pub unified: UnifiedCDR,
    
    // Fraud detection
    pub fraud_info: Option<FraudInfo>,
    
    // Network enrichment
    pub network_info: Option<NetworkInfo>,
    
    // Client/Subscriber enrichment
    pub client_info: Option<ClientInfo>,
    
    // Metadata
    pub enrichment_timestamp: String,
    pub enrichment_version: String,
}

/// Fraud detection information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudInfo {
    pub fraud_score: f64,           // 0.0 - 1.0
    pub risk_level: String,          // low, medium, high
    pub reasons: Vec<String>,        // List of fraud indicators
    pub model_version: String,       // fraud_model_v1
    pub detection_timestamp: String,
}

/// Network enrichment data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub network_name: String,        // Orange, SFR, Bouygues...
    pub network_type: String,        // 3G, 4G, 5G
    pub cell_tower_location: Option<String>, // Lat/Long
    pub signal_strength: Option<i32>, // dBm
    pub handover_count: Option<i32>, // Number of cell handovers
}

/// Client/Subscriber enrichment data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub subscriber_segment: String,  // individual, business, premium
    pub contract_type: String,       // prepaid, postpaid
    pub customer_since: Option<String>, // 2020-01-15
    pub lifetime_value: Option<f64>, // EUR
    pub is_vip: bool,
    pub data_plan_limit_mb: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enriched_cdr_serialization() {
        let fraud_info = FraudInfo {
            fraud_score: 0.85,
            risk_level: "high".to_string(),
            reasons: vec!["roaming_spike".to_string()],
            model_version: "fraud_rules_v1".to_string(),
            detection_timestamp: Utc::now().to_rfc3339(),
        };

        assert_eq!(fraud_info.risk_level, "high");
        assert!(fraud_info.fraud_score > 0.8);
    }
}
