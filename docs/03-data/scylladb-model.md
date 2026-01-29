🗄️ Modèle ScyllaDB — ORION Unified CDR Engine
(contenu à mettre dans docs/03-data/scylladb-model.md)

1. Objectifs du modèle ScyllaDB
Le modèle ScyllaDB doit :

supporter une ingestion massive (≥ 1M CDR/min/pays),

garantir des lectures rapides pour billing, analytics, QoS, fraude,

éviter les partitions trop grosses,

permettre un scale horizontal naturel,

rester simple et prévisible.

Il repose sur trois principes :

1 pattern d’accès = 1 table

Partitionnement temporel systématique

Tables dénormalisées, optimisées pour la lecture

2. Fenêtres temporelles (time windows)
Les CDR sont naturellement temporels.
On utilise donc des fenêtres pour limiter la taille des partitions :

Usage	Fenêtre	Justification
Billing / customer care	Jour	Volume stable, requêtes par abonné
Radio / QoS	Heure	Très granulaire, faible volume par cellule
Reporting groupe	Jour	Agrégation multi‑pays
Analytics	Jour	Requêtes massives mais partitionnées
Roaming	Jour	Facturation inter‑opérateurs
3. Tables principales
3.1. Table : cdr_by_imsi_day
(billing, customer care, anti‑fraude)

Partition key : (imsi, event_date)  
Clustering key : event_time, charging_id

sql
CREATE TABLE cdr_by_imsi_day (
    imsi text,
    event_date date,
    event_time timestamp,
    charging_id text,
    msisdn text,
    imei text,
    event_type text,
    duration int,
    bytes_up bigint,
    bytes_down bigint,
    cell_id text,
    country text,
    roaming_partner text,
    fraud_score float,
    model_version text,
    PRIMARY KEY ((imsi, event_date), event_time, charging_id)
) WITH CLUSTERING ORDER BY (event_time ASC);
Pourquoi ce design ?

Requêtes par abonné → rapides

Partition journalière → taille maîtrisée

Clustering par temps → tri naturel

3.2. Table : cdr_by_cell_hour
(radio, QoS, optimisation réseau)

Partition key : (cell_id, event_hour)  
Clustering key : event_time, charging_id

sql
CREATE TABLE cdr_by_cell_hour (
    cell_id text,
    event_hour timestamp,
    event_time timestamp,
    charging_id text,
    imsi text,
    msisdn text,
    rat text,
    bytes_up bigint,
    bytes_down bigint,
    country text,
    PRIMARY KEY ((cell_id, event_hour), event_time, charging_id)
) WITH CLUSTERING ORDER BY (event_time ASC);
Pourquoi ?

Une cellule génère peu de CDR par heure

Partition très petite → compaction parfaite

Idéal pour dashboards temps réel

3.3. Table : cdr_by_country_day
(reporting groupe, analytics multi‑pays)

Partition key : (country, event_date)  
Clustering key : event_time, charging_id

sql
CREATE TABLE cdr_by_country_day (
    country text,
    event_date date,
    event_time timestamp,
    charging_id text,
    imsi text,
    msisdn text,
    event_type text,
    bytes_up bigint,
    bytes_down bigint,
    PRIMARY KEY ((country, event_date), event_time, charging_id)
);
3.4. Table : cdr_by_partner_day
(roaming, interconnect billing)

Partition key : (roaming_partner, event_date)  
Clustering key : event_time, charging_id

sql
CREATE TABLE cdr_by_partner_day (
    roaming_partner text,
    event_date date,
    event_time timestamp,
    charging_id text,
    imsi text,
    msisdn text,
    roaming_country text,
    bytes_up bigint,
    bytes_down bigint,
    PRIMARY KEY ((roaming_partner, event_date), event_time, charging_id)
);
3.5. Table : cdr_by_event_type_day
(analytics, ML, reporting usage)

Partition key : (event_type, event_date)  
Clustering key : event_time, charging_id

sql
CREATE TABLE cdr_by_event_type_day (
    event_type text,
    event_date date,
    event_time timestamp,
    charging_id text,
    imsi text,
    msisdn text,
    bytes_up bigint,
    bytes_down bigint,
    country text,
    PRIMARY KEY ((event_type, event_date), event_time, charging_id)
);
4. Stratégies de compaction
TWCS (Time Window Compaction Strategy)
→ pour les tables partitionnées par jour/heure
→ idéal pour données append‑only

Configuration typique :

sql
WITH compaction = {
  'class': 'TimeWindowCompactionStrategy',
  'compaction_window_unit': 'DAYS',
  'compaction_window_size': '1'
}
Pour les tables horaires :

sql
'compaction_window_unit': 'HOURS',
'compaction_window_size': '1'
5. TTL et lifecycle
Hot storage (ScyllaDB)
TTL recommandé : 7 à 30 jours

Objectif : requêtes rapides, faible volume

Cold storage (Ceph)
Rétention : 6 à 24 mois

Format : Parquet/ORC compressé

6. Bonnes pratiques ScyllaDB
partitions < 50 Mo

éviter les collections (list, map, set)

éviter les updates fréquents

utiliser des clés naturelles (IMSI, CellID…)

éviter les partitions “hot” (IMSI très actif → sharding par jour)

7. Extensions prévues (V2+)
tables pour TAP/NRTRDE

tables pour QoS avancée (RSRP, SINR)

tables pour 5G SA (slice ID, gNB)

tables pour ML feature store