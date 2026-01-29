# create_docs_structure.ps1

$folders = @(
    "docs",
    "docs/00-overview",
    "docs/01-cadrage",
    "docs/02-architecture",
    "docs/02-architecture/diagrammes",
    "docs/02-architecture/decisions",
    "docs/03-data",
    "docs/04-ml",
    "docs/05-deploiement",
    "docs/06-demo",
    "docs/06-demo/screenshots"
)

$files = @(
    "docs/00-overview/introduction.md",
    "docs/00-overview/glossary.md",
    "docs/00-overview/vision.md",
    "docs/01-cadrage/cahier-des-charges.md",
    "docs/01-cadrage/note-de-cadrage.md",
    "docs/01-cadrage/roadmap.md",
    "docs/02-architecture/architecture-globale.md",
    "docs/02-architecture/architecture-detaillee.md",
    "docs/02-architecture/decisions/adr-001-rust-only.md",
    "docs/02-architecture/decisions/adr-002-scylla.md",
    "docs/02-architecture/decisions/adr-003-ceph.md",
    "docs/03-data/schema-cdr-unifie.md",
    "docs/03-data/scylladb-model.md",
    "docs/03-data/datasets.md",
    "docs/04-ml/fraud-agent.md",
    "docs/04-ml/features.md",
    "docs/04-ml/model.md",
    "docs/05-deploiement/rhel.md",
    "docs/05-deploiement/systemd.md",
    "docs/05-deploiement/docker-local.md",
    "docs/05-deploiement/monitoring.md",
    "docs/06-demo/scenario.md",
    "docs/06-demo/scripts.md"
)

foreach ($folder in $folders) {
    New-Item -ItemType Directory -Force -Path $folder | Out-Null
}

foreach ($file in $files) {
    New-Item -ItemType File -Force -Path $file | Out-Null
}

Write-Host "Structure docs/ créée avec succès."
