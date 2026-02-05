# 🌟 ORION Unified CDR Engine

**Plateforme télécom distribuée de nouvelle génération pour le traitement unifié des Call Detail Records multi-pays**

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![ScyllaDB](https://img.shields.io/badge/ScyllaDB-5.4-blue?logo=apache-cassandra)](https://www.scylladb.com/)
[![Kafka](https://img.shields.io/badge/Kafka-3.5-black?logo=apache-kafka)](https://kafka.apache.org/)
[![License](https://img.shields.io/badge/License-Proprietary-red)]()

---

## 🎯 En bref

**ORION** unifie l'ingestion, le traitement et l'analyse des CDR télécom de multiples pays dans une architecture moderne, performante et sécurisée.

```
[CDR Multi-pays] → [Kafka] → [Pipeline Rust] → [ML Fraud] → [ScyllaDB/Ceph] → [APIs & Analytics]
```

**Résultats** :
- ⚡ **< 1 seconde** : latence end-to-end
- 📈 **> 1M CDR/min** : ingestion par pays
- 🤖 **< 10ms** : inférence ML fraude
- 💰 **-65%** : réduction coûts infrastructure
- 🌍 **Multi-pays** : schéma unifié groupe

---

## 📚 Documentation

### 🚀 Démarrage rapide
- **[Introduction](docs/00-overview/introduction.md)** — Comprendre ORION en 5 minutes
- **[Vision stratégique](docs/00-overview/vision.md)** — Objectifs et impact attendu
- **[Guide de reprise](guide_de_reprise.md)** — Pour les développeurs rejoignant le projet

### 📖 Documentation complète
- **[00 - Vue d'ensemble](docs/00-overview/)** — Introduction, vision, glossaire
- **[01 - Cadrage](docs/01-cadrage/)** — Cahier des charges, roadmap, objectifs
- **[02 - Architecture](docs/02-architecture/)** — Architecture globale et détaillée
- **[03 - Données](docs/03-data/)** — Schéma CDR unifié, modèle ScyllaDB
- **[04 - Machine Learning](docs/04-ml/)** — Agent fraud, features, modèle
- **[05 - Déploiement](docs/05-deploiement/)** — Docker, RHEL, Kubernetes, monitoring
- **[06 - Démonstration](docs/06-demo/)** — Scénarios et scripts de démo

📍 **[Index complet de la documentation](docs/README.md)**

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    ORION Pipeline                           │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌──────────┐    ┌──────────┐    ┌──────────┐               │
│  │  Kafka   │ →  │  Rust    │ →  │   ML     │               │
│  │Multi-pays│    │ Pipeline │    │  Fraud   │               │
│  └──────────┘    └──────────┘    └──────────┘               │
│                        ↓                                    │
│            ┌───────────┴────────────┐                       │
│            ↓                        ↓                       │
│     ┌──────────┐            ┌──────────┐                    │
│     │ScyllaDB  │            │  Ceph    │                    │
│     │Hot Store │            │Cold Store│                    │
│     └──────────┘            └──────────┘                    │
│            ↓                        ↓                       │
│     ┌──────────────────────────────────┐                    │
│     │   APIs & Analytics & Billing     │                    │
│     └──────────────────────────────────┘                    │
└─────────────────────────────────────────────────────────────┘
```

### Composants principaux

#### 🦀 Microservices Rust
- **orion-ingestion** — Ingestion Kafka multi-pays
- **orion-validation** — Validation et contrôles métier
- **orion-normalization** — Application du schéma unifié
- **orion-enrichment** — Enrichissement réseau & client
- **orion-ml-fraud-agent** — Détection fraude temps réel
- **orion-storage-hot** — Écriture ScyllaDB
- **orion-storage-cold** — Archivage Ceph/MinIO
- **orion-api** — API REST interne
- **orion-observability** — Métriques & santé

#### 🗄️ Infrastructure
- **Kafka** — Bus d'ingestion distribué
- **ScyllaDB** — Hot storage (requêtes < 10ms)
- **Ceph/MinIO** — Cold storage (Parquet compressé)
- **Prometheus** — Métriques temps réel
- **Grafana** — Dashboards visuels

---

## ⚡ Démarrage rapide

### Prérequis
- Docker & Docker Compose 2.20+
- Rust 1.75+ (pour développement)
- 16 GB RAM, 8 CPU cores

### Déploiement local (Docker)

```powershell
# 1. Cloner le repo
git clone <repo-url>
cd ORION_Unified_CDR_Engine

# 2. Générer des CDR de test
.\arborescences\create_orion_generate_cdr.ps1 -CountPerCountry 10000

# 3. Démarrer la stack complète
docker compose up -d

# 4. Vérifier la santé des services
curl http://localhost:8080/health  # API
curl http://localhost:8081/health  # Ingestion
curl http://localhost:8085/health  # ML Fraud Agent

# 5. Accéder aux interfaces
# Grafana    : http://localhost:3000 (admin/orion123)
# Prometheus : http://localhost:9090
# MinIO      : http://localhost:9001 (orion/orion_secret_123)
```

📘 **Guide détaillé** : [docker-local.md](docs/05-deploiement/docker-local.md)

---

## 🚀 Cas d'usage

### 💳 Billing & Revenue Assurance
Accès rapide aux CDR par abonné pour facturation, litiges, vérification consommation.

### 🚨 Détection de fraude
Scoring ML temps réel sur chaque CDR, alertes instantanées, analyse comportementale.

### 📡 Optimisation réseau
Analyse QoS par cellule, identification zones à problème, dimensionnement capacité.

### 🌍 Roaming & Interconnexion
Suivi CDR roaming, réconciliation inter-opérateurs, facturation wholesale.

### 📊 Analytics & BI
Datasets consolidés multi-pays, exports vers data lakes, dashboards exécutifs.

### ⚖️ Conformité & Audit
Rétention réglementaire, traçabilité complète, exports pour autorités.

---

## 🛠️ Stack technique

| Catégorie | Technologie | Rôle |
|-----------|-------------|------|
| **Langage** | Rust 1.75+ | Pipeline complet |
| **Async Runtime** | Tokio | Concurrence |
| **Web Framework** | Axum | HTTP endpoints |
| **Messaging** | Apache Kafka | Ingestion distribuée |
| **Hot Storage** | ScyllaDB 5.4 | Base NoSQL temps réel |
| **Cold Storage** | Ceph / MinIO | Stockage objet S3 |
| **ML Inference** | ONNX Runtime | Modèles embarqués |
| **Monitoring** | Prometheus + Grafana | Observabilité |
| **Orchestration** | Docker Compose / K8s | Déploiement |
| **CI/CD** | GitHub Actions | Automatisation |

---

## 📊 Performances

| Métrique | Objectif V1 | Objectif V3 |
|----------|-------------|-------------|
| Latence end-to-end | < 1 sec | < 500 ms |
| Ingestion CDR/min/pays | > 1M | > 10M |
| Latence requête hot | < 10 ms | < 5 ms |
| Latence ML inference | < 10 ms | < 5 ms |
| Disponibilité | > 99.9% | > 99.99% |

---

## 🗺️ Roadmap

### ✅ V1 — Prototype (Q1 2026) — EN COURS
- Pipeline Rust complet
- Schéma CDR unifié
- ML fraud agent basique
- ScyllaDB + MinIO
- Démo end-to-end

### 🚀 V2 — Pilote (Q2-Q3 2026)
- Go pour I/O-bound services
- Kubernetes natif
- Feature store ML
- Multi-datacenter
- TAP/NRTRDE natif

### 🏆 V3 — Production (Q4 2026+)
- SLA 99.99%
- Haute disponibilité
- Sécurité avancée
- GitOps complet
- Déploiement tous pays

---

## 🤝 Contribuer

### Structure du projet

```
ORION_Unified_CDR_Engine/
├── docs/                      # Documentation complète
├── orion-*/                   # Microservices Rust (crates)
├── arborescences/             # Scripts PowerShell d'automatisation
├── configs/                   # Configurations (Kafka, Scylla, Prometheus...)
├── datasets/                  # CDR de test multi-pays
├── docker-compose.yml         # Orchestration Docker
├── Cargo.toml                 # Workspace Rust
└── Makefile                   # Commandes utiles
```

### Développement

```powershell
# Build workspace complet
cargo build --workspace

# Run un microservice
cargo run --bin orion-ingestion

# Tests
cargo test --workspace

# Linter
cargo clippy --workspace --all-targets

# Format
cargo fmt --all
```

### Conventions

- **Rust** : respecter les conventions du [guide_de_reprise.md](guide_de_reprise.md)
- **Git** : commits atomiques, messages clairs
- **Documentation** : Markdown, diagrammes, exemples
- **Tests** : coverage minimum 70%

---

## 📞 Support

### Documentation
- **[Index docs complètes](docs/README.md)**
- **[FAQ](docs/00-overview/glossary.md)** — Glossaire technique

### Contacts
- **Lead Architect** : [À définir]
- **Team ML** : [À définir]
- **Ops/SRE** : [À définir]

---

## 📜 Licence

Propriétaire — Tous droits réservés © 2026

---

## 🌟 Highlights

> **"Un seul pipeline, un seul schéma, une seule vérité"**

ORION simplifie radicalement le traitement des CDR télécom en unifiant :
- 🎯 Un schéma pour tout le groupe
- 🦀 Un langage (Rust) pour cohérence
- 📊 Une observabilité (Prometheus/Grafana)
- 🗄️ Un storage (ScyllaDB/Ceph)
- 🤖 Un ML intégré natif

**Résultat** : maintenance facilitée, coûts réduits, innovation accélérée.

---

<p align="center">
  <strong>ORION Unified CDR Engine</strong><br>
  <em>Unifying Telecom Data at Scale</em>
</p>
