# 🔄 Kafka Pipeline Flow - ORION CDR Processing

## 📋 Vue d'Ensemble

Ce document explique **pourquoi ORION écrit plusieurs fois dans Kafka** tout au long du pipeline de traitement des CDR (Call Detail Records).

> **Réponse courte** : On n'écrit pas "dans" Kafka, on écrit "entre" chaque étape du pipeline via Kafka. C'est un pattern d'**Event-Driven Architecture** où Kafka agit comme **bus de communication asynchrone** entre microservices.

---

## 🔄 Le Pipeline Complet

```
┌─────────────────┐
│  Raw CDR Input  │  ← Données brutes télécom (appels, SMS, data)
└────────┬────────┘
         │
         ▼
    📨 KAFKA #1
    Topic: cdr.raw.{country}
         │ (fr, be, ma, tn, pl, eg, ci, sn, cm, mg)
         │
         ▼
┌─────────────────────┐
│ orion-validation    │  ← Valide format & règles métier
│  ├─ Consumer        │     Lit: cdr.raw.*
│  └─ Producer ───────┼──► 📨 KAFKA #2a: cdr.validated (✅ Valid)
│                     │     📨 KAFKA #2b: cdr.rejected (❌ Invalid)
└─────────────────────┘
         │
         │ (CDR valides seulement)
         ▼
    📨 KAFKA #2a
    Topic: cdr.validated
         │
         ▼
┌─────────────────────┐
│ orion-normalization │  ← Unifie formats multi-pays
│  ├─ Consumer        │     Lit: cdr.validated
│  └─ Producer ───────┼──► 📨 KAFKA #3: cdr.normalized
└─────────────────────┘
         │
         ▼
    📨 KAFKA #3
    Topic: cdr.normalized
         │
         ▼
┌─────────────────────┐
│ orion-enrichment    │  ← Ajoute fraud, network, client info
│  ├─ Consumer        │     Lit: cdr.normalized
│  └─ Producer ───────┼──► 📨 KAFKA #4: cdr.enriched
└─────────────────────┘
         │
         ▼
    📨 KAFKA #4
    Topic: cdr.enriched
         │
         ▼
┌───────��─────────────┐
│ orion-storage-hot   │  ← Stockage ScyllaDB (pas de producer Kafka)
│  └─ Consumer        │     Lit: cdr.enriched
│                     │     Écrit: ScyllaDB
└─────────────────────┘

RÉSULTAT : 4 écritures Kafka + 1 lecture finale = 5 étapes
```

---

## 📝 Exemples de Code dans ORION

### 1️⃣ orion-validation → Kafka (validated & rejected)

**Fichier** : `orion-validation/src/service/kafka_producer.rs`

```rust
impl KafkaProducerService {
    // ✅ Écrit les CDR VALIDES
    pub async fn send_valid(&self, cdr: &ValidatedCDR) -> Result<()> {
        let record = FutureRecord::to(&self.output_topic)  // cdr.validated
            .key(cdr.cdr_id.as_bytes())
            .payload(&payload);
        
        self.producer.send(record, Timeout::After(Duration::from_secs(5))).await?;
        info!("Sent valid CDR {} to {}", cdr.cdr_id, self.output_topic);
    }
    
    // ❌ Écrit les CDR REJETÉS (Dead Letter Queue)
    pub async fn send_rejected(&self, error: &ValidationError) -> Result<()> {
        let record = FutureRecord::to(&self.rejected_topic)  // cdr.rejected
            .key(error.timestamp.as_bytes())
            .payload(&payload);
        
        self.producer.send(record, Timeout::After(Duration::from_secs(5))).await?;
        info!("Sent rejected CDR to {}", self.rejected_topic);
    }
}
```

**Pourquoi 2 topics ?**
- ✅ **Séparation des flux** : CDR valides vs invalides
- ✅ **Dead Letter Queue (DLQ)** : Les erreurs ne polluent pas le pipeline principal
- ✅ **Traçabilité** : Audit des rejets pour améliorer la qualité source
- ✅ **Reprocessing** : Possibilité de rejouer les rejets après correction

