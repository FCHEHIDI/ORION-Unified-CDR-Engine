# ADR-003 : Ceph pour Cold Storage

## Statut
✅ **Accepté** — MinIO en V1 (dev), Ceph en V2+ (prod)

## Contexte
ORION doit archiver les CDR long terme pour :
- Conformité réglementaire (6-24 mois)
- Analytics batch (Trino/Spark)
- Data science / ML training
- Audit & forensics

Exigences :
- Stockage massif (plusieurs PB)
- Coût optimisé
- Format analytics-friendly
- S3 compatible

## Décision
**Ceph (ou MinIO en local) est choisi comme cold storage** avec format Parquet compressé.

## Motivations

### ✅ Avantages Ceph

#### Stockage objet distribué
- Architecture shared-nothing
- Scale horizontal (PB-scale)
- Haute disponibilité (réplication)
- Self-healing

#### S3 compatible
- RGW (RADOS Gateway)
- Compatible tous outils S3
- SDK disponibles (Rust, Go, Python)
- Migration facile cloud public si besoin

#### Perfect pour Parquet/ORC
- Stockage colonne compressé
- Z-ordering spatial
- Partitionnement pays/date/type
- Prédicat pushdown

#### Lifecycle management
- Transition auto hot → cold
- Expiration automatique
- Compression intelligente
- Tiering (SSD → HDD → tape)

#### Open-source
- Pas de licensing
- Communauté active
- Indépendance vendor

### ⚠️ Inconvénients

- Complexité opérationnelle
- Nécessite expertise storage
- Tuning requis
- Monitoring avancé nécessaire

## Alternatives considérées

### Option 1 : AWS S3 (rejetée pour on-prem)
- ✅ Simple à utiliser
- ✅ Très fiable
- ❌ Coûts élevés long terme
- ❌ Lock-in cloud
- ❌ Latence inter-DC
- **→ Possible en cloud-native (V3)**

### Option 2 : MinIO (retenu pour V1 dev)
- ✅ S3-compatible
- ✅ Simple à déployer
- ✅ Parfait pour dev/test
- ⚠️ Moins mature que Ceph pour prod
- **→ Utilisé en V1, Ceph en V2+**

### Option 3 : HDFS (rejetée)
- ✅ Mature pour big data
- ❌ Moins flexible que S3
- ❌ Opérations complexes
- ❌ Moins bon pour small files

### Option 4 : ScyllaDB cold tables (rejetée)
- ✅ Déjà en place
- ❌ Pas optimisé pour analytics
- ❌ Coûteux pour archive
- ❌ Pas de format colonne

## Format de stockage

### Parquet (choisi)
- ✅ Format colonne
- ✅ Compression excellente (5-10x)
- ✅ Schema evolution
- ✅ Compatible Spark/Trino/Presto
- ✅ Prédicat pushdown
- ✅ Statistiques intégrées

### Alternatives
- **ORC** : équivalent, légèrement moins universel
- **Avro** : row-based, moins bon pour analytics
- **JSON.gz** : simple mais peu efficace

## Partitionnement

```
s3://orion-cdr-cold/
  country=FR/
    date=2026-01-29/
      event_type=data/
        cdr_FR_20260129_data_000001.parquet
        cdr_FR_20260129_data_000002.parquet
      event_type=voice/
        ...
  country=TN/
    date=2026-01-29/
      ...
```

**Avantages** :
- Prédicat pushdown efficace
- Scan partiel pour requêtes
- Lifecycle par partition
- Parallélisation Spark/Trino

## Compression

**Codec recommandé** : **Snappy** (balance perf/ratio)

Alternatives :
- **ZSTD** : meilleur ratio, plus lent
- **LZ4** : plus rapide, moins bon ratio
- **Gzip** : bon ratio, très lent

**Ratio attendu** : 5-10x (CDR JSON → Parquet Snappy)

## Conséquences

### Positives
- ✅ Coût optimisé (5-10x compression)
- ✅ Analytics performantes (Parquet)
- ✅ S3-compatible (portabilité)
- ✅ Lifecycle automatisé
- ✅ Open-source (pas de licensing)

### Négatives
- ⚠️ Complexité opérationnelle Ceph
- ⚠️ Expertise storage requise
- ⚠️ Latence > hot storage (acceptable)

## Sizing

**V1 (prototype)** :
- MinIO single-node
- 1 TB disque

**V2 (pilote)** :
- Ceph 3 nodes
- 10 TB each (30 TB total)
- Replication 3x

**V3 (production)** :
- Ceph 10-20 nodes
- 100 TB each (1-2 PB total)
- Tiering : SSD (hot) → HDD (warm) → tape (cold)

**Retention** :
- 6 mois : conformité minimale
- 12-24 mois : analytics long terme
- > 24 mois : tape backup

## Références
- [Ceph Architecture](https://docs.ceph.com/)
- [Apache Parquet](https://parquet.apache.org/)
- [MinIO](https://min.io/)

---

**Date** : Décembre 2025  
**Auteur** : Storage Architecture Team  
**Reviewers** : SRE, Data Engineers