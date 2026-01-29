# 📘 Introduction — ORION Unified CDR Engine

## 1. Qu'est-ce qu'ORION ?

**ORION** (Operational Real-time Insights & Observation Network) est une plateforme télécom distribuée de nouvelle génération conçue pour :

- **Collecter** les Call Detail Records (CDR) de multiples pays et réseaux
- **Normaliser** les formats hétérogènes vers un schéma unifié groupe
- **Enrichir** les données avec des informations réseau, client et géographiques
- **Détecter** la fraude en temps réel via Machine Learning
- **Stocker** efficacement les données (hot & cold storage)
- **Exposer** les données pour billing, analytics, compliance et optimisation réseau

## 2. Pourquoi ORION ?

Les groupes télécom multi-pays font face à des défis complexes :

### 🌍 Diversité des formats
Chaque pays génère des CDR dans des formats différents (CSV, JSON, TAP, formats legacy). ORION unifie tout cela dans un schéma commun.

### 📈 Volumétrie massive
Des millions de CDR par minute, par pays. ORION est conçu pour ingérer, traiter et stocker cette volumétrie avec des latences ultra-faibles.

### 🔐 Sécurité et conformité
GDPR, réglementations locales, audit. ORION intègre la sécurité et la traçabilité dès la conception.

### ⚡ Temps réel
Détection de fraude, alertes réseau, QoS. ORION traite les CDR en quasi temps réel (< 1 seconde end-to-end).

### 💰 Réduction des coûts
Architecture moderne, automatisée, cloud-native. Moins de dépendances legacy, plus d'efficacité opérationnelle.

## 3. Architecture en un coup d'œil

```
┌─────────────┐
│ Pays A/B/C  │ → CDR bruts (formats hétérogènes)
└──────┬──────┘
       ↓
┌─────────────────┐
│ Kafka Ingestion │ → Bus multi-pays
└──────┬──────────┘
       ↓
┌──────────────────────────────┐
│   Pipeline Rust              │
│  - Parsing                   │
│  - Validation                │
│  - Normalisation             │
│  - Enrichissement            │
│  - ML Fraud Scoring          │
└──────┬───────────────────────┘
       ↓
┌──────────────────┬─────────────────┐
│ Hot Storage      │ Cold Storage    │
│ (ScyllaDB)       │ (Ceph/MinIO)    │
│ - Temps réel     │ - Archive       │
│ - Requêtes       │ - Analytics     │
│   rapides        │ - Parquet/ORC   │
└──────────────────┴─────────────────┘
       ↓
┌──────────────────────────────┐
│ APIs & Analytics             │
│ - REST API                   │
│ - Grafana Dashboards         │
│ - Trino/Presto/Spark         │
│ - Exports Billing/BI         │
└──────────────────────────────┘
```

## 4. Stack technologique

### 🦀 Rust (cœur du pipeline)
- **Performance** : latence ultra-faible, concurrence sans overhead
- **Sécurité** : memory-safe, thread-safe par design
- **Fiabilité** : gestion d'erreurs explicite, pas de null pointers
- **Observabilité** : métriques Prometheus natives

### 📡 Kafka (ingestion)
- Bus de données distribué multi-pays
- Backpressure naturel
- Dead Letter Queue (DLQ) pour les erreurs
- Scalabilité horizontale

### 🗄️ ScyllaDB (hot storage)
- Base NoSQL ultra-performante (compatible Cassandra)
- Partitionnement temporel optimisé pour les CDR
- Ingestion massive (millions writes/sec)
- Requêtes temps réel sub-milliseconde

### 🪣 Ceph (cold storage)
- Stockage objet distribué (S3 compatible)
- Format Parquet/ORC compressé
- Lifecycle management automatique
- Perfect pour analytics batch

### 🤖 ML Rust (fraude)
- Agent d'inférence en Rust pur
- Modèle ONNX embarqué
- Latence d'inférence < 10ms
- Scoring temps réel sur chaque CDR

### 📊 Observabilité
- **Prometheus** : métriques temps réel
- **Grafana** : dashboards visuels
- **Loki** : logs centralisés
- **OpenTelemetry** : traces distribuées

## 5. Cas d'usage principaux

### 💳 Billing & Revenue Assurance
Accès rapide aux CDR par abonné, garantie de complétude, traçabilité des événements de facturation.

### 🚨 Détection de fraude
Scoring ML temps réel sur chaque CDR, alertes instantanées, analyse comportementale.

### 📞 Customer Care
Consultation rapide de l'historique client, résolution de litiges, vérification des consommations.

### 📡 Optimisation réseau
Analyse de la QoS par cellule, identification des zones à problème, dimensionnement capacité.

### 🌍 Roaming & Interconnexion
Suivi des CDR roaming, réconciliation inter-opérateurs, facturation wholesale.

### 📊 Analytics & BI
Datasets consolidés multi-pays, exports vers data lakes, dashboards exécutifs.

### ⚖️ Conformité & Audit
Rétention réglementaire, traçabilité complète, exports pour autorités.

## 6. Principes de conception

### 🎯 Modulaire
Microservices Rust indépendants, chacun avec une responsabilité claire. Facilite la maintenance et l'évolution.

### 📈 Scalable
Architecture distribuée horizontalement. Ajoutez des nodes pour augmenter la capacité.

### 🔒 Sécurisé
TLS partout, chiffrement au repos, RBAC, audit, masquage des données sensibles.

### 🚀 Performant
Latence end-to-end < 1 seconde, ingestion > 1M CDR/min/pays, requêtes < 10ms.

### 🔍 Observable
Logs structurés JSON, métriques Prometheus, traces OpenTelemetry, dashboards Grafana.

### 🌐 Multi-datacenter ready
Architecture pensée pour le multi-DC (V2), réplication, géo-distribution.

## 7. Versions et évolution

### 📦 V1 — Prototype (actuel)
- Pipeline Rust complet
- Ingestion Kafka multi-pays
- ScyllaDB + MinIO
- ML fraud agent basique
- Déploiement local + RHEL
- Documentation complète

### 🚀 V2 — Extension
- Introduction de Go pour I/O-bound services
- Kubernetes natif
- Feature store ML complet
- Multi-datacenter réel
- Ingestion TAP/NRTRDE native

### 🏆 V3 — Production
- SLA 99.99%
- Haute disponibilité multi-région
- Sécurité avancée (HSM, vault)
- Dashboards groupe unifiés
- Automatisation complète (GitOps)

## 8. Pour aller plus loin

- **Vision stratégique** : [vision.md](vision.md)
- **Glossaire technique** : [glossary.md](glossary.md)
- **Cadrage projet** : [../01-cadrage/note-de-cadrage.md](../01-cadrage/note-de-cadrage.md)
- **Architecture** : [../02-architecture/architecture-globale.md](../02-architecture/architecture-globale.md)
- **Schéma CDR** : [../03-data/schema-cdr-unifie.md](../03-data/schema-cdr-unifie.md)

---

**ORION Unified CDR Engine** — _Unifying Telecom Data at Scale_
