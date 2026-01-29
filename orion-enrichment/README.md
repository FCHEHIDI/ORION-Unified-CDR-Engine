# ORION Enrichment Service

Service d'enrichissement des CDR avec détection de fraude, informations réseau et données client.

## 📋 Vue d'ensemble

Le service `orion-enrichment` est la **quatrième étape** du pipeline ORION. Il consomme les CDR unifiés depuis `cdr.enriched`, applique des enrichissements (détection de fraude, métadonnées réseau, données client), puis publie les CDR enrichis dans `cdr.stored` pour stockage ScyllaDB.

### Pipeline

```
[orion-normalization] 
    ↓ cdr.enriched (UnifiedCDR)
[orion-enrichment]
    ↓ cdr.stored (EnrichedCDR)
[orion-storage-hot]
```

## ✨ Fonctionnalités

### 1. Détection de fraude (Rule-Based MVP)

Implémentation basée sur **4 règles heuristiques** :

| Règle | Condition | Score | Détails |
|-------|-----------|-------|---------|
| **Durée excessive** | `duration > 7200s` (2h) | +0.3 | Appel anormalement long |
| **Roaming suspect** | Pays hors liste safe | +0.4 | Déplacement inhabituel (hors FR/TN/FN/CH/DE/ES/IT) |
| **Spike data** | `bytes_total > 10GB` | +0.4 | Consommation data excessive |
| **Appel international** | `call_type = international` | +0.2 | Utilisation coûteuse |

**Scoring** :
- Score cumulatif : `0.0 - 1.0`
- Niveau de risque :
  - `high` : score ≥ 0.7
  - `medium` : score ≥ 0.4
  - `low` : score < 0.4

**Exemple** :
```rust
// CDR avec appel international + durée excessive
let fraud_info = FraudInfo {
    fraud_score: 0.5,        // 0.3 (durée) + 0.2 (international)
    risk_level: "medium",
    is_fraud: false,         // Score < 0.7
    reasons: vec!["excessive_duration", "international_call"],
    model_version: "fraud_rules_v1",
};
```

### 2. Informations réseau (Mock)

Mapping **MCC/MNC → Network Name** :

| MCC | MNC | Opérateur | Pays |
|-----|-----|-----------|------|
| 208 | 15 | Orange France | FR |
| 208 | 01 | SFR | FR |
| 208 | 20 | Bouygues Telecom | FR |
| 605 | * | Tunisie Telecom | TN |
| 244 | * | Elisa Finland | FN |
| 228 | * | Swisscom | CH |

**Champs enrichis** :
- `network_name` : Nom commercial de l'opérateur
- `network_type` : 4G (année < 2023) / 5G (année ≥ 2023)
- `signal_strength` : Mock fixe `-75 dBm`
- `handover_count` : `duration / 300` (estimé)

> **TODO** : Remplacer par appel API externe (base de données MCC/MNC opérateurs).

### 3. Données client (Mock)

Simulation **CRM/Billing** :

```rust
// Logique de classification
let is_business = imsi.ends_with("000") || imsi.ends_with("999");

ClientInfo {
    segment: if is_business { "business" } else { "individual" },
    contract_type: if is_roaming { "postpaid" } else { "prepaid" },
    lifetime_value: if is_business { 5000.0 } else { 500.0 },
    is_vip: false,
    data_plan_limit: Some(53687091200), // 50GB
}
```

> **TODO** : Remplacer par appel API CRM/Billing système.

## 📦 Modèle de données

### EnrichedCDR

CDR complet avec fraude + réseau + client :

```rust
pub struct EnrichedCDR {
    // Champs UnifiedCDR (40+ champs)
    pub cdr_id: String,
    pub event_type: EventType,
    pub imsi: String,
    pub msisdn: String,
    // ... (tous les champs UnifiedCDR)
    
    // Enrichissements
    pub fraud_info: Option<FraudInfo>,
    pub network_info: Option<NetworkInfo>,
    pub client_info: Option<ClientInfo>,
}
```

### FraudInfo

