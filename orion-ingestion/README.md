# 📥 orion-ingestion

Microservice d'ingestion Kafka pour ORION Unified CDR Engine.

## 🎯 Rôle

**orion-ingestion** est le premier maillon du pipeline ORION. Il :

- ✅ Consomme les CDR bruts depuis Kafka (multi-pays)
- ✅ Parse les formats hétérogènes (JSON, CSV, texte)
- ✅ Ajoute métadonnées d'ingestion
- ✅ Prépare les CDR pour la validation
- ✅ Expose métriques Prometheus
- ✅ Fournit endpoint /health

## 🏗️ Architecture

```
┌──────────────────┐
│  Kafka Topics    │
│  - cdr.raw.FR    │
│  - cdr.raw.TN    │
│  - cdr.raw.FN    │
│  - cdr.raw.CH    │
└────────┬─────────┘
         ↓
┌────────────────────┐
│ orion-ingestion    │
│  - Consumer Kafka  │
│  - Parser CDR      │
│  - Metrics         │
└────────┬───────────┘
         ↓
┌────────────────────┐
│ Output Channel     │
│ → cdr.validated    │
└────────────────────┘
```

## 📦 Structure

```
src/
  main.rs              # Point d'entrée, orchestration
  config.rs            # Configuration (env vars)
  routes.rs            # Endpoints HTTP (/health, /metrics)
  metrics.rs           # Métriques Prometheus
  service/
    mod.rs             # Exports publics
    kafka_consumer.rs  # Consumer Kafka asynchrone
    processor.rs       # Logique de traitement
    model.rs           # Structures de données
```

## ⚙️ Configuration

Via variables d'environnement :

| Variable | Description | Défaut |
|----------|-------------|--------|
| `KAFKA_BROKERS` | Brokers Kafka | `localhost:9092` |
| `KAFKA_TOPICS` | Topics à consommer (séparés par virgule) | `cdr.raw.FR,cdr.raw.TN,cdr.raw.FN,cdr.raw.CH` |
| `SERVER_HOST` | Host HTTP | `0.0.0.0` |
| `SERVER_PORT` | Port HTTP | `8081` |
| `RUST_LOG` | Niveau de logs | `info` |

### Exemple

```bash
export KAFKA_BROKERS="kafka1:9092,kafka2:9092"
export KAFKA_TOPICS="cdr.raw.FR,cdr.raw.TN"
export SERVER_PORT=8081
export RUST_LOG=info
```

## 🚀 Démarrage

### Build

```bash
cargo build --release
```

### Run local

```bash
cargo run
```

### Run avec Docker

```bash
docker build -t orion-ingestion .
docker run -p 8081:8081 \
  -e KAFKA_BROKERS=kafka:9092 \
  orion-ingestion
```

## 📊 Endpoints

### Health Check

```bash
curl http://localhost:8081/health
# Response: OK
```

### Metrics (Prometheus)

```bash
curl http://localhost:8081/metrics
```

**Métriques exposées** :

- `orion_ingestion_messages_total` — Total messages consommés
- `orion_ingestion_errors_total` — Total erreurs
- `orion_ingestion_bytes_total` — Total bytes ingérés
- `orion_ingestion_latency_seconds` — Latence de traitement

## 🧪 Tests

```bash
# Tests unitaires
cargo test

# Tests avec logs
cargo test -- --nocapture

# Tests d'un module spécifique
cargo test service::processor
```

## 📝 Formats supportés

### JSON (France, Tunisie)

```json
{
  "imsi": "208150123456789",
  "msisdn": "+33612345678",
  "event_type": "data",
  "bytes_up": 123456,
  "bytes_down": 987654
}
```

### CSV (Finlande)

```csv
imsi;msisdn;event_type;bytes_up;bytes_down
212340123456789;+212612345678;data;45678;123456
```

### Texte (Suisse - legacy)

```
208150123456789|+221712345678|voice|120|0|0
```

## 🔍 Logs

Les logs sont structurés en JSON :

```json
{
  "timestamp": "2026-01-29T10:15:00Z",
  "level": "INFO",
  "target": "orion_ingestion",
  "message": "Processed CDR from cdr.raw.FR (country: FR) in 2.3ms"
}
```

## 🐛 Troubleshooting

### Kafka connection failed

```
Error: Kafka error: BrokerTransportFailure
```

**Solution** : Vérifier que Kafka est démarré et accessible.

```bash
docker ps | grep kafka
telnet localhost 9092
```

### No messages consumed

**Solution** : Vérifier les topics Kafka.

```bash
docker exec kafka kafka-topics --list --bootstrap-server localhost:9092
```

### High CPU usage

**Solution** : Augmenter le polling interval ou réduire le nombre de partitions.

## 🔗 Intégration pipeline

### Input

- **Source** : Kafka topics `cdr.raw.*`
- **Format** : JSON, CSV, texte brut

### Output

- **Destination** : Channel interne → Kafka `cdr.validated` (TODO)
- **Format** : JSON structuré avec métadonnées

### Next step

Les CDR traités sont envoyés vers **orion-validation** pour contrôles métier.

## 📚 Dépendances principales

- **axum** : Web framework
- **tokio** : Async runtime
- **rdkafka** : Client Kafka
- **serde** : Serialization
- **tracing** : Logs structurés
- **metrics** : Métriques Prometheus

## 🤝 Contribution

Pour modifier ce service :

1. Respecter la structure modulaire
2. Ajouter tests unitaires
3. Documenter les changements
4. Vérifier les métriques
5. Tester avec Kafka local

## 📖 Références

- [Architecture ORION](../../docs/02-architecture/architecture-detaillee.md)
- [Schéma CDR](../../docs/03-data/schema-cdr-unifie.md)
- [Guide de reprise](../../guide_de_reprise.md)

---

**ORION Ingestion** — _First step in the unified CDR pipeline_
