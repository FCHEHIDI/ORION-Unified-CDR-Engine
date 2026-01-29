# 📚 Glossaire — ORION Unified CDR Engine

## A

### ADR (Architecture Decision Record)
Document traçant une décision architecturale importante, son contexte, les alternatives considérées et les conséquences.

### API (Application Programming Interface)
Interface de programmation permettant l'accès aux données CDR via REST/gRPC.

### Async/Asynchrone
Mode de programmation non-bloquant utilisé massivement dans ORION via Tokio (Rust).

### Axum
Framework web Rust moderne et performant utilisé pour tous les endpoints HTTP d'ORION.

## B

### Backpressure
Mécanisme de contrôle de flux permettant à un consommateur de ralentir un producteur en cas de surcharge.

### Billing
Système de facturation utilisant les CDR comme source de données principale.

### Batch
Traitement par lots (opposé à temps réel), généralement pour les archives ou analytics.

## C

### CDR (Call Detail Record)
Enregistrement détaillé d'un événement télécom (appel, SMS, session data, roaming). Donnée centrale d'ORION.

### CellID
Identifiant unique d'une cellule radio (antenne). Utilisé pour la localisation approximative et l'analyse QoS.

### Ceph
Système de stockage objet distribué utilisé comme cold storage dans ORION (S3 compatible).

### Clustering Key
Dans ScyllaDB/Cassandra, clé de tri au sein d'une partition. Ex : `event_time` dans `cdr_by_imsi_day`.

### Cold Storage
Stockage archive long terme, optimisé pour le coût (Ceph/MinIO + Parquet/ORC compressé).

### Compliance / Conformité
Respect des réglementations (GDPR, rétention légale, audit). ORION intègre la conformité by design.

## D

### DLQ (Dead Letter Queue)
File d'attente Kafka recevant les messages en erreur pour traitement ultérieur.

### Data Lake
Lac de données consolidé pour analytics avancées. ORION peut exporter vers Trino/Spark.

### Drift (ML)
Dérive d'un modèle ML quand les données réelles s'éloignent des données d'entraînement.

## E

### Enrichment / Enrichissement
Phase du pipeline ajoutant des informations contextuelles au CDR (segment client, localisation, score fraude).

### Event Time
Horodatage réel de l'événement télécom (vs. processing time = quand ORION le traite).

## F

### Feature Store
Système centralisé de gestion des features ML. Prévu en V2 d'ORION.

### Fraud Detection / Détection de Fraude
Identification en temps réel de comportements suspects via ML (agent Rust dans ORION).

## G

### GDPR (General Data Protection Regulation)
Réglementation européenne sur la protection des données. ORION masque les IMSI/MSISDN dans les logs.

### Grafana
Plateforme de visualisation utilisée pour les dashboards ORION (métriques, logs, traces).

### gRPC
Protocole RPC haute performance utilisé pour l'agent ML fraud d'ORION.

## H

### Hot Storage
Stockage temps réel haute performance (ScyllaDB) pour requêtes < 10ms.

### Helm
Gestionnaire de packages Kubernetes. ORION fournit un chart Helm pour déploiement K8s.

## I

### Idempotence
Propriété garantissant qu'une opération peut être rejouée sans effet de bord (ex : écriture CDR avec `charging_id` unique).

### IMEI (International Mobile Equipment Identity)
Identifiant unique d'un terminal mobile (15 chiffres).

### IMSI (International Mobile Subscriber Identity)
Identifiant unique d'une carte SIM (jusqu'à 15 chiffres). Donnée sensible masquée dans les logs ORION.

### Ingestion
Première phase du pipeline : lecture des CDR bruts depuis Kafka.

## J

### JSON (JavaScript Object Notation)
Format de données utilisé pour les CDR normalisés et les logs structurés ORION.

## K

### Kafka
Plateforme de streaming distribué utilisée comme bus d'ingestion multi-pays dans ORION.

### Kubernetes / K8s
Orchestrateur de conteneurs. ORION V2 sera Kubernetes-native.

## L

### LAC (Location Area Code)
Code de zone de localisation en 2G/3G.

### Latency / Latence
Temps de traitement. Objectif ORION : < 1 seconde end-to-end, < 10ms pour requêtes ScyllaDB.

### Loki
Système d'agrégation de logs compatible Grafana, recommandé pour ORION.

## M

### MCC (Mobile Country Code)
Code pays mobile (ex : 208 pour France).

### Metrics / Métriques
Données quantitatives exposées par les microservices ORION via endpoint `/metrics` (Prometheus).

### MinIO
Implémentation S3-compatible open-source, utilisée comme alternative à Ceph en local/démo.

### ML (Machine Learning)
Apprentissage automatique. ORION intègre un agent ML Rust pour la détection de fraude.

### MNC (Mobile Network Code)
Code opérateur mobile (ex : 15 pour Orange France).

### MSISDN (Mobile Station International Subscriber Directory Number)
Numéro de téléphone international (ex : +33612345678). Donnée sensible masquée dans les logs.

## N

### Normalization / Normalisation
Phase du pipeline transformant les CDR hétérogènes vers le schéma unifié ORION.