---

### 2️⃣ orion-normalization → Kafka (normalized)

**Fichier** : `orion-normalization/src/service/kafka_producer.rs`

```rust
impl KafkaProducerService {
    pub async fn send(&self, cdr: &UnifiedCDR) -> Result<()> {
        let record = FutureRecord::to(&self.output_topic)  // cdr.normalized
            .key(cdr.cdr_id.as_bytes())
            .payload(&payload);
        
        self.producer.send(record, Timeout::After(Duration::from_secs(5))).await?;
        info!("Sent normalized CDR {} to {}", cdr.cdr_id, self.output_topic);
    }
}
```

**Pourquoi normaliser dans Kafka ?**
- ✅ **Format unifié** : 1 schéma pour tous les pays (France, Belgique, Maroc, Tunisie, etc.)
- ✅ **Découplage** : orion-enrichment ne connaît pas les formats raw spécifiques
- ✅ **Évolutivité** : Ajout facile de nouveaux pays sans modifier enrichment
- ✅ **Replay** : Re-normalisation si les règles de mapping changent

---

### 3️⃣ orion-enrichment → Kafka (enriched)

**Fichier** : `orion-enrichment/src/service/kafka_producer.rs`

```rust
impl KafkaProducerService {
    pub async fn send(&self, cdr: &EnrichedCDR) -> Result<()> {
        let record = FutureRecord::to(&self.output_topic)  // cdr.enriched
            .key(cdr.unified.cdr_id.as_bytes())
            .payload(&payload);
        
        self.producer.send(record, Timeout::After(Duration::from_secs(5))).await?;
        info!("Sent enriched CDR {} to {}", cdr.unified.cdr_id, self.output_topic);
    }
}
```

**Pourquoi enrichir dans Kafka ?**
- ✅ **Données complètes** : Fraud detection, network info, client profiling
- ✅ **Multi-consumers** : Storage, analytics, alerting peuvent tous consommer
- ✅ **Backpressure** : ScyllaDB peut être lent, Kafka bufferise
- ✅ **Audit trail** : Historique complet de l'enrichissement

---

### 4️⃣ orion-storage-hot → ScyllaDB (fin du pipeline)

**Fichier** : `orion-storage-hot/src/service/kafka_consumer.rs`

```rust
impl KafkaConsumerService {
    pub async fn run(self) -> Result<()> {
        loop {
            match self.consumer.recv().await {
                Ok(message) => {
                    let enriched_cdr: EnrichedCDR = serde_json::from_slice(message.payload())?;
                    
                    // ✅ Écriture finale dans ScyllaDB (PAS de producer Kafka)
                    self.storage.insert_cdr(&enriched_cdr).await?;
                    
                    info!("Stored CDR {} in ScyllaDB", enriched_cdr.unified.cdr_id);
                }
                Err(e) => error!("Kafka consumer error: {:?}", e),
            }
        }
    }
}
```

**Fin du pipeline** : Les données sont maintenant persistées dans ScyllaDB.

---

## 🎯 Les 7 Raisons Clés

### 1. **Découplage des Services**
Chaque service est **indépendant** et peut évoluer/redémarrer sans affecter les autres.

```
Si orion-normalization crash → orion-validation continue à écrire dans Kafka
Les messages attendent dans cdr.validated jusqu'au redémarrage
```

### 2. **Scalabilité Horizontale**
On peut scaler chaque étape indépendamment selon les besoins.

```
Validation rapide (1 instance) → Enrichment lent (5 instances)
Kafka distribue automatiquement la charge via partitions
```

### 3. **Replay & Reprocessing**
On peut rejouer le pipeline à partir de n'importe quelle étape.

```
Bug dans enrichment détecté ?
→ Fix le code
→ Reset consumer offset sur cdr.normalized
→ Reprocesse tous les CDR depuis normalization
```

### 4. **Tolérance aux Pannes**
Si un service crash, les messages restent dans Kafka (at-least-once delivery).

