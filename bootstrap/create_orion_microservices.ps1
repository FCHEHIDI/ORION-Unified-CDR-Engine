# create_orion_microservices.ps1

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

foreach ($svc in $services) {
    $path = "./$svc"

    # Create directory structure
    New-Item -ItemType Directory -Force -Path "$path/src" | Out-Null

    # Create Cargo.toml
    $cargo = @"
[package]
name = "$svc"
version = "0.1.0"
edition = "2021"

[dependencies]
"@
    Set-Content -Path "$path/Cargo.toml" -Value $cargo

    # Create main.rs
    $main = @"
fn main() {
    println!("$svc service running...");
}
"@
    Set-Content -Path "$path/src/main.rs" -Value $main
}

Write-Host "Arborescence des microservices ORION créée avec succès."
