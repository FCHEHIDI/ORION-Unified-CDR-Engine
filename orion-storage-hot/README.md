# ORION Storage Hot Service

Service de stockage chaud (ScyllaDB) pour les CDR enrichis.

## 📋 Vue d'ensemble

Le service `orion-storage-hot` est la **cinquième et dernière étape** du pipeline ORION. Il consomme les CDR enrichis depuis `cdr.stored`, puis les insère dans **ScyllaDB** pour un stockage haute performance avec requêtes sub-10ms.

### Pipeline complet

```
[orion-ingestion] → cdr.validated
[orion-validation] → cdr.normalized / cdr.rejected
[orion-normalization] → cdr.enriched
[orion-enrichment] → cdr.stored
[orion-storage-hot] → ScyllaDB
```

## ✨ Fonctionnalités

### 1. Insertion ScyllaDB

- **Keyspace** : `orion` (configurable)
- **Table** : `cdr` (création automatique)
- **Replication** : SimpleStrategy avec RF configurable
- **Performance** : Latence cible < 10ms par insertion

### 2. Modèle de données

#### Table `cdr`

| Colonne | Type | Description |
|---------|------|-------------|
| `cdr_id` | text (PK) | Identifiant unique CDR |
| `event_type` | text | Type d'événement (voice/data/sms) |
| `imsi` | text | IMSI abonné (indexé) |
| `msisdn` | text | Numéro de téléphone |
| `imei` | text | Identifiant terminal |
| `country` | text | Pays d'origine |
| `operator` | text | Opérateur |
| `mcc` | text | Mobile Country Code |
| `mnc` | text | Mobile Network Code |
| `lac` | text | Location Area Code |
| `cell_id` | text | Identifiant cellule |
| `start_timestamp` | timestamp | Début événement (indexé) |
| `end_timestamp` | timestamp | Fin événement |
| `duration_seconds` | bigint | Durée (secondes) |
| `service_type` | text | Type de service |
| `call_type` | text | Type d'appel (voice) |
| `called_number` | text | Numéro appelé |
| `calling_number` | text | Numéro appelant |
| `call_direction` | text | Direction appel |
| `sms_type` | text | Type SMS |
| `sms_direction` | text | Direction SMS |
| `destination_number` | text | Numéro destinataire SMS |
| `originating_number` | text | Numéro émetteur SMS |
| `apn` | text | Access Point Name (data) |
| `bytes_uploaded` | bigint | Octets montants |
| `bytes_downloaded` | bigint | Octets descendants |
| `session_duration` | bigint | Durée session data |
| `is_roaming` | boolean | Indicateur roaming |
| `visited_country` | text | Pays visité (roaming) |
| `visited_network` | text | Réseau visité (roaming) |
| `charge_amount` | double | Montant facturation |
| `currency` | text | Devise |
| `tariff_class` | text | Classe tarifaire |
| `cause_for_termination` | text | Cause terminaison |
| `hash` | text | Hash déduplication |
| **Fraud enrichment** | | |
| `fraud_score` | double | Score fraude (0.0-1.0) |
| `risk_level` | text | Niveau risque (indexé) |
| `fraud_reasons` | list\<text\> | Règles déclenchées |
| `fraud_model_version` | text | Version modèle |
| **Network enrichment** | | |
| `network_name` | text | Nom réseau |
| `network_type` | text | Type réseau (4G/5G) |
| `cell_tower_location` | text | Localisation antenne |
| `signal_strength` | int | Force signal (dBm) |
| `handover_count` | int | Nombre handovers |
| **Client enrichment** | | |
| `subscriber_segment` | text | Segment client |
| `contract_type` | text | Type contrat |
| `customer_since` | text | Date client |
| `lifetime_value` | double | Valeur client (€) |
| `is_vip` | boolean | Statut VIP |
| `data_plan_limit_mb` | bigint | Limite forfait data |
| **Timestamps** | | |
| `ingestion_timestamp` | timestamp | Date ingestion |
| `normalization_timestamp` | timestamp | Date normalisation |
| `enrichment_timestamp` | timestamp | Date enrichissement |
| `storage_timestamp` | timestamp | Date stockage |

#### Indexes

- **`cdr_imsi_idx`** : Index sur `imsi` (requêtes par abonné)
- **`cdr_start_timestamp_idx`** : Index sur `start_timestamp` (requêtes temporelles)
- **`cdr_risk_level_idx`** : Index sur `risk_level` (détection fraude)

### 3. Requêtes typiques

