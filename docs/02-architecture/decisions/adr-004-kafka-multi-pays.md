# ADR-004 : Kafka pour Ingestion Multi-pays

## Statut
✅ **Accepté** — Implémenté en V1

## Contexte
ORION doit ingérer des CDR de multiples pays avec :
- Volumetrie : > 1M CDR/min/pays
- Fiabilité : aucune perte de données
- Résilience : tolérance aux pannes
- Backpressure : ne pas surcharger le pipeline
- Isolation : erreurs d'un pays n'impactent pas les autres

## Décision
**Kafka est utilisé comme bus d'ingestion multi-pays** avec topics par pays et DLQ.

## Motivations

### ✅ Avantages Kafka

#### Ingestion massive
- **> 10M messages/sec** par cluster
- Partitioning horizontal
- Zero-copy architecture
- Compression native

#### Backpressure naturel
- Consommateurs contrôlent le rythme
- Pas de surcharge pipeline
- Lag monitoring
- Consumer groups

#### DLQ (Dead Letter Queue)
- Isolation des erreurs
- Retraitement ultérieur
- Analytics sur erreurs
- Pas de blocage pipeline

#### Multi-topics par pays
- `cdr.raw.FR`, `cdr.raw.TN`, etc.
- Isolation logique
- Scaling indépendant
- Monitoring granulaire

#### Durabilité
- Persistence sur disque
- Réplication (RF=3)
- Retention configurable
- At-least-once guarantee

#### Écosystème mature
- Clients Rust/Go/Python
- Kafka Connect pour legacy
- Monitoring tools (Prometheus exporter)
- Ops matures

### ⚠️ Inconvénients

- Complexité opérationnelle
- Zookeeper dependency (Kafka < 3.0)
- Tuning requis
- Latence > direct DB write

## Alternatives considérées

### Option 1 : RabbitMQ (rejetée)
- ✅ Simple à déployer
- ❌ Moins performant que Kafka
- ❌ Pas de retention long terme
- ❌ Scalabilité limitée

### Option 2 : Redis Streams (rejetée)
- ✅ Ultra rapide
- ❌ In-memory limité
- ❌ Pas de durabilité garantie
- ❌ Ops moins matures que Kafka

### Option 3 : Pulsar (considérée pour V2)
- ✅ Architecture moderne
- ✅ Séparation compute/storage
- ⚠️ Moins mature que Kafka
- ⚠️ Moins d'outils/expertise
- **→ À reconsidérer en V2**

### Option 4 : Direct DB write (rejetée)
- ✅ Plus simple
- ❌ Pas de backpressure
- ❌ Couplage fort
- ❌ Pas de replay possible

## Architecture Topics

### Topics principaux

```
cdr.raw.FR          # CDR bruts France
cdr.raw.TN          # CDR bruts Tunisie
cdr.raw.MA          # CDR bruts Maroc
cdr.raw.SN          # CDR bruts Sénégal

cdr.validated       # Après validation
cdr.normalized      # Après normalisation
cdr.enriched        # Après enrichissement
cdr.final           # Prêt pour storage

cdr.dlq             # Dead Letter Queue
```

### Partitioning

**Stratégie** : par pays et hash(IMSI)

```
cdr.raw.FR:
  - partitions: 12
  - key: IMSI
  - replication: 3
  - retention: 7 days
```

**Avantages** :
- Parallélisation (12 consumers)
- Ordre garanti par IMSI
- Scale horizontal facile

### Replication Factor

**RF=3** pour tous les topics (haute disponibilité)

### Retention

| Topic | Retention | Raison |
|-------|-----------|--------|
| `cdr.raw.*` | 7 jours | Replay en cas d'erreur |
| `cdr.validated` | 3 jours | Buffer validation |
| `cdr.normalized` | 3 jours | Buffer normalisation |
| `cdr.enriched` | 1 jour | Buffer enrichissement |
| `cdr.final` | 1 jour | Buffer avant storage |
| `cdr.dlq` | 30 jours | Debug & retraitement |

## Configuration Kafka

### Broker config (production)

```properties
# Performance
num.network.threads=8
num.io.threads=16
socket.send.buffer.bytes=1048576
socket.receive.buffer.bytes=1048576

# Durabilité
default.replication.factor=3
min.insync.replicas=2
unclean.leader.election.enable=false

# Retention
log.retention.hours=168
log.segment.bytes=1073741824
log.retention.check.interval.ms=300000

# Compression
compression.type=snappy
```

### Producer config (Rust)

```toml
[producer]
acks = "all"              # Durabilité max
compression.type = "snappy"
batch.size = 16384
linger.ms = 10
max.in.flight = 5
retries = 2147483647
```

### Consumer config (Rust)

```toml
[consumer]
auto.offset.reset = "earliest"
enable.auto.commit = false    # Manual commit
max.poll.records = 500
session.timeout.ms = 30000
```

## Monitoring

### Métriques Kafka
- `kafka_broker_messages_in_per_sec`
- `kafka_broker_bytes_in_per_sec`
- `kafka_consumer_lag`
- `kafka_producer_record_error_rate`

### Métriques ORION
- `orion_kafka_ingestion_rate`
- `orion_kafka_lag_seconds`
- `orion_kafka_errors_total`

### Alertes
- Lag > 1 minute
- Error rate > 1%
- Broker down
- Disk usage > 80%

## Conséquences

### Positives
- ✅ Ingestion massive garantie
- ✅ Backpressure naturel
- ✅ Isolation par pays
- ✅ DLQ pour résilience
- ✅ Replay possible
- ✅ At-least-once delivery

### Négatives
- ⚠️ Complexité opérationnelle
- ⚠️ Latence ajoutée (acceptable)
- ⚠️ Monitoring avancé requis

## Sizing

**V1 (prototype)** :
- 1 broker (dev)
- 4 CPU, 8 GB RAM
- 500 GB SSD

**V2 (pilote)** :
- 3 brokers
- 16 CPU, 32 GB RAM each
- 2 TB SSD each

**V3 (production)** :
- 5-7 brokers per cluster
- 32 CPU, 64 GB RAM each
- 10 TB SSD each
- Multi-cluster (per region)

## Références
- [Apache Kafka](https://kafka.apache.org/)
- [rdkafka Rust client](https://github.com/fede1024/rust-rdkafka)
- Kafka best practices (internes)

---

**Date** : Décembre 2025  
**Auteur** : Platform Architecture Team  
**Reviewers** : SRE, Data Engineers