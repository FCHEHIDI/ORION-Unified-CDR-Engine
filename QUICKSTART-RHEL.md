# 🚀 Quick Start - ORION sur RHEL avec VirtualBox

## Déploiement en 3 étapes (Automatique avec Vagrant)

### 1️⃣ Vérifier les prérequis
```powershell
# PowerShell (Admin)
cd C:\Users\Fares\ORION-Unified-CDR-Engine
.\scripts\setup-virtualbox.ps1
```

Ce script vérifie automatiquement :
- ✅ VirtualBox installé
- ✅ Vagrant installé (ou propose l'installation)
- ✅ Git installé
- ✅ Ressources système suffisantes
- ✅ Hyper-V désactivé (évite conflits)

### 2️⃣ Démarrer la VM RHEL
```powershell
# Installation automatique RHEL + Docker + ORION
vagrant up

# Durée: 10-15 minutes
# Actions:
# - Télécharge AlmaLinux 9 (RHEL-compatible)
# - Crée VM VirtualBox (16GB RAM, 4 CPU, 150GB disk)
# - Installe Docker + dépendances
# - Clone le repo ORION
# - Configure firewall + kernel tuning
```

### 3️⃣ Déployer ORION
```bash
# SSH dans la VM
vagrant ssh

# Build des images Docker (15-20 minutes)
cd ORION-Unified-CDR-Engine
docker compose build

# Démarrer tous les services
docker compose up -d

# Vérifier l'état
docker compose ps
```

## Accès depuis Windows

Une fois ORION démarré, accédez aux services :

| Service | URL | Login |
|---------|-----|-------|
| Grafana | http://localhost:3000 | admin/admin |
| Prometheus | http://localhost:9090 | - |
| API | http://localhost:8080/health | - |
| Ingestion | http://localhost:8081/health | - |
| Storage Hot | http://localhost:8085/health | - |
| Traffic Gen | http://localhost:9200/health | - |

## Commandes Vagrant utiles

```bash
vagrant status          # État de la VM
vagrant ssh            # Connexion SSH
vagrant halt           # Arrêter la VM
vagrant reload         # Redémarrer la VM
vagrant destroy -f     # Supprimer la VM
vagrant up             # (Re)créer et démarrer
```

## Commandes Docker (dans la VM)

```bash
# Logs en temps réel
docker compose logs -f

# Logs d'un service spécifique
docker compose logs -f orion-enrichment

# Redémarrer un service
docker compose restart orion-storage-hot

# Arrêter tout
docker compose down

# Démarrer tout
docker compose up -d
```

## Troubleshooting

### VM ne démarre pas
```powershell
# Vérifier Hyper-V désactivé
bcdedit /set hypervisorlaunchtype off
# Redémarrer Windows
```

### Build Docker échoue
```bash
# Dans Vagrantfile, augmentez la RAM:
vb.memory = "24576"  # 24 GB au lieu de 16

# Puis:
vagrant reload
```

### Port déjà utilisé
```powershell
# Vérifier quel processus utilise le port (ex: 3000)
netstat -ano | findstr :3000

# Tuer le processus
taskkill /PID <PID> /F
```

## Déploiement Manuel (sans Vagrant)

Si vous préférez créer la VM manuellement :

1. Téléchargez RHEL/AlmaLinux ISO
2. Suivez le guide complet : [docs/05-deploiement/virtualbox-rhel-setup.md](../docs/05-deploiement/virtualbox-rhel-setup.md)

## Ressources

- **Guide complet** : [virtualbox-rhel-setup.md](../docs/05-deploiement/virtualbox-rhel-setup.md)
- **Architecture ORION** : [docs/02-architecture/](../docs/02-architecture/)
- **Monitoring** : [docs/05-deploiement/monitoring.md](../docs/05-deploiement/monitoring.md)

---

**Date** : Janvier 2026  
**Version** : 1.0
