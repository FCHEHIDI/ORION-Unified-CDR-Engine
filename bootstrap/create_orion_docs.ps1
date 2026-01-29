# create_orion_docs.ps1
# Génère la documentation Markdown complète d'ORION

$root = "./docs"
New-Item -ItemType Directory -Force -Path $root | Out-Null

# ----------- ARCHITECTURE -----------
$architecture = @"
# ORION - Architecture Globale

ORION est une plateforme unifiée de traitement CDR multi-pays, composée de :

- Ingestion Kafka
- Validation
- Normalisation
- Enrichment (ML)
- Stockage (Scylla + MinIO)
- API
- Observabilité (Prometheus + Grafana)
- Déploiement (Docker, Kubernetes, Helm)

Chaque microservice est indépendant, modulaire et scalable.
"@
Set-Content "$root/ARCHITECTURE.md" $architecture

# ----------- PIPELINE -----------
$pipeline = @"
# ORION - Pipeline CDR

1. Ingestion Kafka
2. Validation syntaxique et métier
3. Normalisation multi-pays
4. Enrichissement ML (fraude)
5. Stockage Hot (Scylla)
6. Stockage Cold (MinIO)
7. Exposition API
"@
Set-Content "$root/PIPELINE.md" $pipeline

# ----------- SERVICES -----------
$services = @"
# ORION - Microservices

- orion-ingestion
- orion-validation
- orion-normalization
- orion-enrichment
- orion-ml-fraud-agent
- orion-storage-hot
- orion-storage-cold
- orion-api
- orion-observability

Chaque service expose :
- /health
- /metrics (Prometheus)
"@
Set-Content "$root/SERVICES.md" $services

# ----------- DEPLOYMENT -----------
$deployment = @"
# ORION - Déploiement

## Docker Compose
docker compose up -d

## Kubernetes
kubectl apply -f k8s/

## Helm
helm install orion ./helm/orion -n orion
"@
Set-Content "$root/DEPLOYMENT.md" $deployment

# ----------- DEMO GUIDE -----------
$demo = @"
# ORION - Guide de Démo

1. Générer les CDR
   make cdr

2. Envoyer dans Kafka
   make kafka

3. Vérifier ingestion
   Grafana → Dashboard Ingestion

4. Vérifier pipeline
   Grafana → Pipeline CDR

5. Vérifier stockage
   Scylla + MinIO

6. Vérifier API
   https://api.orion.local
"@
Set-Content "$root/DEMO_GUIDE.md" $demo

Write-Host "Documentation ORION générée avec succès."
