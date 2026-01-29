# 🐳 Déploiement Docker Local — ORION Unified CDR Engine

## 1. Objectif

Ce document décrit le déploiement local d'ORION via **Docker Compose** pour :
- Démonstration end-to-end
- Développement et tests
- Validation du pipeline complet

## 2. Architecture Docker

```
┌─────────────────────────────────────────────────┐
│              docker-compose.yml                 │
├─────────────────────────────────────────────────┤
│                                                 │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐     │
│  │  Kafka   │  │ Zookeeper│  │ ScyllaDB │     │
│  │  :9092   │  │  :2181   │  │  :9042   │     │
│  └──────────┘  └──────────┘  └──────────┘     │
│                                                 │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐     │
│  │  MinIO   │  │Prometheus│  │ Grafana  │     │
│  │  :9000   │  │  :9090   │  │  :3000   │     │
│  └──────────┘  └──────────┘  └──────────┘     │
│                                                 │
│  ┌─────────────────────────────────────────┐   │
│  │     ORION Microservices (Rust)          │   │
│  │  - orion-ingestion                      │   │
│  │  - orion-validation                     │   │
│  │  - orion-normalization                  │   │
│  │  - orion-enrichment                     │   │
│  │  - orion-ml-fraud-agent                 │   │
│  │  - orion-storage-hot                    │   │
│  │  - orion-storage-cold                   │   │
│  │  - orion-api                            │   │
│  │  - orion-observability                  │   │
│  └─────────────────────────────────────────┘   │
│                                                 │
│  ┌──────────────────────────────────┐          │
│  │  orion-traffic-generator         │          │
│  │  (génération CDR simulés)        │          │
│  └──────────────────────────────────┘          │
└─────────────────────────────────────────────────┘
```

## 3. Prérequis

### 3.1. Logiciels requis
- **Docker** : version 24+ 
- **Docker Compose** : version 2.20+
- **Rust** : 1.75+ (pour build local)
- **PowerShell** : 7+ (Windows)
- **Ressources** :
  - CPU : 8+ cores
  - RAM : 16+ GB
  - Disque : 50+ GB

### 3.2. Vérification

```powershell
docker --version
docker compose version
rustc --version
```

## 4. Structure des fichiers

```
ORION_Unified_CDR_Engine/
├── docker-compose.yml              # Orchestration complète
├── .env                            # Variables d'environnement
├── Dockerfile.ingestion            # Build orion-ingestion
├── Dockerfile.validation           # Build orion-validation
├── Dockerfile.normalization        # Build orion-normalization
├── Dockerfile.enrichment           # Build orion-enrichment
├── Dockerfile.ml-fraud-agent       # Build orion-ml-fraud-agent
├── Dockerfile.storage-hot          # Build orion-storage-hot
├── Dockerfile.storage-cold         # Build orion-storage-cold
├── Dockerfile.api                  # Build orion-api
├── Dockerfile.observability        # Build orion-observability
├── Dockerfile.traffic-generator    # Build orion-traffic-generator
├── configs/
│   ├── kafka/                      # Config Kafka
│   ├── scylla/                     # Init ScyllaDB
│   ├── prometheus/
│   │   └── prometheus.yml          # Config Prometheus
│   └── grafana/
│       └── dashboards/             # Dashboards JSON
└── datasets/
    ├── FR/                         # CDR France
    ├── TN/                         # CDR Tunisie
    ├── MA/                         # CDR Maroc
    └── SN/                         # CDR Sénégal
```

## 5. docker-compose.yml complet

