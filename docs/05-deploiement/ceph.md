# Déploiement Ceph pour ORION

Guide complet pour comprendre et déployer Ceph Object Storage (RGW) en production.

## Pourquoi Ceph ?

### Avantages pour ORION CDR Engine

1. **Scalabilité horizontale** : Ajouter des nœuds sans interruption
2. **Résilience** : Réplication automatique des données (3 copies par défaut)
3. **Haute disponibilité** : Pas de SPOF (Single Point Of Failure)
4. **S3-compatible** : RADOS Gateway (RGW) expose API S3
5. **Adoption entreprise** : Red Hat, OVH, Deutsche Telekom utilisent Ceph

### MinIO vs Ceph

| Critère | MinIO | Ceph |
|---------|-------|------|
| Déploiement | 1 container | Cluster multi-nœuds |
| RAM | ~200MB | ~2-4GB minimum |
| Cas d'usage | Dev/test | Production |
| Résilience | Limitée | Distribuée (CRUSH) |
| API | S3 natif | S3 via RGW |

**Notre stratégie** : MinIO en dev, Ceph en prod (même API S3).

## Architecture Ceph

### Composants essentiels

```
┌─────────────────────────────────────────────────────┐
│                 Client (ORION)                       │
│                                                      │
│  orion-storage-cold (AWS SDK S3)                    │
└───────────────────┬──────────────────────────────────┘
                    │ HTTP/S3 API
                    ▼
┌─────────────────────────────────────────────────────┐
│            RADOS Gateway (RGW)                       │
│                                                      │
│  - Endpoint S3/Swift                                │
│  - Gestion buckets                                  │
│  - ACL et authentification                          │
└───────────────────┬──────────────────────────────────┘
                    │ librados
                    ▼
┌─────────────────────────────────────────────────────┐
│              RADOS (Cluster)                         │
│                                                      │
│  ┌────────┐  ┌────────┐  ┌────────┐                │
│  │  MON   │  │  MON   │  │  MON   │  Monitors       │
│  │ (node1)│  │ (node2)│  │ (node3)│  (quorum)       │
│  └────────┘  └────────┘  └────────┘                │
│                                                      │
│  ┌────────┐  ┌────────┐  ┌────────┐                │
│  │  OSD   │  │  OSD   │  │  OSD   │  Object Storage │
│  │ (disk1)│  │ (disk2)│  │ (disk3)│  Daemons        │
│  └────────┘  └────────┘  └────────┘                │
│                                                      │
│  ┌────────┐                                         │
│  │  MGR   │  Manager (métriques, dashboard)         │
│  └────────┘                                         │
└─────────────────────────────────────────────────────┘
```

### Rôle des composants

#### 1. MON (Monitor)
- **Fonction** : Maintenir la carte du cluster (cluster map)
- **Quorum** : Minimum 3 nœuds (tolérance panne : 1 nœud)
- **Données** : CRUSH map, OSD map, PG map
- **Exemple** : `ceph mon dump`

#### 2. OSD (Object Storage Daemon)
- **Fonction** : Stocker les objets sur disques
- **1 OSD = 1 disque** (HDD ou SSD)
- **Réplication** : 3 copies par défaut (configurable)
- **Exemple** : 10 disques = 10 OSDs

#### 3. MGR (Manager)
- **Fonction** : Monitoring, métriques, dashboard web
- **Modules** : Prometheus exporter, REST API
- **Accès** : `http://<mgr-node>:7000` (dashboard Ceph)

#### 4. RGW (RADOS Gateway)
- **Fonction** : Traduire S3/Swift → RADOS
- **Endpoint** : `http://<rgw-node>:7480`
- **Utilisateurs** : Créer access_key/secret_key

## Algorithme CRUSH

### Concept clé

**CRUSH** = Controlled Replication Under Scalable Hashing

Au lieu de stocker une table centrale "objet → serveur", CRUSH **calcule** la position avec un algorithme déterministe.

### Exemple concret (ORION)

Fichier Parquet : `year=2024/month=01/day=15/country=US/cdr_123456.parquet`

1. **Hash de l'objet** : `hash("cdr_123456.parquet") → 0x8A3F...`
2. **Sélection du Placement Group (PG)** : `PG_ID = hash % nb_PGs`
3. **CRUSH calcule les OSDs** : PG 42 → [OSD.1, OSD.5, OSD.9] (3 copies)
4. **Écriture** : Client écrit directement sur OSD.1, OSD.5, OSD.9

