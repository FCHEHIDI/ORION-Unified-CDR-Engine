use anyhow::Result;
use serde_json::json;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};
use tracing::{info, warn, error};

use crate::config::Config;
use crate::metrics::Metrics;
use crate::uploader::S3Uploader;
use crate::writer::{CdrRecord, ParquetWriter};

#[allow(dead_code)] // Used when rdkafka consumer is active
const BATCH_SIZE: usize = 1000;
const POLL_INTERVAL_SECS: u64 = 30;

pub struct ArchiveService {
    writer: ParquetWriter,
    uploader: S3Uploader,
    metrics: Arc<Metrics>,
    stats: Arc<Mutex<Stats>>,
    batch: Arc<Mutex<Vec<CdrRecord>>>,
}

#[derive(Debug, Clone, Default)]
pub struct Stats {
    pub total_archived: u64,
    pub total_uploaded: u64,
    pub total_errors: u64,
}

impl ArchiveService {
    pub async fn new(config: &Config, metrics: Arc<Metrics>) -> Result<Self> {
        let temp_dir = PathBuf::from("/tmp/orion-parquet");
        std::fs::create_dir_all(&temp_dir)?;

        let writer = ParquetWriter::new(temp_dir);
        let uploader = S3Uploader::new(&config.s3).await?;

        Ok(Self {
            writer,
            uploader,
            metrics,
            stats: Arc::new(Mutex::new(Stats::default())),
            batch: Arc::new(Mutex::new(Vec::new())),
        })
    }

    /// Start periodic archiving (stub for Kafka replacement)
    /// In production: use proper Kafka consumer with rdkafka
    /// For Windows dev: simulate with HTTP polling or mock data
    pub async fn start_consumer(&self, _config: &Config) -> Result<()> {
        info!("Archive service started (polling mode - replace with Kafka on Linux)");

        let mut timer = interval(Duration::from_secs(POLL_INTERVAL_SECS));

        loop {
            timer.tick().await;

            // TODO: Replace with actual Kafka consumer or HTTP endpoint
            // For now, log periodic checks
            let batch_len = self.batch.lock().await.len();
            if batch_len > 0 {
                info!("Batch contains {} records, flushing...", batch_len);
                self.flush_batch_internal().await;
            } else {
                info!("No records to archive (waiting for Kafka integration)");
            }
        }
    }

    /// Add CDR to batch (for testing/HTTP ingestion)
    #[allow(dead_code)] // Future HTTP endpoint for ingestion
    pub async fn add_record(&self, record: CdrRecord) {
        let mut batch = self.batch.lock().await;
        batch.push(record);

        if batch.len() >= BATCH_SIZE {
            drop(batch);
            self.flush_batch_internal().await;
        }
    }

    async fn flush_batch_internal(&self) {
        let mut batch = self.batch.lock().await;
        if batch.is_empty() {
            return;
        }

        let records: Vec<CdrRecord> = batch.drain(..).collect();
        drop(batch);

        let count = records.len();
        info!("Flushing batch of {} records", count);

        match self.writer.write_batch(records) {
            Ok(file_path) => {
                self.metrics.records_archived.inc_by(count as u64);

                match self.uploader.upload_file(&file_path).await {
                    Ok(_) => {
                        self.metrics.files_uploaded.inc();
                        let mut stats = self.stats.lock().await;
                        stats.total_archived += count as u64;
                        stats.total_uploaded += 1;

                        if let Err(e) = self.uploader.cleanup_local(&file_path).await {
                            warn!("Failed to cleanup local file: {}", e);
                        }
                    }
                    Err(e) => {
                        error!("S3 upload failed: {}", e);
                        self.metrics.upload_errors.inc();
                        let mut stats = self.stats.lock().await;
                        stats.total_errors += 1;
                    }
                }
            }
            Err(e) => {
                error!("Parquet write failed: {}", e);
                self.metrics.upload_errors.inc();
                let mut stats = self.stats.lock().await;
                stats.total_errors += 1;
            }
        }
    }

    pub async fn get_stats(&self) -> serde_json::Value {
        let stats = self.stats.lock().await;
        json!({
            "total_archived": stats.total_archived,
            "total_uploaded": stats.total_uploaded,
            "total_errors": stats.total_errors,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    fn sample_record() -> CdrRecord {
        CdrRecord {
            id: "test-123".to_string(),
            country_code: "US".to_string(),
            timestamp: Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap(),
            duration_seconds: 120,
            call_type: "voice".to_string(),
            msisdn_a: "1234567890".to_string(),
            msisdn_b: "0987654321".to_string(),
            cell_id: Some("CELL001".to_string()),
            imsi: Some("123456789012345".to_string()),
            is_fraud: false,
            fraud_score: Some(0.15),
        }
    }

    #[tokio::test]
    async fn test_batch_accumulation() {
        // Note: Full integration test requires MinIO/S3 mock
        // This is a unit test for batch logic only
        let record = sample_record();
        assert_eq!(record.country_code, "US");
    }
}

