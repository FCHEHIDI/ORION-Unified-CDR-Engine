use serde::{Deserialize, Serialize};

/// Raw CDR from Kafka - can be in various formats
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RawCDR {
    Json(serde_json::Value),
    Text(String),
}

/// Processed CDR ready for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedCDR {
    pub raw_payload: String,
    pub source_topic: String,
    pub country: String,
    pub ingestion_timestamp: String,
}

impl ProcessedCDR {
    pub fn new(raw: RawCDR, topic: &str) -> Self {
        let country = extract_country_from_topic(topic);
        let raw_payload = match raw {
            RawCDR::Json(val) => serde_json::to_string(&val).unwrap_or_default(),
            RawCDR::Text(text) => text,
        };

        Self {
            raw_payload,
            source_topic: topic.to_string(),
            country,
            ingestion_timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

fn extract_country_from_topic(topic: &str) -> String {
    // Topic format: cdr.raw.FR -> FR
    topic
        .split('.')
        .last()
        .unwrap_or("UNKNOWN")
        .to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_country() {
        assert_eq!(extract_country_from_topic("cdr.raw.FR"), "FR");
        assert_eq!(extract_country_from_topic("cdr.raw.FN"), "FN");
        assert_eq!(extract_country_from_topic("cdr.raw.MA"), "MA");
    }

    #[test]
    fn test_processed_cdr_creation() {
        let raw = RawCDR::Text("test".to_string());
        let processed = ProcessedCDR::new(raw, "cdr.raw.FR");
        
        assert_eq!(processed.country, "FR");
        assert_eq!(processed.source_topic, "cdr.raw.FR");
        assert_eq!(processed.raw_payload, "test");
    }
}
