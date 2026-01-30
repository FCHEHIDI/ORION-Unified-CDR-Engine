# ORION - VirtualBox & Vagrant Setup Script
# Vérifie et installe les prérequis pour le déploiement RHEL
# Auteur: ORION Team
# Date: Janvier 2026

Write-Host "=========================================" -ForegroundColor Cyan
Write-Host "🚀 ORION - VirtualBox & Vagrant Setup" -ForegroundColor Cyan
Write-Host "=========================================" -ForegroundColor Cyan
Write-Host ""

# Vérification des privilèges admin
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdmin) {
    Write-Host "⚠️  Ce script nécessite des privilèges administrateur" -ForegroundColor Yellow
    Write-Host "📌 Relancez PowerShell en tant qu'administrateur" -ForegroundColor Yellow
    Write-Host ""
    pause
    exit 1
}

# Fonction de vérification logiciel
function Test-Software {
    param (
        [string]$Name,
        [string]$Command,
        [string]$MinVersion
    )
    
    Write-Host "🔍 Vérification de $Name..." -NoNewline
    
    try {
        $version = & $Command --version 2>$null | Select-Object -First 1
        if ($version) {
            Write-Host " ✅ Installé ($version)" -ForegroundColor Green
            return $true
        }
    } catch {
        Write-Host " ❌ Non installé" -ForegroundColor Red
        return $false
    }
    
    Write-Host " ❌ Non trouvé" -ForegroundColor Red
    return $false
}

# Vérification VirtualBox
Write-Host ""
Write-Host "📦 VirtualBox" -ForegroundColor Yellow
Write-Host "─────────────────────────────────────────" -ForegroundColor Gray

$vboxPath = "C:\Program Files\Oracle\VirtualBox\VBoxManage.exe"
if (Test-Path $vboxPath) {
    $vboxVersion = & $vboxPath --version
    Write-Host "✅ VirtualBox installé" -ForegroundColor Green
    Write-Host "   Version: $vboxVersion" -ForegroundColor Gray
    Write-Host "   Path: $vboxPath" -ForegroundColor Gray
} else {
    Write-Host "❌ VirtualBox non trouvé" -ForegroundColor Red
    Write-Host "📥 Téléchargez VirtualBox: https://www.virtualbox.org/wiki/Downloads" -ForegroundColor Yellow
    $installVBox = Read-Host "Voulez-vous ouvrir le site de téléchargement? (o/n)"
    if ($installVBox -eq "o" -or $installVBox -eq "O") {
        Start-Process "https://www.virtualbox.org/wiki/Downloads"
    }
}

# Vérification Vagrant
Write-Host ""
Write-Host "📦 Vagrant" -ForegroundColor Yellow
Write-Host "─────────────────────────────────────────" -ForegroundColor Gray

$vagrantInstalled = Test-Software -Name "Vagrant" -Command "vagrant"

if (-not $vagrantInstalled) {
    Write-Host ""
    Write-Host "⚠️  Vagrant n'est pas installé" -ForegroundColor Yellow
    Write-Host "📌 Vagrant permet l'automatisation complète du déploiement RHEL" -ForegroundColor Gray
    Write-Host ""
    Write-Host "Options d'installation:" -ForegroundColor Cyan
    Write-Host "  1. Chocolatey (recommandé): choco install vagrant" -ForegroundColor White
    Write-Host "  2. Téléchargement direct: https://www.vagrantup.com/downloads" -ForegroundColor White
    Write-Host ""
    
    # Vérifier si Chocolatey est installé
    $chocoInstalled = Test-Software -Name "Chocolatey" -Command "choco"
    
    if ($chocoInstalled) {
        $installVagrant = Read-Host "Installer Vagrant via Chocolatey? (o/n)"
        if ($installVagrant -eq "o" -or $installVagrant -eq "O") {
            Write-Host "📥 Installation de Vagrant..." -ForegroundColor Cyan
            choco install vagrant -y
            
            if ($LASTEXITCODE -eq 0) {
                Write-Host "✅ Vagrant installé avec succès" -ForegroundColor Green
                Write-Host "⚠️  Redémarrez PowerShell pour appliquer les changements" -ForegroundColor Yellow
            } else {
                Write-Host "❌ Erreur lors de l'installation" -ForegroundColor Red
            }
        }
    } else {
        Write-Host "💡 Pour installer Chocolatey (gestionnaire de paquets):" -ForegroundColor Cyan
        Write-Host "   Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))" -ForegroundColor Gray
        Write-Host ""
        
        $openWebsite = Read-Host "Ouvrir le site de téléchargement Vagrant? (o/n)"
        if ($openWebsite -eq "o" -or $openWebsite -eq "O") {
            Start-Process "https://www.vagrantup.com/downloads"
        }
    }
}

# Vérification Git
Write-Host ""
Write-Host "📦 Git" -ForegroundColor Yellow
Write-Host "─────────────────────────────────────────" -ForegroundColor Gray

$gitInstalled = Test-Software -Name "Git" -Command "git"

if (-not $gitInstalled) {
    Write-Host "⚠️  Git n'est pas installé (nécessaire pour cloner le repo)" -ForegroundColor Yellow
    $installGit = Read-Host "Installer Git via Chocolatey? (o/n)"
    if ($installGit -eq "o" -or $installGit -eq "O") {
        choco install git -y
    }
}