```yaml
version: '3.9'

services:
  # ==================== Infrastructure ====================
  
  zookeeper:
    image: confluentinc/cp-zookeeper:7.5.0
    container_name: orion-zookeeper
    environment:
      ZOOKEEPER_CLIENT_PORT: 2181
      ZOOKEEPER_TICK_TIME: 2000
    ports:
      - "2181:2181"
    networks:
      - orion-net

  kafka:
    image: confluentinc/cp-kafka:7.5.0
    container_name: orion-kafka
    depends_on:
      - zookeeper
    environment:
      KAFKA_BROKER_ID: 1
      KAFKA_ZOOKEEPER_CONNECT: zookeeper:2181
      KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://kafka:9092
      KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR: 1
      KAFKA_AUTO_CREATE_TOPICS_ENABLE: "true"
    ports:
      - "9092:9092"
    networks:
      - orion-net

  scylla:
    image: scylladb/scylla:5.4
    container_name: orion-scylla
    command: --smp 2 --memory 4G --overprovisioned 1
    ports:
      - "9042:9042"
      - "9160:9160"
    volumes:
      - scylla-data:/var/lib/scylla
      - ./configs/scylla/init.cql:/docker-entrypoint-initdb.d/init.cql
    networks:
      - orion-net
    healthcheck:
      test: ["CMD", "cqlsh", "-e", "describe keyspaces"]
      interval: 30s
      timeout: 10s
      retries: 5

  minio:
    image: minio/minio:latest
    container_name: orion-minio
    command: server /data --console-address ":9001"
    environment:
      MINIO_ROOT_USER: orion
      MINIO_ROOT_PASSWORD: orion_secret_123
    ports:
      - "9000:9000"
      - "9001:9001"
    volumes:
      - minio-data:/data
    networks:
      - orion-net

  prometheus:
    image: prom/prometheus:latest
    container_name: orion-prometheus
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.path=/prometheus'
    ports:
      - "9090:9090"
    volumes:
      - ./configs/prometheus/prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus-data:/prometheus
    networks:
      - orion-net

  grafana:
    image: grafana/grafana:latest
    container_name: orion-grafana
    environment:
      GF_SECURITY_ADMIN_PASSWORD: orion123
      GF_USERS_ALLOW_SIGN_UP: "false"
    ports:
      - "3000:3000"
    volumes:
      - grafana-data:/var/lib/grafana
      - ./configs/grafana/dashboards:/etc/grafana/provisioning/dashboards
    networks:
      - orion-net

  # ==================== ORION Pipeline ====================

  orion-ingestion:
    build:
      context: .
      dockerfile: Dockerfile.ingestion
    container_name: orion-ingestion
    depends_on:
      - kafka
    environment:
      KAFKA_BROKERS: kafka:9092
      KAFKA_TOPIC_IN: cdr.raw
      KAFKA_TOPIC_OUT: cdr.validated
      RUST_LOG: info
    ports:
      - "8081:8080"
    networks:
      - orion-net

  orion-validation:
    build:
      context: .
      dockerfile: Dockerfile.validation
    container_name: orion-validation
    depends_on:
      - kafka
    environment:
      KAFKA_BROKERS: kafka:9092
      KAFKA_TOPIC_IN: cdr.validated
      KAFKA_TOPIC_OUT: cdr.normalized
      RUST_LOG: info
    ports:
      - "8082:8080"
    networks:
      - orion-net

  orion-normalization:
    build:
      context: .
      dockerfile: Dockerfile.normalization
    container_name: orion-normalization
    depends_on:
      - kafka
    environment:
      KAFKA_BROKERS: kafka:9092
      KAFKA_TOPIC_IN: cdr.normalized
      KAFKA_TOPIC_OUT: cdr.enriched
      RUST_LOG: info
    ports:
      - "8083:8080"
    networks:
      - orion-net

  orion-enrichment:
    build:
      context: .
      dockerfile: Dockerfile.enrichment
    container_name: orion-enrichment
    depends_on:
      - kafka
      - orion-ml-fraud-agent
    environment:
      KAFKA_BROKERS: kafka:9092
      KAFKA_TOPIC_IN: cdr.enriched
      KAFKA_TOPIC_OUT: cdr.final
      ML_AGENT_URL: http://orion-ml-fraud-agent:50051
      RUST_LOG: info
    ports:
      - "8084:8080"
    networks:
      - orion-net

  orion-ml-fraud-agent:
    build:
      context: .
      dockerfile: Dockerfile.ml-fraud-agent
    container_name: orion-ml-fraud-agent
    environment:
      RUST_LOG: info
      MODEL_PATH: /models/fraud-v1.0.0.onnx
    ports:
      - "50051:50051"
      - "8085:8080"
    volumes:
      - ./models:/models
    networks:
      - orion-net

  orion-storage-hot:
    build:
      context: .
      dockerfile: Dockerfile.storage-hot
    container_name: orion-storage-hot
    depends_on:
      - kafka
      - scylla
    environment:
      KAFKA_BROKERS: kafka:9092
      KAFKA_TOPIC_IN: cdr.final
      SCYLLA_HOSTS: scylla:9042
      SCYLLA_KEYSPACE: orion
      RUST_LOG: info
    ports:
      - "8086:8080"
    networks:
      - orion-net

  orion-storage-cold:
    build:
      context: .
      dockerfile: Dockerfile.storage-cold
    container_name: orion-storage-cold
    depends_on:
      - kafka
      - minio
    environment:
      KAFKA_BROKERS: kafka:9092
      KAFKA_TOPIC_IN: cdr.final
      S3_ENDPOINT: http://minio:9000
      S3_ACCESS_KEY: orion
      S3_SECRET_KEY: orion_secret_123
      S3_BUCKET: orion-cdr-cold
      RUST_LOG: info
    ports:
      - "8087:8080"
    networks:
      - orion-net

  orion-api:
    build:
      context: .
      dockerfile: Dockerfile.api
    container_name: orion-api
    depends_on:
      - scylla
    environment:
      SCYLLA_HOSTS: scylla:9042
      SCYLLA_KEYSPACE: orion
      RUST_LOG: info
    ports:
      - "8080:8080"
    networks:
      - orion-net

  orion-observability:
    build:
      context: .
      dockerfile: Dockerfile.observability
    container_name: orion-observability
    environment:
      RUST_LOG: info
    ports:
      - "8088:8080"
    networks:
      - orion-net

  # ==================== Traffic Generator ====================

  orion-traffic-generator:
    build:
      context: .
      dockerfile: Dockerfile.traffic-generator
    container_name: orion-traffic-generator
    depends_on:
      - kafka
    environment:
      KAFKA_BROKERS: kafka:9092
      KAFKA_TOPIC: cdr.raw
      CDR_RATE: 1000
      COUNTRIES: FR,TN,MA,SN
      RUST_LOG: info
    networks:
      - orion-net

networks:
  orion-net:
    driver: bridge

volumes:
  scylla-data:
  minio-data:
  prometheus-data:
  grafana-data:
```

