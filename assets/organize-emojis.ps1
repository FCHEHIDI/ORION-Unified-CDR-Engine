# Script PowerShell pour organiser les emojis téléchargés
# Usage: .\organize-emojis.ps1

$downloadsPath = "$env:USERPROFILE\Downloads"
$projectRoot = "D:\Users\ORION_Unified_CDR_Engine"
$emojiBase = "$projectRoot\docs\assets\emojis"

# Créer structure de dossiers
$folders = @(
    "$emojiBase\microservices",
    "$emojiBase\tech",
    "$emojiBase\concepts",
    "$emojiBase\status",
    "$emojiBase\countries",
    "$emojiBase\branding"
)

foreach ($folder in $folders) {
    if (-not (Test-Path $folder)) {
        New-Item -ItemType Directory -Path $folder -Force | Out-Null
        Write-Host "✓ Créé: $folder" -ForegroundColor Green
    }
}

# Mapping: ancien nom → nouveau nom + destination
$emojiMap = @{
    # Batch 1 - Microservices
    "midjourney_1_*.png" = @{name="orion-ingestion-emoji.png"; dest="microservices"}
    "midjourney_2_*.png" = @{name="orion-validation-emoji.png"; dest="microservices"}
    "midjourney_3_*.png" = @{name="orion-normalization-emoji.png"; dest="microservices"}
    "midjourney_4_*.png" = @{name="orion-enrichment-emoji.png"; dest="microservices"}
    "midjourney_5_*.png" = @{name="orion-storage-hot-emoji.png"; dest="microservices"}
    "midjourney_6_*.png" = @{name="orion-storage-cold-emoji.png"; dest="microservices"}
    "midjourney_7_*.png" = @{name="orion-ml-fraud-emoji.png"; dest="microservices"}
    "midjourney_8_*.png" = @{name="orion-api-emoji.png"; dest="microservices"}
    "midjourney_9_*.png" = @{name="orion-observability-emoji.png"; dest="microservices"}
    
    # Batch 2 - Tech Stack
    "midjourney_10_*.png" = @{name="kafka-emoji.png"; dest="tech"}
    "midjourney_11_*.png" = @{name="scylladb-emoji.png"; dest="tech"}
    "midjourney_12_*.png" = @{name="rust-emoji.png"; dest="tech"}
    "midjourney_13_*.png" = @{name="grafana-emoji.png"; dest="tech"}
    "midjourney_14_*.png" = @{name="prometheus-emoji.png"; dest="tech"}
    "midjourney_15_*.png" = @{name="kubernetes-emoji.png"; dest="tech"}
    "midjourney_16_*.png" = @{name="docker-emoji.png"; dest="tech"}
    
    # Batch 3 - CDR Concepts
    "midjourney_17_*.png" = @{name="cdr-record-emoji.png"; dest="concepts"}
    "midjourney_18_*.png" = @{name="voice-call-emoji.png"; dest="concepts"}
    "midjourney_19_*.png" = @{name="sms-emoji.png"; dest="concepts"}
    "midjourney_20_*.png" = @{name="data-session-emoji.png"; dest="concepts"}
    "midjourney_21_*.png" = @{name="roaming-emoji.png"; dest="concepts"}
    "midjourney_22_*.png" = @{name="fraud-detection-emoji.png"; dest="concepts"}
    
    # Batch 4 - Statuts
    "midjourney_23_*.png" = @{name="validated-emoji.png"; dest="status"}
    "midjourney_24_*.png" = @{name="error-emoji.png"; dest="status"}
    "midjourney_25_*.png" = @{name="processing-emoji.png"; dest="status"}
    "midjourney_26_*.png" = @{name="queued-emoji.png"; dest="status"}
    "midjourney_27_*.png" = @{name="completed-emoji.png"; dest="status"}
    "midjourney_28_*.png" = @{name="deployed-emoji.png"; dest="status"}
    
    # Batch 5 - Pays
    "midjourney_29_*.png" = @{name="country-fr-emoji.png"; dest="countries"}
    "midjourney_30_*.png" = @{name="country-tn-emoji.png"; dest="countries"}
    "midjourney_31_*.png" = @{name="country-fn-emoji.png"; dest="countries"}
    "midjourney_32_*.png" = @{name="country-ch-emoji.png"; dest="countries"}
    
    # Batch 6 - Branding
    "midjourney_33_*.png" = @{name="orion-logo-emoji.png"; dest="branding"}
    "midjourney_34_*.png" = @{name="unified-cdr-emoji.png"; dest="branding"}
    "midjourney_35_*.png" = @{name="pipeline-emoji.png"; dest="branding"}
}

Write-Host "`n🔄 Recherche des emojis dans $downloadsPath..." -ForegroundColor Cyan

$processed = 0
$notFound = 0

foreach ($pattern in $emojiMap.Keys) {
    $files = Get-ChildItem -Path $downloadsPath -Filter $pattern -ErrorAction SilentlyContinue
    
    if ($files.Count -gt 0) {
        $file = $files[0] # Prendre le premier si plusieurs matches
        $info = $emojiMap[$pattern]
        $destination = Join-Path $emojiBase $info.dest
        $newPath = Join-Path $destination $info.name
        
        Move-Item -Path $file.FullName -Destination $newPath -Force
        Write-Host "✓ Déplacé: $($info.name) → $($info.dest)/" -ForegroundColor Green
        $processed++
    } else {
        Write-Host "⚠ Non trouvé: $pattern" -ForegroundColor Yellow
        $notFound++
    }
}

Write-Host "`n📊 Résumé:" -ForegroundColor Cyan
Write-Host "  ✓ Traités: $processed emojis" -ForegroundColor Green
Write-Host "  ⚠ Manquants: $notFound emojis" -ForegroundColor Yellow
Write-Host "`n✨ Organisation terminée!" -ForegroundColor Magenta

# Optionnel: Ouvrir le dossier emojis dans l'explorateur
Start-Process explorer.exe $emojiBase
