# create_orion_systemd.ps1

$services = @(
    "orion-ingestion",
    "orion-validation",
    "orion-normalization",
    "orion-enrichment",
    "orion-ml-fraud-agent",
    "orion-storage-hot",
    "orion-storage-cold",
    "orion-api",
    "orion-observability"
)

$systemdDir = "./systemd"
New-Item -ItemType Directory -Force -Path $systemdDir | Out-Null

foreach ($svc in $services) {
    $serviceFile = "$systemdDir/$svc.service"

    $content = @"
[Unit]
Description=$svc Service
After=network.target

[Service]
User=$svc
ExecStart=/opt/orion/bin/$svc --config /opt/orion/config/$svc.toml
Restart=always
RestartSec=3
LimitNOFILE=100000
Environment="RUST_LOG=info"

[Install]
WantedBy=multi-user.target
"@

    Set-Content -Path $serviceFile -Value $content
}

Write-Host "Fichiers systemd ORION générés avec succès."
