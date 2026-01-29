# create_orion_workspace.ps1
# Génère un Cargo workspace pour tous les microservices ORION

$workspaceFile = "./Cargo.toml"

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

$membersList = $services | ForEach-Object { "    \"$_\"" } | Out-String

$content = @"
[workspace]
members = [
$membersList]
resolver = "2"
"@

Set-Content -Path $workspaceFile -Value $content

Write-Host "Cargo workspace ORION généré avec succès."
