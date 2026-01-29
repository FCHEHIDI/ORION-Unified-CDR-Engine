📘 1. Note de cadrage — ORION Unified CDR Engine
Version 1.0 — Prototype démonstrable
🎯 Objectif de la note de cadrage
Cette note définit :

le périmètre du prototype ORION,

les choix technologiques retenus pour la première version,

les exclusions et évolutions futures,

les livrables attendus,

les contraintes et hypothèses.

Elle sert de référence pour cadrer le développement, la démonstration et les discussions avec une équipe d’experts.

1. Contexte
Le groupe télécom opère dans plusieurs pays et souhaite disposer d’une plateforme unifiée pour :

collecter les CDR multi‑pays,

les normaliser,

les enrichir,

les stocker (hot/cold),

les analyser,

détecter la fraude en temps réel.

Le prototype ORION doit démontrer la faisabilité technique, la robustesse du pipeline et la pertinence d’un moteur Rust pour les workloads télécom.

2. Périmètre du prototype (Version 1)
La version 1 du projet ORION inclut :

✔️ Ingestion Kafka
lecture de CDR simulés via Kafka

gestion du backpressure

DLQ simple

✔️ Pipeline Rust complet
parsing

validation

normalisation

enrichissement simple (CellID, RAT, pays)

appel à un agent ML Rust pour scoring fraude

stockage hot (ScyllaDB)

stockage cold (Ceph ou mock local)

✔️ Schéma CDR unifié groupe
version minimale mais réaliste

compatible multi‑pays

✔️ Observabilité
logs structurés

métriques Prometheus

traces internes

✔️ Documentation complète
architecture

schéma data

pipeline

déploiement local

scénario de démonstration

3. Hors périmètre (Version 1)
Ces éléments sont explicitement exclus de la première version, mais prévus dans l’évolution :

❌ Multi‑datacenter réel
→ simulé uniquement.

❌ Intégration Go pour les services I/O‑bound
→ une note technique justifiera ce refactoring futur.

❌ Kubernetes / orchestrateurs avancés
→ déploiement local + RHEL simple.

❌ Feature store ML complet
→ version simplifiée (cache local ou mock).

❌ Connecteurs TAP/NRTRDE réels
→ simulation via fichiers ou Kafka.

4. Choix technologiques (Version 1)
🦀 Rust (100 % du pipeline)
ingestion

validation

enrichment

ML inference

hot storage writer

cold storage writer

API interne

supervision

Justification :

performance CPU

sécurité mémoire

robustesse

démonstration claire et cohérente

🗃️ ScyllaDB (hot storage)
tables partitionnées par pays/jour/heure

ingestion massive

requêtes rapides

🪣 Ceph (cold storage)
Parquet/ORC

compression

lifecycle

🔮 ML Rust (fraude)
modèle ONNX ou Rust‑native

scoring temps réel

5. Livrables
code source Rust (multi‑crates)

schéma ScyllaDB

configuration Ceph (ou mock)

dataset de démonstration

documentation complète (dans docs/)

diagrammes d’architecture

scénario de démonstration

tableau de bord Grafana

6. Hypothèses
Kafka est disponible (local ou cluster simulé)

ScyllaDB est accessible (local ou docker)

Ceph peut être simulé par MinIO si nécessaire

Le modèle ML est simple (binaire ou score)

Le volume simulé est représentatif mais réduit

7. Risques identifiés
complexité du pipeline Rust multi‑crates

gestion des performances en local

disponibilité des datasets multi‑pays

temps de développement du ML agent

8. Évolutions prévues (Version 2+)
introduction de Go pour les services I/O‑bound

multi‑datacenter réel

feature store ML complet

ingestion TAP/NRTRDE réelle

orchestration Kubernetes

dashboards avancés

API publiques

📌 Conclusion
Cette note de cadrage fixe un périmètre clair, réaliste et démontrable pour ORION Unified CDR Engine.
Elle garantit une première version cohérente, robuste et présentable à une équipe d’experts, tout en préparant les évolutions futures.