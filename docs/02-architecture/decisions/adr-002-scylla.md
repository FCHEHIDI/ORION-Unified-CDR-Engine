# ADR-002 : ScyllaDB pour Hot Storage

## Statut
✅ **Accepté** — Implémenté en V1

## Contexte
ORION doit stocker des millions de CDR par minute avec des requêtes temps réel (< 10ms) pour :
- Billing (requêtes par IMSI)
- QoS réseau (requêtes par cellule)
- Analytics groupe (requêtes par pays)
- Fraud detection (requêtes temps réel)

## Décision
**ScyllaDB est choisi comme hot storage** pour tous les CDR enrichis.

## Motivations

### ✅ Avantages ScyllaDB

#### Ingestion massive
- **> 1M writes/sec/node**
- Architecture shared-nothing
- Pas de bottleneck central
- Linear scalability

#### Faible latence
- **P99 < 10ms** pour reads
- **P99 < 5ms** pour writes
- Pas de GC pauses (C++)
- Optimisé pour NVMe

#### Modèle partitionné idéal pour CDR
- Time-series native (TWCS)
- Partitionnement par (IMSI, date)
- Clustering par timestamp
- TTL automatique

#### Scale horizontal
- Ajout de nodes sans downtime
- Réplication configurable (RF=3)
- Multi-datacenter natif
- Auto-rebalancing

#### Compatible Cassandra
- CQL standard
- Drivers existants
- Migration Cassandra → Scylla simple
- Écosystème mature

### ⚠️ Inconvénients

- Courbe d'apprentissage modélisation
- Pas de JOINs (design dénormalisé requis)
- Opérations complexes (tuning, compaction)
- Moins de tooling que PostgreSQL

## Alternatives considérées

### Option 1 : Cassandra (rejetée)
- ✅ Même modèle que Scylla
- ❌ 5-10x plus lent que Scylla
- ❌ GC pauses (JVM)
- ❌ Plus de ressources nécessaires
- **→ Scylla est drop-in replacement performant**

### Option 2 : PostgreSQL + TimescaleDB (rejetée)
- ✅ SQL complet
- ✅ Tooling riche
- ❌ Scalabilité limitée vs NoSQL
- ❌ Pas de multi-DC natif
- ❌ Latence > 50ms à forte charge

### Option 3 : ClickHouse (rejetée)
- ✅ Excellent pour analytics
- ✅ Très performant en lecture
- ❌ Moins bon en écriture temps réel
- ❌ Pas de updates (append-only)
- **→ Possible pour cold storage analytics (V2)**

### Option 4 : MongoDB (rejetée)
- ✅ Flexible schema
- ❌ Moins performant que ScyllaDB
- ❌ Sharding complexe
- ❌ Pas optimisé pour time-series

### Option 5 : Redis (rejetée)
- ✅ Ultra rapide (< 1ms)
- ❌ In-memory = coûteux
- ❌ Pas de persistance garantie
- ❌ Scalabilité limitée
- **→ Possible pour cache (V2)**

## Modèle de données

Tables principales :
- `cdr_by_imsi_day` : partition (IMSI, date)
- `cdr_by_cell_hour` : partition (CellID, hour)
- `cdr_by_country_day` : partition (country, date)

Voir [scylladb-model.md](../../03-data/scylladb-model.md) pour détails.

## Conséquences

### Positives
- ✅ Performance garantie (> 1M writes/sec)
- ✅ Latence prévisible (< 10ms P99)
- ✅ Scale horizontal naturel
- ✅ Multi-DC ready
- ✅ Open-source (pas de licensing)

### Négatives
- ⚠️ Modélisation dénormalisée (learning curve)
- ⚠️ Opérations nécessitent expertise
- ⚠️ Pas de transactions ACID multi-partitions

## Sizing

**V1 (prototype)** :
- 1 node (dev)
- 4 CPU, 8 GB RAM
- 100 GB SSD

**V2 (pilote)** :
- 3 nodes
- 16 CPU, 32 GB RAM each
- 1 TB NVMe each
- RF=3

**V3 (production)** :
- 6-12 nodes per DC
- 32 CPU, 64 GB RAM each
- 4 TB NVMe each
- Multi-DC (RF=3 per DC)

## Références
- [ScyllaDB Architecture](https://docs.scylladb.com/)
- [scylladb-model.md](../../03-data/scylladb-model.md)
- Benchmarks internes ScyllaDB vs Cassandra

---

**Date** : Décembre 2025  
**Auteur** : Data Architecture Team  
**Reviewers** : DBA, SRE