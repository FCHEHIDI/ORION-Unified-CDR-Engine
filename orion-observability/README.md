# 📊 ORION Observability Service

**Centralized health monitoring and metrics aggregation for the ORION CDR pipeline**

## 🎯 Purpose

Monitors the health of all ORION microservices, aggregates their status, and provides unified observability endpoints for operations teams. Acts as a single pane of glass for pipeline health.

## 🏗️ Architecture

```
┌─────────────────────────────────────────┐
│     orion-observability                 │
├─────────────────────────────────────────┤
│                                         │
│  Background Health Checker              │
│    ↓                                    │
│  Periodic HTTP Health Checks            │
│    ├─ traffic-generator                 │
│    ├─ ingestion                         │
│    ├─ validation                        │
│    ├─ normalization                     │
│    ├─ enrichment                        │
│    ├─ ml-fraud-agent                    │
│    └─ storage-hot                       │
│    ↓                                    │
│  Aggregate Health Status                │
│    ↓                                    │
│  REST API + Prometheus Metrics          │
│                                         │
└─────────────────────────────────────────┘
```

## 🔌 API Endpoints

### Self Health Check
```bash
GET /health
```

Response:
```json
{
  "status": "healthy",
  "service": "orion-observability",
  "version": "0.1.0"
}
```

### Pipeline Aggregated Health
```bash
GET /pipeline/health
```

Response:
```json
{
  "status": "Healthy",
  "services": [
    {
      "name": "ingestion",
      "status": "Healthy",
      "url": "http://orion-ingestion:8081",
      "last_check": "2026-01-29T18:30:00Z",
      "response_time_ms": 12.5,
      "error_message": null
    },
    {
      "name": "ml-fraud-agent",
      "status": "Unhealthy",
      "url": "http://orion-ml-fraud-agent:8090",
      "last_check": "2026-01-29T18:30:00Z",
      "response_time_ms": null,
      "error_message": "Connection refused"
    }
  ],
  "healthy_count": 6,
  "total_count": 7,
  "timestamp": "2026-01-29T18:30:00Z"
}
```

### Individual Service Health
```bash
GET /services/{service-name}/health
```

Example:
```bash
GET /services/ingestion/health
```

Response:
```json
{
  "name": "ingestion",
  "status": "Healthy",
  "url": "http://orion-ingestion:8081",
  "last_check": "2026-01-29T18:30:00Z",
  "response_time_ms": 12.5,
  "error_message": null
}
```

### Metrics (Prometheus)
```bash
GET /metrics
```

## 📊 Monitored Services

| Service | Default URL | Port |
|---------|-------------|------|
| traffic-generator | http://orion-traffic-generator:9200 | 9200 |
| ingestion | http://orion-ingestion:8081 | 8081 |
| validation | http://orion-validation:8082 | 8082 |
| normalization | http://orion-normalization:8083 | 8083 |
| enrichment | http://orion-enrichment:8084 | 8084 |
| ml-fraud-agent | http://orion-ml-fraud-agent:8090 | 8090 |
| storage-hot | http://orion-storage-hot:8085 | 8085 |

## ⚙️ Configuration

Environment variables:

```bash
# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=9100

# Health check settings
HEALTH_CHECK_INTERVAL_SECS=30

# Service URLs (optional overrides)
TRAFFIC_GENERATOR_URL=http://orion-traffic-generator:9200
INGESTION_URL=http://orion-ingestion:8081
VALIDATION_URL=http://orion-validation:8082
NORMALIZATION_URL=http://orion-normalization:8083
ENRICHMENT_URL=http://orion-enrichment:8084
ML_FRAUD_AGENT_URL=http://orion-ml-fraud-agent:8090
STORAGE_HOT_URL=http://orion-storage-hot:8085

# Logging
RUST_LOG=info
```

## 🚀 Usage

### Development
```bash
# Build
cargo build --release

# Run tests
cargo test

# Run service
cargo run
```

### Docker
```bash
# Build image
docker build -t orion-observability .

# Run container
docker run -p 9100:9100 \
  -e HEALTH_CHECK_INTERVAL_SECS=30 \
  orion-observability
```

### Test the API
```bash
# Self health
curl http://localhost:9100/health

# Pipeline health
curl http://localhost:9100/pipeline/health | jq

# Specific service
curl http://localhost:9100/services/ingestion/health | jq

# Metrics
curl http://localhost:9100/metrics | grep observability
```

## 📈 Metrics

Prometheus metrics exposed at `/metrics`:

- `observability_service_healthy{service="<name>"}` - Health status per service (1=healthy, 0=unhealthy)
- `observability_health_checks_total{service="<name>"}` - Total health checks performed
- `observability_health_check_failures_total{service="<name>"}` - Failed health checks
- `observability_pipeline_services_up` - Number of healthy services
- `observability_pipeline_services_total` - Total monitored services

## 🧪 Testing

```bash
# Unit tests
cargo test

# Integration test (requires services running)
curl http://localhost:9100/pipeline/health
```

## 🔄 Integration

### Grafana Dashboard Query Examples

```promql
# Pipeline health percentage
(observability_pipeline_services_up / observability_pipeline_services_total) * 100

# Service availability over time
avg_over_time(observability_service_healthy{service="ingestion"}[5m])

# Failed health checks rate
rate(observability_health_check_failures_total[5m])
```

### Alerting Rules

```yaml
# Alert when any service is down
- alert: ServiceDown
  expr: observability_service_healthy == 0
  for: 2m
  annotations:
    summary: "Service {{ $labels.service }} is down"

# Alert when pipeline is degraded
- alert: PipelineDegraded
  expr: (observability_pipeline_services_up / observability_pipeline_services_total) < 0.8
  for: 5m
  annotations:
    summary: "ORION pipeline degraded"
```

## 🎯 Features

- **Automatic Discovery**: Monitors all configured ORION services
- **Cached Results**: Minimizes load on monitored services
- **Timeout Protection**: 5-second timeout per health check
- **Background Monitoring**: Non-blocking health checks
- **Prometheus Integration**: Standard metrics format
- **RESTful API**: Easy integration with dashboards

## 📝 Notes

- Health checks run every 30 seconds by default
- Failed health checks are retried on next interval
- Service timeouts don't block other checks
- All timestamps are in RFC3339 format (UTC)
- Response time metrics in milliseconds

## 🚧 Future Enhancements

- [ ] Configurable health check endpoints
- [ ] Service dependency graph
- [ ] Historical health data storage
- [ ] Alerting webhook integration
- [ ] Service auto-discovery from Kubernetes
- [ ] Custom health check logic per service
