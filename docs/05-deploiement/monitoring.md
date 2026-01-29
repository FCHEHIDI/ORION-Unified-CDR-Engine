📡 2) Monitoring — Prometheus & Grafana
📁 docs/05-deploiement/monitoring.md

📊 Monitoring ORION — Prometheus & Grafana
1. Objectif
Assurer :

la visibilité du pipeline,

la détection des anomalies,

le suivi des performances,

la supervision ML.

2. Metrics exposées par ORION
Chaque microservice Rust expose /metrics :

Ingestion
orion_ingestion_rate

orion_kafka_lag

Validation
orion_validation_errors_total

Enrichment
orion_enrichment_latency_ms

ML
orion_ml_inference_latency_ms

orion_ml_requests_total

Storage
orion_scylla_write_latency_ms

orion_ceph_write_latency_ms

3. Dashboards Grafana
Dashboard 1 : Pipeline Overview
ingestion rate

validation errors

enrichment latency

ML latency

Scylla write latency

Dashboard 2 : ML Fraud Agent
inference latency

score distribution

model version usage

Dashboard 3 : Storage
Scylla throughput

Ceph throughput

compaction metrics

4. Alerting
Alertes critiques
Kafka lag > seuil

ML latency > 50 ms

Scylla write latency > 20 ms

erreurs validation > seuil