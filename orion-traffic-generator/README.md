# ORION Traffic Generator

Générateur de trafic CDR synthétique pour tester le pipeline ORION en conditions réalistes de production.

## 🎯 Objectif

Simuler un **stream continu de CDR** avec tous les problèmes potentiels de production :
- Latence réseau variable
- Erreurs intermittentes (timeouts, connexions perdues)
- Pics de trafic (burst mode)
- CDR malformés pour tester la validation
- Scénarios de fraude variés

## ✨ Fonctionnalités

### 1. Génération de CDR réalistes

**Types d'événements** :
- **Voice** : Appels vocaux avec durée, numéros appelant/appelé
- **Data** : Sessions internet avec octets montants/descendants, APN
- **SMS** : Messages texte avec longueur

**Multi-pays** :
| Pays | MCC | MNC | Topic Kafka |
|------|-----|-----|-------------|
| France | 208 | 01/15/20 | `cdr.raw.FR` |
| Tunisie | 605 | 01 | `cdr.raw.TN` |
| Finlande | 244 | 05 | `cdr.raw.FN` |
| Suisse | 228 | 01 | `cdr.raw.CH` |

### 2. Scénarios de fraude

Taux configurable (défaut 10%) avec patterns réalistes :

| Scénario | Caractéristiques |
|----------|------------------|
| **Durée excessive** | Appels 2-4 heures (vs normal 30s-15min) |
| **Data spike** | 15-50 GB en une session (vs 1-500 MB) |
| **Roaming suspect** | Pays à haut risque (RU, etc.) |
| **Appels internationaux** | Destinations multiples |

### 3. Simulation de problèmes

#### Latence réseau
- **Configurable** : 10-500ms (défaut)
- **Variable** : Simule jitter réseau
- **But** : Tester résilience du pipeline

#### Erreurs Kafka
- **Taux** : 3% par défaut
- **Types** : Timeouts, connexions perdues
- **Retry** : Backoff exponentiel (configurable)

#### CDR malformés
- **Taux** : 2% par défaut
- **Patterns** :
  - Champs requis manquants
  - Formats invalides (IMSI, MSISDN)
  - Types d'événements inconnus
  - JSON invalide

### 4. Burst mode

Simulation de **pics de trafic** :
- **Déclenchement** : Toutes les 5 minutes
- **Durée** : 30 secondes (configurable)
- **Multiplicateur** : 5x le taux normal (configurable)
- **But** : Tester scalabilité et backpressure

## 🔧 Configuration

### Variables d'environnement

| Variable | Description | Défaut |
|----------|-------------|--------|
| **Kafka** | | |
| `KAFKA_BROKERS` | Adresses Kafka | `localhost:9092` |
| `KAFKA_BASE_TOPIC` | Topic de base | `cdr.raw` |
| **Génération** | | |
| `GENERATION_RATE` | CDR/seconde | `100` |
| `BURST_ENABLED` | Activer burst | `true` |
| `BURST_MULTIPLIER` | Multiplicateur burst | `5` |
| `BURST_DURATION_SECS` | Durée burst | `30` |
| `FRAUD_RATE_PERCENT` | Taux fraude | `10` |
| `MALFORMED_RATE_PERCENT` | Taux CDR malformés | `2` |
| **Simulation** | | |
| `SIMULATE_LATENCY` | Activer latence | `true` |
| `MIN_LATENCY_MS` | Latence min | `10` |
| `MAX_LATENCY_MS` | Latence max | `500` |
| `SIMULATE_ERRORS` | Activer erreurs | `true` |
| `ERROR_RATE_PERCENT` | Taux erreurs | `3` |
| `ENABLE_RETRY` | Activer retry | `true` |
| `MAX_RETRIES` | Tentatives max | `3` |
| **Serveur** | | |
| `SERVER_HOST` | Bind HTTP | `0.0.0.0` |
| `SERVER_PORT` | Port HTTP | `9200` |

### Fichier .env

```bash
cp .env.example .env
# Adapter selon vos besoins
```

## 🚀 Utilisation

### Développement local

```bash
# Lancer Kafka
docker run -d -p 9092:9092 apache/kafka

# Lancer le générateur
cargo run

# Avec logs debug
RUST_LOG=debug cargo run
```

### Docker

```bash
# Build
docker build -t orion-traffic-generator:latest .

# Run
docker run -d \
  --name traffic-generator \
  -p 9200:9200 \
  -e KAFKA_BROKERS=kafka:9092 \
  -e GENERATION_RATE=200 \
  -e FRAUD_RATE_PERCENT=15 \
  orion-traffic-generator:latest
```

### Docker Compose

```yaml
traffic-generator:
  build: ./orion-traffic-generator
  ports:
    - "9200:9200"
  environment:
    KAFKA_BROKERS: kafka:9092
    GENERATION_RATE: 100
    BURST_ENABLED: "true"
    FRAUD_RATE_PERCENT: 10
    SIMULATE_ERRORS: "true"
  depends_on:
    - kafka
```

## 📊 Endpoints HTTP

### GET /health

Health check.

**Réponse** :
```
OK
```

