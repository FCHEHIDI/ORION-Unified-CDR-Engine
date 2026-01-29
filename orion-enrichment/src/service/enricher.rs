use crate::config::EnrichmentConfig;
use crate::metrics;
use crate::service::model::*;
use chrono::Utc;
use std::time::Instant;

pub struct Enricher {
    config: EnrichmentConfig,
}

impl Enricher {
    pub fn new(config: EnrichmentConfig) -> Self {
        Self { config }
    }

    pub async fn enrich(&self, unified: UnifiedCDR) -> anyhow::Result<EnrichedCDR> {
        let start = Instant::now();
        metrics::increment_messages_total();

        // Fraud detection (rule-based for now, TODO: gRPC to ML fraud agent)
        let fraud_info = if self.config.enable_fraud_detection {
            Some(self.detect_fraud(&unified).await)
        } else {
            None
        };

        // Track fraud detection
        if let Some(ref info) = fraud_info {
            if info.risk_level == "high" {
                metrics::increment_fraud_detected_total();
            }
        }

        // Network enrichment (mock data for now, TODO: external API)
        let network_info = if self.config.enable_network_data {
            Some(self.fetch_network_info(&unified).await)
        } else {
            None
        };

        // Client enrichment (mock data for now, TODO: CRM API)
        let client_info = if self.config.enable_client_data {
            Some(self.fetch_client_info(&unified).await)
        } else {
            None
        };

        let enriched = EnrichedCDR {
            unified,
            fraud_info,
            network_info,
            client_info,
            enrichment_timestamp: Utc::now().to_rfc3339(),
            enrichment_version: "v1.0.0".to_string(),
        };

        metrics::record_latency(start.elapsed().as_secs_f64());
        Ok(enriched)
    }

    /// Simple rule-based fraud detection (placeholder for ML model)
    async fn detect_fraud(&self, cdr: &UnifiedCDR) -> FraudInfo {
        let mut fraud_score = 0.0;
        let mut reasons = Vec::new();

        // Rule 1: Excessive duration (> 2 hours)
        if let Some(duration) = cdr.duration_seconds {
            if duration > 7200 {
                fraud_score += 0.3;
                reasons.push("excessive_duration".to_string());
            }
        }

        // Rule 2: Suspicious roaming
        if cdr.is_roaming {
            // Check if roaming in high-risk country (simplified)
            if let Some(ref visited) = cdr.visited_country {
                if !["FR", "TN", "FN", "CH", "DE", "ES", "IT"].contains(&visited.as_str()) {
                    fraud_score += 0.4;
                    reasons.push("suspicious_roaming_country".to_string());
                }
            } else {
                fraud_score += 0.2;
                reasons.push("roaming_no_details".to_string());
            }
        }

        // Rule 3: Excessive data usage (> 10GB in one session)
        if let (Some(up), Some(down)) = (cdr.bytes_uploaded, cdr.bytes_downloaded) {
            let total_gb = (up + down) as f64 / 1_000_000_000.0;
            if total_gb > 10.0 {
                fraud_score += 0.4;
                reasons.push("excessive_data_usage".to_string());
            }
        }

        // Rule 4: International calls (high risk)
        if let Some(ref call_type) = cdr.call_type {
            if call_type == "international" {
                fraud_score += 0.2;
                reasons.push("international_call".to_string());
            }
        }

        // Determine risk level
        let risk_level = if fraud_score >= 0.7 {
            "high"
        } else if fraud_score >= 0.4 {
            "medium"
        } else {
            "low"
        };

        // TODO: Replace with gRPC call to orion-ml-fraud-agent
        // let response = self.grpc_client.predict_fraud(&cdr).await?;

        FraudInfo {
            fraud_score: fraud_score.min(1.0),
            risk_level: risk_level.to_string(),
            reasons,
            model_version: "fraud_rules_v1".to_string(),
            detection_timestamp: Utc::now().to_rfc3339(),
        }
    }

    /// Fetch network information (mock data for now)
    async fn fetch_network_info(&self, cdr: &UnifiedCDR) -> NetworkInfo {
        // TODO: Call external network API
        // let response = reqwest::get(format!("http://network-api/cell/{}", cdr.cell_id)).await?;
        
        // Mock data based on MCC/MNC
        let network_name = match (cdr.mcc.as_deref(), cdr.mnc.as_deref()) {
            (Some("208"), Some("15")) => "Orange France",
            (Some("208"), Some("01")) => "SFR",
            (Some("208"), Some("20")) => "Bouygues Telecom",
            (Some("605"), _) => "Tunisie Telecom",
            (Some("244"), _) => "Elisa Finland",
            (Some("228"), _) => "Swisscom",
            _ => "Unknown Network",
        };

        NetworkInfo {
            network_name: network_name.to_string(),
            network_type: if cdr.start_timestamp.year() >= 2024 { "5G" } else { "4G" }.to_string(),
            cell_tower_location: cdr.cell_id.as_ref().map(|_| "48.8566,2.3522".to_string()), // Paris mock
            signal_strength: Some(-75), // -75 dBm (good signal)
            handover_count: if cdr.duration_seconds.unwrap_or(0) > 600 { Some(2) } else { Some(0) },
        }
    }