```rust
pub struct FraudInfo {
    pub fraud_score: f64,              // 0.0 - 1.0
    pub risk_level: String,            // "low", "medium", "high"
    pub is_fraud: bool,                // true si score ≥ 0.7
    pub reasons: Vec<String>,          // ["excessive_duration", ...]
    pub model_version: String,         // "fraud_rules_v1"
}
```

### NetworkInfo

```rust
pub struct NetworkInfo {
    pub network_name: String,          // "Orange France"
    pub network_type: String,          // "4G", "5G"
    pub signal_strength: i32,          // -75 dBm (mock)
    pub handover_count: u32,           // Estimé
}
```

### ClientInfo

```rust
pub struct ClientInfo {
    pub segment: String,               // "business", "individual"
    pub contract_type: String,         // "postpaid", "prepaid"
    pub lifetime_value: f64,           // Valeur client (€)
    pub is_vip: bool,                  // Statut VIP
    pub data_plan_limit: Option<u64>,  // Limite forfait (bytes)
}
```

## 🔧 Configuration

### Variables d'environnement

| Variable | Description | Défaut |
|----------|-------------|--------|
| `KAFKA_BROKERS` | Adresses Kafka | `localhost:9092` |
| `KAFKA_INPUT_TOPIC` | Topic source | `cdr.enriched` |
| `KAFKA_OUTPUT_TOPIC` | Topic destination | `cdr.stored` |
| `KAFKA_CONSUMER_GROUP` | Groupe consommateur | `orion-enrichment` |
| `SERVER_HOST` | Bind HTTP | `0.0.0.0` |
| `SERVER_PORT` | Port HTTP | `8084` |
| `ENABLE_FRAUD_DETECTION` | Activer détection fraude | `true` |
| `FRAUD_AGENT_URL` | URL gRPC ML agent (futur) | `http://localhost:50051` |
| `ENABLE_NETWORK_DATA` | Activer enrichissement réseau | `true` |
| `ENABLE_CLIENT_DATA` | Activer enrichissement client | `true` |
| `RUST_LOG` | Niveau de log | `info` |

### Fichier .env

```bash
cp .env.example .env
# Adapter les valeurs si nécessaire
```

## 🚀 Utilisation

### Développement local

```bash
# Installer les dépendances
cargo build

# Lancer le service
cargo run

# Avec logs debug
RUST_LOG=debug cargo run
```

### Docker

```bash
# Build
docker build -t orion-enrichment:latest .

# Run
docker run -d \
  --name orion-enrichment \
  -p 8084:8084 \
  -e KAFKA_BROKERS=kafka:9092 \
  -e ENABLE_FRAUD_DETECTION=true \
  orion-enrichment:latest
```

### Docker Compose

```yaml
orion-enrichment:
  build: ./orion-enrichment
  ports:
    - "8084:8084"
  environment:
    KAFKA_BROKERS: kafka:9092
    KAFKA_INPUT_TOPIC: cdr.enriched
    KAFKA_OUTPUT_TOPIC: cdr.stored
    ENABLE_FRAUD_DETECTION: "true"
    ENABLE_NETWORK_DATA: "true"
    ENABLE_CLIENT_DATA: "true"
  depends_on:
    - kafka
```

## 📊 Endpoints HTTP

### GET /health

Health check du service.

**Réponse** :
```json
{
  "status": "ok",
  "service": "orion-enrichment"
}
```

### GET /metrics

Métriques Prometheus.

**Métriques exposées** :
- `orion_enrichment_messages_total` : Nombre total de messages traités
- `orion_enrichment_errors_total` : Nombre d'erreurs
- `orion_enrichment_fraud_detected_total` : Nombre de fraudes détectées (score ≥ 0.7)
- `orion_enrichment_latency_seconds` : Latence de traitement (histogram)

**Exemple** :
```
# HELP orion_enrichment_messages_total Total messages processed
# TYPE orion_enrichment_messages_total counter
orion_enrichment_messages_total 12450

# HELP orion_enrichment_fraud_detected_total Fraudulent CDRs detected
# TYPE orion_enrichment_fraud_detected_total counter
orion_enrichment_fraud_detected_total 87
```

## 🧪 Tests

### Tests unitaires

```bash
# Lancer tous les tests
cargo test

# Tests avec logs
cargo test -- --nocapture

# Test spécifique
cargo test test_fraud_detection_high_risk
```

