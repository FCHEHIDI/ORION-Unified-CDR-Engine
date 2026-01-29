🧭 Architecture globale — ORION Unified CDR Engine
(contenu à mettre dans docs/02-architecture/architecture-globale.md)

1. Vision d’ensemble
ORION est une plateforme Rust unifiée permettant :

l’ingestion multi‑pays,

la normalisation des CDR,

l’enrichissement réseau & client,

la détection de fraude en temps réel,

le stockage hot/cold,

l’exposition des données aux systèmes billing/analytics.

L’architecture est pensée pour être :

scalable,

sécurisée,

multi‑datacenter,

Rust‑only en V1,

extensible (Go en V2 pour I/O‑bound).

2. Macro‑zones de la plateforme
🟦 Zone 1 — Ingestion multi‑pays
Kafka (clusters par pays ou fédérés)

Connecteurs SFTP/Batch (legacy)

Gateways d’entrée

DLQ

🟩 Zone 2 — Compute Rust
ingestion Rust

validation Rust

normalisation Rust

enrichment Rust

ML Fraud Agent (Rust)

orchestrateur interne Rust

🟧 Zone 3 — Storage
Hot storage : ScyllaDB

tables partitionnées (IMSI/day, Cell/hour…)

Cold storage : Ceph

Parquet/ORC

compression

lifecycle

🟪 Zone 4 — Analytics & exposition
APIs Rust

exports batch

accès Trino/Presto/Spark

dashboards Grafana

🟥 Zone 5 — Administration & Observabilité
Prometheus

Grafana

Loki/ELK

Alertmanager

supervision Rust

3. Flux principal (macro‑pipeline)
Code
[Pays A/B/C] 
      ↓
[Kafka Ingestion] 
      ↓
[Pipeline Rust]
  - parsing
  - validation
  - normalisation
  - enrichment
  - ML fraud scoring
      ↓
[Hot Storage - ScyllaDB]
      ↓
[Cold Storage - Ceph]
      ↓
[Analytics / Billing / Fraud / BI]
4. Sécurité intégrée (macro)
segmentation réseau par zones

TLS obligatoire

RBAC par service

audit complet

masquage IMSI/MSISDN dans logs

chiffrement au repos (Scylla/Ceph)

5. Multi‑pays & multi‑datacenter
ingestion par pays

normalisation unifiée

stockage hot/cold centralisé ou distribué

réplication configurable

isolation logique par pays

6. Observabilité
métriques Rust → Prometheus

logs structurés → Loki/ELK

traces internes → OpenTelemetry

dashboards → Grafana

7. Évolutivité
scale horizontal automatique

ajout de nouveaux pays sans refonte

introduction future de Go pour I/O‑bound

support Kubernetes en V2