## 6. Configuration Prometheus

`configs/prometheus/prometheus.yml` :

```yaml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: 'orion-ingestion'
    static_configs:
      - targets: ['orion-ingestion:8080']

  - job_name: 'orion-validation'
    static_configs:
      - targets: ['orion-validation:8080']

  - job_name: 'orion-normalization'
    static_configs:
      - targets: ['orion-normalization:8080']

  - job_name: 'orion-enrichment'
    static_configs:
      - targets: ['orion-enrichment:8080']

  - job_name: 'orion-ml-fraud-agent'
    static_configs:
      - targets: ['orion-ml-fraud-agent:8080']

  - job_name: 'orion-storage-hot'
    static_configs:
      - targets: ['orion-storage-hot:8080']

  - job_name: 'orion-storage-cold'
    static_configs:
      - targets: ['orion-storage-cold:8080']

  - job_name: 'orion-api'
    static_configs:
      - targets: ['orion-api:8080']
```

## 7. Initialisation ScyllaDB

`configs/scylla/init.cql` :

```sql
-- Création du keyspace
CREATE KEYSPACE IF NOT EXISTS orion
WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 1};

USE orion;

-- Table principale : CDR par IMSI/jour
CREATE TABLE IF NOT EXISTS cdr_by_imsi_day (
    imsi text,
    event_date date,
    event_time timestamp,
    charging_id text,
    msisdn text,
    imei text,
    event_type text,
    duration int,
    bytes_up bigint,
    bytes_down bigint,
    cell_id text,
    country text,
    roaming_partner text,
    fraud_score float,
    model_version text,
    PRIMARY KEY ((imsi, event_date), event_time, charging_id)
) WITH CLUSTERING ORDER BY (event_time ASC);

-- Table : CDR par cellule/heure
CREATE TABLE IF NOT EXISTS cdr_by_cell_hour (
    cell_id text,
    event_hour timestamp,
    event_time timestamp,
    charging_id text,
    imsi text,
    msisdn text,
    rat text,
    bytes_up bigint,
    bytes_down bigint,
    country text,
    PRIMARY KEY ((cell_id, event_hour), event_time, charging_id)
) WITH CLUSTERING ORDER BY (event_time ASC);

-- Table : CDR par pays/jour
CREATE TABLE IF NOT EXISTS cdr_by_country_day (
    country text,
    event_date date,
    event_time timestamp,
    charging_id text,
    imsi text,
    event_type text,
    bytes_up bigint,
    bytes_down bigint,
    PRIMARY KEY ((country, event_date), event_time, charging_id)
) WITH CLUSTERING ORDER BY (event_time ASC);
```

