use crate::config::ServicesConfig;
use crate::metrics;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info};

/// Service health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Unhealthy,
    Unknown,
}

/// Individual service health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    pub name: String,
    pub status: HealthStatus,
    pub url: String,
    pub last_check: String,
    pub response_time_ms: Option<f64>,
    pub error_message: Option<String>,
}

/// Aggregated health status for all services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineHealth {
    pub status: HealthStatus,
    pub services: Vec<ServiceHealth>,
    pub healthy_count: usize,
    pub total_count: usize,
    pub timestamp: String,
}

/// Health checker that monitors all ORION services
pub struct HealthChecker {
    config: ServicesConfig,
    client: reqwest::Client,
    cache: Arc<RwLock<PipelineHealth>>,
}

impl HealthChecker {
    /// Create a new health checker
    pub fn new(config: ServicesConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .expect("Failed to create HTTP client");
        
        let cache = Arc::new(RwLock::new(PipelineHealth {
            status: HealthStatus::Unknown,
            services: Vec::new(),
            healthy_count: 0,
            total_count: 0,
            timestamp: Utc::now().to_rfc3339(),
        }));
        
        Self {
            config,
            client,
            cache,
        }
    }

    /// Start the background health checking loop
    pub async fn start_monitoring(self: Arc<Self>) {
        let interval_secs = self.config.health_check_interval_secs;
        info!("Starting health check monitoring (interval: {}s)", interval_secs);
        
        loop {
            self.check_all_services().await;
            tokio::time::sleep(tokio::time::Duration::from_secs(interval_secs)).await;
        }
    }

    /// Check health of all services
    async fn check_all_services(&self) {
        debug!("Checking health of all services");
        
        let mut services = Vec::new();
        
        // Define all services to check
        let service_urls = vec![
            ("traffic-generator", &self.config.traffic_generator_url),
            ("ingestion", &self.config.ingestion_url),
            ("validation", &self.config.validation_url),
            ("normalization", &self.config.normalization_url),
            ("enrichment", &self.config.enrichment_url),
            ("ml-fraud-agent", &self.config.ml_fraud_agent_url),
            ("storage-hot", &self.config.storage_hot_url),
        ];
        
        // Check each service
        for (name, base_url) in service_urls {
            let health = self.check_service(name, base_url).await;
            
            // Record metrics
            metrics::record_service_health(name, health.status == HealthStatus::Healthy);
            
            services.push(health);
        }
        
        // Calculate aggregate health
        let healthy_count = services.iter()
            .filter(|s| s.status == HealthStatus::Healthy)
            .count();
        let total_count = services.len();
        
        let pipeline_status = if healthy_count == total_count {
            HealthStatus::Healthy
        } else if healthy_count == 0 {
            HealthStatus::Unhealthy
        } else {
            HealthStatus::Unknown
        };
        
        // Update cache
        let pipeline_health = PipelineHealth {
            status: pipeline_status,
            services,
            healthy_count,
            total_count,
            timestamp: Utc::now().to_rfc3339(),
        };
        
        *self.cache.write().await = pipeline_health.clone();
        
        // Record aggregate metrics
        metrics::record_pipeline_health(healthy_count, total_count);
        
        info!(
            "Health check complete: {}/{} services healthy",
            healthy_count, total_count
        );
    }

    /// Check health of a single service
    async fn check_service(&self, name: &str, base_url: &str) -> ServiceHealth {
        let health_url = format!("{}/health", base_url);
        let start = std::time::Instant::now();
        
        match self.client.get(&health_url).send().await {
            Ok(response) => {
                let response_time_ms = start.elapsed().as_secs_f64() * 1000.0;
                
                if response.status().is_success() {
                    debug!("Service {} is healthy ({:.2}ms)", name, response_time_ms);
                    ServiceHealth {
                        name: name.to_string(),
                        status: HealthStatus::Healthy,
                        url: base_url.to_string(),
                        last_check: Utc::now().to_rfc3339(),
                        response_time_ms: Some(response_time_ms),
                        error_message: None,
                    }
                } else {
                    error!("Service {} returned status {}", name, response.status());
                    ServiceHealth {
                        name: name.to_string(),
                        status: HealthStatus::Unhealthy,
                        url: base_url.to_string(),
                        last_check: Utc::now().to_rfc3339(),
                        response_time_ms: Some(response_time_ms),
                        error_message: Some(format!("HTTP {}", response.status())),
                    }
                }
            }
            Err(e) => {
                error!("Service {} health check failed: {}", name, e);
                ServiceHealth {
                    name: name.to_string(),
                    status: HealthStatus::Unhealthy,
                    url: base_url.to_string(),
                    last_check: Utc::now().to_rfc3339(),
                    response_time_ms: None,
                    error_message: Some(e.to_string()),
                }
            }
        }
    }

    /// Get current cached pipeline health
    pub async fn get_pipeline_health(&self) -> PipelineHealth {
        self.cache.read().await.clone()
    }

    /// Get individual service health from cache
    pub async fn get_service_health(&self, service_name: &str) -> Option<ServiceHealth> {
        let pipeline = self.cache.read().await;
        pipeline.services.iter()
            .find(|s| s.name == service_name)
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_status_equality() {
        assert_eq!(HealthStatus::Healthy, HealthStatus::Healthy);
        assert_ne!(HealthStatus::Healthy, HealthStatus::Unhealthy);
    }

    #[test]
    fn test_service_health_structure() {
        let health = ServiceHealth {
            name: "test-service".to_string(),
            status: HealthStatus::Healthy,
            url: "http://test:8080".to_string(),
            last_check: Utc::now().to_rfc3339(),
            response_time_ms: Some(15.5),
            error_message: None,
        };
        
        assert_eq!(health.name, "test-service");
        assert_eq!(health.status, HealthStatus::Healthy);
    }
}
