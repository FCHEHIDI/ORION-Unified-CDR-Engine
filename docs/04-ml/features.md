# 🔬 Features ML — ORION Fraud Detection

## 1. Objectif

Ce document décrit les **features** (caractéristiques) extraites des CDR pour alimenter le modèle ML de détection de fraude.

> ⚠️ **Note** : Ce document sera complété lors de la phase ML avancée (projet séparé). La version actuelle couvre les features de base pour le prototype.

## 2. Catégories de features

### 📍 2.1. Features de localisation

#### Mobilité anormale
| Feature | Description | Calcul |
|---------|-------------|--------|
| `distance_last_cell` | Distance depuis dernière cellule | Géolocalisation CellID |
| `cell_changes_1h` | Nb changements cellule/1h | Comptage rolling window |
| `country_changes_24h` | Nb changements pays/24h | Comptage rolling window |
| `impossible_travel` | Vitesse déplacement > seuil | Distance / temps |

**Indicateurs de fraude** :
- Sauts géographiques impossibles (Paris → Tokyo en 1h)
- Changements de cellule trop fréquents
- Roaming multi-pays simultané

---

### ⏰ 2.2. Features temporelles

#### Patterns d'usage inhabituel
| Feature | Description | Calcul |
|---------|-------------|--------|
| `is_night_usage` | Utilisation 00h-06h | Boolean |
| `weekend_ratio` | % usage weekend vs semaine | Ratio rolling 7 jours |
| `events_per_hour` | Nb événements/heure | Comptage |
| `burst_events` | Pic soudain d'activité | Détection anomalie |

**Indicateurs de fraude** :
- Activité massive nocturne inhabituelle
- Burst soudain (SIM clonée, box)
- Pattern régulier anormal (bot)

---

### 📞 2.3. Features d'usage

#### Volume et type d'événements
| Feature | Description | Calcul |
|---------|-------------|--------|
| `data_volume_24h` | Total bytes 24h | Somme rolling window |
| `voice_duration_24h` | Total minutes voix/24h | Somme rolling window |
| `sms_count_24h` | Nb SMS/24h | Comptage |
| `international_calls_ratio` | % appels internationaux | Ratio |
| `premium_sms_count` | Nb SMS surtaxés | Comptage (préfixes) |

**Indicateurs de fraude** :
- Volume data explosif (SIM volée)
- SMS premium massifs
- Appels internationaux anormaux
- Changement brutal de profil

---

### 🌍 2.4. Features roaming

#### Comportement en itinérance
| Feature | Description | Calcul |
|---------|-------------|--------|
| `is_roaming` | En roaming | Boolean |
| `roaming_country` | Pays visité | String → encoding |
| `roaming_duration_h` | Durée roaming/session | Différence temps |
| `roaming_partners_24h` | Nb partenaires différents/24h | Count distinct |
| `roaming_without_movement` | Roaming sans changement zone | Détection incohérence |

**Indicateurs de fraude** :
- Roaming sans mobilité (box pirate)
- Multi-partenaires simultanés
- Roaming permanent (fraude SIMbox)

---

### 👤 2.5. Features abonné

#### Profil et segment
| Feature | Description | Calcul |
|---------|-------------|--------|
| `subscriber_type` | Prépayé/postpayé | Categorial |
| `segment` | Segment client (VIP, pro, perso) | Categorial |
| `account_age_days` | Ancienneté compte | Date activation |
| `avg_monthly_spend` | Dépense moyenne mensuelle | Moyenne glissante |
| `is_new_subscriber` | < 30 jours | Boolean |

**Indicateurs de fraude** :
- Nouveaux abonnés avec usage anormal
- Changement brutal de comportement
- Comptes prépayés jetables

---

### 📊 2.6. Features historiques

#### Comparaison vs baseline personnel
| Feature | Description | Calcul |
|---------|-------------|--------|
| `usage_vs_baseline` | Usage actuel vs moyenne | Z-score |
| `location_vs_usual` | Localisation vs zones habituelles | Distance euclidienne |
| `time_vs_usual` | Horaire vs pattern habituel | Divergence KL |
| `imei_consistency` | Stabilité IMEI | Changements récents |

