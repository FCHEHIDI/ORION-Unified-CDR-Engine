# 🎬 Scripts de démonstration — ORION Unified CDR Engine

## 1. Objectif

Ce document présente les scripts PowerShell et Rust utilisés pour automatiser la démonstration end-to-end d'ORION.

## 2. Scripts disponibles

### 📦 2.1. Génération des CDR

`arborescences/create_orion_generate_cdr.ps1`

**Fonction** : Génère des CDR simulés multi-pays

```powershell
# Générer 10k CDR par pays
.\arborescences\create_orion_generate_cdr.ps1 -CountPerCountry 10000

# Générer avec distribution personnalisée
.\arborescences\create_orion_generate_cdr.ps1 `
    -CountPerCountry 5000 `
    -Countries @("FR", "TN", "MA", "SN") `
    -OutputDir "datasets"
```

**Sortie** :
```
datasets/
  FR/
    cdr_fr_20260129_001.csv
    cdr_fr_20260129_002.csv
  TN/
    cdr_tn_20260129_001.json
  MA/
    cdr_ma_20260129_001.csv
  SN/
    cdr_sn_20260129_001.txt
```

---

### 📨 2.2. Producteur Kafka

`arborescences/create_orion_kafka_producer.ps1`

**Fonction** : Injecte les CDR dans Kafka

```powershell
# Démarrage avec délai de 10ms entre messages
.\arborescences\create_orion_kafka_producer.ps1 -DelayMs 10

# Production massive (sans délai)
.\arborescences\create_orion_kafka_producer.ps1 -DelayMs 0 -BatchSize 1000
```

**Topics Kafka créés** :
- `cdr.raw.FR`
- `cdr.raw.TN`
- `cdr.raw.MA`
- `cdr.raw.SN`

---

### 🏗️ 2.3. Build des microservices

```powershell
# Build tous les crates
cargo build --release --workspace

# Build un seul microservice
cargo build --release -p orion-ingestion

# Avec optimisations maximales
cargo build --release --workspace
$env:RUSTFLAGS="-C target-cpu=native"
```

---

### 🚀 2.4. Démarrage du pipeline

**Option 1 : Docker Compose**

```powershell
# Démarrage complet
docker compose up -d

# Démarrage séquentiel (recommandé)
docker compose up -d zookeeper kafka scylla minio
Start-Sleep -Seconds 30
docker compose up -d prometheus grafana
docker compose up -d orion-ingestion orion-validation orion-normalization
docker compose up -d orion-enrichment orion-ml-fraud-agent
docker compose up -d orion-storage-hot orion-storage-cold orion-api
docker compose up -d orion-traffic-generator
```

**Option 2 : Systemd (RHEL)**

```bash
# Démarrage des services
sudo systemctl start orion-ingestion
sudo systemctl start orion-validation
sudo systemctl start orion-normalization
sudo systemctl start orion-enrichment
sudo systemctl start orion-ml-fraud-agent
sudo systemctl start orion-storage-hot
sudo systemctl start orion-storage-cold
sudo systemctl start orion-api

# Vérification statut
sudo systemctl status orion-*
```

**Option 3 : Orchestrateur PowerShell**

```powershell
# Exécution complète automatisée
.\arborescences\orion_orchestrator.ps1
```

---

### 🔍 2.5. Vérification du pipeline

**Health checks**

```powershell
# Script de vérification complet
function Test-OrionHealth {
    $services = @(
        @{Name="Ingestion"; Port=8081},
        @{Name="Validation"; Port=8082},
        @{Name="Normalization"; Port=8083},
        @{Name="Enrichment"; Port=8084},
        @{Name="ML Fraud Agent"; Port=8085},
        @{Name="Storage Hot"; Port=8086},
        @{Name="Storage Cold"; Port=8087},
        @{Name="API"; Port=8080}
    )
    
    foreach ($service in $services) {
        try {
            $response = Invoke-WebRequest -Uri "http://localhost:$($service.Port)/health" -TimeoutSec 2
            if ($response.StatusCode -eq 200) {
                Write-Host "✅ $($service.Name) : OK" -ForegroundColor Green
            }
        } catch {
            Write-Host "❌ $($service.Name) : FAILED" -ForegroundColor Red
        }
    }
}

Test-OrionHealth
```

**Métriques Prometheus**

```powershell
# Récupérer les métriques d'un service
Invoke-WebRequest -Uri "http://localhost:8081/metrics" | Select-Object -ExpandProperty Content

# Vérifier le débit d'ingestion
(Invoke-WebRequest "http://localhost:8081/metrics").Content | Select-String "orion_ingestion_rate"
```

---

### 📊 2.6. Monitoring en temps réel

**Lag Kafka**

```powershell
# Vérifier le lag des consumer groups
docker exec orion-kafka kafka-consumer-groups `
    --bootstrap-server localhost:9092 `
    --describe `
    --all-groups
```

