# create_orion_configs.ps1

$services = @{
    "orion-ingestion"      = "ingestion.toml"
    "orion-validation"     = "validation.toml"
    "orion-normalization"  = "normalization.toml"
    "orion-enrichment"     = "enrichment.toml"
    "orion-ml-fraud-agent" = "ml.toml"
    "orion-storage-hot"    = "storage_hot.toml"
    "orion-storage-cold"   = "storage_cold.toml"
    "orion-api"            = "api.toml"
    "orion-observability"  = "observability.toml"
}

foreach ($svc in $services.Keys) {
    $configDir = "./$svc/config"
    New-Item -ItemType Directory -Force -Path $configDir | Out-Null

    $fileName = $services[$svc]
    $filePath = "$configDir/$fileName"

    $content = @"
# Configuration for $svc

[service]
name = "$svc"
log_level = "info"

[kafka]
brokers = ["localhost:9092"]
input_topic = "cdr.raw"
output_topic = "cdr.processed"

[scylla]
hosts = ["localhost"]
port = 9042
keyspace = "orion"

[ceph]
endpoint = "http://localhost:9000"
access_key = "admin"
secret_key = "admin123"
bucket = "orion"

[ml]
endpoint = "http://localhost:50051"
timeout_ms = 50
"@

    Set-Content -Path $filePath -Value $content
}

Write-Host "Fichiers de configuration ORION générés avec succès."
