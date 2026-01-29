🔐 Parenthèse Sécurité — Principes Directeurs ORION
(à intégrer dans docs/02-architecture/architecture-globale.md ou dans une section dédiée)

1. Sécurité Réseau (macro‑segmentation)
ORION doit être déployé dans une architecture en zones :

Zone Ingestion  
Kafka, connecteurs, passerelles multi‑pays.

Zone Compute Rust  
ingestion, validation, enrichment, ML.

Zone Storage  
ScyllaDB, Ceph.

Zone Administration  
monitoring, logs, CI/CD.

Chaque zone est isolée par firewall, avec des règles strictes “least privilege”.

2. Sécurité des données (data‑centric)
Chiffrement :
TLS obligatoire entre tous les services (Kafka, Scylla, Ceph, APIs).

Chiffrement au repos pour Scylla et Ceph.

Masquage :
IMSI/MSISDN doivent être masqués dans :

logs,

dashboards,

exports non sécurisés.

Rétention :
hot storage : court terme (7–30 jours),

cold storage : long terme (6–24 mois),

règles de purge conformes GDPR.

3. Sécurité applicative
Authentification :
chaque microservice Rust possède une identité (token court, rotation automatique).

Autorisation :
RBAC par service (ex : ingestion ne peut pas lire Ceph).

Audit :
toutes les actions critiques sont journalisées :

accès DB,

appels ML,

erreurs de parsing,

anomalies réseau.

4. Sécurité opérationnelle
Durcissement RHEL :
SELinux enforcing,

users dédiés par service,

pas de login root,

journaux signés.

Secrets :
stockés dans un coffre (Vault, SOPS, KMS),

jamais dans le code ou les configs.

Monitoring sécurité :
alertes sur :

lag Kafka,

latence Scylla,

anomalies ML,

pics de trafic suspects.

5. Sécurité ML (fraude)
Même l’agent ML doit respecter :

isolation réseau,

audit des scores,

versioning des modèles,

contrôle des dérives (drift detection).