# create_orion_grafana_dashboards.ps1
# Génère des dashboards Grafana JSON pour ORION

$root = "./grafana_dashboards"
New-Item -ItemType Directory -Force -Path $root | Out-Null

# ---- Dashboard 1 : Ingestion ----
$ingestion = @"
{
  "title": "ORION - Ingestion",
  "panels": [
    {
      "type": "graph",
      "title": "Débit CDR (messages/s)",
      "targets": [
        { "expr": "rate(orion_ingestion_cdr_total[1m])" }
      ]
    },
    {
      "type": "graph",
      "title": "Latence Ingestion (ms)",
      "targets": [
        { "expr": "histogram_quantile(0.95, sum(rate(orion_ingestion_latency_bucket[5m])) by (le))" }
      ]
    },
    {
      "type": "graph",
      "title": "Erreurs Ingestion",
      "targets": [
        { "expr": "increase(orion_ingestion_errors_total[5m])" }
      ]
    }
  ]
}
"@
Set-Content -Path "$root/orion_ingestion.json" -Value $ingestion

# ---- Dashboard 2 : Pipeline ----
$pipeline = @"
{
  "title": "ORION - Pipeline CDR",
  "panels": [
    {
      "type": "graph",
      "title": "Validation - Erreurs",
      "targets": [
        { "expr": "increase(orion_validation_errors_total[5m])" }
      ]
    },
    {
      "type": "graph",
      "title": "Normalisation - Latence (ms)",
      "targets": [
        { "expr": "histogram_quantile(0.95, sum(rate(orion_normalization_latency_bucket[5m])) by (le))" }
      ]
    },
    {
      "type": "graph",
      "title": "Enrichment - Appels ML/s",
      "targets": [
        { "expr": "rate(orion_enrichment_ml_calls_total[1m])" }
      ]
    }
  ]
}
"@
Set-Content -Path "$root/orion_pipeline.json" -Value $pipeline

# ---- Dashboard 3 : Infrastructure ----
$infra = @"
{
  "title": "ORION - Infrastructure",
  "panels": [
    {
      "type": "graph",
      "title": "Kafka - Messages In",
      "targets": [
        { "expr": "rate(kafka_server_brokertopicmetrics_messagesin_total[1m])" }
      ]
    },
    {
      "type": "graph",
      "title": "Scylla - Latence Lecture",
      "targets": [
        { "expr": "histogram_quantile(0.95, sum(rate(scylla_storage_proxy_coordinator_read_latency_bucket[5m])) by (le))" }
      ]
    },
    {
      "type": "graph",
      "title": "CPU Services ORION (%)",
      "targets": [
        { "expr": "avg(rate(process_cpu_seconds_total[1m])) * 100" }
      ]
    }
  ]
}
"@
Set-Content -Path "$root/orion_infrastructure.json" -Value $infra

Write-Host "Dashboards Grafana ORION générés avec succès."
