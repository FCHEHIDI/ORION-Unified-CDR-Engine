🧠 5) ML — model.md
📁 docs/04-ml/model.md

🧠 Modèle ML — ORION Fraud Detection
1. Objectif
Détecter des comportements suspects en temps réel via un score 0.0–1.0.

2. Type de modèle
Pour la V1 :

modèle léger, rapide, embarquable en Rust

options possibles :

Random Forest → export ONNX

Gradient Boosting → export ONNX

petit réseau dense → ONNX

3. Format du modèle
ONNX recommandé

versionné : fraud-v1.0.0.onnx

chargé au démarrage du service ML

4. Pipeline d’inférence
réception du CDR normalisé

construction du vecteur de features

normalisation / encodage

passage dans le modèle

renvoi du score + version

5. Contraintes
latence < 10 ms

stateless

pas de stockage interne

fallback si modèle indisponible

6. Évolutions futures
feature store dédié

modèles séquentiels (LSTM/Transformer)

détection de dérive (drift)

entraînement continu