use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrichedCDR {
    pub unified: UnifiedCDR,
    pub fraud_info: Option<FraudInfo>,
    pub network_info: Option<NetworkInfo>,
    pub client_info: Option<ClientInfo>,
    pub enrichment_timestamp: String,
    pub enrichment_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedCDR {
    pub cdr_id: String,
    pub event_type: String,
    pub imsi: String,
    pub msisdn: String,
    pub imei: Option<String>,
    pub country: String,
    pub operator: Option<String>,
    pub mcc: Option<String>,
    pub mnc: Option<String>,
    pub lac: Option<String>,
    pub cell_id: Option<String>,
    pub start_timestamp: String,
    pub end_timestamp: Option<String>,
    pub duration_seconds: Option<u64>,
    pub service_type: Option<String>,
    pub call_type: Option<String>,
    pub called_number: Option<String>,
    pub calling_number: Option<String>,
    pub call_direction: Option<String>,
    pub sms_type: Option<String>,
    pub sms_direction: Option<String>,
    pub destination_number: Option<String>,
    pub originating_number: Option<String>,
    pub apn: Option<String>,
    pub bytes_uploaded: Option<u64>,
    pub bytes_downloaded: Option<u64>,
    pub session_duration: Option<u64>,
    pub is_roaming: bool,
    pub visited_country: Option<String>,
    pub visited_network: Option<String>,
    pub charge_amount: Option<f64>,
    pub currency: Option<String>,
    pub tariff_class: Option<String>,
    pub cause_for_termination: Option<String>,
    pub hash: String,
    pub ingestion_timestamp: String,
    pub normalization_timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudInfo {
    pub fraud_score: f64,
    pub risk_level: String,
    pub reasons: Vec<String>,
    pub model_version: String,
    pub detection_timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub network_name: String,
    pub network_type: String,
    pub cell_tower_location: Option<String>,
    pub signal_strength: Option<i32>,
    pub handover_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub subscriber_segment: String,
    pub contract_type: String,
    pub customer_since: Option<String>,
    pub lifetime_value: Option<f64>,
    pub is_vip: bool,
    pub data_plan_limit_mb: Option<u64>,
}
