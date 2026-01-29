# create_orion_env.ps1
# Génère un fichier .env standardisé pour ORION

$envFile = "./.env"

$content = @"
# ============================================
# ORION Unified CDR Engine - Environment File
# ============================================

# ----------- Kafka -----------
KAFKA_BROKERS=localhost:9092
KAFKA_INPUT_TOPIC=cdr.raw
KAFKA_OUTPUT_TOPIC=cdr.processed

# ----------- ScyllaDB --------
SCYLLA_HOSTS=localhost
SCYLLA_PORT=9042
SCYLLA_KEYSPACE=orion

# ----------- MinIO / Ceph ----
CEPH_ENDPOINT=http://localhost:9000
CEPH_ACCESS_KEY=admin
CEPH_SECRET_KEY=admin123
CEPH_BUCKET=orion

# ----------- ML Agent --------
ML_ENDPOINT=http://localhost:50051
ML_TIMEOUT_MS=50

# ----------- API -------------
API_PORT=8080

# ----------- Logging ---------
RUST_LOG=info

# ----------- Countries (demo) --------
ORION_COUNTRIES=fr,be,pl,ma,tn,eg,ci,sn,cm,mg
"@

Set-Content -Path $envFile -Value $content

Write-Host ".env ORION généré avec succès."
