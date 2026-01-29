# Fix all Dockerfiles to work with root workspace context
$services = @("orion-validation", "orion-normalization", "orion-enrichment", "orion-storage-hot", "orion-traffic-generator")

foreach ($service in $services) {
    $dockerfilePath = ".\$service\Dockerfile"
    
    Write-Host "Fixing $dockerfilePath..." -ForegroundColor Cyan
    
    # Read content
    $content = Get-Content $dockerfilePath -Raw
    
    # Replace workspace COPY pattern with direct service copy
    $content = $content -replace 'COPY Cargo\.toml \./', "COPY $service/Cargo.toml ./Cargo.toml"
    $content = $content -replace "COPY $service \./$service", "COPY $service/src ./src"
    $content = $content -replace "RUN cd $service && cargo build --release", "RUN cargo build --release"
    
    # Fix binary path
    $serviceName = $service -replace "orion-", ""
    $content = $content -replace "/app/$service/target/release/$service", "/app/target/release/$service"
    
    # Write back
    Set-Content -Path $dockerfilePath -Value $content -NoNewline
    
    Write-Host "✅ Fixed $service" -ForegroundColor Green
}

Write-Host "`n✨ All Dockerfiles fixed!" -ForegroundColor Yellow
