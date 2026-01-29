# create_orion_docker_compose.ps1

$compose = @"
version: "3.9"

services:
  kafka:
    image: bitnami/kafka:latest
    container_name: kafka
    ports:
      - "9092:9092"
    environment:
      - KAFKA_ENABLE_KRAFT=yes
      - KAFKA_CFG_NODE_ID=1
      - KAFKA_CFG_PROCESS_ROLES=broker,controller
      - KAFKA_CFG_CONTROLLER_QUORUM_VOTERS=1@kafka:9093
      - KAFKA_CFG_LISTENERS=PLAINTEXT://:9092,CONTROLLER://:9093
      - KAFKA_CFG_ADVERTISED_LISTENERS=PLAINTEXT://localhost:9092
      - ALLOW_PLAINTEXT_LISTENER=yes

  scylla:
    image: scylladb/scylla:latest
    container_name: scylla
    command: --smp 1 --memory 750M
    ports:
      - "9042:9042"

  minio:
    image: minio/minio
    container_name: minio
    command: server /data
    ports:
      - "9000:9000"
    environment:
      MINIO_ROOT_USER: admin
      MINIO_ROOT_PASSWORD: admin123
    volumes:
      - minio_data:/data

  prometheus:
    image: prom/prometheus
    container_name: prometheus
    ports:
      - "9090:9090"
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml

  grafana:
    image: grafana/grafana
    container_name: grafana
    ports:
      - "3000:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
    volumes:
      - grafana_data:/var/lib/grafana

  # ORION microservices (à activer au fur et à mesure)
  # orion-ingestion:
  #   build: ./orion-ingestion
  #   depends_on:
  #     - kafka
  #   environment:
  #     - RUST_LOG=info

  # orion-validation:
  #   build: ./orion-validation

  # orion-normalization:
  #   build: ./orion-normalization

  # orion-enrichment:
  #   build: ./orion-enrichment

  # orion-ml-fraud-agent:
  #   build: ./orion-ml-fraud-agent

  # orion-storage-hot:
  #   build: ./orion-storage-hot

  # orion-storage-cold:
  #   build: ./orion-storage-cold

  # orion-api:
  #   build: ./orion-api

volumes:
  minio_data:
  grafana_data:
"@

Set-Content -Path "./docker-compose.yml" -Value $compose

Write-Host "docker-compose.yml généré avec succès."
