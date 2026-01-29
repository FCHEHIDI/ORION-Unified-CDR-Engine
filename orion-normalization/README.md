# 🔄 orion-normalization

Microservice de normalisation des CDR vers schéma unifié pour ORION Unified CDR Engine.

## 🎯 Rôle

**orion-normalization** est le troisième maillon du pipeline ORION. Il :

- ✅ Consomme les CDR validés depuis Kafka
- ✅ Transforme vers schéma unifié ORION
- ✅ Extrait MCC/MNC depuis IMSI
- ✅ Détecte roaming automatiquement
- ✅ Normalise champs voice/data/SMS
- ✅ Calcule hash pour dédoublonnage
- ✅ Expose métriques Prometheus

## 🏗️ Architecture

```
┌──────────────────┐
│ cdr.normalized   │  (input: ValidatedCDR)
└────────┬─────────┘
         ↓
┌────────────────────┐
│ orion-             │
│ normalization      │
│  - Extract MCC/MNC │
│  - Detect roaming  │
│  - Unified schema  │
└────────┬───────────┘
         ↓
┌────────────────────┐
│ cdr.enriched       │  (output: UnifiedCDR)
└────────────────────┘
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
    kafka_producer.rs      # Producer Kafka
    normalizer.rs          # Logique de normalisation
    model.rs               # Structures UnifiedCDR
```

## ⚙️ Configuration

| Variable | Description | Défaut |
|----------|-------------|--------|
| `KAFKA_BROKERS` | Brokers Kafka | `localhost:9092` |
| `KAFKA_INPUT_TOPIC` | Topic d'entrée | `cdr.normalized` |
| `KAFKA_OUTPUT_TOPIC` | Topic de sortie | `cdr.enriched` |
| `KAFKA_CONSUMER_GROUP` | Consumer group ID | `orion-normalization` |
| `SERVER_HOST` | Host HTTP | `0.0.0.0` |
| `SERVER_PORT` | Port HTTP | `8083` |
| `RUST_LOG` | Niveau de logs | `info` |

## 🚀 Démarrage

```bash
# Build
cargo build --release

# Run
cargo run

# Docker
docker build -t orion-normalization .
docker run -p 8083:8083 \
  -e KAFKA_BROKERS=kafka:9092 \
  orion-normalization
```

## 📊 Endpoints

- **GET /health** → `OK`
- **GET /metrics** → Métriques Prometheus

**Métriques exposées** :
- `orion_normalization_messages_total` — Total CDR normalisés
- `orion_normalization_voice_total` — Total voice
- `orion_normalization_data_total` — Total data
- `orion_normalization_sms_total` — Total SMS
- `orion_normalization_latency_seconds` — Latence

## 📋 Schéma UnifiedCDR

### Champs principaux

```rust
{
  // Identifiers
  cdr_id: String,
  session_id: Option<String>,
  
  // Subscriber
  imsi: String,              // 14-15 digits
  msisdn: String,            // +33612345678
  imei: Option<String>,
  
  // Event
  event_type: Voice|Data|Sms|Unknown,
  service_type: Standard|Premium|Roaming|Emergency,
  
  // Temporal
  start_timestamp: DateTime<Utc>,
  end_timestamp: Option<DateTime<Utc>>,
  duration_seconds: Option<i64>,
  
  // Location
  country_code: String,      // FR, TN, FN, CH
  mcc: Option<String>,       // 208 (extracted from IMSI)
  mnc: Option<String>,       // 15 (extracted from IMSI)
  lac: Option<String>,
  cell_id: Option<String>,
  
  // Voice specific
  calling_number: Option<String>,
  called_number: Option<String>,
  call_type: Option<Mobile|Landline|International|Emergency>,
  
  // Data specific
  bytes_uploaded: Option<i64>,
  bytes_downloaded: Option<i64>,
  apn: Option<String>,
  
  // SMS specific
  sms_type: Option<MoSms|MtSms>,
  message_length: Option<i32>,
  
  // Roaming
  is_roaming: bool,          // Auto-detected via MCC
  visited_country: Option<String>,
  visited_network: Option<String>,
  
  // Charging
  charging_id: Option<String>,
  rated_amount: Option<f64>,
  currency: Option<String>,
  
  // Metadata
  normalization_timestamp: String,
  source_system: String,
  raw_data_hash: String      // For deduplication
}
```

