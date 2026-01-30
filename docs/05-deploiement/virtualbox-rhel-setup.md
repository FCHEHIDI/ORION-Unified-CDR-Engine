# 🖥️ Guide Déploiement ORION sur RHEL avec VirtualBox

## Table des matières
1. [Prérequis](#prérequis)
2. [Téléchargement RHEL](#téléchargement-rhel)
3. [Création VM VirtualBox](#création-vm-virtualbox)
4. [Installation RHEL](#installation-rhel)
5. [Configuration initiale](#configuration-initiale)
6. [Déploiement ORION](#déploiement-orion)
7. [Automatisation avec Vagrant](#automatisation-vagrant)

---

## 1. Prérequis

### Matériel minimum recommandé
- **CPU** : 4 cœurs (8 recommandé)
- **RAM** : 16 GB (32 GB recommandé pour production)
- **Disque** : 100 GB (200 GB recommandé)
- **Réseau** : Connexion Internet

### Logiciels
- ✅ **VirtualBox** : Déjà installé (`C:\Program Files\Oracle\VirtualBox`)
- 📦 **Vagrant** (optionnel) : Pour automatisation complète
- 🔑 **Compte Red Hat** : Nécessaire pour télécharger RHEL

---

## 2. Téléchargement RHEL

### Option A : RHEL 9.x (Recommandé)
1. Allez sur : https://developers.redhat.com/products/rhel/download
2. Créez un compte Red Hat gratuit (Developer Subscription)
3. Téléchargez : **RHEL 9.x Boot ISO** (environ 1 GB)
4. Sauvegardez dans : `C:\Users\Fares\Downloads\rhel-9.x-x86_64-dvd.iso`

### Option B : AlmaLinux 9 (Alternative gratuite)
Si vous préférez un clone RHEL gratuit sans inscription :
```bash
# AlmaLinux 9.3 (compatible RHEL)
https://repo.almalinux.org/almalinux/9/isos/x86_64/AlmaLinux-9.3-x86_64-dvd.iso
```

---

## 3. Création VM VirtualBox

### Méthode GUI (Interface graphique)

#### Étape 1 : Nouvelle VM
```
VirtualBox → Nouvelle
  Nom           : ORION-RHEL-Production
  Type          : Linux
  Version       : Red Hat (64-bit)
  Dossier VM    : C:\Users\Fares\VirtualBox VMs\
```

#### Étape 2 : Mémoire
```
RAM : 16384 MB (16 GB)
```

#### Étape 3 : Disque dur
```
☑ Créer un disque dur virtuel maintenant
  Type          : VDI (VirtualBox Disk Image)
  Stockage      : Dynamiquement alloué
  Taille        : 150 GB
```

#### Étape 4 : Configuration avancée
Après création, clic droit → **Configuration** :

**Système** :
- Processeur : 4 cœurs (ou plus)
- ☑ Activer PAE/NX
- ☑ Activer VT-x/AMD-V

**Stockage** :
- Contrôleur IDE → Lecteur CD → Choisir l'ISO RHEL
- ☑ Live CD/DVD

**Réseau** :
- Carte 1 : NAT (Internet)
- Carte 2 : Réseau privé hôte (Host-Only) - Pour accès depuis Windows

**Affichage** :
- Mémoire vidéo : 128 MB
- ☑ Activer l'accélération 3D

---

## 4. Installation RHEL

### Démarrage
1. Démarrez la VM : **Démarrer** → Mode Normal
2. Boot sur l'ISO RHEL
3. Choisissez : **Install Red Hat Enterprise Linux 9.x**

### Configuration installation

#### Langue
```
Français (France) ou English (United States)
```

#### Date & Heure
```
Fuseau horaire : Europe/Paris
NTP activé      : ✅ pool.ntp.org
```

#### Installation Destination
```
Disque          : VirtualBox Disk (150 GB)
Partitionnement : Automatique (LVM)
```

#### Réseau & Nom d'hôte
```
Ethernet (enp0s3) : ✅ Activé (NAT)
Ethernet (enp0s8) : ✅ Activé (Host-Only)
Nom d'hôte        : orion-rhel-prod.local
```

#### Sélection de logiciels
```
Base Environment : Server with GUI (ou Minimal Install pour production)

Add-ons :
☑ Development Tools
☑ System Tools
☑ Container Management
```

#### Root Password
```
Root Password : <votre-mot-de-passe-sécurisé>
☑ Allow root SSH login with password (temporaire)
```

#### Utilisateur
```
Nom complet     : ORION Administrator
Nom utilisateur : orion-admin
Mot de passe    : <mot-de-passe>
☑ Make this user administrator
```

### Lancer l'installation
Cliquez **Begin Installation** → Attendez 10-15 minutes → **Reboot System**

---

## 5. Configuration initiale

### Connexion SSH depuis Windows

#### 5.1 Récupérer l'IP de la VM
Dans la VM RHEL :
```bash
# Login avec orion-admin
ip addr show enp0s8 | grep "inet "
# Note : 192.168.56.XXX
```

#### 5.2 Connexion depuis Windows PowerShell
```powershell
ssh orion-admin@192.168.56.XXX
```

### 5.3 Installation des dépendances
```bash
# Mise à jour système
sudo dnf update -y

# Outils essentiels
sudo dnf install -y \
    git \
    curl \
    wget \
    vim \
    htop \
    tmux \
    net-tools \
    firewalld \
    policycoreutils-python-utils

# Docker & Docker Compose
sudo dnf config-manager --add-repo=https://download.docker.com/linux/rhel/docker-ce.repo
sudo dnf install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin

# Démarrer Docker
sudo systemctl enable --now docker
sudo usermod -aG docker orion-admin
```

### 5.4 Configuration firewall
```bash
# Ouvrir les ports ORION
sudo firewall-cmd --permanent --add-port=8080/tcp   # API
sudo firewall-cmd --permanent --add-port=8081/tcp   # Ingestion
sudo firewall-cmd --permanent --add-port=8085/tcp   # Storage Hot
sudo firewall-cmd --permanent --add-port=9200/tcp   # Traffic Gen
sudo firewall-cmd --permanent --add-port=3000/tcp   # Grafana
sudo firewall-cmd --permanent --add-port=9090/tcp   # Prometheus
sudo firewall-cmd --reload
```

---

## 6. Déploiement ORION

### 6.1 Cloner le repository
```bash
cd ~
git clone https://github.com/FCHEHIDI/ORION-Unified-CDR-Engine.git
cd ORION-Unified-CDR-Engine
```

### 6.2 Build des images Docker
```bash
# Déconnexion/reconnexion pour appliquer le groupe docker
exit
# Reconnexion SSH
ssh orion-admin@192.168.56.XXX
cd ORION-Unified-CDR-Engine

# Build de toutes les images (15-20 minutes)
docker-compose build
```

### 6.3 Démarrer ORION
```bash
# Lancer tous les services
docker-compose up -d

# Vérifier l'état
docker-compose ps

# Suivre les logs
docker-compose logs -f --tail 50
```

### 6.4 Vérification
```bash
# Vérifier la santé de tous les services
docker ps --format "table {{.Names}}\t{{.Status}}"

# Tester l'API depuis Windows
curl http://192.168.56.XXX:8085/health
curl http://192.168.56.XXX:9200/health
```

### 6.5 Accès Grafana depuis Windows
```
URL      : http://192.168.56.XXX:3000
Login    : admin
Password : admin (changez au premier login)
```

---

## 7. Automatisation avec Vagrant

Pour un déploiement complètement automatisé, utilisez Vagrant.

### 7.1 Installer Vagrant
```powershell
# Windows (PowerShell Admin)
choco install vagrant
# ou téléchargez : https://www.vagrantup.com/downloads
```

### 7.2 Utiliser le Vagrantfile
```bash
cd C:\Users\Fares\ORION-Unified-CDR-Engine

# Démarrer la VM RHEL + installation automatique
vagrant up

# SSH automatique
vagrant ssh

# Arrêter
vagrant halt

# Détruire et recréer
vagrant destroy -f
vagrant up
```

Le Vagrantfile fourni automatise :
- ✅ Création VM VirtualBox
- ✅ Installation RHEL/AlmaLinux
- ✅ Configuration réseau
- ✅ Installation Docker
- ✅ Clone du repo
- ✅ Build des images
- ✅ Démarrage ORION

---

## 8. Production Deployment Checklist

### Sécurité
- [ ] SELinux en mode `enforcing`
- [ ] Firewall configuré (ports minimaux)
- [ ] SSH avec clés uniquement (désactiver password)
- [ ] Utilisateurs non-root uniquement
- [ ] Certificats TLS pour tous les services
- [ ] Rotation automatique des logs

### Performance
- [ ] Kernel tuning (`sysctl.conf`)
- [ ] Limites fichiers (`ulimit -n 100000`)
- [ ] Swap désactivé (pour ScyllaDB)
- [ ] I/O scheduler = `deadline` ou `noop`

### Monitoring
- [ ] Prometheus scraping activé
- [ ] Grafana dashboards importés
- [ ] Alerting configuré
- [ ] Logs centralisés (Loki/ELK)

### Backup
- [ ] Snapshots VirtualBox quotidiens
- [ ] Backup ScyllaDB (nodetool snapshot)
- [ ] Export Kafka topics
- [ ] Backup MinIO S3

---

## 9. Troubleshooting

### VM ne démarre pas
```bash
# Vérifier VT-x activé dans BIOS
# Vérifier Hyper-V désactivé (Windows)
bcdedit /set hypervisorlaunchtype off
# Redémarrer Windows
```

### Pas de réseau dans la VM
```bash
# Dans la VM
sudo nmcli connection up enp0s3
sudo nmcli connection up enp0s8
```

### Docker build échoue
```bash
# Augmenter RAM de la VM à 24 GB
# Augmenter espace disque à 200 GB
```

### Services unhealthy
```bash
# Vérifier les logs
docker-compose logs <service-name>

# Redémarrer un service
docker-compose restart <service-name>
```

---

## 10. Ressources

- **Documentation RHEL** : https://access.redhat.com/documentation/
- **VirtualBox Manual** : https://www.virtualbox.org/manual/
- **Vagrant Docs** : https://www.vagrantup.com/docs
- **ORION GitHub** : https://github.com/FCHEHIDI/ORION-Unified-CDR-Engine

---

## 11. Support

Pour questions ou problèmes :
1. Vérifier les logs : `docker-compose logs -f`
2. Vérifier la doc : `docs/05-deploiement/rhel.md`
3. GitHub Issues : https://github.com/FCHEHIDI/ORION-Unified-CDR-Engine/issues

---

**Date** : Janvier 2026  
**Version** : 1.0  
**Auteur** : ORION Team