    /// Fetch client/subscriber information (mock data for now)
    async fn fetch_client_info(&self, cdr: &UnifiedCDR) -> ClientInfo {
        // TODO: Call CRM/billing API
        // let response = reqwest::get(format!("http://crm-api/subscriber/{}", cdr.imsi)).await?;
        
        // Mock data based on IMSI pattern
        let is_business = cdr.imsi.ends_with("000") || cdr.imsi.ends_with("999");
        
        ClientInfo {
            subscriber_segment: if is_business { "business" } else { "individual" }.to_string(),
            contract_type: if cdr.is_roaming { "postpaid" } else { "prepaid" }.to_string(),
            customer_since: Some("2020-01-15".to_string()),
            lifetime_value: if is_business { Some(5000.0) } else { Some(500.0) },
            is_vip: is_business,
            data_plan_limit_mb: Some(50_000), // 50GB
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fraud_detection_high_risk() {
        let config = EnrichmentConfig {
            enable_fraud_detection: true,
            fraud_agent_url: "http://localhost:50051".to_string(),
            enable_network_data: false,
            enable_client_data: false,
        };
        
        let enricher = Enricher::new(config);
        
        let cdr = UnifiedCDR {
            cdr_id: "test-123".to_string(),
            session_id: None,
            imsi: "208150123456789".to_string(),
            msisdn: "+33612345678".to_string(),
            imei: None,
            event_type: "data".to_string(),
            service_type: "roaming".to_string(),
            start_timestamp: Utc::now(),
            end_timestamp: None,
            duration_seconds: Some(10000), // Very long
            country_code: "FR".to_string(),
            mcc: Some("208".to_string()),
            mnc: Some("15".to_string()),
            lac: None,
            cell_id: None,
            calling_number: None,
            called_number: None,
            call_type: None,
            bytes_uploaded: Some(15_000_000_000), // 15GB
            bytes_downloaded: Some(5_000_000_000),
            apn: None,
            sms_type: None,
            message_length: None,
            is_roaming: true,
            visited_country: Some("XX".to_string()), // Suspicious country
            visited_network: None,
            charging_id: None,
            rated_amount: None,
            currency: None,
            normalization_timestamp: Utc::now().to_rfc3339(),
            source_system: "test".to_string(),
            raw_data_hash: "abc123".to_string(),
        };

        let fraud_info = enricher.detect_fraud(&cdr).await;
        assert_eq!(fraud_info.risk_level, "high");
        assert!(fraud_info.fraud_score > 0.7);
    }

    #[tokio::test]
    async fn test_enrich_full() {
        let config = EnrichmentConfig {
            enable_fraud_detection: true,
            fraud_agent_url: "http://localhost:50051".to_string(),
            enable_network_data: true,
            enable_client_data: true,
        };
        
        let enricher = Enricher::new(config);
        
        let cdr = UnifiedCDR {
            cdr_id: "test-456".to_string(),
            session_id: None,
            imsi: "208150123456789".to_string(),
            msisdn: "+33612345678".to_string(),
            imei: None,
            event_type: "voice".to_string(),
            service_type: "standard".to_string(),
            start_timestamp: Utc::now(),
            end_timestamp: None,
            duration_seconds: Some(120),
            country_code: "FR".to_string(),
            mcc: Some("208".to_string()),
            mnc: Some("15".to_string()),
            lac: None,
            cell_id: Some("12345".to_string()),
            calling_number: Some("+33612345678".to_string()),
            called_number: Some("+33698765432".to_string()),
            call_type: Some("mobile".to_string()),
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
            source_system: "test".to_string(),
            raw_data_hash: "def456".to_string(),
        };

        let enriched = enricher.enrich(cdr).await.unwrap();
        
        assert!(enriched.fraud_info.is_some());
        assert!(enriched.network_info.is_some());
        assert!(enriched.client_info.is_some());
        
        let network = enriched.network_info.unwrap();
        assert_eq!(network.network_name, "Orange France");
    }
}
