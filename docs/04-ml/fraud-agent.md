🧠 ORION ML Fraud Agent — Architecture d’inférence
(à mettre dans docs/04-ml/fraud-agent.md)

1. Rôle de l’agent
Recevoir un événement CDR normalisé depuis orion-enrichment

Calculer un score de fraude en temps (quasi) réel

Retourner :

fraud_score (float)

model_version (string)

Ne pas stocker de données métier (stateless en V1)

2. Position dans le pipeline
Flux :

text
orion-normalization
        ↓
  orion-enrichment
        ↓ (gRPC)
  orion-ml-fraud-agent
        ↓
  orion-enrichment (enrichi)
        ↓
  orion-storage-hot / cold
3. Interface gRPC
Request :

json
{
  "imsi": "208150123456789",
  "msisdn": "+33612345678",
  "event_time": "2026-01-29T10:15:00Z",
  "country": "FR",
  "event_type": "data",
  "bytes_up": 123456,
  "bytes_down": 987654,
  "cell_id": "FR1234",
  "roaming_partner": null,
  "is_roaming": false
}
Response :

json
{
  "fraud_score": 0.87,
  "model_version": "fraud-v1.0.0"
}
4. Composants internes (Rust)
Loader de modèle

charge un modèle ONNX ou un modèle Rust embarqué

versionné (model_version)

Feature builder

transforme le CDR unifié en vecteur de features

applique normalisation / encodage

Inference engine

exécute le modèle

retourne un score 0.0–1.0

Adapter gRPC

expose l’API

gère les timeouts / erreurs

5. Contraintes de performance
Latence d’inférence cible : < 10 ms

QPS cible : dimensionné pour suivre le pipeline (pas de backlog)

Timeouts côté orion-enrichment + fallback (score neutre si ML down)

6. Sécurité
gRPC sur TLS

Auth interne (token court)

Logs sans données sensibles (IMSI/MSISDN masqués)

Audit des versions de modèle utilisées

7. Projet ML séparé (future session)
Ce document ne couvre que :

l’inférence,

l’intégration dans ORION.

Le projet d’entraînement du modèle sera :

un repo / module séparé,

avec :

préparation des datasets,

feature engineering,

entraînement,

évaluation,

export ONNX / format Rust.

On le traitera comme un projet frère d’ORION, branché sur les CDR stockés (Scylla / Ceph).