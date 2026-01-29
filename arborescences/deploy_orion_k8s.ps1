# deploy_orion_k8s.ps1
# Déploie ORION dans Kubernetes via Helm ou manifests YAML

param(
    [switch]$UseHelm
)

Write-Host "=== ORION Kubernetes Deployment ==="

# Vérification kubectl
if (-not (Get-Command "kubectl" -ErrorAction SilentlyContinue)) {
    Write-Host "kubectl n'est pas installé. Abandon."
    exit 1
}

# Namespace
$namespace = "orion"

# Création du namespace si absent
$nsExists = kubectl get namespace $namespace -o name 2>$null
if (-not $nsExists) {
    Write-Host "Création du namespace 'orion'..."
    kubectl create namespace $namespace | Out-Null
}

# Mode Helm
if ($UseHelm) {
    if (-not (Test-Path "./helm/orion")) {
        Write-Host "Chart Helm introuvable. Abandon."
        exit 1
    }

    Write-Host "Déploiement via Helm..."
    helm upgrade --install orion ./helm/orion -n orion

    Write-Host "Déploiement Helm terminé."
}
else {
    # Mode YAML
    if (-not (Test-Path "./k8s")) {
        Write-Host "Dossier k8s/ introuvable. Abandon."
        exit 1
    }

    Write-Host "Déploiement via manifests YAML..."
    kubectl apply -f ./k8s/ -n orion
    Write-Host "Déploiement YAML terminé."
}

# Résumé
Write-Host "`n=== Ressources ORION déployées ==="
kubectl get all -n orion

Write-Host "`nORION est maintenant déployé dans Kubernetes."