```
orion-storage-hot down pour maintenance ?
→ cdr.enriched accumule les messages
→ Au redémarrage, tout est retraité automatiquement
```

### 5. **Monitoring & Observabilité**
Chaque topic = point de mesure du pipeline.

```yaml
Métriques Prometheus:
  - kafka_consumer_lag{topic="cdr.normalized"}   # Retard normalization
  - kafka_consumer_lag{topic="cdr.enriched"}     # Retard enrichment
  - orion_kafka_errors_total{service="validation"} # Erreurs par service

Alertes:
  - Lag > 1 minute → Bottleneck détecté
  - Error rate > 1% → Problème de qualité données
```

### 6. **Séparation des Flux (Success vs Errors)**
Dead Letter Queue (DLQ) pour isoler les erreurs.

```
cdr.validated → CDR valides (95%)
cdr.rejected  → CDR invalides (5%, séparés pour analyse)
```

### 7. **Multi-Consumers Pattern**
Plusieurs services peuvent consommer le même topic (fan-out).

```
cdr.enriched est lu par:
  ├─ orion-storage-hot (ScyllaDB)    ← Priorité 1
  ├─ orion-analytics (future)        ← Analyse temps réel
  ├─ orion-alerting (future)         ← Alertes fraud
  └─ orion-export-s3 (future)        ← Archivage cold storage
```

---

## 📊 Topologie Kafka Complète

### Topics et Rétention

| Topic | Partitions | Retention | Raison |
|-------|-----------|-----------|--------|
| `cdr.raw.fr` | 10 | 7 jours | Replay en cas d'erreur ingestion |
| `cdr.raw.be` | 5 | 7 jours | Moins de volume que France |
| `cdr.raw.ma` | 8 | 7 jours | Maroc (volume moyen) |
| `cdr.validated` | 20 | 3 jours | Buffer validation → normalization |
| `cdr.rejected` | 5 | 30 jours | **DLQ** : debug & retraitement |
| `cdr.normalized` | 20 | 3 jours | Buffer normalization → enrichment |
| `cdr.enriched` | 30 | 1 jour | Buffer enrichment → storage |

**Total topics** : ~15 (10 pays × raw + 4 stages)

### Configuration Production

**Broker config** (voir `adr-004-kafka-multi-pays.md`) :
```properties
# Performance
num.network.threads=8
num.io.threads=16

# Durabilité
default.replication.factor=3
min.insync.replicas=2
unclean.leader.election.enable=false

# Compression
compression.type=snappy
```

**Producer config** (Rust) :
```toml
acks = "all"              # Durabilité max
compression.type = "snappy"
batch.size = 16384
linger.ms = 10
retries = 2147483647
```

**Consumer config** (Rust) :
```toml
auto.offset.reset = "earliest"
enable.auto.commit = false    # Manual commit
max.poll.records = 500
session.timeout.ms = 30000
```

---

## 🎓 FAQ Entretien

### Q: "Pourquoi pas une seule écriture Kafka à la fin ?"

> **Réponse** : On perdrait la résilience et le replay granulaire. Si enrichment crash après avoir traité 1M CDR, on devrait tout refaire depuis le début au lieu de juste rejouer depuis `cdr.normalized`.

### Q: "Quel est le coût de ces écritures multiples ?"

> **Réponse** : 
> - **Latency** : ~5-10ms par write Kafka → ~30-40ms total pour le pipeline
> - **Stockage** : Kafka a une rétention courte (1-7 jours), données finales dans ScyllaDB
> - **Réseau** : Compression snappy → ~50% réduction de bande passante
> - **ROI** : Le gain en résilience, scalabilité et observabilité compense largement

### Q: "Et la duplication de données ?"

> **Réponse** : 
> - Kafka = **buffer temporaire** (1-7 jours)
> - ScyllaDB = **storage permanent** (années)
> - Coût Kafka négligeable vs bénéfices (replay, multi-consumers, découplage)

### Q: "Comment gérez-vous l'ordre des messages ?"

