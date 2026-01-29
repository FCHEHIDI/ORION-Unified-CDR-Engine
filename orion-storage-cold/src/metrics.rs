use prometheus::{IntCounter, Registry, TextEncoder, Encoder};
use std::sync::Arc;

#[derive(Clone)]
pub struct Metrics {
    pub records_archived: IntCounter,
    pub files_uploaded: IntCounter,
    pub upload_errors: IntCounter,
    #[allow(dead_code)] // Used for metrics export endpoint
    registry: Arc<Registry>,
}

impl Metrics {
    pub fn new() -> Self {
        let registry = Arc::new(Registry::new());

        let records_archived = IntCounter::new(
            "orion_storage_cold_records_archived_total",
            "Total CDR records archived to cold storage",
        )
        .unwrap();

        let files_uploaded = IntCounter::new(
            "orion_storage_cold_files_uploaded_total",
            "Total Parquet files uploaded to S3",
        )
        .unwrap();

        let upload_errors = IntCounter::new(
            "orion_storage_cold_upload_errors_total",
            "Total S3 upload errors",
        )
        .unwrap();

        registry.register(Box::new(records_archived.clone())).unwrap();
        registry.register(Box::new(files_uploaded.clone())).unwrap();
        registry.register(Box::new(upload_errors.clone())).unwrap();

        Self {
            records_archived,
            files_uploaded,
            upload_errors,
            registry,
        }
    }

    #[allow(dead_code)] // Used for Prometheus metrics endpoint
    pub fn export(&self) -> String {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }
}