# Vérification Hyper-V (conflit avec VirtualBox)
Write-Host ""
Write-Host "⚠️  Vérification des conflits" -ForegroundColor Yellow
Write-Host "─────────────────────────────────────────" -ForegroundColor Gray

$hyperv = Get-WindowsOptionalFeature -FeatureName Microsoft-Hyper-V-All -Online -ErrorAction SilentlyContinue

if ($hyperv -and $hyperv.State -eq "Enabled") {
    Write-Host "❌ Hyper-V est activé et peut causer des conflits avec VirtualBox" -ForegroundColor Red
    Write-Host "📌 Pour désactiver Hyper-V:" -ForegroundColor Yellow
    Write-Host "   bcdedit /set hypervisorlaunchtype off" -ForegroundColor White
    Write-Host "   (Redémarrage requis)" -ForegroundColor Gray
    Write-Host ""
    
    $disableHyperV = Read-Host "Désactiver Hyper-V maintenant? (o/n)"
    if ($disableHyperV -eq "o" -or $disableHyperV -eq "O") {
        bcdedit /set hypervisorlaunchtype off
        Write-Host "✅ Hyper-V sera désactivé au prochain redémarrage" -ForegroundColor Green
    }
} else {
    Write-Host "✅ Hyper-V désactivé (pas de conflit)" -ForegroundColor Green
}

# Vérification ressources système
Write-Host ""
Write-Host "💻 Ressources système" -ForegroundColor Yellow
Write-Host "─────────────────────────────────────────" -ForegroundColor Gray

$ram = [math]::Round((Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory / 1GB, 2)
$cpu = (Get-CimInstance Win32_Processor).NumberOfLogicalProcessors
$disk = Get-PSDrive C | Select-Object -ExpandProperty Free
$diskGB = [math]::Round($disk / 1GB, 2)

Write-Host "RAM totale   : $ram GB" -ForegroundColor White
Write-Host "CPU cores    : $cpu" -ForegroundColor White
Write-Host "Disque libre : $diskGB GB" -ForegroundColor White
Write-Host ""

# Recommandations
if ($ram -lt 24) {
    Write-Host "⚠️  Recommandation: 24 GB RAM minimum (vous avez $ram GB)" -ForegroundColor Yellow
    Write-Host "   Ajustez la mémoire VM dans le Vagrantfile si nécessaire" -ForegroundColor Gray
}
if ($cpu -lt 4) {
    Write-Host "⚠️  Recommandation: 4 CPU cores minimum (vous avez $cpu)" -ForegroundColor Yellow
}
if ($diskGB -lt 150) {
    Write-Host "⚠️  Recommandation: 150 GB d'espace libre minimum (vous avez $diskGB GB)" -ForegroundColor Yellow
}

# Prochaines étapes
Write-Host ""
Write-Host "=========================================" -ForegroundColor Cyan
Write-Host "📋 Prochaines étapes" -ForegroundColor Cyan
Write-Host "=========================================" -ForegroundColor Cyan
Write-Host ""

if (Test-Path $vboxPath) {
    Write-Host "Option 1 - Déploiement manuel:" -ForegroundColor Yellow
    Write-Host "  1. Téléchargez RHEL/AlmaLinux ISO" -ForegroundColor White
    Write-Host "  2. Suivez le guide: docs\05-deploiement\virtualbox-rhel-setup.md" -ForegroundColor White
    Write-Host ""
}

if ($vagrantInstalled -or ((Read-Host "Vagrant sera-t-il installé? (o/n)") -eq "o")) {
    Write-Host "Option 2 - Déploiement automatisé (Vagrant):" -ForegroundColor Yellow
    Write-Host "  1. cd C:\Users\Fares\ORION-Unified-CDR-Engine" -ForegroundColor White
    Write-Host "  2. vagrant up" -ForegroundColor White
    Write-Host "  3. vagrant ssh" -ForegroundColor White
    Write-Host "  4. cd ORION-Unified-CDR-Engine" -ForegroundColor White
    Write-Host "  5. docker compose build" -ForegroundColor White
    Write-Host "  6. docker compose up -d" -ForegroundColor White
    Write-Host ""
}

Write-Host "📖 Documentation complète:" -ForegroundColor Cyan
Write-Host "   docs\05-deploiement\virtualbox-rhel-setup.md" -ForegroundColor White
Write-Host ""

# Ouverture documentation
$openDoc = Read-Host "Ouvrir la documentation maintenant? (o/n)"
if ($openDoc -eq "o" -or $openDoc -eq "O") {
    $docPath = Join-Path $PSScriptRoot "docs\05-deploiement\virtualbox-rhel-setup.md"
    if (Test-Path $docPath) {
        Start-Process $docPath
    } else {
        Write-Host "❌ Documentation non trouvée à: $docPath" -ForegroundColor Red
    }
}

Write-Host ""
Write-Host "=========================================" -ForegroundColor Cyan
Write-Host "✅ Vérification terminée" -ForegroundColor Green
Write-Host "=========================================" -ForegroundColor Cyan
Write-Host ""

pause
