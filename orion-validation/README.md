# 🔍 orion-validation

Microservice de validation des CDR pour ORION Unified CDR Engine.

## 🎯 Rôle

**orion-validation** est le deuxième maillon du pipeline ORION. Il :

- ✅ Consomme les CDR ingérés depuis Kafka
- ✅ Valide les champs obligatoires (IMSI, MSISDN)
- ✅ Vérifie les formats et contraintes métier
- ✅ Route les CDR valides vers normalisation
- ✅ Route les CDR invalides vers topic de rejet
- ✅ Expose métriques Prometheus
- ✅ Fournit endpoint /health

## 🏗️ Architecture

```
┌──────────────────┐
│  cdr.validated   │  (input: ProcessedCDR)
└────────┬─────────┘
         ↓
┌────────────────────┐
│ orion-validation   │
│  - Validator       │
│  - Regex checks    │
│  - Business rules  │
└────┬───────┬───────┘
     ↓       ↓
  Valid   Invalid
     ↓       ↓
┌────────┐ ┌────────────┐
│cdr.    │ │cdr.        │
│norm... │ │rejected    │
└────────┘ └────────────┘
```

## 📦 Structure

```
src/
  main.rs                  # Point d'entrée
  config.rs                # Configuration
  metrics.rs               # Métriques Prometheus
  routes.rs                # Endpoints HTTP
  service/
    mod.rs                 # Exports
    kafka_consumer.rs      # Consumer Kafka
    kafka_producer.rs      # Producer Kafka (output + rejected)
    validator.rs           # Logique de validation
    model.rs               # Structures de données
```

## ⚙️ Configuration

Via variables d'environnement :

| Variable | Description | Défaut |
|----------|-------------|--------|
| `KAFKA_BROKERS` | Brokers Kafka | `localhost:9092` |
| `KAFKA_INPUT_TOPIC` | Topic d'entrée | `cdr.validated` |
| `KAFKA_OUTPUT_TOPIC` | Topic de sortie (valides) | `cdr.normalized` |
| `KAFKA_REJECTED_TOPIC` | Topic de rejet (invalides) | `cdr.rejected` |
| `KAFKA_CONSUMER_GROUP` | Consumer group ID | `orion-validation` |
| `SERVER_HOST` | Host HTTP | `0.0.0.0` |
| `SERVER_PORT` | Port HTTP | `8082` |
| `RUST_LOG` | Niveau de logs | `info` |

### Exemple

```bash
export KAFKA_BROKERS="kafka1:9092,kafka2:9092"
export KAFKA_INPUT_TOPIC="cdr.validated"
export KAFKA_OUTPUT_TOPIC="cdr.normalized"
export KAFKA_REJECTED_TOPIC="cdr.rejected"
export SERVER_PORT=8082
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
docker build -t orion-validation .
docker run -p 8082:8082 \
  -e KAFKA_BROKERS=kafka:9092 \
  orion-validation
```

## 📊 Endpoints

### Health Check

```bash
curl http://localhost:8082/health
# Response: OK
```

### Metrics (Prometheus)

```bash
curl http://localhost:8082/metrics
```

**Métriques exposées** :

- `orion_validation_messages_total` — Total messages traités
- `orion_validation_valid_total` — Total CDR valides
- `orion_validation_invalid_total` — Total CDR invalides
- `orion_validation_errors_total` — Total erreurs
- `orion_validation_latency_seconds` — Latence de validation

## 🔍 Règles de validation

### Champs obligatoires

- **IMSI** : 14-15 chiffres (ex: `208150123456789`)
- **MSISDN** : 10-15 chiffres avec + optionnel (ex: `+33612345678`)

### Champs optionnels

- **event_type** : `voice`, `data`, `sms`, ou `unknown`
- **timestamp** : ISO 8601
- **raw_data** : JSON libre

### Types d'erreurs

| Code | Description | Action |
|------|-------------|--------|
| `missing_field` | Champ obligatoire manquant | → `cdr.rejected` |
| `invalid_imsi` | Format IMSI invalide | → `cdr.rejected` |
| `invalid_msisdn` | Format MSISDN invalide | → `cdr.rejected` |
| `json_parse_error` | JSON malformé | → `cdr.rejected` |

