# orion.ps1
# CLI ORION - Unified CDR Engine

param(
    [Parameter(Position=0)]
    [string]$Command,
    [string]$SubCommand,
    [string]$Arg1,
    [string]$Arg2
)

function Header {
    Write-Host "`n=== ORION Unified CDR Engine ===" -ForegroundColor Cyan
}

function Help {
    Header
    Write-Host "Commandes disponibles :`n"
    Write-Host "  orion init                 - Initialise l'environnement ORION"
    Write-Host "  orion generate cdr         - Génère les CDR multi-pays"
    Write-Host "  orion kafka feed           - Envoie les CDR dans Kafka"
    Write-Host "  orion deploy k8s           - Déploie ORION dans Kubernetes"
    Write-Host "  orion deploy helm          - Déploie ORION via Helm"
    Write-Host "  orion docs                 - Génère la documentation"
    Write-Host "  orion status               - Affiche l'état du cluster"
}

switch ($Command) {

    "init" {
        Header
        pwsh ./create_orion_env.ps1
        pwsh ./create_orion_workspace.ps1
        Write-Host "Environnement ORION initialisé."
    }

    "generate" {
        if ($SubCommand -eq "cdr") {
            Header
            pwsh ./create_orion_generate_cdr.ps1 -CountPerCountry 2000
        }
    }

    "kafka" {
        if ($SubCommand -eq "feed") {
            Header
            pwsh ./create_orion_kafka_producer.ps1 -DelayMs 5
        }
    }

    "deploy" {
        if ($SubCommand -eq "k8s") {
            Header
            pwsh ./deploy_orion_k8s.ps1
        }
        elseif ($SubCommand -eq "helm") {
            Header
            pwsh ./deploy_orion_k8s.ps1 -UseHelm
        }
    }

    "docs" {
        Header
        pwsh ./create_orion_docs.ps1
    }

    "status" {
        Header
        kubectl get pods -n orion
    }

    default {
        Help
    }
}
