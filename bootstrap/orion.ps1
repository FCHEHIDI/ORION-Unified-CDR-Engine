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
    Write-Host "Initialisation:"
    Write-Host "  orion init                 - Initialise l'environnement ORION"
    Write-Host "  orion generate cdr         - Génère les CDR multi-pays"
    Write-Host ""
    Write-Host "Docker Compose:"
    Write-Host "  orion compose up           - Lance tous les services"
    Write-Host "  orion compose down         - Arrête tous les services"
    Write-Host "  orion compose logs [svc]   - Affiche les logs (optionnel: service)"
    Write-Host "  orion compose ps           - Liste les conteneurs actifs"
    Write-Host "  orion compose restart [svc]- Redémarre un service"
    Write-Host ""
    Write-Host "Ceph (Storage):"
    Write-Host "  orion ceph deploy          - Déploie cluster Ceph (Docker)"
    Write-Host "  orion ceph demo            - Exécute démo Ceph (commandes + user)"
    Write-Host "  orion ceph migrate         - Migre MinIO → Ceph"
    Write-Host "  orion ceph status          - Statut cluster Ceph"
    Write-Host ""
    Write-Host "Kafka:"
    Write-Host "  orion kafka feed           - Envoie les CDR dans Kafka"
    Write-Host "  orion kafka topics         - Liste les topics Kafka"
    Write-Host ""
    Write-Host "Tests:"
    Write-Host "  orion test unit            - Lance tests unitaires (cargo test)"
    Write-Host "  orion test e2e             - Tests end-to-end complets"
    Write-Host "  orion test pipeline        - Vérifie pipeline CDR complet"
    Write-Host ""
    Write-Host "Déploiement:"
    Write-Host "  orion deploy k8s           - Déploie ORION dans Kubernetes"
    Write-Host "  orion deploy helm          - Déploie ORION via Helm"
    Write-Host ""
    Write-Host "Monitoring:"
    Write-Host "  orion status               - Affiche l'état du cluster"
    Write-Host "  orion health               - Vérifie santé de tous les services"
    Write-Host "  orion metrics              - Ouvre Prometheus (http://localhost:9090)"
    Write-Host "  orion dashboard            - Ouvre Grafana (http://localhost:3000)"
    Write-Host ""
    Write-Host "Utilitaires:"
    Write-Host "  orion docs                 - Génère la documentation"
    Write-Host "  orion clean                - Nettoie volumes Docker"
    Write-Host "  orion build [service]      - Build un service spécifique"
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

    "compose" {
        Header
        switch ($SubCommand) {
            "up" {
                Write-Host "🚀 Lancement de la stack ORION..."
                docker-compose up -d
                Write-Host "`n✅ Stack démarrée. Vérifiez: orion compose ps"
            }
            "down" {
                Write-Host "🛑 Arrêt de la stack ORION..."
                docker-compose down
            }
            "logs" {
                if ($Arg1) {
                    docker-compose logs -f $Arg1
                } else {
                    docker-compose logs -f
                }
            }
            "ps" {
                docker-compose ps
            }
            "restart" {
                if ($Arg1) {
                    docker-compose restart $Arg1
                } else {
                    Write-Host "❌ Spécifiez un service: orion compose restart <service>"
                }
            }
            default {
                Write-Host "Commandes compose: up, down, logs [service], ps, restart <service>"
            }
        }
    }

    "ceph" {
        Header
        Set-Location ../scripts/storage
        switch ($SubCommand) {
            "deploy" {
                Write-Host "🐘 Déploiement cluster Ceph..."
                bash ./deploy-ceph-docker.sh
            }
            "demo" {
                Write-Host "🎯 Exécution démo Ceph..."
                bash ./ceph-demo.sh
            }
            "migrate" {
                Write-Host "🔄 Migration MinIO → Ceph..."
                bash ./migrate-minio-to-ceph.sh
            }
            "status" {
                docker exec ceph-mon1 ceph -s
            }
            default {
                Write-Host "Commandes ceph: deploy, demo, migrate, status"
            }
        }
        Set-Location ../../bootstrap
    }

    "kafka" {
        Header
        switch ($SubCommand) {
            "feed" {
                pwsh ./create_orion_kafka_producer.ps1 -DelayMs 5
            }
            "topics" {
                docker exec orion-kafka kafka-topics --bootstrap-server localhost:9092 --list
            }
            default {
                Write-Host "Commandes kafka: feed, topics"
            }
        }
    }

    "test" {
        Header
        Set-Location ..
        switch ($SubCommand) {
            "unit" {
                Write-Host "🧪 Tests unitaires..."
                cargo test --workspace
            }
            "e2e" {
                Write-Host "🔬 Tests end-to-end..."
                Write-Host "1. Démarrage stack..."
                docker-compose up -d
                Start-Sleep -Seconds 30
                Write-Host "2. Vérification santé services..."
                $response = Invoke-WebRequest -Uri "http://localhost:9100/pipeline/health" -UseBasicParsing
                Write-Host $response.Content
                Write-Host "3. Test génération CDR..."
                Invoke-WebRequest -Uri "http://localhost:9200/generate?count=10" -UseBasicParsing
                Start-Sleep -Seconds 5
                Write-Host "4. Vérification API..."
                Invoke-WebRequest -Uri "http://localhost:8080/health" -UseBasicParsing
                Write-Host "`n✅ Tests end-to-end terminés"
            }
            "pipeline" {
                Write-Host "⚙️  Vérification pipeline CDR..."
                $services = @(
                    "http://localhost:9200/health",
                    "http://localhost:8081/health",
                    "http://localhost:8082/health",
                    "http://localhost:8083/health",
                    "http://localhost:8084/health",
                    "http://localhost:8085/health",
                    "http://localhost:8090/health"
                )
                foreach ($url in $services) {
                    try {
                        $response = Invoke-WebRequest -Uri $url -UseBasicParsing -TimeoutSec 2
                        Write-Host "✅ $url"
                    } catch {
                        Write-Host "❌ $url"
                    }
                }
            }
            default {
                Write-Host "Commandes test: unit, e2e, pipeline"
            }
        }
        Set-Location bootstrap
    }

    "deploy" {
        Header
        switch ($SubCommand) {
            "k8s" {
                pwsh ./deploy_orion_k8s.ps1
            }
            "helm" {
                pwsh ./deploy_orion_k8s.ps1 -UseHelm
            }
            default {
                Write-Host "Commandes deploy: k8s, helm"
            }
        }
    }

    "health" {
        Header
        Write-Host "🏥 Vérification santé ORION...`n"
        try {
            $response = Invoke-RestMethod -Uri "http://localhost:9100/pipeline/health" -UseBasicParsing
            $response | ConvertTo-Json -Depth 3
        } catch {
            Write-Host "❌ Service observability non disponible"
            Write-Host "   Démarrez avec: orion compose up"
        }
    }

    "metrics" {
        Header
        Write-Host "📊 Ouverture Prometheus..."
        Start-Process "http://localhost:9090"
    }

    "dashboard" {
        Header
        Write-Host "📈 Ouverture Grafana..."
        Write-Host "   User: admin"
        Write-Host "   Pass: orion2026"
        Start-Process "http://localhost:3000"
    }

    "docs" {
        Header
        pwsh ./create_orion_docs.ps1
    }

    "status" {
        Header
        Write-Host "Docker Compose:"
        docker-compose ps
        Write-Host "`nKubernetes:"
        kubectl get pods -n orion 2>$null
    }

    "clean" {
        Header
        Write-Host "🧹 Nettoyage volumes Docker..."
        $confirm = Read-Host "Supprimer tous les volumes ORION? (y/N)"
        if ($confirm -eq "y") {
            docker-compose down -v
            Write-Host "✅ Volumes supprimés"
        }
    }

    "build" {
        Header
        Set-Location ..
        if ($SubCommand) {
            Write-Host "🔨 Build $SubCommand..."
            docker-compose build $SubCommand
        } else {
            Write-Host "🔨 Build tous les services..."
            docker-compose build
        }
        Set-Location bootstrap
    }

    default {
        Help
    }
}
