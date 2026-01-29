# ORION Integration Stack

Guide complet pour lancer et tester l'infrastructure complète ORION.

## 🚀 Démarrage rapide

### Prérequis

- Docker Engine 24.0+
- Docker Compose V2
- 16 GB RAM minimum
- 50 GB disque disponible

### Lancer la stack complète

```bash
# Depuis la racine du projet
docker-compose up -d

# Vérifier les services
docker-compose ps

# Voir les logs
docker-compose logs -f
```

### Arrêt et nettoyage

```bash
# Arrêter proprement
docker-compose down

# Nettoyer volumes (⚠️ perte de données)
docker-compose down -v
```

## 📦 Services déployés

| Service | Port(s) | URL | Credentials |
|---------|---------|-----|-------------|
| **Infrastructure** |
| Zookeeper | 2181 | - | - |
| Kafka | 9092, 9093 | - | - |
| ScyllaDB | 9042, 19042 | - | - |
| **Pipeline ORION** |
| Traffic Generator | 9200 | http://localhost:9200 | - |
| Ingestion | 8081 | http://localhost:8081 | - |
| Validation | 8082 | http://localhost:8082 | - |
| Normalization | 8083 | http://localhost:8083 | - |
| Enrichment | 8084 | http://localhost:8084 | - |
| Storage Hot | 8085 | http://localhost:8085 | - |
| **Observability** |
| Prometheus | 9090 | http://localhost:9090 | - |
| Grafana | 3000 | http://localhost:3000 | admin / orion2026 |

## 🔍 Vérification du déploiement

### 1. Health checks

```bash
# Tous les services
for port in 9200 8081 8082 8083 8084 8085; do
  echo "Port $port: $(curl -s http://localhost:$port/health)"
done

# Kafka topics
docker exec orion-kafka kafka-topics --list --bootstrap-server localhost:9092

# ScyllaDB
docker exec orion-scylladb cqlsh -e "DESCRIBE KEYSPACES;"
```

### 2. Métriques Prometheus

```bash
# Vérifier scrape targets
curl -s http://localhost:9090/api/v1/targets | jq '.data.activeTargets[] | {job: .labels.job, health: .health}'

# Query exemple: taux génération CDR
curl -s "http://localhost:9090/api/v1/query?query=rate(traffic_generator_cdr_generated_total[1m])" | jq
```

### 3. Grafana Dashboards

1. Ouvrir http://localhost:3000
2. Login: `admin` / `orion2026`
3. Naviguer: **Dashboards → ORION**
4. Dashboards disponibles:
   - **Pipeline Overview**: Vue globale (throughput, latency, errors)
   - **Traffic Generator**: Génération CDR + simulation

## 🧪 Scénarios de test

### Test 1: Pipeline nominal

**Objectif**: Vérifier flux complet sans erreurs

```bash
# 1. Vérifier génération CDR
curl http://localhost:9200/metrics | grep traffic_generator_cdr_generated_total

# 2. Vérifier pipeline (5 min)
# Grafana → Overview → tous les graphes doivent monter

# 3. Query ScyllaDB
docker exec -it orion-scylladb cqlsh -e "
  SELECT COUNT(*) FROM orion_cdr.unified_cdr;
  SELECT * FROM orion_cdr.unified_cdr LIMIT 5;
"
```

**Résultats attendus**:
- ✅ 100 CDR/s générés
- ✅ Latence p95 < 100ms par stage
- ✅ Success rate > 95%
- ✅ CDR visibles dans ScyllaDB

### Test 2: Détection fraude

**Objectif**: Valider alertes fraude

```bash
# 1. Consulter CDR frauduleux détectés
curl http://localhost:8084/metrics | grep enrichment_fraud_detected_total

# 2. Query ScyllaDB pour risk_level HIGH
docker exec -it orion-scylladb cqlsh -e "
  SELECT imsi, risk_level, risk_score, fraud_reason 
  FROM orion_cdr.unified_cdr 
  WHERE risk_level = 'high' 
  ALLOW FILTERING;
"
```

**Résultats attendus**:
- ✅ ~10% fraude détectée (10 CDR/s)
- ✅ `risk_level = 'high'` avec `risk_score >= 0.7`
- ✅ `fraud_reason` non vide

### Test 3: Burst mode

**Objectif**: Tester scalabilité sous charge

```bash
# 1. Attendre burst automatique (toutes les 5 min)
# OU forcer burst via env var:
docker-compose exec orion-traffic-generator sh -c '
  export BURST_ENABLED=true
  export BURST_MULTIPLIER=10
  export GENERATION_RATE=500
'

# 2. Observer Grafana → Overview → Throughput
# Pic attendu: 5000 CDR/s (500 * 10) pendant 30s

# 3. Vérifier latence reste stable
curl http://localhost:9090/api/v1/query?query=histogram_quantile(0.95,rate(storage_insert_latency_seconds_bucket[1m]))
```

**Résultats attendus**:
- ✅ Pipeline absorbe 5000 CDR/s
- ✅ Latence p95 < 500ms (acceptable sous burst)
- ✅ Aucun service ne crash

