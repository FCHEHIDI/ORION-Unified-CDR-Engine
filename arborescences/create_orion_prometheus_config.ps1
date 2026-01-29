# create_orion_prometheus_config.ps1
# Génère un fichier prometheus.yml pour scraper tous les services ORION

$promFile = "./prometheus.yml"

$content = @"
global:
  scrape_interval: 5s
  evaluation_interval: 5s

scrape_configs:

  # ---- ORION Microservices ----
  - job_name: 'orion-ingestion'
    static_configs:
      - targets: ['orion-ingestion:9100']

  - job_name: 'orion-validation'
    static_configs:
      - targets: ['orion-validation:9101']

  - job_name: 'orion-normalization'
    static_configs:
      - targets: ['orion-normalization:9102']

  - job_name: 'orion-enrichment'
    static_configs:
      - targets: ['orion-enrichment:9103']

  - job_name: 'orion-ml-fraud-agent'
    static_configs:
      - targets: ['orion-ml-fraud-agent:9104']

  - job_name: 'orion-storage-hot'
    static_configs:
      - targets: ['orion-storage-hot:9105']

  - job_name: 'orion-storage-cold'
    static_configs:
      - targets: ['orion-storage-cold:9106']

  - job_name: 'orion-api'
    static_configs:
      - targets: ['orion-api:9107']

  - job_name: 'orion-observability'
    static_configs:
      - targets: ['orion-observability:9108']

  # ---- Kafka ----
  - job_name: 'kafka'
    static_configs:
      - targets: ['kafka:7071']

  # ---- ScyllaDB ----
  - job_name: 'scylla'
    static_configs:
      - targets: ['scylla:9180']

  # ---- MinIO ----
  - job_name: 'minio'
    static_configs:
      - targets: ['minio:9000']

  # ---- Traffic Generator (désactivé pour l'instant) ----
  # - job_name: 'orion-traffic-generator'
  #   static_configs:
  #     - targets: ['traffic-generator:9200']
"@

Set-Content -Path $promFile -Value $content

Write-Host "prometheus.yml généré avec succès."