> **Réponse** :
> - Partitionnement par `cdr_id` (key Kafka)
> - Garantie d'ordre **par partition**
> - Consumer group pour parallélisme sans duplication

### Q: "Que se passe-t-il si Kafka tombe ?"

> **Réponse** :
> - **Réplication** : 3 brokers, min.insync.replicas=2
> - **Haute disponibilité** : Cluster multi-AZ
> - **Graceful degradation** : Services bufferisent localement si nécessaire
> - **Monitoring** : Alertes Prometheus + PagerDuty

---

## 📈 Métriques & Monitoring

### Dashboards Grafana

```promql
# Throughput par topic
rate(kafka_server_brokertopicmetrics_messagesin_total{topic="cdr.enriched"}[1m])

# Consumer lag (critique !)
kafka_consumergroup_lag{group="orion-storage-hot", topic="cdr.enriched"}

# Latency end-to-end
histogram_quantile(0.99, 
  rate(orion_pipeline_duration_seconds_bucket[5m])
)

# Error rate
rate(orion_kafka_errors_total[1m]) / rate(orion_cdr_processed_total[1m])
```

### Alertes

```yaml
- alert: KafkaConsumerLagHigh
  expr: kafka_consumergroup_lag > 10000
  for: 5m
  annotations:
    summary: "Consumer {{ $labels.group }} lagging on {{ $labels.topic }}"

- alert: KafkaErrorRateHigh
  expr: rate(orion_kafka_errors_total[5m]) > 0.01  # 1%
  for: 2m
  annotations:
    summary: "High error rate in {{ $labels.service }}"
```

---

## 🔧 Commandes Utiles

### Vérifier le lag d'un consumer group
```bash
kafka-consumer-groups.sh --bootstrap-server localhost:9092 \
  --group orion-enrichment \
  --describe
```

### Compter les messages dans un topic
```bash
kafka-run-class.sh kafka.tools.GetOffsetShell \
  --broker-list localhost:9092 \
  --topic cdr.enriched \
  --time -1
```

### Reset offset pour replay
```bash
kafka-consumer-groups.sh --bootstrap-server localhost:9092 \
  --group orion-storage-hot \
  --topic cdr.enriched \
  --reset-offsets --to-earliest \
  --execute
```

### Consommer les derniers messages (debug)
```bash
kcat -C -b localhost:9092 -t cdr.enriched -o -10 -f '%k: %s\n'
```

---

## 📚 Références

- [ADR-004 : Kafka Multi-Pays](./decisions/adr-004-kafka-multi-pays.md)
- [Architecture Globale](./architecture-globale.md)
- [Apache Kafka Documentation](https://kafka.apache.org/documentation/)
- [rdkafka Rust Client](https://github.com/fede1024/rust-rdkafka)

---

**Auteur** : Architecture Team  
**Date** : 2026-02-19  
**Version** : 1.0  
**Status** : ✅ Production

---

## 💡 Résumé pour Entretien

**Question** : *"Pourquoi on retourne vers Kafka plusieurs fois ?"*

**Réponse concise** :

> "Dans ORION, on utilise Kafka comme **bus de communication asynchrone** entre chaque étape du pipeline de traitement des CDR.
> 
> On a **4 écritures Kafka** :
> 1. **Validation** → `cdr.validated` (✅) + `cdr.rejected` (❌ DLQ)
> 2. **Normalization** → `cdr.normalized` (format unifié)
> 3. **Enrichment** → `cdr.enriched` (fraud + network + client)
> 4. **Storage** → ScyllaDB (fin du pipeline)
> 
> **Bénéfices** :
> - ✅ **Découplage** : chaque service scale indépendamment
> - ✅ **Résilience** : at-least-once delivery, replay possible
> - ✅ **Observabilité** : monitoring du lag par étape
> - ✅ **Multi-consumers** : analytics, alerting peuvent s'abonner
> 
> C'est un pattern d'**Event-Driven Architecture** qui nous permet de traiter **100k+ CDR/sec** avec une latency P99 < 500ms.