### Tests d'intégration

```bash
# Publier un CDR unifié dans cdr.enriched
kafka-console-producer --bootstrap-server localhost:9092 --topic cdr.enriched
{"cdr_id":"123","event_type":"voice","imsi":"208150123456789","msisdn":"+33612345678","duration":8000,"is_roaming":false,"bytes_total":0,"service_type":"voice","call_type":"international","timestamp":"2024-01-15T10:30:00Z"}

# Consommer le résultat dans cdr.stored
kafka-console-consumer --bootstrap-server localhost:9092 --topic cdr.stored --from-beginning

# Vérifier les métriques
curl http://localhost:8084/metrics | grep orion_enrichment
```

## 🐛 Troubleshooting

### Problème : Aucun message consommé

**Solution** :
1. Vérifier que `orion-normalization` publie dans `cdr.enriched` :
   ```bash
   kafka-console-consumer --bootstrap-server localhost:9092 --topic cdr.enriched --from-beginning
   ```
2. Vérifier les logs du service :
   ```bash
   docker logs orion-enrichment | grep ERROR
   ```

### Problème : Fraude non détectée

**Solution** :
1. Vérifier que `ENABLE_FRAUD_DETECTION=true`
2. Tester les règles manuellement :
   ```bash
   # Durée excessive : duration > 7200s
   # Roaming suspect : mcc != 208/605/244/228
   # Data spike : bytes_total > 10737418240 (10GB)
   # International : call_type = "international"
   ```
3. Consulter les métriques :
   ```bash
   curl localhost:8084/metrics | grep fraud_detected
   ```

### Problème : Network name vide

**Solution** :
1. Vérifier que `ENABLE_NETWORK_DATA=true`
2. Consulter les logs pour le mapping MCC/MNC :
   ```bash
   docker logs orion-enrichment | grep "network_name"
   ```
3. Vérifier que le CDR contient `imsi` valide (14-15 chiffres)

### Problème : Client info null

**Solution** :
1. Vérifier que `ENABLE_CLIENT_DATA=true`
2. Vérifier que le CDR contient `imsi` et `is_roaming`
3. Consulter les logs :
   ```bash
   docker logs orion-enrichment | grep "client_info"
   ```

## 🗺️ Roadmap

### Phase 1 : MVP (actuel)
- ✅ Détection de fraude rule-based (4 règles)
- ✅ Mock network info (MCC/MNC mapping)
- ✅ Mock client info (CRM simulation)
- ✅ Feature flags (fraud/network/client)

### Phase 2 : ML Integration
- ⏳ Intégration gRPC `orion-ml-fraud-agent`
- ⏳ Modèle XGBoost/LightGBM (40+ features)
- ⏳ Threshold dynamique (0.85 fraude)
- ⏳ A/B testing (rules vs ML)

### Phase 3 : External APIs
- ⏳ API Network Info (base MCC/MNC externe)
- ⏳ API CRM/Billing (système existant)
- ⏳ API GeoIP (localisation)
- ⏳ Cache Redis pour optimisation

## 📚 Documentation

- [Architecture globale](../docs/02-architecture/architecture-globale.md)
- [Détection de fraude](../docs/04-ml/fraud-agent.md)
- [Features ML](../docs/04-ml/features.md)
- [Monitoring](../docs/05-deploiement/monitoring.md)

## 🔗 Dépendances

| Service | Consomme | Produit |
|---------|----------|---------|
| orion-normalization | - | `cdr.enriched` |
| **orion-enrichment** | `cdr.enriched` | `cdr.stored` |
| orion-storage-hot | `cdr.stored` | ScyllaDB |

## 📝 Notes

- **Mock data** : Les enrichissements réseau/client sont simulés en Phase 1. Remplacer par API externes en production.
- **Fraud rules** : Règles MVP calibrées pour télécoms français. Adapter aux cas d'usage spécifiques.
- **Performance** : Latence cible < 50ms/CDR. Monitorer `orion_enrichment_latency_seconds`.
- **Scalability** : Déployer plusieurs instances avec `KAFKA_CONSUMER_GROUP` identique pour parallélisation.