### GET /metrics

Métriques Prometheus.

**Métriques exposées** :
- `traffic_generator_cdr_generated_total` : CDR générés
- `traffic_generator_cdr_sent_total` : CDR envoyés avec succès
- `traffic_generator_errors_total` : Erreurs Kafka
- `traffic_generator_retries_total` : Tentatives de retry
- `traffic_generator_malformed_total` : CDR malformés générés
- `traffic_generator_fraud_total` : CDR frauduleux générés
- `traffic_generator_latency_seconds` : Latence génération (histogram)
- `traffic_generator_kafka_latency_seconds` : Latence Kafka (histogram)

**Exemple** :
```
# HELP traffic_generator_cdr_generated_total CDR generated
# TYPE traffic_generator_cdr_generated_total counter
traffic_generator_cdr_generated_total 124580

# HELP traffic_generator_fraud_total Fraudulent CDR generated
# TYPE traffic_generator_fraud_total counter
traffic_generator_fraud_total 12458
```

## 🧪 Scénarios de test

### Test 1 : Trafic normal

```bash
GENERATION_RATE=50 \
FRAUD_RATE_PERCENT=5 \
MALFORMED_RATE_PERCENT=1 \
SIMULATE_ERRORS=false \
cargo run
```

### Test 2 : Stress test (burst)

```bash
GENERATION_RATE=500 \
BURST_ENABLED=true \
BURST_MULTIPLIER=10 \
cargo run
```

### Test 3 : Réseau dégradé

```bash
SIMULATE_LATENCY=true \
MIN_LATENCY_MS=500 \
MAX_LATENCY_MS=2000 \
SIMULATE_ERRORS=true \
ERROR_RATE_PERCENT=10 \
cargo run
```

### Test 4 : Détection de fraude

```bash
FRAUD_RATE_PERCENT=50 \
GENERATION_RATE=20 \
cargo run
```

### Test 5 : Validation robustesse

```bash
MALFORMED_RATE_PERCENT=30 \
ERROR_RATE_PERCENT=20 \
MAX_RETRIES=5 \
cargo run
```

## 📈 Monitoring

### Dashboard Grafana

Métriques clés à surveiller :
- **Throughput** : CDR/s (generated vs sent)
- **Success rate** : sent / (generated - malformed)
- **Latence** : p50, p95, p99
- **Taux d'erreur** : errors / generated
- **Taux de retry** : retries / errors
- **Distribution fraude** : fraud / generated

### Alertes recommandées

```yaml
- alert: HighErrorRate
  expr: rate(traffic_generator_errors_total[5m]) > 0.1
  annotations:
    summary: "Taux d'erreur > 10%"

- alert: KafkaLatencyHigh
  expr: histogram_quantile(0.95, traffic_generator_kafka_latency_seconds) > 1
  annotations:
    summary: "p95 Kafka latency > 1s"
```

## 🔍 Analyse des problèmes

### Erreur : Kafka unreachable

**Symptômes** :
- `traffic_generator_errors_total` élevé
- Logs : "Kafka send failed"

**Solutions** :
1. Vérifier connectivité Kafka : `telnet kafka 9092`
2. Vérifier firewall / security groups
3. Augmenter `MAX_RETRIES`

### Latence élevée

**Symptômes** :
- `traffic_generator_kafka_latency_seconds` p95 > 500ms

**Solutions** :
1. Vérifier charge Kafka brokers
2. Augmenter partitions des topics
3. Réduire `GENERATION_RATE`

### CDR non routés

**Symptômes** :
- CDR générés mais pas reçus par `orion-ingestion`

**Solutions** :
1. Vérifier topics Kafka existent : `kafka-topics.sh --list`
2. Vérifier consommateurs actifs
3. Inspecter logs ingestion

## 🗺️ Roadmap

### Phase 1 : MVP (actuel)
- ✅ Génération CDR multi-types
- ✅ Scénarios fraude
- ✅ Simulation problèmes réseau
- ✅ Burst mode
- ✅ Métriques Prometheus

### Phase 2 : Avancé
- ⏳ Patterns temporels (jour/nuit, jours fériés)
- ⏳ Corrélation CDR (mêmes abonnés)
- ⏳ Géolocalisation (cell towers)
- ⏳ Import datasets réels CSV

### Phase 3 : Analyse
- ⏳ Dashboard dédié Grafana
- ⏳ Export statistiques JSON
- ⏳ Replay CDR historiques
- ⏳ A/B testing scenarios

## 📚 Documentation

- [Architecture ORION](../docs/02-architecture/architecture-globale.md)
- [Monitoring](../docs/05-deploiement/monitoring.md)
- [Scénario démo](../docs/06-demo/scenario.md)

## 📝 Notes

- **Production** : Désactiver en production ! C'est un outil de test uniquement.
- **Performance** : 500 CDR/s soutenus sur machine standard (4 cores, 8GB RAM).
- **Kafka** : Crée automatiquement les topics si `auto.create.topics.enable=true`.
- **Retry** : Backoff exponentiel : 100ms, 200ms, 400ms, 800ms...
