# orion_orchestrator.ps1
# Orchestrateur global pour la plateforme ORION
# Exécute tous les scripts d'automatisation dans le bon ordre

function Step {
    param([string]$Message)
    Write-Host "`n=== $Message ===" -ForegroundColor Cyan
}

Step "Chargement de l'environnement"
pwsh ./create_orion_env.ps1

Step "Création du workspace Cargo"
pwsh ./create_orion_workspace.ps1

Step "Génération des CDR multi-pays"
pwsh ./create_orion_generate_cdr.ps1 -CountPerCountry 2000

Step "Producteur Kafka - préparation"
pwsh ./create_orion_kafka_producer.ps1 -DelayMs 5

Step "Génération des Dockerfiles"
pwsh ./create_orion_dockerfiles.ps1

Step "Configuration Prometheus"
pwsh ./create_orion_prometheus_config.ps1

Step "Dashboards Grafana"
pwsh ./create_orion_grafana_dashboards.ps1

Step "Pipeline CI/CD GitHub Actions"
pwsh ./create_orion_ci_pipeline.ps1

Step "Manifests Kubernetes"
pwsh ./create_orion_k8s_manifests.ps1

Step "Ingress Kubernetes"
pwsh ./create_orion_k8s_ingress.ps1

Step "Helm Chart ORION"
pwsh ./create_orion_helm_chart.ps1

Step "Déploiement Kubernetes (Helm)"
pwsh ./deploy_orion_k8s.ps1 -UseHelm

Write-Host "`n=== ORION est entièrement orchestré et opérationnel ===" -ForegroundColor Green
