use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cdr {
    pub cdr_id: String,
    pub msisdn: String,
    pub imsi: Option<String>,
    pub imei: Option<String>,
    pub call_type: String,
    pub direction: String,
    pub start_time: String,
    pub duration: u32,
    pub bytes_uploaded: u64,
    pub bytes_downloaded: u64,
    pub destination: Option<String>,
    pub cell_id: Option<String>,
    pub fraud_score: Option<f32>,
    pub location: Option<Location>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub country: Option<String>,
    pub city: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct ApiClient {
    base_url: String,
    client: reqwest::Client,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_cdr(&self, id: &str) -> Result<Cdr> {
        let url = format!("{}/cdr/{}", self.base_url, id);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            anyhow::bail!("Failed to get CDR: {}", response.status());
        }

        let cdr = response.json::<Cdr>().await?;
        Ok(cdr)
    }

    pub async fn search_cdrs(&self, filters: SearchFilters) -> Result<Vec<Cdr>> {
        let url = format!("{}/cdr/search", self.base_url);
        let response = self.client
            .get(&url)
            .query(&filters)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to search CDRs: {}", response.status());
        }

        let cdrs = response.json::<Vec<Cdr>>().await?;
        Ok(cdrs)
    }

    pub async fn health_check(&self) -> Result<HealthStatus> {
        let url = format!("{}/health", self.base_url);
        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            anyhow::bail!("Health check failed: {}", response.status());
        }

        let health = response.json::<HealthStatus>().await?;
        Ok(health)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchFilters {
    pub msisdn: Option<String>,
    pub imsi: Option<String>,
    pub call_type: Option<String>,
    pub fraud_min: Option<f32>,
    pub fraud_only: Option<bool>,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub service: String,
}
