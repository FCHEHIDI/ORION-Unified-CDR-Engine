# ============================================
# ORION Unified CDR Engine - Makefile Global
# ============================================

SHELL := pwsh

# ----------- Build & Test -----------
build:
    @echo "== Build ORION Workspace =="
    cargo build --workspace --release

test:
    @echo "== Tests ORION =="
    cargo test --workspace

fmt:
    @echo "== Format Rust =="
    cargo fmt --all

lint:
    @echo "== Clippy Lint =="
    cargo clippy --workspace -- -D warnings

# ----------- Dataset & Kafka -----------
cdr:
    @echo "== Génération CDR =="
    pwsh ./create_orion_generate_cdr.ps1 -CountPerCountry 2000

kafka:
    @echo "== Envoi CDR dans Kafka =="
    pwsh ./create_orion_kafka_producer.ps1 -DelayMs 5

# ----------- Docker -----------
dockerfiles:
    @echo "== Génération Dockerfiles =="
    pwsh ./create_orion_dockerfiles.ps1

docker-build:
    @echo "== Build Docker images =="
    docker compose build

docker-up:
    @echo "== Docker Compose Up =="
    docker compose up -d

docker-down:
    @echo "== Docker Compose Down =="
    docker compose down

# ----------- Kubernetes -----------
k8s:
    @echo "== Déploiement Kubernetes =="
    pwsh ./deploy_orion_k8s.ps1

helm:
    @echo "== Déploiement Helm =="
    pwsh ./deploy_orion_k8s.ps1 -UseHelm

# ----------- Observabilité -----------
prometheus:
    @echo "== Config Prometheus =="
    pwsh ./create_orion_prometheus_config.ps1

grafana:
    @echo "== Dashboards Grafana =="
    pwsh ./create_orion_grafana_dashboards.ps1

# ----------- Documentation -----------
docs:
    @echo "== Génération Documentation ORION =="
    pwsh ./create_orion_docs.ps1

# ----------- Clean -----------
clean:
    @echo "== Nettoyage =="
    cargo clean