### Test 4: Résilience erreurs

**Objectif**: Valider retry et backoff

```bash
# 1. Vérifier erreurs simulées
curl http://localhost:9200/metrics | grep traffic_generator_errors_total

# 2. Vérifier retries
curl http://localhost:9200/metrics | grep traffic_generator_retries_total

# 3. Ratio retry/error doit être > 1 (plusieurs tentatives)
```

**Résultats attendus**:
- ✅ ~3% erreurs (3 CDR/s)
- ✅ Retries > Errors (exponential backoff actif)
- ✅ CDR finalement envoyés après retry

### Test 5: CDR malformés

**Objectif**: Valider rejet données invalides

```bash
# 1. Vérifier CDR rejetés
curl http://localhost:8082/metrics | grep validation_invalid_cdr_total

# 2. Query Kafka topic rejected
docker exec orion-kafka kafka-console-consumer \
  --bootstrap-server localhost:9092 \
  --topic cdr.rejected \
  --from-beginning \
  --max-messages 5

# 3. Vérifier types d'erreurs
curl http://localhost:9200/metrics | grep traffic_generator_malformed_total
```

**Résultats attendus**:
- ✅ ~2% CDR malformés (2 CDR/s)
- ✅ Routés vers `cdr.rejected`
- ✅ Pas d'insertion dans ScyllaDB

## 📊 Dashboards Grafana

### Pipeline Overview

**Métriques clés**:
- **Throughput**: CDR/s à chaque étape (ingestion → storage)
- **Latency p95**: Temps traitement par microservice
- **Error rate**: % erreurs par stage
- **Success rate**: % CDR complétant le pipeline

**Alertes**:
- 🔴 Success rate < 90%
- 🟡 Latency p95 > 500ms
- 🔴 Error rate > 10%

### Traffic Generator

**Métriques clés**:
- **Generation rate**: CDR générés/s
- **Fraud rate**: % CDR frauduleux
- **Kafka latency**: Temps envoi vers Kafka
- **Retry rate**: Tentatives retry/s

## 🐛 Troubleshooting

### Kafka unreachable

**Symptômes**: Services ne démarrent pas, logs "Kafka connection failed"

**Solutions**:
```bash
# Vérifier Kafka
docker logs orion-kafka | tail -50

# Recréer topics
docker exec orion-kafka kafka-topics --create --topic cdr.raw.FR --bootstrap-server localhost:9092 --partitions 3
```

### ScyllaDB schema manquant

**Symptômes**: `storage-hot` crash avec "Keyspace not found"

**Solutions**:
```bash
# Init manuel schema
docker exec -it orion-scylladb cqlsh -e "
CREATE KEYSPACE IF NOT EXISTS orion_cdr 
WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 1};

CREATE TABLE IF NOT EXISTS orion_cdr.unified_cdr (
  cdr_id uuid PRIMARY KEY,
  imsi text,
  msisdn text,
  -- ... (voir orion-storage-hot/schema.cql)
);
"

# Restart storage
docker-compose restart orion-storage-hot
```

### Grafana dashboards vides

**Symptômes**: Grafana ne montre pas de données

**Solutions**:
```bash
# Vérifier datasource Prometheus
curl http://localhost:3000/api/datasources

# Tester query manuelle
curl "http://localhost:9090/api/v1/query?query=up"

# Reload Grafana config
docker-compose restart grafana
```

### Out of memory

**Symptômes**: Services killed, `docker-compose ps` shows `Exited (137)`

**Solutions**:
```bash
# Réduire consommation
# Dans docker-compose.yml, ajuster:
# - ScyllaDB: --memory 1G --smp 1
# - Kafka: KAFKA_HEAP_OPTS=-Xmx512M

# Augmenter Docker Desktop RAM
# Settings → Resources → Memory: 16 GB
```

## 🔧 Configuration avancée

### Tuning performance

```yaml
# docker-compose.override.yml
services:
  orion-traffic-generator:
    environment:
      GENERATION_RATE: 500  # 5x débit
      BURST_MULTIPLIER: 10   # Burst plus intense
  
  kafka:
    environment:
      KAFKA_NUM_PARTITIONS: 10  # Plus de parallélisme
```

### Monitoring custom

```yaml
# config/prometheus.yml
scrape_configs:
  - job_name: 'custom-app'
    static_configs:
      - targets: ['my-app:8080']
```

## 📚 Références

- [Architecture ORION](../docs/02-architecture/architecture-globale.md)
- [Monitoring](../docs/05-deploiement/monitoring.md)
- [Traffic Generator](./orion-traffic-generator/README.md)
- [Docker Local](../docs/05-deploiement/docker-local.md)

## 🚦 Prochaines étapes

1. **Exécuter Test 1-5**: Valider tous les scénarios
2. **Capturer screenshots**: Grafana dashboards pour démo
3. **Enregistrer vidéos**: Pipeline en action
4. **Benchmarks**: Mesurer limites scalabilité
5. **Production**: Adapter configs pour env réel
