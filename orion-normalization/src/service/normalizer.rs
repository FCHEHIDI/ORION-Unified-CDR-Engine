use crate::metrics;
use crate::service::model::*;
use chrono::Utc;
use std::time::Instant;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct Normalizer;

impl Normalizer {
    pub fn new() -> Self {
        Self
    }

    pub async fn normalize(&self, validated: &ValidatedCDR) -> anyhow::Result<UnifiedCDR> {
        let start = Instant::now();
        metrics::increment_messages_total();

        // Track by event type
        match validated.event_type {
            EventType::Voice => metrics::increment_voice_total(),
            EventType::Data => metrics::increment_data_total(),
            EventType::Sms => metrics::increment_sms_total(),
            _ => {}
        }

        // Extract MCC/MNC from IMSI (first 5-6 digits)
        let (mcc, mnc) = Self::extract_mcc_mnc(&validated.imsi);

        // Determine service type
        let service_type = Self::determine_service_type(&validated);

        // Extract voice-specific fields
        let (calling_number, called_number, call_type, duration) = 
            Self::extract_voice_fields(&validated.raw_data, &validated.event_type);

        // Extract data-specific fields
        let (bytes_uploaded, bytes_downloaded, apn) = 
            Self::extract_data_fields(&validated.raw_data, &validated.event_type);

        // Extract SMS-specific fields
        let (sms_type, message_length) = 
            Self::extract_sms_fields(&validated.raw_data, &validated.event_type);

        // Detect roaming
        let is_roaming = Self::detect_roaming(&validated.country, mcc.as_deref());

        // Calculate hash of raw data for deduplication
        let raw_data_hash = Self::calculate_hash(&validated.raw_data);

        let unified = UnifiedCDR {
            cdr_id: validated.cdr_id.clone(),
            session_id: validated.raw_data.get("session_id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            
            imsi: validated.imsi.clone(),
            msisdn: validated.msisdn.clone(),
            imei: validated.raw_data.get("imei")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            
            event_type: validated.event_type.clone(),
            service_type,
            
            start_timestamp: validated.timestamp,
            end_timestamp: validated.raw_data.get("end_timestamp")
                .and_then(|v| v.as_str())
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            duration_seconds: duration,
            
            country_code: validated.country.clone(),
            mcc,
            mnc,
            lac: validated.raw_data.get("lac")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            cell_id: validated.raw_data.get("cell_id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            
            calling_number,
            called_number,
            call_type,
            
            bytes_uploaded,
            bytes_downloaded,
            apn,
            
            sms_type,
            message_length,
            
            is_roaming,
            visited_country: validated.raw_data.get("visited_country")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            visited_network: validated.raw_data.get("visited_network")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            
            charging_id: validated.raw_data.get("charging_id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            rated_amount: validated.raw_data.get("amount")
                .and_then(|v| v.as_f64()),
            currency: validated.raw_data.get("currency")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            
            normalization_timestamp: Utc::now().to_rfc3339(),
            source_system: "orion-ingestion".to_string(),
            raw_data_hash,
        };

        metrics::record_latency(start.elapsed().as_secs_f64());
        Ok(unified)
    }

    fn extract_mcc_mnc(imsi: &str) -> (Option<String>, Option<String>) {
        if imsi.len() >= 5 {
            let mcc = Some(imsi[0..3].to_string());
            let mnc = Some(imsi[3..5].to_string());
            (mcc, mnc)
        } else {
            (None, None)
        }
    }

    fn determine_service_type(validated: &ValidatedCDR) -> ServiceType {
        if validated.raw_data.get("is_premium").and_then(|v| v.as_bool()).unwrap_or(false) {
            return ServiceType::Premium;
        }
        
        if validated.raw_data.get("is_roaming").and_then(|v| v.as_bool()).unwrap_or(false) {
            return ServiceType::Roaming;
        }
        
        if validated.raw_data.get("is_emergency").and_then(|v| v.as_bool()).unwrap_or(false) {
            return ServiceType::Emergency;
        }
        
        ServiceType::Standard
    }

    fn extract_voice_fields(raw_data: &serde_json::Value, event_type: &EventType) -> 
        (Option<String>, Option<String>, Option<CallType>, Option<i64>) {
        
        if *event_type != EventType::Voice {
            return (None, None, None, None);
        }

        let calling = raw_data.get("calling_number")
            .or_else(|| raw_data.get("msisdn"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        let called = raw_data.get("called_number")
            .or_else(|| raw_data.get("destination"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        let call_type = raw_data.get("call_type")
            .and_then(|v| v.as_str())
            .map(|s| match s.to_lowercase().as_str() {
                "mobile" => CallType::Mobile,
                "landline" | "fixed" => CallType::Landline,
                "international" => CallType::International,
                "emergency" => CallType::Emergency,
                _ => CallType::Unknown,
            })
            .or(Some(CallType::Mobile)); // Default to mobile
        
        let duration = raw_data.get("duration")
            .or_else(|| raw_data.get("duration_seconds"))
            .and_then(|v| v.as_i64());
        
        (calling, called, call_type, duration)
    }

    fn extract_data_fields(raw_data: &serde_json::Value, event_type: &EventType) -> 
        (Option<i64>, Option<i64>, Option<String>) {
        
        if *event_type != EventType::Data {
            return (None, None, None);
        }

        let bytes_up = raw_data.get("bytes_uploaded")
            .or_else(|| raw_data.get("bytes_up"))
            .and_then(|v| v.as_i64());
        
        let bytes_down = raw_data.get("bytes_downloaded")
            .or_else(|| raw_data.get("bytes_down"))
            .and_then(|v| v.as_i64());
        
        let apn = raw_data.get("apn")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        (bytes_up, bytes_down, apn)
    }

    fn extract_sms_fields(raw_data: &serde_json::Value, event_type: &EventType) -> 
        (Option<SmsType>, Option<i32>) {
        
        if *event_type != EventType::Sms {
            return (None, None);
        }

        let sms_type = raw_data.get("sms_type")
            .and_then(|v| v.as_str())
            .map(|s| match s.to_lowercase().as_str() {
                "mt" | "mt_sms" => SmsType::MtSms,
                "mo" | "mo_sms" => SmsType::MoSms,
                _ => SmsType::Unknown,
            })
            .or(Some(SmsType::MoSms)); // Default to MO
        
        let length = raw_data.get("message_length")
            .or_else(|| raw_data.get("length"))
            .and_then(|v| v.as_i64())
            .map(|l| l as i32);
        
        (sms_type, length)
    }

    fn detect_roaming(country: &str, mcc: Option<&str>) -> bool {
        // Simple roaming detection: MCC doesn't match country
        match (country, mcc) {
            ("FR", Some(mcc)) => !mcc.starts_with("208"),
            ("TN", Some(mcc)) => !mcc.starts_with("605"),
            ("FN", Some(mcc)) => !mcc.starts_with("244"),
            ("CH", Some(mcc)) => !mcc.starts_with("228"),
            _ => false,
        }
    }

    fn calculate_hash(data: &serde_json::Value) -> String {
        let mut hasher = DefaultHasher::new();
        data.to_string().hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
}

impl Default for Normalizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_normalize_voice_cdr() {
        let normalizer = Normalizer::new();
        
        let raw_data = serde_json::json!({
            "calling_number": "+33612345678",
            "called_number": "+33698765432",
            "duration": 120,
            "call_type": "mobile"
        });
        
        let validated = ValidatedCDR {
            cdr_id: "test-123".to_string(),
            event_type: EventType::Voice,
            imsi: "208150123456789".to_string(),
            msisdn: "+33612345678".to_string(),
            timestamp: Utc::now(),
            country: "FR".to_string(),
            raw_data,
            validation_timestamp: Utc::now().to_rfc3339(),
        };

        let result = normalizer.normalize(&validated).await;
        assert!(result.is_ok());
        
        let unified = result.unwrap();
        assert_eq!(unified.event_type, EventType::Voice);
        assert_eq!(unified.mcc, Some("208".to_string()));
        assert_eq!(unified.calling_number, Some("+33612345678".to_string()));
    }

    #[test]
    fn test_extract_mcc_mnc() {
        let (mcc, mnc) = Normalizer::extract_mcc_mnc("208150123456789");
        assert_eq!(mcc, Some("208".to_string()));
        assert_eq!(mnc, Some("15".to_string()));
    }

    #[test]
    fn test_detect_roaming() {
        assert!(!Normalizer::detect_roaming("FR", Some("208")));
        assert!(Normalizer::detect_roaming("FR", Some("605")));
    }
}