## 🧪 Tests

```bash
# Tests unitaires
cargo test

# Tests avec logs
cargo test -- --nocapture

# Tests d'un module spécifique
cargo test service::validator
```

**Tests disponibles** :
- ✅ Validation CDR valide
- ✅ IMSI manquant
- ✅ Format IMSI invalide
- ✅ Format MSISDN invalide
- ✅ Event type classification

## 📝 Format des données

### Input (ProcessedCDR)

```json
{
  "raw_payload": "{\"imsi\":\"208150123456789\",\"msisdn\":\"+33612345678\",\"event_type\":\"voice\"}",
  "source_topic": "cdr.raw.FR",
  "country": "FR",
  "ingestion_timestamp": "2026-01-29T10:00:00Z"
}
```

### Output Valid (ValidatedCDR)

```json
{
  "cdr_id": "550e8400-e29b-41d4-a716-446655440000",
  "event_type": "voice",
  "imsi": "208150123456789",
  "msisdn": "+33612345678",
  "timestamp": "2026-01-29T10:00:01Z",
  "country": "FR",
  "raw_data": {...},
  "validation_timestamp": "2026-01-29T10:00:01Z"
}
```

### Output Invalid (ValidationError)

```json
{
  "error_type": "invalid_imsi",
  "message": "IMSI must be 14-15 digits, got: 123",
  "field": "imsi",
  "original_cdr": "{...}",
  "timestamp": "2026-01-29T10:00:01Z"
}
```

## 🔍 Logs

Les logs sont structurés en JSON :

```json
{
  "timestamp": "2026-01-29T10:15:00Z",
  "level": "INFO",
  "target": "orion_validation",
  "message": "Received CDR from cdr.raw.FR (country: FR)"
}
```

Erreurs de validation :

```json
{
  "timestamp": "2026-01-29T10:15:01Z",
  "level": "WARN",
  "target": "orion_validation",
  "message": "CDR validation failed: invalid_imsi - IMSI must be 14-15 digits, got: 123"
}
```

## 🐛 Troubleshooting

### No messages consumed

**Solution** : Vérifier que orion-ingestion publie vers `cdr.validated`.

```bash
docker exec kafka kafka-console-consumer \
  --bootstrap-server localhost:9092 \
  --topic cdr.validated \
  --from-beginning
```

### High rejection rate

**Solution** : Analyser le topic `cdr.rejected` pour identifier les erreurs récurrentes.

```bash
docker exec kafka kafka-console-consumer \
  --bootstrap-server localhost:9092 \
  --topic cdr.rejected \
  --from-beginning
```

### Performance issues

**Solution** : Augmenter les partitions Kafka ou déployer plusieurs instances.

```bash
# Vérifier les consumer lag
docker exec kafka kafka-consumer-groups \
  --bootstrap-server localhost:9092 \
  --group orion-validation \
  --describe
```

## 🔗 Intégration pipeline

### Input

- **Source** : Kafka topic `cdr.validated`
- **Format** : JSON `ProcessedCDR`

### Output (valides)

- **Destination** : Kafka topic `cdr.normalized`
- **Format** : JSON `ValidatedCDR` avec UUID

### Output (invalides)

- **Destination** : Kafka topic `cdr.rejected`
- **Format** : JSON `ValidationError` avec détails

### Next step

Les CDR validés sont envoyés vers **orion-normalization** pour transformation vers schéma unifié.

## 📚 Dépendances principales

- **axum** : Web framework
- **tokio** : Async runtime
- **rdkafka** : Client Kafka
- **serde** : Serialization
- **regex** : Validation patterns
- **uuid** : ID generation
- **validator** : Règles de validation

## 🤝 Contribution

Pour modifier ce service :

1. Respecter la structure modulaire
2. Ajouter tests pour nouvelles règles
3. Documenter les erreurs dans README
4. Vérifier les métriques
5. Tester avec Kafka local

## 📖 Références

- [Architecture ORION](../../docs/02-architecture/architecture-detaillee.md)
- [Schéma CDR](../../docs/03-data/schema-cdr-unifie.md)
- [orion-ingestion](../orion-ingestion/README.md)

---

**ORION Validation** — _Second step: ensure CDR quality_
