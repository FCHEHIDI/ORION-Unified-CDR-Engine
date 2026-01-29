📘 Cahier des charges — Version corrigée
ORION Unified CDR Engine
Plateforme Groupe de Traitement, Consolidation et Analyse des CDR
1. Objectifs stratégiques
La plateforme ORION doit permettre au groupe télécom :

de collecter, normaliser, enrichir, analyser et stocker les CDR provenant de plusieurs pays ;

de fournir une vision unifiée des usages voix/SMS/data/roaming ;

de supporter les besoins billing, fraude, analytics, reporting réglementaire et optimisation réseau ;

d’offrir un pipeline robuste, scalable, sécurisé et auditable ;

d’intégrer un agent ML de détection de fraude en temps réel ;

de réduire les coûts d’exploitation via une architecture moderne, modulaire et automatisée.

2. Périmètre fonctionnel
2.1. Collecte multi‑pays
ORION doit ingérer des CDR provenant de :

réseaux mobiles (2G/3G/4G/5G),

réseaux fixes,

plateformes roaming (TAP, NRTRDE),

systèmes legacy (fichiers batch, SFTP),

flux temps réel (Kafka multi‑cluster).

2.2. Normalisation
La plateforme doit :

parser des formats hétérogènes,

valider les champs obligatoires,

appliquer un schéma CDR unifié groupe,

gérer les erreurs via une DLQ.

2.3. Enrichissement
ORION doit enrichir les CDR avec :

données réseau (CellID → localisation approximative),

données client (segment, plan tarifaire),

données roaming (pays, partenaire),

données techniques (RAT, QoS),

score de fraude via un agent ML Rust.

2.4. Stockage Hot (temps réel)
Les CDR enrichis doivent être stockés dans une base :

distribuée,

haute performance,

multi‑datacenter,

optimisée pour les requêtes temps réel.

→ ScyllaDB/Cassandra.

2.5. Stockage Cold (archive)
Les CDR doivent être archivés dans :

un stockage objet Ceph,

format Parquet/ORC,

compressé,

partitionné par pays/jour/type.

2.6. Exposition des données
La plateforme doit fournir :

APIs internes (Rust),

exports batch,

accès analytique (Trino/Presto/Spark),

dashboards (Grafana/Metabase),

flux vers systèmes billing/fraud/BI.

3. Exigences techniques
3.1. Langage & stack
La première version du projet est 100 % Rust, incluant :

ingestion Kafka,

parsing/validation,

enrichment,

ML inference,

stockage hot/cold,

APIs internes,

orchestration interne.

Une note de cadrage proposera un refactoring ultérieur pour introduire Go sur les services I/O‑bound (orchestrateur, API, storage gateways).

3.2. Architecture
microservices Rust par domain,

communication via Kafka + gRPC,

backpressure natif,

supervision intégrée,

observabilité complète (logs, metrics, traces).

3.3. Performance
ingestion ≥ 1 million CDR/minute par pays,

latence d’enrichissement < 50 ms,

stockage hot < 10 ms par écriture,

archivage cold < 5 minutes.

3.4. Scalabilité
scale horizontal automatique,

multi‑datacenter actif/actif,

réplication inter‑pays configurable.

3.5. Sécurité
chiffrement en transit (TLS),

chiffrement au repos (Ceph, Scylla),

RBAC par service,

audit complet,

conformité GDPR + obligations locales.

4. Exigences data
4.1. Schéma CDR unifié groupe
Le CDR unifié doit inclure :

identifiants (IMSI, MSISDN, IMEI),

localisation (CellID, LAC/TAC, pays),

temporalité (event_time, window),

usage (duration, bytes_up/down),

type d’événement (voice, sms, data, roaming),

enrichissements (segment, plan, RAT),

score ML (fraude),

métadonnées techniques.

4.2. Modélisation ScyllaDB
Tables principales :

cdr_by_imsi_day

cdr_by_cell_hour

cdr_by_country_day

cdr_by_partner_day

cdr_by_event_type_day

Partitionnement :

(imsi, day)

(cell_id, hour)

(country, day)

(partner, day)

Fenêtres temporelles :

heure pour radio/QoS,

jour pour billing,

mois pour analytics.

5. Exigences ML (fraude)
5.1. Agent ML Rust
L’agent doit :

charger un modèle ML (ONNX ou Rust‑native),

calculer un score de fraude en temps réel,

exposer une API interne gRPC,

être stateless (feature store externe).

5.2. Features
mobilité anormale,

usage nocturne,

patterns de roaming,

anomalies de volume,

historique IMSI/MSISDN.

5.3. Latence
inference < 10 ms.

6. Exigences d’exploitation
déploiement sur RHEL,

services systemd ou Kubernetes,

monitoring Prometheus,

logs structurés,

rotation automatique,

alerting (lag Kafka, latence DB, erreurs ML).

7. Livrables
code source Rust,

schéma ScyllaDB complet,

configuration Ceph,

documentation d’architecture,

scripts de déploiement,

dataset de démonstration,

scénario de démonstration (end‑to‑end),

tableau de bord Grafana.

8. Critères de succès
pipeline stable sous forte charge,

démonstration fluide et compréhensible,

architecture claire et modulaire,

ML anti‑fraude fonctionnel,

stockage hot/cold opérationnel,

documentation professionnelle.