⚙️ 3) systemd — Services ORION
📁 docs/05-deploiement/systemd.md

🧩 Services systemd — ORION Unified CDR Engine
1. Objectif
Déployer ORION sur RHEL via systemd :

démarrage automatique,

redémarrage en cas de crash,

logs journald,

isolation utilisateur.

2. Exemple d’unité systemd
/etc/systemd/system/orion-ingestion.service

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
3. Commandes utiles
bash
systemctl daemon-reload
systemctl enable orion-ingestion
systemctl start orion-ingestion
systemctl status orion-ingestion
journalctl -u orion-ingestion -f