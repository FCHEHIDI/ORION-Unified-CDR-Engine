🖥️ Déploiement RHEL — ORION Unified CDR Engine
(contenu à mettre dans docs/05-deploiement/rhel.md)

1. Objectif du document
Ce document décrit :

les prérequis système pour exécuter ORION sur RHEL,

la configuration des services systemd,

les bonnes pratiques de durcissement,

la gestion des utilisateurs, permissions et logs,

la structure des répertoires,

les dépendances nécessaires (Kafka, Scylla, Ceph).

Il sert de référence pour un déploiement entreprise.

2. Version RHEL recommandée
RHEL 8.x ou RHEL 9.x

Kernel optimisé pour I/O et réseau

SELinux enforcing (pas permissive)

3. Utilisateurs et permissions
Chaque microservice ORION doit tourner sous un utilisateur dédié :

Code
useradd -r -s /sbin/nologin orion
useradd -r -s /sbin/nologin orion_ingestion
useradd -r -s /sbin/nologin orion_validation
useradd -r -s /sbin/nologin orion_normalization
useradd -r -s /sbin/nologin orion_enrichment
useradd -r -s /sbin/nologin orion_fraud
useradd -r -s /sbin/nologin orion_storage
useradd -r -s /sbin/nologin orion_api
Pourquoi ?  
Isolation, audit, sécurité.

4. Structure des répertoires
Code
/opt/orion/
    bin/                # exécutables Rust
    config/             # fichiers TOML/YAML
    logs/               # logs JSON
    data/               # buffers temporaires
    certs/              # certificats TLS
    systemd/            # unités systemd
5. Services systemd
Chaque microservice Rust est un service systemd.

Exemple : orion-ingestion.service

Code
[Unit]
Description=ORION Ingestion Service
After=network.target

[Service]
User=orion_ingestion
ExecStart=/opt/orion/bin/orion-ingestion --config /opt/orion/config/ingestion.toml
Restart=always
RestartSec=3
LimitNOFILE=100000
Environment="RUST_LOG=info"

[Install]
WantedBy=multi-user.target
Points clés :

Restart=always

LimitNOFILE élevé (Kafka)

logs via journald + export vers Loki/ELK

6. Configuration réseau
Ports internes (exemples)
Service	Port	Protocole
gRPC ML Agent	50051	TLS
API interne	8080	TLS
Kafka brokers	9092	TLS
ScyllaDB	9042	TLS
Ceph RGW	7480	TLS
Firewall (firewalld)
Code
firewall-cmd --permanent --add-port=8080/tcp
firewall-cmd --permanent --add-port=50051/tcp
firewall-cmd --reload
7. TLS et certificats
Certificats générés via PKI interne

Rotation automatique (cron ou Vault)

Tous les services Rust doivent refuser le plaintext

8. SELinux
Mode : enforcing

Politiques recommandées :

interdiction d’accès aux répertoires hors /opt/orion/

interdiction d’exécution hors /opt/orion/bin/

interdiction d’écriture hors /opt/orion/logs/ et /opt/orion/data/

9. Logs et observabilité
Logs
format JSON

rotation via logrotate

export vers Loki ou ELK

Metrics
endpoint Prometheus /metrics

dashboards Grafana fournis

10. Dépendances externes
Kafka
cluster 3–5 brokers

SASL + TLS

ACL par topic

ScyllaDB
cluster 3–6 nodes

RF=3

TWCS activé

Ceph
cluster 3–6 OSD

RGW activé

S3 API

11. Déploiement local (démo)
Pour la démonstration :

Docker Compose minimal

MinIO au lieu de Ceph

ScyllaDB en single‑node

Kafka en single‑broker

12. Durcissement RHEL
désactivation SSH root

auditd activé

journaux signés

packages minimaux

pas de compilers sur les machines de prod

13. Supervision
Alertes :

lag Kafka

latence Scylla

erreurs ML

saturation CPU

saturation disque

erreurs TLS