### NRTRDE (Near Real-Time Roaming Data Exchange)
Standard d'échange de données roaming temps réel (V2).

## O

### Observability / Observabilité
Capacité à comprendre l'état interne du système via logs, métriques et traces.

### ONNX (Open Neural Network Exchange)
Format standard pour modèles ML. Utilisé pour l'agent fraud ORION.

### OpenTelemetry
Framework d'observabilité pour traces distribuées (prévu ORION V2).

## P

### Parquet
Format de stockage colonne optimisé pour analytics (utilisé dans cold storage ORION).

### Partition Key
Dans ScyllaDB/Cassandra, clé déterminant la distribution des données. Ex : `(imsi, event_date)`.

### Pipeline
Chaîne de traitement : ingestion → validation → normalisation → enrichissement → storage.

### Prometheus
Système de monitoring time-series utilisé pour toutes les métriques ORION.

## Q

### QoS (Quality of Service)
Qualité de service réseau. ORION permet l'analyse QoS par cellule via la table `cdr_by_cell_hour`.

## R

### RAT (Radio Access Technology)
Technologie d'accès radio : 2G / 3G / 4G / 5G.

### RBAC (Role-Based Access Control)
Contrôle d'accès basé sur les rôles. Prévu pour l'API ORION.

### Replication Factor (RF)
Nombre de copies d'une donnée dans ScyllaDB. Recommandé : RF=3 pour ORION.

### REST API
Interface HTTP exposée par `orion-api` pour requêtes CDR.

### Roaming
Utilisation du réseau mobile hors du pays d'origine. ORION suit les CDR roaming et partenaires.

### Rust
Langage système moderne utilisé pour 100% du pipeline ORION (V1). Garantit performance et sécurité mémoire.

## S

### ScyllaDB
Base NoSQL haute performance (compatible Cassandra) utilisée comme hot storage ORION.

### Schema / Schéma
Structure de données. Le "schéma CDR unifié" est le cœur d'ORION.

### Segment (client)
Catégorie client (pro, perso, VIP). Utilisé pour enrichissement CDR.

### SELinux
Système de sécurité Linux obligatoire sur les déploiements RHEL ORION (mode enforcing).

### SFTP (SSH File Transfer Protocol)
Protocole de transfert de fichiers sécurisé. Utilisé pour ingestion legacy dans ORION.

### Stateless
Sans état. L'agent ML ORION est stateless pour simplifier le scale.

### systemd
Gestionnaire de services Linux utilisé pour déploiement RHEL ORION.

## T

### TAC (Tracking Area Code)
Code de zone de suivi en 4G/5G (équivalent LAC).

### TAP (Transferred Account Procedure)
Format d'échange de données roaming (standard GSMA).

### Time-Series
Données indexées par temps. ScyllaDB est optimisé pour time-series (CDR).

### TLS (Transport Layer Security)
Chiffrement réseau obligatoire pour tous les services ORION.

### Tokio
Runtime async Rust utilisé partout dans ORION.

### Tracing
Traçage distribué des requêtes à travers les microservices (OpenTelemetry en V2).

### Trino / Presto
Moteurs SQL distribués pour analytics. Compatible avec le cold storage ORION (Parquet sur Ceph/S3).

## U

### Unified Schema / Schéma Unifié
Le schéma CDR commun à tous les pays dans ORION. Permet la consolidation groupe.

## V

### Validation
Phase du pipeline vérifiant la conformité des CDR (champs obligatoires, types, cohérence métier).

### Vault (HashiCorp)
Gestionnaire de secrets recommandé pour ORION production (certificats, tokens, clés).

## W

### Workspace Cargo
Projet Rust multi-crates. ORION utilise un workspace avec 10 microservices.

### Write Path
Chemin d'écriture des données : Kafka → Pipeline → ScyllaDB/Ceph.

## X

### xDR
Terme générique englobant CDR, EDR (Event Detail Record), UDR (Usage Detail Record).

## Z

### Zero-Copy
Technique d'optimisation évitant les copies mémoire inutiles. Rust permet le zero-copy naturellement.

---

## Acronymes rapides

| Acronyme | Signification |
|----------|---------------|
| **CDR** | Call Detail Record |
| **API** | Application Programming Interface |
| **ML** | Machine Learning |
| **gRPC** | gRPC Remote Procedure Call |
| **TLS** | Transport Layer Security |
| **IMSI** | International Mobile Subscriber Identity |
| **MSISDN** | Mobile Station ISDN Number |
| **IMEI** | International Mobile Equipment Identity |
| **RAT** | Radio Access Technology |
| **QoS** | Quality of Service |
| **MCC** | Mobile Country Code |
| **MNC** | Mobile Network Code |
| **LAC** | Location Area Code |
| **TAC** | Tracking Area Code |
| **GDPR** | General Data Protection Regulation |
| **DLQ** | Dead Letter Queue |
| **RF** | Replication Factor |
| **RBAC** | Role-Based Access Control |
| **ADR** | Architecture Decision Record |
| **K8s** | Kubernetes |

---

**ORION Unified CDR Engine** — _Glossaire technique V1.0_
