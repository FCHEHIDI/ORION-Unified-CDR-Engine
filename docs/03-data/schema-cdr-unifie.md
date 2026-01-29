🧬 Schéma CDR Unifié Groupe — ORION Unified CDR Engine
(contenu à mettre dans docs/03-data/schema-cdr-unifie.md)

1. Objectif du schéma unifié
Le schéma CDR unifié groupe permet :

d’harmoniser les formats multi‑pays, multi‑réseaux, multi‑technologies,

de simplifier le pipeline Rust (normalisation → enrichment → storage),

de garantir la cohérence des données dans ScyllaDB et Ceph,

de fournir un modèle stable pour billing, analytics et ML.

Il est conçu pour être :

minimal mais complet,

compatible multi‑pays,

optimisé pour ScyllaDB,

adapté au ML anti‑fraude,

facile à étendre.

2. Structure générale du CDR unifié
Le CDR unifié est structuré en 6 blocs logiques :

Identité abonné / équipement

Contexte réseau

Usage / consommation

Temporalité

Roaming / pays

Enrichissements (segment, ML, etc.)

3. Schéma complet (V1)
Voici la version V1 stable, adaptée au prototype et déjà prête pour ScyllaDB.

3.1. Identité abonné / équipement
Champ	Type	Description
imsi	string	Identifiant SIM
msisdn	string	Numéro abonné
imei	string	Identifiant terminal
subscriber_type	string	prépayé / postpayé / corporate
3.2. Contexte réseau
Champ	Type	Description
rat	string	2G / 3G / 4G / 5G
cell_id	string	Cellule radio
lac_tac	string	LAC (2G/3G) ou TAC (4G/5G)
mcc	string	Mobile Country Code
mnc	string	Mobile Network Code
3.3. Usage / consommation
Champ	Type	Description
event_type	string	voice / sms / data / roaming
duration	int	durée (voix)
bytes_up	bigint	upload
bytes_down	bigint	download
charging_id	string	identifiant de session
3.4. Temporalité
Champ	Type	Description
event_time	timestamp	horodatage réel
event_date	date	partition journalière
event_hour	timestamp	partition horaire
timezone	string	fuseau du pays source
3.5. Roaming / pays
Champ	Type	Description
country	string	pays d’origine du CDR
roaming_partner	string	opérateur visité
roaming_country	string	pays visité
is_roaming	bool	indicateur roaming
3.6. Enrichissements
Champ	Type	Description
segment	string	segment client (pro, perso…)
plan	string	plan tarifaire
geo_approx	string	localisation approximative
fraud_score	float	score ML
model_version	string	version du modèle ML
4. Exemple JSON (V1)
json
{
  "imsi": "208150123456789",
  "msisdn": "+33612345678",
  "imei": "356789012345678",
  "subscriber_type": "postpaid",

  "rat": "4G",
  "cell_id": "FR1234",
  "lac_tac": "45678",
  "mcc": "208",
  "mnc": "15",

  "event_type": "data",
  "duration": 0,
  "bytes_up": 123456,
  "bytes_down": 987654,
  "charging_id": "CHG123456",

  "event_time": "2026-01-29T10:15:00Z",
  "event_date": "2026-01-29",
  "event_hour": "2026-01-29T10:00:00Z",
  "timezone": "Europe/Paris",

  "country": "FR",
  "roaming_partner": null,
  "roaming_country": null,
  "is_roaming": false,

  "segment": "premium",
  "plan": "5G-100GB",
  "geo_approx": "Paris-Est",
  "fraud_score": 0.12,
  "model_version": "1.0.0"
}
5. Compatibilité ScyllaDB
Le schéma est conçu pour :

partitionner par (imsi, event_date)

partitionner par (cell_id, event_hour)

partitionner par (country, event_date)

partitionner par (event_type, event_date)

Il est strictement normalisé pour éviter :

les types complexes,

les collections lourdes,

les structures imbriquées.

6. Extensions prévues (V2+)
enrichissements géographiques avancés (lat/lon),

QoS détaillée (RSRP, SINR, throughput),

enrichissements ML supplémentaires,

support TAP/NRTRDE natif,

champs spécifiques 5G SA (gNB, slice ID).