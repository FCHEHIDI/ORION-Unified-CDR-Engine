🎬 Scénario de démonstration ORION
1. Objectif
Montrer le pipeline complet, du CDR brut au stockage hot/cold.

2. Étapes
Étape 1 — Génération des CDR bruts
100k CDR FR

100k CDR TN

100k CDR MA

100k CDR SN

Étape 2 — Ingestion Kafka
push des fichiers dans cdr.raw.countryX

Étape 3 — Pipeline Rust
parsing

validation

normalisation

enrichment

ML scoring

Étape 4 — Stockage
ScyllaDB (hot)

MinIO (cold)

Étape 5 — Visualisation
Grafana : ingestion rate, ML latency

API Rust : /cdr/imsi/{id}

Étape 6 — Analyse
requêtes Scylla

fichiers Parquet dans MinIO

3. Résultat attendu
pipeline stable

latence maîtrisée

ML fonctionnel

stockage hot/cold opérationnel

démonstration fluide