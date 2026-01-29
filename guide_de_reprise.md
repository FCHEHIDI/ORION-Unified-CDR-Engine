📘 Guidelines pour Claude Sonnet 4.5 — Reprise du code Rust ORION
🎯 Objectif
Tu vas prendre le relais sur la partie Rust du projet ORION, une plateforme télécom complète construite autour d’un pipeline CDR multi‑pays.
Ton rôle : implémenter, améliorer et étendre les microservices Rust, en respectant l’architecture existante et les conventions établies.

🧩 1. Présentation rapide d’ORION
ORION est une plateforme modulaire composée de :

un pipeline CDR complet (ingestion → validation → normalisation → enrichissement → stockage → API)

des microservices Rust indépendants

un orchestrateur PowerShell

un Makefile global

un chart Helm

des manifests Kubernetes

un générateur de trafic (traffic‑generator)

un système d’observabilité (Prometheus + Grafana)

L’ensemble est conçu pour être reproductible, industrialisé, et facile à démontrer.

🏗️ 2. Structure du workspace Rust
Le workspace Cargo ressemble à ceci :

Code
orion/
  Cargo.toml (workspace)
  orion-ingestion/
  orion-validation/
  orion-normalization/
  orion-enrichment/
  orion-ml-fraud-agent/
  orion-storage-hot/
  orion-storage-cold/
  orion-api/
  orion-observability/
  orion-traffic-generator/
Chaque microservice est un crate Rust indépendant, avec :

un main.rs

un serveur HTTP Axum

/health

/metrics

une logique métier propre

🧱 3. Conventions Rust à respecter
✔️ Frameworks
Serveur HTTP : Axum

Async runtime : Tokio

Kafka : rdkafka

Metrics : metrics + metrics-exporter-prometheus

JSON : serde / serde_json

✔️ Endpoints obligatoires
Chaque microservice doit exposer :

Code
/health   → retourne "OK"
/metrics  → exporter Prometheus
✔️ Style
code clair, modulaire, découpé en modules (mod)

pas de logique dans main.rs

erreurs gérées proprement (thiserror ou anyhow)

logs structurés (tracing)

pas de blocage (async partout)

✔️ Structure recommandée
Code
src/
  main.rs
  routes.rs
  metrics.rs
  service/
    mod.rs
    logic.rs
    model.rs
🔌 4. Intégration avec l’orchestration
Le projet est piloté par :

un orchestrateur PowerShell (orion_orchestrator.ps1)

un Makefile

un CLI ORION (orion.ps1)

des scripts de génération automatique

Kubernetes + Helm

Tu n’as pas besoin de modifier ces scripts, mais ton code Rust doit rester compatible :

ports exposés : 9100 (sauf exceptions)

/metrics doit être compatible Prometheus

/health doit être instantané

pas de dépendance système exotique

🚦 5. Microservice à traiter en priorité
Le premier microservice à implémenter ou améliorer est :

👉 orion-ingestion
Rôle :

consommer Kafka

valider le format brut

publier vers le topic suivant

Claude devra :

créer la structure du service

implémenter un consumer Kafka asynchrone

exposer /health et /metrics

structurer le code proprement

préparer les modules pour la suite du pipeline

🧪 6. Tests
Les tests unitaires doivent :

être dans tests/ ou src/.../tests.rs

couvrir la logique métier

mocker Kafka si nécessaire

être reproductibles

🧭 7. Ce que Claude doit éviter
modifier l’orchestration PowerShell

changer les ports des services

casser la compatibilité Kubernetes

introduire des dépendances non cross‑platform

écrire du code Rust monolithique dans main.rs

🚀 8. Ce que Claude peut faire librement
améliorer la qualité du code Rust

proposer des abstractions plus propres

ajouter des tests

optimiser les performances

enrichir le traffic‑generator

proposer des patterns de fraude / roaming

améliorer la structure des crates

🧠 9. Résumé pour Claude
Tu arrives sur un projet :

bien structuré

entièrement orchestré

avec une architecture claire

où ton rôle est de donner vie aux microservices Rust

Tu peux avancer service par service, en respectant :

Axum

Tokio

rdkafka

metrics

conventions du workspace

Ton premier objectif : implémenter orion-ingestion proprement.

🎁 10. Phrase d’introduction que tu peux lui donner