🧱 Architecture détaillée — ORION Unified CDR Engine
(contenu à mettre dans docs/02-architecture/architecture-detaillee.md)

1. Vue d’ensemble des composants Rust
ORION est structuré en crates Rust indépendantes, chacune représentant un domaine fonctionnel :

1.1. orion-ingestion
Consommation Kafka (multi‑topics, multi‑pays)

Gestion du backpressure

Parsing brut (formats hétérogènes)

Envoi vers orion-validation

1.2. orion-validation
Validation des champs obligatoires

Normalisation des types

Contrôles métier (durée, RAT, pays)

Envoi vers orion-normalization

1.3. orion-normalization
Application du schéma CDR unifié groupe

Mapping multi‑pays → modèle unique

Gestion des erreurs → DLQ

Envoi vers orion-enrichment

1.4. orion-enrichment
Enrichissement réseau (CellID → localisation approximative)

Enrichissement client (segment, plan tarifaire)

Enrichissement roaming (pays, partenaire)

Appel au ML Fraud Agent

Envoi vers orion-storage-hot

1.5. orion-ml-fraud-agent
API gRPC interne

Chargement modèle ONNX ou Rust‑native

Scoring temps réel

Stateless (feature store externe ou mock)

1.6. orion-storage-hot
Écriture ScyllaDB

Gestion des batchs

Idempotence (call_id)

Tables partitionnées (IMSI/day, Cell/hour…)

1.7. orion-storage-cold
Écriture Ceph (ou MinIO en local)

Format Parquet/ORC

Compression

Partitionnement pays/jour/type

1.8. orion-api
API interne pour consultation

Endpoints :

/cdr/imsi/{id}

/cdr/cell/{id}

/fraud/score/{imsi}

Authentification interne (token court)

1.9. orion-observability
Export métriques Prometheus

Logs JSON structurés

Traces OpenTelemetry

2. Flux interne détaillé
Code
[Kafka multi-pays]
       ↓
[orion-ingestion]
       ↓
[orion-validation]
       ↓
[orion-normalization]
       ↓
[orion-enrichment]
       ↓
[orion-ml-fraud-agent] ←→ (feature store / mock)
       ↓
[orion-storage-hot (ScyllaDB)]
       ↓
[orion-storage-cold (Ceph/MinIO)]
       ↓
[orion-api / analytics / billing]
3. Topics Kafka
3.1. Entrée
cdr.raw.countryA

cdr.raw.countryB

cdr.raw.countryC

3.2. Intermédiaires
cdr.validated

cdr.normalized

cdr.enriched

3.3. Erreurs
cdr.dlq.parsing

cdr.dlq.validation

cdr.dlq.normalization

4. Modèle ScyllaDB (hot storage)
Tables principales :

4.1. cdr_by_imsi_day
Partition : (imsi, day)  
Usage : billing, customer care

4.2. cdr_by_cell_hour
Partition : (cell_id, hour)  
Usage : QoS, radio analytics

4.3. cdr_by_country_day
Partition : (country, day)  
Usage : reporting groupe

4.4. cdr_by_partner_day
Partition : (partner, day)  
Usage : roaming

4.5. cdr_by_event_type_day
Partition : (event_type, day)  
Usage : analytics

5. Stockage cold (Ceph)
Format : Parquet  
Partitionnement :

Code
/country=FR/year=2026/month=01/day=29/type=data/file.parquet
/country=TN/year=2026/month=01/day=29/type=voice/file.parquet
Compression : ZSTD  
Lifecycle : 6–24 mois

6. Interactions ML
6.1. Appel gRPC
orion-enrichment → orion-ml-fraud-agent

Payload :

Code
{
  imsi,
  msisdn,
  event_time,
  bytes_up,
  bytes_down,
  cell_id,
  country,
  roaming_partner
}
Réponse :

Code
{
  fraud_score: f32,
  model_version: "1.0.0"
}
6.2. Feature store
Version V1 : mock ou cache local
Version V2 : Redis / Scylla / ClickHouse

7. Observabilité
7.1. Metrics
ingestion_rate

validation_errors

enrichment_latency

ml_latency

scylla_write_latency

ceph_write_latency

7.2. Logs
JSON

masquage IMSI/MSISDN

corrélation via trace_id

7.3. Traces
OpenTelemetry

spans par microservice

8. Sécurité intégrée
TLS obligatoire

RBAC par service

tokens courts

audit complet

segmentation réseau (zones ingestion/compute/storage/admin)