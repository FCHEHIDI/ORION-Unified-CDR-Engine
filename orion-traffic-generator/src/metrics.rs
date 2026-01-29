use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::sync::OnceLock;

static PROM_HANDLE: OnceLock<PrometheusHandle> = OnceLock::new();

pub fn init_metrics() {
    let builder = PrometheusBuilder::new();
    let handle = builder.install().expect("failed to install Prometheus recorder");
    PROM_HANDLE.set(handle).unwrap();
}

pub async fn export_metrics() -> String {
    PROM_HANDLE.get().unwrap().render()
}