## 8. Démarrage

### 8.1. Build des images

```powershell
# Build tous les microservices
docker compose build

# Ou build individuel
docker compose build orion-ingestion
```

### 8.2. Démarrage complet

```powershell
# Démarrer toute la stack
docker compose up -d

# Vérifier les logs
docker compose logs -f

# Vérifier le statut
docker compose ps
```

### 8.3. Démarrage séquentiel (recommandé)

```powershell
# 1. Infrastructure d'abord
docker compose up -d zookeeper kafka scylla minio

# Attendre 30 secondes pour init

# 2. Observabilité
docker compose up -d prometheus grafana

# 3. Pipeline ORION
docker compose up -d orion-ingestion orion-validation orion-normalization orion-enrichment orion-ml-fraud-agent orion-storage-hot orion-storage-cold orion-api

# 4. Traffic generator
docker compose up -d orion-traffic-generator
```

## 9. Accès aux interfaces

| Service | URL | Credentials |
|---------|-----|-------------|
| **Grafana** | http://localhost:3000 | admin / orion123 |
| **Prometheus** | http://localhost:9090 | - |
| **MinIO Console** | http://localhost:9001 | orion / orion_secret_123 |
| **ORION API** | http://localhost:8080 | - |
| **ScyllaDB** | localhost:9042 | cqlsh |

## 10. Vérification du pipeline

### 10.1. Santé des services

```powershell
# Health checks
curl http://localhost:8081/health  # ingestion
curl http://localhost:8082/health  # validation
curl http://localhost:8083/health  # normalization
curl http://localhost:8084/health  # enrichment
curl http://localhost:8085/health  # ml-fraud-agent
curl http://localhost:8086/health  # storage-hot
curl http://localhost:8087/health  # storage-cold
curl http://localhost:8080/health  # api
```

### 10.2. Métriques

```powershell
# Métriques Prometheus
curl http://localhost:8081/metrics | Select-String "orion_"
```

### 10.3. Kafka topics

```powershell
docker exec orion-kafka kafka-topics --list --bootstrap-server localhost:9092
```

### 10.4. ScyllaDB

```powershell
docker exec -it orion-scylla cqlsh

# Dans cqlsh:
SELECT COUNT(*) FROM orion.cdr_by_imsi_day;
```

## 11. Arrêt et nettoyage

```powershell
# Arrêter tous les services
docker compose down

# Arrêter + supprimer volumes
docker compose down -v

# Nettoyer images
docker compose down --rmi all -v
```

## 12. Troubleshooting

### Kafka ne démarre pas
```powershell
docker compose logs kafka
# Vérifier que zookeeper est up
docker compose ps zookeeper
```

### ScyllaDB ne répond pas
```powershell
docker exec orion-scylla nodetool status
```

### Pipeline bloqué
```powershell
# Vérifier le lag Kafka
docker exec orion-kafka kafka-consumer-groups --bootstrap-server localhost:9092 --describe --all-groups
```

### Mémoire insuffisante
```powershell
# Réduire ressources ScyllaDB
# Modifier command: --smp 1 --memory 2G
```

## 13. Mode démo rapide

Pour une démo rapide sans build :

```powershell
# Utiliser uniquement l'infrastructure
docker compose up -d zookeeper kafka scylla minio prometheus grafana

# Exécuter les microservices Rust localement (sans Docker)
cargo run --bin orion-ingestion
cargo run --bin orion-validation
# etc.
```

## 14. Prochaines étapes

- **Production** : voir [rhel.md](rhel.md)
- **Kubernetes** : voir Helm chart (à venir)
- **Monitoring** : voir [monitoring.md](monitoring.md)
- **Démo** : voir [scenario.md](../06-demo/scenario.md)

---

**ORION Docker Local** — _Déploiement simplifié pour développement et démonstration_