**Indicateurs de fraude** :
- Divergence forte vs comportement habituel
- Changement IMEI fréquent
- Usage atypique pour le profil

---

### 🔗 2.7. Features réseau

#### Qualité et cohérence technique
| Feature | Description | Calcul |
|---------|-------------|--------|
| `rat_changes_1h` | Changements 2G/3G/4G/5G | Comptage |
| `failed_calls_ratio` | % appels échoués | Ratio |
| `handover_rate` | Taux de handover | Comptage/durée |
| `imei_rat_mismatch` | IMEI incompatible avec RAT | Lookup base IMEI |

**Indicateurs de fraude** :
- Incohérences techniques (IMEI 2G sur 5G)
- Taux d'échec anormal
- Handovers impossibles

---

## 3. Feature Engineering

### 3.1. Normalisation
Toutes les features numériques sont normalisées (Z-score ou Min-Max) :

```python
z_score = (x - mean) / std
min_max = (x - min) / (max - min)
```

### 3.2. Encoding categorical
Features catégorielles encodées en one-hot ou label encoding :

```python
subscriber_type: [prepaid, postpaid, corporate] → [0, 1, 2]
country: [FR, TN, MA, SN, ...] → one-hot vectors
```

### 3.3. Time windows
Features agrégées sur plusieurs fenêtres :
- **1 heure** : détection rapide
- **24 heures** : pattern quotidien
- **7 jours** : tendance hebdomadaire
- **30 jours** : baseline long terme

---

## 4. Feature Store (V2)

En V2, ORION intégrera un feature store centralisé :

```
┌─────────────┐
│ CDR Stream  │
└──────┬──────┘
       ↓
┌──────────────────┐
│ Feature Pipeline │ (calcul temps réel)
└──────┬───────────┘
       ↓
┌──────────────────┐
│  Feature Store   │ (Redis/Scylla)
│  - Features 1h   │
│  - Features 24h  │
│  - Baseline 30d  │
└──────┬───────────┘
       ↓
┌──────────────────┐
│   ML Inference   │
└──────────────────┘
```

**Avantages** :
- Features pré-calculées
- Latence d'inférence réduite
- Réutilisation pour entraînement
- Versioning des features

---

## 5. Features V1 (prototype)

Pour le prototype ORION V1, on se limite à **10 features essentielles** :

| # | Feature | Type | Source |
|---|---------|------|--------|
| 1 | `is_roaming` | Boolean | CDR |
| 2 | `is_night_usage` | Boolean | event_time |
| 3 | `data_volume_mb` | Float | bytes_up + bytes_down |
| 4 | `duration_min` | Float | duration |
| 5 | `event_type` | Categorical | voice/sms/data |
| 6 | `country` | Categorical | country |
| 7 | `rat` | Categorical | 2G/3G/4G/5G |
| 8 | `subscriber_type` | Categorical | prepaid/postpaid |
| 9 | `hour_of_day` | Integer | 0-23 |
| 10 | `day_of_week` | Integer | 0-6 |

Ces features simples permettent déjà une première détection de fraude basique.

---

## 6. Feature Importance (à venir)

Après entraînement du modèle, on analysera l'importance des features :

```
Feature Importance (Random Forest exemple):
1. is_roaming              : 0.18
2. data_volume_mb          : 0.15
3. is_night_usage          : 0.12
4. country                 : 0.10
5. event_type              : 0.09
...
```

---

## 7. Évolution des features

### V1 (actuel)
- ✅ 10 features basiques
- ✅ Calcul en ligne simple
- ✅ Pas de feature store

### V2 (futur)
- 🔄 30-50 features avancées
- 🔄 Feature store (Redis/Scylla)
- 🔄 Features historiques (baseline)

### V3 (long terme)
- 🚀 Features séquentielles (LSTM)
- 🚀 Graph features (réseau social)
- 🚀 Deep features (embeddings)

---

## 8. Références

- **Feature engineering** : voir projet ML séparé (à venir)
- **Agent ML** : [fraud-agent.md](fraud-agent.md)
- **Modèle** : [model.md](model.md)

---

**ORION ML Features** — _Version 1.0 (prototype)_