**Avantages** :
- Pas de métadonnées centralisées
- Scalabilité linéaire (ajout OSD = recalcul minimal)
- Clients autonomes (pas de goulot d'étranglement)

### Pools et Placement Groups

#### Pool
- **Définition** : Namespace logique pour objets
- **Exemple** : `orion-cdr-archive` (pool pour CDRs)
- **Paramètres** : Règle réplication, taille PG

```bash
ceph osd pool create orion-cdr-archive 128  # 128 PGs
ceph osd pool set orion-cdr-archive size 3  # 3 réplicas
```

#### Placement Group (PG)
- **Définition** : Groupement d'objets pour distribution
- **Calcul optimal** : `(nb_OSDs * 100) / réplication`
- **Exemple** : 10 OSDs, réplication 3 → ~333 PGs (arrondir puissance 2 = 512)

```bash
ceph osd pool get orion-cdr-archive pg_num
```

## Configuration S3 pour ORION

### 1. Créer utilisateur RGW

```bash
radosgw-admin user create \
  --uid=orion-storage-cold \
  --display-name="ORION Cold Storage Service" \
  --access-key=AKIAIOSFODNN7EXAMPLE \
  --secret-key=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
```

Sortie :
```json
{
  "user_id": "orion-storage-cold",
  "display_name": "ORION Cold Storage Service",
  "keys": [{
    "user": "orion-storage-cold",
    "access_key": "AKIAIOSFODNN7EXAMPLE",
    "secret_key": "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"
  }]
}
```

### 2. Créer bucket

```bash
s3cmd mb s3://orion-cdr-archive --host=rgw-node:7480
```

Ou via `orion-storage-cold` (création automatique au démarrage).

### 3. Configuration ORION

`.env` ou variables d'environnement :
```bash
S3_ENDPOINT=http://rgw-node:7480
S3_REGION=default  # ou votre région RGW
S3_BUCKET=orion-cdr-archive
S3_ACCESS_KEY=AKIAIOSFODNN7EXAMPLE
S3_SECRET_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
S3_PATH_STYLE=true  # CRITIQUE pour Ceph RGW
```

**Note** : `S3_PATH_STYLE=true` force le format `http://endpoint/bucket/key` (style path) au lieu de `http://bucket.endpoint/key` (style virtual-hosted).

## Commandes essentielles

### Monitoring cluster

```bash
# Statut global
ceph -s

# Santé détaillée
ceph health detail

# Utilisation disques
ceph df

# Liste OSDs
ceph osd tree

# Performance I/O
ceph osd perf
```

### Gestion pools

```bash
# Lister pools
ceph osd lspools

# Statistiques pool
rados df

# Modifier réplication
ceph osd pool set orion-cdr-archive size 3
ceph osd pool set orion-cdr-archive min_size 2  # Minimum pour écriture
```

### RGW (S3)

```bash
# Lister utilisateurs
radosgw-admin user list

# Afficher clés utilisateur
radosgw-admin user info --uid=orion-storage-cold

# Statistiques buckets
radosgw-admin bucket stats --bucket=orion-cdr-archive

# Vérifier objets
radosgw-admin bucket list --bucket=orion-cdr-archive
```

## Déploiement

### Option 1 : Docker Compose (Dev/Test)

Voir [scripts/deploy-ceph-docker.sh](../../scripts/deploy-ceph-docker.sh)

Configuration minimale :
- 3 conteneurs MON
- 3 conteneurs OSD (1 volume par OSD)
- 1 MGR
- 1 RGW

### Option 2 : Production (RHEL/CentOS)

#### Installation Cephadm

```bash
# RHEL 8/9
dnf install -y cephadm

# Bootstrap cluster
cephadm bootstrap --mon-ip <IP_NODE1>

# Ajouter nœuds
ceph orch host add node2 <IP_NODE2>
ceph orch host add node3 <IP_NODE3>

# Déployer OSDs (disques /dev/sdb, /dev/sdc...)
ceph orch daemon add osd node1:/dev/sdb
ceph orch daemon add osd node2:/dev/sdc

# Déployer RGW
ceph orch apply rgw orion-rgw --placement="3"
```

#### Monitoring Prometheus

```bash
# Activer module Prometheus
ceph mgr module enable prometheus

# Endpoint métriques
curl http://mgr-node:9283/metrics
```

Intégration avec `config/prometheus.yml` d'ORION :
```yaml
scrape_configs:
  - job_name: 'ceph'
    static_configs:
      - targets: ['mgr-node:9283']
```

## Migration MinIO → Ceph

Script complet : [scripts/migrate-minio-to-ceph.sh](../../scripts/migrate-minio-to-ceph.sh)

### Étapes

1. **Backup MinIO** : `mc mirror minio/orion-cdr-archive /backup`
2. **Créer pool Ceph** : `ceph osd pool create orion-cdr-archive 128`
3. **Configurer RGW** : Créer utilisateur + bucket
4. **Sync données** : `s3cmd sync /backup s3://orion-cdr-archive --host=rgw`
5. **Tester ORION** : Modifier `S3_ENDPOINT`, vérifier lectures/écritures
6. **Décommissionner MinIO** : Une fois validé

## Troubleshooting

### Cluster en état HEALTH_WARN

```bash
ceph health detail
# Causes fréquentes :
# - PG dégradés (OSD down)
# - Clock skew (NTP non synchronisé)
# - Espace disque faible
```

### RGW 503 Service Unavailable

```bash
# Vérifier RGW actif
ceph orch ps | grep rgw

# Logs RGW
journalctl -u ceph-radosgw@rgw.<hostname> -f

# Tester endpoint
curl http://rgw-node:7480
```

### Performances lentes

```bash
# Vérifier latence OSDs
ceph osd perf

# Identifier OSDs lents
ceph tell osd.* bench

# Vérifier réseau (CRUSH utilise réseau intensivement)
iperf3 -s  # Sur un nœud
iperf3 -c <node-ip>  # Depuis autre nœud
```

## Ressources

- [Documentation officielle Ceph](https://docs.ceph.com/)
- [CRUSH paper](https://ceph.io/assets/pdfs/weil-crush-sc06.pdf) (algorithme détaillé)
- [Red Hat Ceph Storage](https://access.redhat.com/documentation/en-us/red_hat_ceph_storage/)
- [Ceph Dashboard](https://docs.ceph.com/en/latest/mgr/dashboard/)

## Prochaines étapes

1. ✅ Comprendre architecture Ceph (ce document)
2. ⏭️ Exécuter [scripts/deploy-ceph-docker.sh](../../scripts/deploy-ceph-docker.sh) (cluster local)
3. ⏭️ Tester [scripts/ceph-demo.sh](../../scripts/ceph-demo.sh) (commandes essentielles)
4. ⏭️ Préparer [scripts/migrate-minio-to-ceph.sh](../../scripts/migrate-minio-to-ceph.sh) (prod)
