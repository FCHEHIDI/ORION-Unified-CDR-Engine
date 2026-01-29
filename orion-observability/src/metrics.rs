use metrics::{counter, describe_counter, describe_gauge, gauge};

/// Initialize all metrics with descriptions
pub fn init_metrics() {
    // Service health metrics
    describe_gauge!(
        "observability_service_healthy",
        "Health status of each service (1=healthy, 0=unhealthy)"
    );
    
    describe_counter!(
        "observability_health_checks_total",
        "Total number of health checks performed"
    );
    
    describe_counter!(
        "observability_health_check_failures_total",
        "Total number of failed health checks"
    );
    
    // Aggregated pipeline metrics
    describe_gauge!(
        "observability_pipeline_services_up",
        "Number of pipeline services currently healthy"
    );
    
    describe_gauge!(
        "observability_pipeline_services_total",
        "Total number of pipeline services monitored"
    );
}

/// Record service health status
pub fn record_service_health(service_name: &str, is_healthy: bool) {
    let status = if is_healthy { 1.0 } else { 0.0 };
    gauge!("observability_service_healthy", "service" => service_name.to_string())
        .set(status);
    
    counter!("observability_health_checks_total", "service" => service_name.to_string())
        .increment(1);
    
    if !is_healthy {
        counter!("observability_health_check_failures_total", "service" => service_name.to_string())
            .increment(1);
    }
}

/// Record pipeline aggregate health
pub fn record_pipeline_health(healthy_count: usize, total_count: usize) {
    gauge!("observability_pipeline_services_up").set(healthy_count as f64);
    gauge!("observability_pipeline_services_total").set(total_count as f64);
}
