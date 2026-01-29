📘 Datasets — ORION Unified CDR Engine
1. Objectif du document
Ce document décrit :

les datasets utilisés pour les tests, la démonstration et l’entraînement ML,

leur structure,

leur volumétrie,

leur mode de génération,

leur stockage (local, Scylla, Ceph/MinIO),

les règles de qualité et de cohérence.

Il sert de référence pour la simulation multi‑pays et la reproductibilité des tests.

2. Types de datasets
ORION utilise trois catégories de datasets :

2.1. Datasets CDR bruts (raw)
Formats hétérogènes simulant les pays du groupe :

CSV

JSON

formats legacy (TAP-like simplifié)

fichiers batch SFTP

2.2. Datasets CDR normalisés
Conformes au schéma unifié ORION.

2.3. Datasets ML
Features dérivées + labels (fraude / non fraude) pour entraînement futur.

3. Datasets CDR bruts (raw)
3.1. Structure
Chaque pays possède un format différent :

Pays	Format	Particularités
FR	CSV	colonnes ordonnées, séparateur ;
TN	JSON	structure imbriquée
MA	CSV	colonnes manquantes possibles
SN	Legacy	format texte fixe
3.2. Exemple FR (CSV)
Code
imsi;msisdn;imei;cell_id;rat;bytes_up;bytes_down;event_time
208150123456789;+33612345678;356789012345678;FR1234;4G;123456;987654;2026-01-29T10:15:00Z
3.3. Exemple TN (JSON)
json
{
  "subscriber": {
    "imsi": "605010987654321",
    "msisdn": "+21650123456"
  },
  "network": {
    "cell": "TN5678",
    "rat": "4G"
  },
  "usage": {
    "up": 45678,
    "down": 123456
  },
  "timestamp": "2026-01-29T11:00:00Z"
}
4. Datasets normalisés
Après passage dans :

ingestion

validation

normalisation

enrichment

ML

… les CDR deviennent conformes au schéma unifié ORION.

4.1. Format final (JSON)
json
{
  "imsi": "208150123456789",
  "msisdn": "+33612345678",
  "event_type": "data",
  "bytes_up": 123456,
  "bytes_down": 987654,
  "event_time": "2026-01-29T10:15:00Z",
  "event_date": "2026-01-29",
  "event_hour": "2026-01-29T10:00:00Z",
  "country": "FR",
  "fraud_score": 0.12,
  "model_version": "1.0.0"
}
5. Datasets ML
5.1. Structure
Feature	Description
bytes_up	volume montant
bytes_down	volume descendant
rat	technologie
cell_id	cellule
is_roaming	booléen
hour	heure de la journée
delta_prev	temps depuis dernier événement
label	fraude / non fraude
5.2. Format (CSV)
Code
bytes_up,bytes_down,rat,is_roaming,hour,delta_prev,label
123456,987654,4G,0,10,120,0
456789,123456,4G,1,02,30,1
6. Volumétrie recommandée
Pour la démo :

100k CDR par pays

4 pays

total : 400k CDR

Pour tests de charge :

1M CDR

ingestion Kafka en rafale

Pour ML :

50k à 200k lignes

ratio fraude : 1–5 %

7. Stockage
Local
Code
datasets/
  raw/
    fr/
    tn/
    ma/
    sn/
  normalized/
  ml/
Ceph / MinIO
Code
/country=FR/year=2026/month=01/day=29/type=data/file.parquet
8. Génération automatique
Un script Rust ou Python peut générer :

CDR bruts multi‑pays,

anomalies réseau,

comportements frauduleux simulés.

9. Qualité & cohérence
timestamps cohérents

IMSI/MSISDN valides

RAT réalistes

volumes plausibles

distribution temporelle naturelle