use anyhow::{Context, Result};
use arrow::array::{
    ArrayRef, BooleanArray, Float64Array, Int32Array, StringArray, TimestampMillisecondArray,
};
use arrow::record_batch::RecordBatch;
use chrono::{DateTime, Utc};
use chrono::Datelike;
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{info, warn};

use crate::schema::cdr_schema;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdrRecord {
    pub id: String,
    pub country_code: String,
    pub timestamp: DateTime<Utc>,
    pub duration_seconds: i32,
    pub call_type: String,
    pub msisdn_a: String,
    pub msisdn_b: String,
    pub cell_id: Option<String>,
    pub imsi: Option<String>,
    pub is_fraud: bool,
    pub fraud_score: Option<f64>,
}

pub struct ParquetWriter {
    base_path: PathBuf,
}

impl ParquetWriter {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }

    /// Generate partition path: year=YYYY/month=MM/day=DD/country=CC/
    fn partition_path(&self, record: &CdrRecord) -> PathBuf {
        let date = record.timestamp.date_naive();
        self.base_path
            .join(format!("year={}", date.year_ce().1))
            .join(format!("month={:02}", date.month0() + 1))
            .join(format!("day={:02}", date.day0() + 1))
            .join(format!("country={}", record.country_code))
    }

    /// Write batch of CDRs to Parquet file
    pub fn write_batch(&self, records: Vec<CdrRecord>) -> Result<PathBuf> {
        if records.is_empty() {
            warn!("Empty batch, skipping write");
            return Ok(PathBuf::new());
        }

        let partition_path = self.partition_path(&records[0]);
        std::fs::create_dir_all(&partition_path)
            .context("Failed to create partition directory")?;

        let timestamp = Utc::now().timestamp_millis();
        let file_path = partition_path.join(format!("cdr_{}.parquet", timestamp));

        let schema = cdr_schema();
        let batch = self.records_to_batch(&records, schema.clone())?;

        let file = File::create(&file_path)
            .context("Failed to create Parquet file")?;

        let props = WriterProperties::builder()
            .set_compression(parquet::basic::Compression::SNAPPY)
            .build();

        let mut writer = ArrowWriter::try_new(file, schema, Some(props))
            .context("Failed to create Arrow writer")?;

        writer.write(&batch)
            .context("Failed to write record batch")?;

        writer.close()
            .context("Failed to close writer")?;

        info!("Wrote {} records to {:?}", records.len(), file_path);
        Ok(file_path)
    }

    fn records_to_batch(&self, records: &[CdrRecord], schema: Arc<arrow::datatypes::Schema>) -> Result<RecordBatch> {
        let ids: ArrayRef = Arc::new(StringArray::from(
            records.iter().map(|r| r.id.as_str()).collect::<Vec<_>>(),
        ));

        let country_codes: ArrayRef = Arc::new(StringArray::from(
            records.iter().map(|r| r.country_code.as_str()).collect::<Vec<_>>(),
        ));

        let timestamps: ArrayRef = Arc::new(TimestampMillisecondArray::from(
            records.iter().map(|r| r.timestamp.timestamp_millis()).collect::<Vec<_>>(),
        ));

        let durations: ArrayRef = Arc::new(Int32Array::from(
            records.iter().map(|r| r.duration_seconds).collect::<Vec<_>>(),
        ));

        let call_types: ArrayRef = Arc::new(StringArray::from(
            records.iter().map(|r| r.call_type.as_str()).collect::<Vec<_>>(),
        ));

        let msisdn_as: ArrayRef = Arc::new(StringArray::from(
            records.iter().map(|r| r.msisdn_a.as_str()).collect::<Vec<_>>(),
        ));

        let msisdn_bs: ArrayRef = Arc::new(StringArray::from(
            records.iter().map(|r| r.msisdn_b.as_str()).collect::<Vec<_>>(),
        ));

        let cell_ids: ArrayRef = Arc::new(StringArray::from(
            records.iter().map(|r| r.cell_id.as_deref()).collect::<Vec<_>>(),
        ));

        let imsis: ArrayRef = Arc::new(StringArray::from(
            records.iter().map(|r| r.imsi.as_deref()).collect::<Vec<_>>(),
        ));

        let is_frauds: ArrayRef = Arc::new(BooleanArray::from(
            records.iter().map(|r| r.is_fraud).collect::<Vec<_>>(),
        ));

        let fraud_scores: ArrayRef = Arc::new(Float64Array::from(
            records.iter().map(|r| r.fraud_score).collect::<Vec<_>>(),
        ));

        RecordBatch::try_new(
            schema,
            vec![
                ids,
                country_codes,
                timestamps,
                durations,
                call_types,
                msisdn_as,
                msisdn_bs,
                cell_ids,
                imsis,
                is_frauds,
                fraud_scores,
            ],
        )
        .context("Failed to create record batch")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use tempfile::TempDir;

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

    #[test]
    fn test_partition_path() {
        let temp_dir = TempDir::new().unwrap();
        let writer = ParquetWriter::new(temp_dir.path().to_path_buf());
        let record = sample_record();

        let path = writer.partition_path(&record);
        assert!(path.to_string_lossy().contains("year=2024"));
        assert!(path.to_string_lossy().contains("month=01"));
        assert!(path.to_string_lossy().contains("day=15"));
        assert!(path.to_string_lossy().contains("country=US"));
    }

    #[test]
    fn test_write_batch() {
        let temp_dir = TempDir::new().unwrap();
        let writer = ParquetWriter::new(temp_dir.path().to_path_buf());
        let records = vec![sample_record()];

        let result = writer.write_batch(records);
        assert!(result.is_ok());

        let file_path = result.unwrap();
        assert!(file_path.exists());
        assert!(file_path.extension().unwrap() == "parquet");
    }
}
