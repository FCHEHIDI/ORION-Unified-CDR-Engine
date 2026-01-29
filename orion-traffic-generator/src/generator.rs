use crate::config::Config;
use crate::metrics;
use anyhow::Result;
use chrono::Utc;
use rand::Rng;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use serde_json::json;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing;
use uuid::Uuid;

pub struct TrafficGenerator {
    producer: FutureProducer,
    config: Config,
}

impl TrafficGenerator {
    pub fn new(config: Config) -> Result<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", config.kafka.brokers.join(","))
            .set("message.timeout.ms", "5000")
            .create()?;

        Ok(Self { producer, config })
    }

    pub async fn run(&self) {
        tracing::info!("Starting traffic generator");
        let mut burst_mode = false;
        let mut last_burst = Instant::now();

        loop {
            let start = Instant::now();

            // Burst mode logic
            if self.config.generation.burst_enabled {
                let elapsed = last_burst.elapsed().as_secs();
                if elapsed > 300 && !burst_mode {
                    // Every 5 minutes, burst for 30 seconds
                    burst_mode = true;
                    last_burst = Instant::now();
                    tracing::info!("🚀 Burst mode activated");
                } else if burst_mode && elapsed > self.config.generation.burst_duration_secs {
                    burst_mode = false;
                    tracing::info!("⏸️  Burst mode deactivated");
                }
            }

            let rate = if burst_mode {
                self.config.generation.rate_per_second * self.config.generation.burst_multiplier
            } else {
                self.config.generation.rate_per_second
            };

            // Generate CDR batch
            for _ in 0..rate {
                if let Err(e) = self.generate_and_send_cdr().await {
                    tracing::error!("Failed to generate/send CDR: {}", e);
                    metrics::increment_errors();
                }
            }

            metrics::record_latency(start.elapsed().as_secs_f64());

            // Sleep to maintain rate
            sleep(Duration::from_secs(1)).await;
        }
    }

    async fn generate_and_send_cdr(&self) -> Result<()> {
        metrics::increment_cdr_generated();

        // Simulate latency if enabled
        if self.config.simulation.enable_latency {
            self.simulate_latency().await;
        }

        // Simulate random errors
        if self.config.simulation.enable_errors && self.should_trigger_error() {
            metrics::increment_errors();
            anyhow::bail!("Simulated Kafka error (firewall/timeout)");
        }

        // Generate CDR
        let cdr = if self.should_generate_malformed() {
            metrics::increment_malformed();
            self.generate_malformed_cdr()
        } else {
            let is_fraud = self.should_generate_fraud();
            if is_fraud {
                metrics::increment_fraud();
            }
            self.generate_cdr(is_fraud)
        };

        // Send to Kafka with retry
        self.send_with_retry(&cdr).await?;

        Ok(())
    }

    async fn send_with_retry(&self, cdr: &serde_json::Value) -> Result<()> {
        let country = cdr["country"].as_str().unwrap_or("FR");
        let topic = format!("{}.{}", self.config.kafka.base_topic, country);
        let payload = serde_json::to_string(cdr)?;

        let mut attempts = 0;
        let max_attempts = if self.config.simulation.enable_retry {
            self.config.simulation.max_retries
        } else {
            1
        };

        loop {
            let start = Instant::now();
            let result = self
                .producer
                .send(
                    FutureRecord::to(&topic).payload(&payload).key(&Uuid::new_v4().to_string()),
                    Duration::from_secs(5),
                )
                .await;

            metrics::record_kafka_latency(start.elapsed().as_secs_f64());

            match result {
                Ok(_) => {
                    metrics::increment_cdr_sent();
                    return Ok(());
                }
                Err((e, _)) => {
                    attempts += 1;
                    if attempts >= max_attempts {
                        return Err(anyhow::anyhow!("Kafka send failed after {} attempts: {:?}", attempts, e));
                    }
                    metrics::increment_retries();
                    tracing::warn!("Retry {}/{} after error: {:?}", attempts, max_attempts, e);
                    sleep(Duration::from_millis(100 * (2_u64.pow(attempts)))).await; // Exponential backoff
                }
            }
        }
    }

    fn generate_cdr(&self, is_fraud: bool) -> serde_json::Value {
        let mut rng = rand::thread_rng();

        // Random country
        let countries = ["FR", "TN", "FN", "CH"];
        let country = countries[rng.gen_range(0..countries.len())];

        // MCC/MNC mapping
        let (mcc, mnc) = match country {
            "FR" => ("208", ["01", "15", "20"][rng.gen_range(0..3)]),
            "TN" => ("605", "01"),
            "FN" => ("244", "05"),
            "CH" => ("228", "01"),
            _ => ("208", "01"),
        };

        // Random event type
        let event_types = ["voice", "data", "sms"];
        let event_type = event_types[rng.gen_range(0..event_types.len())];

        let imsi = format!("{}{}{:010}", mcc, mnc, rng.gen_range(1000000000..9999999999u64));
        let msisdn = format!("+{}{:09}", mcc, rng.gen_range(100000000..999999999));

        let mut cdr = json!({
            "event_type": event_type,
            "imsi": imsi,
            "msisdn": msisdn,
            "country": country,
            "timestamp": Utc::now().to_rfc3339(),
        });

        // Event-specific fields
        match event_type {
            "voice" => {
                let duration = if is_fraud {
                    rng.gen_range(7200..14400) // Fraud: 2-4 hours
                } else {
                    rng.gen_range(30..900) // Normal: 30s-15min
                };
                cdr["duration"] = json!(duration);
                cdr["called_number"] = json!(format!("+{}{:09}", mcc, rng.gen_range(100000000..999999999)));
                cdr["call_type"] = json!(if is_fraud { "international" } else { "domestic" });
            }
            "data" => {
                let bytes = if is_fraud {
                    rng.gen_range(15_000_000_000..50_000_000_000u64) // Fraud: 15-50GB
                } else {
                    rng.gen_range(1_000_000..500_000_000) // Normal: 1MB-500MB
                };
                cdr["bytes_uploaded"] = json!(bytes / 5);
                cdr["bytes_downloaded"] = json!(bytes * 4 / 5);
                cdr["apn"] = json!("internet.example.com");
            }
            "sms" => {
                cdr["destination_number"] = json!(format!("+{}{:09}", mcc, rng.gen_range(100000000..999999999)));
                cdr["sms_type"] = json!("text");
                cdr["message_length"] = json!(rng.gen_range(10..160));
            }
            _ => {}
        }

        // Fraud: suspicious roaming
        if is_fraud && rng.gen_bool(0.5) {
            cdr["is_roaming"] = json!(true);
            cdr["visited_country"] = json!("RU"); // High-risk country
        }

        cdr
    }

    fn generate_malformed_cdr(&self) -> serde_json::Value {
        let mut rng = rand::thread_rng();

        match rng.gen_range(0..4) {
            0 => json!({"invalid": "missing required fields"}), // Missing fields
            1 => json!({"imsi": "INVALID", "msisdn": "WRONG"}), // Invalid format
            2 => json!({"event_type": "unknown", "imsi": "12345"}), // Unknown event type
            _ => json!("not even an object"), // Invalid JSON structure
        }
    }

    async fn simulate_latency(&self) {
        let latency_ms = rand::thread_rng().gen_range(
            self.config.simulation.min_latency_ms..=self.config.simulation.max_latency_ms,
        );
        sleep(Duration::from_millis(latency_ms)).await;
    }

    fn should_generate_fraud(&self) -> bool {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..100) < self.config.generation.fraud_rate_percent
    }

    fn should_generate_malformed(&self) -> bool {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..100) < self.config.generation.malformed_rate_percent
    }

    fn should_trigger_error(&self) -> bool {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..100) < self.config.simulation.error_rate_percent
    }
}