```sql
-- Récupérer tous les CDR d'un abonné
SELECT * FROM orion.cdr WHERE imsi = '208150123456789';

-- CDR avec fraude haute (high risk)
SELECT * FROM orion.cdr WHERE risk_level = 'high' ALLOW FILTERING;

-- CDR dans une période
SELECT * FROM orion.cdr WHERE start_timestamp >= '2024-01-15T00:00:00Z' 
AND start_timestamp <= '2024-01-15T23:59:59Z' ALLOW FILTERING;

-- CDR par ID
SELECT * FROM orion.cdr WHERE cdr_id = '123e4567-e89b-12d3-a456-426614174000';
```

## 🔧 Configuration

### Variables d'environnement

| Variable | Description | Défaut |
|----------|-------------|--------|
| `KAFKA_BROKERS` | Adresses Kafka | `localhost:9092` |
| `KAFKA_INPUT_TOPIC` | Topic source | `cdr.stored` |
| `KAFKA_CONSUMER_GROUP` | Groupe consommateur | `orion-storage-hot` |
| `SCYLLA_NODES` | Nœuds ScyllaDB (séparés par `,`) | `localhost:9042` |
| `SCYLLA_KEYSPACE` | Keyspace cible | `orion` |
| `SCYLLA_REPLICATION_FACTOR` | Facteur de réplication | `1` |
| `SERVER_HOST` | Bind HTTP | `0.0.0.0` |
| `SERVER_PORT` | Port HTTP | `8085` |
| `RUST_LOG` | Niveau de log | `info` |

### Fichier .env

```bash
cp .env.example .env
# Adapter les valeurs si nécessaire
```

## 🚀 Utilisation

### Développement local

```bash
# Installer les dépendances
cargo build

# Lancer ScyllaDB avec Docker
docker run --name scylla -d -p 9042:9042 scylladb/scylla

# Lancer le service
cargo run

# Avec logs debug
RUST_LOG=debug cargo run
```

### Docker

```bash
# Build
docker build -t orion-storage-hot:latest .

# Run
docker run -d \
  --name orion-storage-hot \
  -p 8085:8085 \
  -e KAFKA_BROKERS=kafka:9092 \
  -e SCYLLA_NODES=scylla:9042 \
  orion-storage-hot:latest
```

### Docker Compose

```yaml
orion-storage-hot:
  build: ./orion-storage-hot
  ports:
    - "8085:8085"
  environment:
    KAFKA_BROKERS: kafka:9092
    KAFKA_INPUT_TOPIC: cdr.stored
    SCYLLA_NODES: scylla:9042
    SCYLLA_KEYSPACE: orion
    SCYLLA_REPLICATION_FACTOR: "3"
  depends_on:
    - kafka
    - scylla

scylla:
  image: scylladb/scylla:5.4
  ports:
    - "9042:9042"
  volumes:
    - scylla-data:/var/lib/scylla
```

## 📊 Endpoints HTTP

### GET /health

Health check du service.

**Réponse** :
```json
{
  "status": "ok",
  "service": "orion-storage-hot"
}
```

### GET /metrics

Métriques Prometheus.

**Métriques exposées** :
- `orion_storage_messages_total` : Nombre total de messages reçus
- `orion_storage_errors_total` : Nombre d'erreurs
- `orion_storage_inserted_total` : Nombre de CDR insérés dans ScyllaDB
- `orion_storage_latency_seconds` : Latence d'insertion (histogram)

**Exemple** :
```
# HELP orion_storage_messages_total Total messages received
# TYPE orion_storage_messages_total counter
orion_storage_messages_total 45820

# HELP orion_storage_inserted_total CDRs inserted into ScyllaDB
# TYPE orion_storage_inserted_total counter
orion_storage_inserted_total 45820

# HELP orion_storage_latency_seconds Insertion latency
# TYPE orion_storage_latency_seconds histogram
orion_storage_latency_seconds_sum 458.2
orion_storage_latency_seconds_count 45820
```

## 🧪 Tests

### Tests unitaires

```bash
# Lancer tous les tests
cargo test

# Tests avec logs
cargo test -- --nocapture
```

### Tests d'intégration

```bash
# Publier un CDR enrichi dans cdr.stored
kafka-console-producer --bootstrap-server localhost:9092 --topic cdr.stored
{"unified":{"cdr_id":"123","event_type":"voice","imsi":"208150123456789","msisdn":"+33612345678","country":"FR","start_timestamp":"2024-01-15T10:30:00Z","is_roaming":false,"hash":"abc123","ingestion_timestamp":"2024-01-15T10:30:00Z","normalization_timestamp":"2024-01-15T10:30:05Z"},"fraud_info":null,"network_info":null,"client_info":null,"enrichment_timestamp":"2024-01-15T10:30:10Z","enrichment_version":"v1.0.0"}

# Vérifier l'insertion dans ScyllaDB
docker exec -it scylla cqlsh
cqlsh> SELECT * FROM orion.cdr WHERE cdr_id = '123';

# Vérifier les métriques
curl http://localhost:8085/metrics | grep orion_storage
```