**ScyllaDB**

```powershell
# Nombre de CDR ingérés
docker exec -it orion-scylla cqlsh -e "SELECT COUNT(*) FROM orion.cdr_by_imsi_day;"

# Derniers CDR
docker exec -it orion-scylla cqlsh -e "SELECT * FROM orion.cdr_by_imsi_day LIMIT 10;"
```

**MinIO (Cold Storage)**

```powershell
# Lister les objets via mc (MinIO Client)
docker run --rm --network orion_orion-net `
    minio/mc alias set orion http://minio:9000 orion orion_secret_123

docker run --rm --network orion_orion-net `
    minio/mc ls orion/orion-cdr-cold
```

---

### 📈 2.7. Génération de trafic continu

**Traffic generator**

```powershell
# Démarrage générateur
docker compose up -d orion-traffic-generator

# Avec rate personnalisé
docker compose run --rm orion-traffic-generator `
    --rate 5000 `
    --countries FR,TN,MA,SN `
    --duration 3600
```

**Simulation de pic de charge**

```powershell
# Script de stress test
function Start-OrionStressTest {
    param(
        [int]$DurationSeconds = 300,
        [int]$RatePerSecond = 10000
    )
    
    Write-Host "🔥 Démarrage stress test : $RatePerSecond CDR/sec pendant $DurationSeconds sec"
    
    docker compose run --rm orion-traffic-generator `
        --rate $RatePerSecond `
        --duration $DurationSeconds `
        --burst-mode
}

Start-OrionStressTest -DurationSeconds 600 -RatePerSecond 15000
```

---

### 🔎 2.8. Requêtes API

**Récupération CDR par IMSI**

```powershell
# CDR pour un IMSI spécifique
$imsi = "208150123456789"
$date = "2026-01-29"

Invoke-RestMethod -Uri "http://localhost:8080/api/v1/cdr/imsi/$imsi?date=$date" | ConvertTo-Json -Depth 5
```

**Récupération CDR par cellule**

```powershell
# CDR pour une cellule spécifique
$cellId = "FR1234"
$hour = "2026-01-29T10:00:00Z"

Invoke-RestMethod -Uri "http://localhost:8080/api/v1/cdr/cell/$cellId?hour=$hour" | ConvertTo-Json -Depth 5
```

**Score de fraude**

```powershell
# Score fraude pour un IMSI
$imsi = "208150123456789"

Invoke-RestMethod -Uri "http://localhost:8080/api/v1/fraud/score/$imsi" | ConvertTo-Json -Depth 5
```

---

### 📉 2.9. Dashboards Grafana

**Import automatique**

```powershell
# Import des dashboards via API Grafana
$grafanaUrl = "http://localhost:3000"
$apiKey = "admin:orion123"

$dashboards = Get-ChildItem -Path "configs/grafana/dashboards/*.json"

foreach ($dashboard in $dashboards) {
    $json = Get-Content $dashboard.FullName -Raw
    
    Invoke-RestMethod -Uri "$grafanaUrl/api/dashboards/db" `
        -Method Post `
        -Headers @{Authorization="Basic $([Convert]::ToBase64String([Text.Encoding]::ASCII.GetBytes($apiKey)))"} `
        -Body $json `
        -ContentType "application/json"
    
    Write-Host "✅ Dashboard importé : $($dashboard.Name)" -ForegroundColor Green
}
```

**Accès aux dashboards** :
- http://localhost:3000/d/orion-pipeline-overview
- http://localhost:3000/d/orion-ml-fraud-agent
- http://localhost:3000/d/orion-storage-metrics

---

### 🧹 2.10. Nettoyage

**Arrêt propre**

```powershell
# Arrêter gracefully
docker compose down

# Avec suppression des volumes
docker compose down -v

# Nettoyage complet
docker compose down --rmi all -v
docker system prune -af --volumes
```

**Reset ScyllaDB**

```powershell
# Supprimer toutes les données
docker exec -it orion-scylla cqlsh -e "TRUNCATE orion.cdr_by_imsi_day;"
docker exec -it orion-scylla cqlsh -e "TRUNCATE orion.cdr_by_cell_hour;"
docker exec -it orion-scylla cqlsh -e "TRUNCATE orion.cdr_by_country_day;"
```

---

## 3. Scénario de démo complet

### 📝 Script de démonstration guidée

```powershell
# demo.ps1

Write-Host "🚀 === ORION Unified CDR Engine - Démonstration ===" -ForegroundColor Cyan
Write-Host ""

# Étape 1 : Génération des CDR
Write-Host "📦 Étape 1 : Génération de 50k CDR multi-pays..." -ForegroundColor Yellow
.\arborescences\create_orion_generate_cdr.ps1 -CountPerCountry 50000
Write-Host "✅ CDR générés" -ForegroundColor Green
Start-Sleep -Seconds 2

# Étape 2 : Démarrage infrastructure
Write-Host "🏗️ Étape 2 : Démarrage de l'infrastructure..." -ForegroundColor Yellow
docker compose up -d zookeeper kafka scylla minio prometheus grafana
Write-Host "⏳ Attente initialisation (60 sec)..." -ForegroundColor Yellow
Start-Sleep -Seconds 60
Write-Host "✅ Infrastructure prête" -ForegroundColor Green

# Étape 3 : Démarrage pipeline ORION
Write-Host "⚙️ Étape 3 : Démarrage du pipeline ORION..." -ForegroundColor Yellow
docker compose up -d orion-ingestion orion-validation orion-normalization `
    orion-enrichment orion-ml-fraud-agent orion-storage-hot orion-storage-cold orion-api
Start-Sleep -Seconds 20
Write-Host "✅ Pipeline opérationnel" -ForegroundColor Green

# Étape 4 : Injection des CDR
Write-Host "📨 Étape 4 : Injection des CDR dans Kafka..." -ForegroundColor Yellow
.\arborescences\create_orion_kafka_producer.ps1 -DelayMs 5
Write-Host "✅ Injection terminée" -ForegroundColor Green

# Étape 5 : Monitoring
Write-Host "📊 Étape 5 : Surveillance du pipeline..." -ForegroundColor Yellow
Start-Sleep -Seconds 30

Write-Host ""
Write-Host "🔍 Vérification de la santé des services..." -ForegroundColor Cyan
Test-OrionHealth

Write-Host ""
Write-Host "📈 Vérification des CDR dans ScyllaDB..." -ForegroundColor Cyan
$count = docker exec -it orion-scylla cqlsh -e "SELECT COUNT(*) FROM orion.cdr_by_imsi_day;" | Select-String "count"
Write-Host "CDR stockés : $count" -ForegroundColor Green

Write-Host ""
Write-Host "✅ === Démonstration ORION terminée avec succès ===" -ForegroundColor Green
Write-Host ""
Write-Host "🌐 Accès aux interfaces :" -ForegroundColor Cyan
Write-Host "  - Grafana  : http://localhost:3000 (admin/orion123)" -ForegroundColor White
Write-Host "  - Prometheus : http://localhost:9090" -ForegroundColor White
Write-Host "  - API ORION : http://localhost:8080" -ForegroundColor White
Write-Host "  - MinIO    : http://localhost:9001 (orion/orion_secret_123)" -ForegroundColor White
```

---

## 4. Scripts de diagnostic

### 🔧 Diagnostic complet

```powershell
# diagnose.ps1

function Get-OrionDiagnostics {
    Write-Host "🔍 === Diagnostics ORION ===" -ForegroundColor Cyan
    
    # Services Docker
    Write-Host "`n📦 Services Docker :" -ForegroundColor Yellow
    docker compose ps
    
    # Kafka topics
    Write-Host "`n📨 Topics Kafka :" -ForegroundColor Yellow
    docker exec orion-kafka kafka-topics --list --bootstrap-server localhost:9092
    
    # Lag Kafka
    Write-Host "`n⏳ Lag Kafka :" -ForegroundColor Yellow
    docker exec orion-kafka kafka-consumer-groups --bootstrap-server localhost:9092 --describe --all-groups
    
    # ScyllaDB status
    Write-Host "`n🗄️ ScyllaDB Status :" -ForegroundColor Yellow
    docker exec orion-scylla nodetool status
    
    # CDR counts
    Write-Host "`n📊 Nombre de CDR :" -ForegroundColor Yellow
    docker exec -it orion-scylla cqlsh -e "SELECT COUNT(*) FROM orion.cdr_by_imsi_day;"
    
    # Métriques services
    Write-Host "`n📈 Métriques Ingestion :" -ForegroundColor Yellow
    (Invoke-WebRequest "http://localhost:8081/metrics").Content | Select-String "orion_" | Select-Object -First 10
}

Get-OrionDiagnostics
```

---

## 5. Références

- **Scénario détaillé** : [scenario.md](scenario.md)
- **Docker Compose** : [../05-deploiement/docker-local.md](../05-deploiement/docker-local.md)
- **Orchestrateur** : `arborescences/orion_orchestrator.ps1`

---

**ORION Demo Scripts** — _Automatisation de la démonstration end-to-end_
