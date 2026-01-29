# 📚 Documentation ORION — Index complet

Bienvenue dans la documentation complète d'**ORION Unified CDR Engine**.

Cette documentation est organisée en 6 sections principales, couvrant tous les aspects du projet : vision, architecture, données, ML, déploiement et démonstration.

---

## 🗺️ Navigation rapide

| Section | Description | Docs clés |
|---------|-------------|-----------|
| **[00 - Vue d'ensemble](#00---vue-densemble)** | Introduction, vision, glossaire | Pour découvrir ORION |
| **[01 - Cadrage](#01---cadrage)** | Objectifs, roadmap, périmètre | Pour comprendre le projet |
| **[02 - Architecture](#02---architecture)** | Design global et détaillé | Pour les architectes |
| **[03 - Données](#03---données)** | Schémas, modèles, datasets | Pour les data engineers |
| **[04 - Machine Learning](#04---machine-learning)** | Agent fraud, features, modèle | Pour les data scientists |
| **[05 - Déploiement](#05---déploiement)** | Docker, RHEL, K8s, monitoring | Pour les ops/SRE |
| **[06 - Démonstration](#06---démonstration)** | Scénarios, scripts | Pour les démos |

---

## 00 - Vue d'ensemble

Découvrez ORION en quelques minutes : qu'est-ce que c'est, pourquoi ça existe, comment ça fonctionne.

### 📄 Documents

| Fichier | Description | Audience |
|---------|-------------|----------|
| **[introduction.md](00-overview/introduction.md)** | Introduction complète à ORION : objectifs, architecture, stack, cas d'usage | Tous |
| **[vision.md](00-overview/vision.md)** | Vision stratégique, impact attendu, roadmap long terme, philosophie | Management, architectes |
| **[glossary.md](00-overview/glossary.md)** | Glossaire technique complet : CDR, IMSI, Kafka, ScyllaDB, ML, etc. | Tous |

### 🎯 Commencer ici si...
- ❓ Vous découvrez le projet
- 👀 Vous voulez comprendre la vision
- 📖 Vous cherchez une définition technique

---

## 01 - Cadrage

Comprenez les objectifs, le périmètre, la roadmap et les livrables du projet.

### 📄 Documents

| Fichier | Description | Audience |
|---------|-------------|----------|
| **[cahier-des-charges.md](01-cadrage/cahier-des-charges.md)** | Cahier des charges complet : périmètre fonctionnel, exigences techniques, critères de succès | Chef de projet, architectes |
| **[note-de-cadrage.md](01-cadrage/note-de-cadrage.md)** | Note de cadrage V1 : contexte, périmètre prototype, exclusions, livrables | Management |
| **[roadmap.md](01-cadrage/roadmap.md)** | Roadmap multi-phases : V1 prototype → V2 pilote → V3 production | Tous |

### 🎯 Commencer ici si...
- 📋 Vous définissez le périmètre
- 🗓️ Vous planifiez le projet
- ✅ Vous validez les objectifs

---

## 02 - Architecture

Plongez dans l'architecture technique : design global, composants détaillés, décisions clés, sécurité.

### 📄 Documents

| Fichier | Description | Audience |
|---------|-------------|----------|
| **[architecture-globale.md](02-architecture/architecture-globale.md)** | Vue d'ensemble : macro-zones, flux principal, stack technologique | Architectes, développeurs |
| **[architecture-detaillee.md](02-architecture/architecture-detaillee.md)** | Détail des microservices Rust, interactions, patterns, APIs internes | Développeurs |
| **[securite.md](02-architecture/securite.md)** | Principes de sécurité : réseau, données, applicatif, opérationnel, ML | Sécurité, ops |

### 📁 Sous-dossiers

#### 📂 `decisions/` — Architecture Decision Records (ADR)
Documentation des décisions architecturales majeures.

| Fichier | Décision | Justification |
|---------|----------|---------------|
| **[adr-001-rust-only.md](02-architecture/decisions/adr-001-rust-only.md)** | V1 100% Rust | Cohérence, performance, sécurité |
| **[adr-002-scylla.md](02-architecture/decisions/adr-002-scylla.md)** | ScyllaDB pour hot storage | Ingestion massive, faible latence |
| **[adr-003-ceph.md](02-architecture/decisions/adr-003-ceph.md)** | Ceph pour cold storage | Stockage objet, S3, Parquet |
| **[adr-004-kafka-multi-pays.md](02-architecture/decisions/adr-004-kafka-multi-pays.md)** | Kafka comme bus | Backpressure, DLQ, multi-topics |
| **[adr-005-ml-grpc.md](02-architecture/decisions/adr-005-ml-grpc.md)** | ML agent via gRPC | Faible latence, typage strict |

#### 📂 `diagrammes/`
Diagrammes d'architecture (à venir : PlantUML, Mermaid, Draw.io).

### 🎯 Commencer ici si...
- 🏗️ Vous concevez l'architecture
- 🔍 Vous devez justifier des choix techniques
- 🔒 Vous travaillez sur la sécurité

---

## 03 - Données

Tout sur les données : schéma CDR unifié, modèle ScyllaDB, datasets de test.

### 📄 Documents

| Fichier | Description | Audience |
|---------|-------------|----------|
| **[schema-cdr-unifie.md](03-data/schema-cdr-unifie.md)** | Schéma CDR unifié groupe : structure, champs, types, exemples JSON | Data engineers, développeurs |
| **[scylladb-model.md](03-data/scylladb-model.md)** | Modèle de données ScyllaDB : tables, partition keys, clustering, fenêtres temporelles | Data engineers, DBAs |
| **[datasets.md](03-data/datasets.md)** | Datasets CDR : formats bruts multi-pays, normalisés, ML, génération, volumétrie | Data engineers, ML engineers |

### 🎯 Commencer ici si...
- 📊 Vous travaillez sur le modèle de données
- 🗄️ Vous concevez les tables ScyllaDB
- 🧪 Vous générez des datasets de test

---

## 04 - Machine Learning

Détection de fraude temps réel : agent ML, features, modèle.

### 📄 Documents

| Fichier | Description | Audience |
|---------|-------------|----------|
| **[fraud-agent.md](04-ml/fraud-agent.md)** | Agent ML Rust : architecture, interface gRPC, inférence, sécurité | ML engineers, développeurs |
| **[features.md](04-ml/features.md)** | Features ML : localisation, temporelles, usage, roaming, abonné, réseau | Data scientists |
| **[model.md](04-ml/model.md)** | Modèle ML : type, format ONNX, pipeline d'inférence, évolutions | Data scientists |

### ⚠️ Note
La section ML sera complétée lors de la phase avancée (projet séparé). La V1 couvre les bases.

### 🎯 Commencer ici si...
- 🤖 Vous travaillez sur la détection de fraude
- 📈 Vous définissez les features ML
- 🧠 Vous intégrez un modèle ONNX

---

## 05 - Déploiement

Déployez ORION : local (Docker), production (RHEL), Kubernetes, monitoring.

### 📄 Documents

| Fichier | Description | Audience |
|---------|-------------|----------|
| **[docker-local.md](05-deploiement/docker-local.md)** | Déploiement Docker Compose complet : architecture, prérequis, docker-compose.yml, configuration | Développeurs, ops |
| **[rhel.md](05-deploiement/rhel.md)** | Déploiement RHEL : utilisateurs, répertoires, TLS, SELinux, firewall, dépendances | Ops, SRE |
| **[systemd.md](05-deploiement/systemd.md)** | Services systemd : unités, démarrage, logs, commandes | Ops, SRE |
| **[monitoring.md](05-deploiement/monitoring.md)** | Monitoring : métriques Prometheus, dashboards Grafana, alerting | Ops, SRE |

### 🎯 Commencer ici si...
- 🐳 Vous déployez en local (démo)
- 🖥️ Vous déployez sur RHEL (production)
- 📊 Vous configurez la supervision

---

## 06 - Démonstration

Scénarios de démo, scripts d'automatisation, procédures de validation.

### 📄 Documents

| Fichier | Description | Audience |
|---------|-------------|----------|
| **[scenario.md](06-demo/scenario.md)** | Scénario de démonstration end-to-end : étapes, résultat attendu | Tous |
| **[scripts.md](06-demo/scripts.md)** | Scripts PowerShell : génération CDR, producteur Kafka, health checks, monitoring, diagnostics | Développeurs, ops |

### 📁 Sous-dossiers

#### 📂 `screenshots/`
Captures d'écran des dashboards, interfaces, résultats (à venir).

### 🎯 Commencer ici si...
- 🎬 Vous préparez une démonstration
- 🧪 Vous testez le pipeline end-to-end
- 🔧 Vous automatisez la validation

---

## 📖 Guides transverses

### 👨‍💻 Pour les développeurs
1. **[../guide_de_reprise.md](../guide_de_reprise.md)** — Guide complet pour reprendre le code Rust
2. **[02-architecture/architecture-detaillee.md](02-architecture/architecture-detaillee.md)** — Détail des microservices
3. **[03-data/schema-cdr-unifie.md](03-data/schema-cdr-unifie.md)** — Schéma de données
4. **[05-deploiement/docker-local.md](05-deploiement/docker-local.md)** — Environnement de dev

### 🏗️ Pour les architectes
1. **[00-overview/vision.md](00-overview/vision.md)** — Vision stratégique
2. **[02-architecture/architecture-globale.md](02-architecture/architecture-globale.md)** — Vue d'ensemble
3. **[02-architecture/decisions/](02-architecture/decisions/)** — ADR (décisions)
4. **[01-cadrage/cahier-des-charges.md](01-cadrage/cahier-des-charges.md)** — Exigences complètes

### 📊 Pour les data engineers
1. **[03-data/schema-cdr-unifie.md](03-data/schema-cdr-unifie.md)** — Schéma CDR
2. **[03-data/scylladb-model.md](03-data/scylladb-model.md)** — Modèle ScyllaDB
3. **[03-data/datasets.md](03-data/datasets.md)** — Datasets
4. **[04-ml/features.md](04-ml/features.md)** — Features ML

### 🤖 Pour les data scientists
1. **[04-ml/fraud-agent.md](04-ml/fraud-agent.md)** — Agent ML
2. **[04-ml/features.md](04-ml/features.md)** — Features disponibles
3. **[04-ml/model.md](04-ml/model.md)** — Modèle et inférence
4. **[03-data/datasets.md](03-data/datasets.md)** — Datasets ML

### ⚙️ Pour les ops/SRE
1. **[05-deploiement/docker-local.md](05-deploiement/docker-local.md)** — Déploiement local
2. **[05-deploiement/rhel.md](05-deploiement/rhel.md)** — Déploiement RHEL
3. **[05-deploiement/monitoring.md](05-deploiement/monitoring.md)** — Supervision
4. **[02-architecture/securite.md](02-architecture/securite.md)** — Sécurité

### 🎬 Pour les démos
1. **[06-demo/scenario.md](06-demo/scenario.md)** — Scénario de démo
2. **[06-demo/scripts.md](06-demo/scripts.md)** — Scripts d'automatisation
3. **[00-overview/introduction.md](00-overview/introduction.md)** — Présentation ORION
4. **[05-deploiement/docker-local.md](05-deploiement/docker-local.md)** — Setup démo

---

## 🔗 Liens externes

### Outils & Technologies
- **[Rust Book](https://doc.rust-lang.org/book/)** — Apprendre Rust
- **[Tokio Docs](https://tokio.rs/)** — Runtime async Rust
- **[ScyllaDB Docs](https://docs.scylladb.com/)** — Documentation ScyllaDB
- **[Kafka Docs](https://kafka.apache.org/documentation/)** — Apache Kafka
- **[Prometheus Docs](https://prometheus.io/docs/)** — Monitoring
- **[Grafana Docs](https://grafana.com/docs/)** — Dashboards

### Standards télécom
- **[GSMA TAP](https://www.gsma.com/services/tap/)** — Roaming data exchange
- **[3GPP CDR Specs](https://www.3gpp.org/)** — Spécifications CDR

---

## 🆘 Besoin d'aide ?

### 🔍 Vous ne trouvez pas ce que vous cherchez ?
Consultez le **[glossaire](00-overview/glossary.md)** pour les définitions techniques.

### 💡 Vous avez une question ?
Référez-vous aux sections appropriées ci-dessus ou contactez l'équipe.

### 🐛 Vous avez trouvé une erreur dans la doc ?
Ouvrez une issue ou proposez une correction (PR).

---

<p align="center">
  <strong>Documentation ORION V1.0</strong><br>
  <em>Dernière mise à jour : Janvier 2026</em>
</p>