## 🔍 Logique de normalisation

### 1. Extraction MCC/MNC

```rust
IMSI: 208150123456789
  → MCC: 208 (France)
  → MNC: 15 (Orange)
```

### 2. Détection roaming

```rust
Country: FR + MCC: 605 → Roaming: true (605 = Tunisia)
Country: FR + MCC: 208 → Roaming: false
```

### 3. Transformation par event_type

**Voice** :
- `calling_number` ← `msisdn` ou `calling_number`
- `called_number` ← `destination` ou `called_number`
- `duration_seconds` ← `duration`
- `call_type` ← détecté depuis format numéro

**Data** :
- `bytes_uploaded` ← `bytes_up`
- `bytes_downloaded` ← `bytes_down`
- `apn` ← `apn`

**SMS** :
- `sms_type` ← `mo_sms` ou `mt_sms`
- `message_length` ← `length`

### 4. Hash pour dédoublonnage

```rust
raw_data_hash: "a1b2c3d4e5f6" // SHA hash du raw_data
```

## 🧪 Tests

```bash
cargo test
cargo test service::normalizer
```

**Tests disponibles** :
- ✅ Normalisation voice CDR
- ✅ Extraction MCC/MNC
- ✅ Détection roaming
- ✅ Transformation data/SMS

## 📝 Exemples

### Input (ValidatedCDR)

```json
{
  "cdr_id": "550e8400-e29b-41d4-a716-446655440000",
  "event_type": "voice",
  "imsi": "208150123456789",
  "msisdn": "+33612345678",
  "timestamp": "2026-01-29T10:00:00Z",
  "country": "FR",
  "raw_data": {
    "calling_number": "+33612345678",
    "called_number": "+33698765432",
    "duration": 120,
    "call_type": "mobile"
  },
  "validation_timestamp": "2026-01-29T10:00:01Z"
}
```

### Output (UnifiedCDR)

```json
{
  "cdr_id": "550e8400-e29b-41d4-a716-446655440000",
  "event_type": "voice",
  "imsi": "208150123456789",
  "msisdn": "+33612345678",
  "mcc": "208",
  "mnc": "15",
  "country_code": "FR",
  "calling_number": "+33612345678",
  "called_number": "+33698765432",
  "call_type": "mobile",
  "duration_seconds": 120,
  "is_roaming": false,
  "service_type": "standard",
  "start_timestamp": "2026-01-29T10:00:00Z",
  "normalization_timestamp": "2026-01-29T10:00:02Z",
  "source_system": "orion-ingestion",
  "raw_data_hash": "a1b2c3d4e5f67890"
}
```

## 🔍 Logs

```json
{
  "timestamp": "2026-01-29T10:15:00Z",
  "level": "INFO",
  "target": "orion_normalization",
  "message": "Received validated CDR 550e8400 (event: Voice, country: FR)"
}
```

## 🐛 Troubleshooting

### No messages normalized

Vérifier que **orion-validation** publie vers `cdr.normalized`.

```bash
docker exec kafka kafka-console-consumer \
  --bootstrap-server localhost:9092 \
  --topic cdr.normalized \
  --from-beginning
```

### Incorrect MCC/MNC extraction

Les 3 premiers chiffres de l'IMSI = MCC, les 2-3 suivants = MNC.

### Roaming always false

Vérifier le mapping MCC → Country dans `detect_roaming()`.

## 🔗 Intégration pipeline

### Input
- **Source** : Kafka `cdr.normalized`
- **Format** : `ValidatedCDR`

### Output
- **Destination** : Kafka `cdr.enriched`
- **Format** : `UnifiedCDR` (schéma complet)

### Next step
Les CDR unifiés sont envoyés vers **orion-enrichment** pour ajout données externes (réseau, client, ML fraud).

## 📚 Dépendances principales

- **axum** : Web framework
- **tokio** : Async runtime
- **rdkafka** : Client Kafka
- **serde** : Serialization
- **chrono** : Date/time handling
- **uuid** : ID generation

## 📖 Références

- [Architecture ORION](../../docs/02-architecture/architecture-detaillee.md)
- [Schéma CDR unifié](../../docs/03-data/schema-cdr-unifie.md)
- [orion-validation](../orion-validation/README.md)

---

**ORION Normalization** — _Third step: unified CDR schema_