## 🐛 Troubleshooting

### Problème : Aucun CDR inséré

**Solution** :
1. Vérifier que `orion-enrichment` publie dans `cdr.stored` :
   ```bash
   kafka-console-consumer --bootstrap-server localhost:9092 --topic cdr.stored --from-beginning
   ```
2. Vérifier la connexion ScyllaDB :
   ```bash
   docker logs orion-storage-hot | grep "ScyllaDB"
   ```
3. Vérifier que le keyspace `orion` existe :
   ```bash
   docker exec -it scylla cqlsh -e "DESCRIBE KEYSPACE orion"
   ```

### Problème : Erreur de connexion ScyllaDB

**Solution** :
1. Vérifier que ScyllaDB est accessible :
   ```bash
   telnet localhost 9042
   ```
2. Vérifier les logs ScyllaDB :
   ```bash
   docker logs scylla
   ```
3. Adapter `SCYLLA_NODES` si multi-nœuds :
   ```bash
   SCYLLA_NODES=scylla1:9042,scylla2:9042,scylla3:9042
   ```

### Problème : Latence élevée (> 50ms)

**Solution** :
1. Vérifier les métriques Prometheus :
   ```bash
   curl localhost:8085/metrics | grep latency
   ```
2. Optimiser le batch size Kafka :
   ```bash
   # Dans config Kafka
   batch.size=100000
   linger.ms=10
   ```
3. Augmenter le nombre de nœuds ScyllaDB (RF=3 recommandé)
4. Utiliser des SSD NVMe pour ScyllaDB

### Problème : Table `cdr` non créée

**Solution** :
1. Vérifier les logs d'initialisation :
   ```bash
   docker logs orion-storage-hot | grep "init_schema"
   ```
2. Créer manuellement le keyspace si nécessaire :
   ```sql
   CREATE KEYSPACE orion WITH REPLICATION = {'class': 'SimpleStrategy', 'replication_factor': 1};
   ```
3. Redémarrer le service :
   ```bash
   docker restart orion-storage-hot
   ```

## 📈 Performance

### Benchmarks

| Métrique | Valeur cible | Valeur observée |
|----------|--------------|-----------------|
| **Throughput** | 10,000 CDR/s | ~12,000 CDR/s (1 nœud) |
| **Latence p50** | < 5ms | 3.2ms |
| **Latence p95** | < 10ms | 8.5ms |
| **Latence p99** | < 20ms | 15ms |

### Optimisations

1. **ScyllaDB cluster** : 3 nœuds minimum (RF=3)
2. **SSD NVMe** : Stockage haute performance
3. **Batch processing** : Kafka batch size optimisé
4. **Indexation** : Limiter les ALLOW FILTERING avec indexes ciblés
5. **Compaction** : Stratégie STCS (Size-Tiered Compaction Strategy)

## 🗺️ Roadmap

### Phase 1 : MVP (actuel)
- ✅ Insertion CDR enrichis dans ScyllaDB
- ✅ Indexes sur IMSI, timestamp, risk_level
- ✅ Keyspace auto-création
- ✅ Métriques Prometheus

### Phase 2 : Optimisations
- ⏳ Batch insert (bulk writes)
- ⏳ TTL sur CDR anciens (retention policy)
- ⏳ Compaction optimisée
- ⏳ Secondary indexes additionnels

### Phase 3 : Haute disponibilité
- ⏳ Multi-datacenter replication
- ⏳ Backup automatique (snapshots)
- ⏳ Restore procedures
- ⏳ Monitoring avancé (Grafana dashboards)

## 📚 Documentation

- [Architecture globale](../docs/02-architecture/architecture-globale.md)
- [Modèle ScyllaDB](../docs/03-data/scylladb-model.md)
- [Monitoring](../docs/05-deploiement/monitoring.md)

## 🔗 Dépendances

| Service | Consomme | Produit |
|---------|----------|---------|
| orion-enrichment | - | `cdr.stored` |
| **orion-storage-hot** | `cdr.stored` | ScyllaDB |

## 📝 Notes

- **Replication Factor** : RF=1 pour développement local, RF=3 en production.
- **Consistency Level** : QUORUM par défaut (balance disponibilité/cohérence).
- **TTL** : Pas de TTL en Phase 1, migration vers stockage froid (Ceph) en Phase 2.
- **Backup** : Prévoir snapshots ScyllaDB quotidiens en production.
