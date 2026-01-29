# create_orion_dataset_structure.ps1
# Génère l'arborescence datasets réaliste pour ORION (10 pays)

$root = "./datasets"

# Liste réaliste de 10 pays du groupe
$pays = @(
    "fr", # France
    "be", # Belgique
    "pl", # Pologne
    "ma", # Maroc
    "tn", # Tunisie
    "eg", # Égypte
    "ci", # Côte d'Ivoire
    "sn", # Sénégal
    "cm", # Cameroun
    "mg"  # Madagascar
)

# Création des dossiers RAW par pays
foreach ($p in $pays) {
    $path = "$root/raw/$p"
    New-Item -ItemType Directory -Force -Path $path | Out-Null
}

# Dossiers normalisés et ML
New-Item -ItemType Directory -Force -Path "$root/normalized" | Out-Null
New-Item -ItemType Directory -Force -Path "$root/ml" | Out-Null

# README RAW
$readmeRaw = @"
# Datasets RAW (multi-pays)

Ce dossier contient les CDR bruts par pays.
Chaque sous-dossier représente un pays du groupe.

Pays inclus :
- fr (France)
- be (Belgique)
- pl (Pologne)
- ma (Maroc)
- tn (Tunisie)
- eg (Égypte)
- ci (Côte d'Ivoire)
- sn (Sénégal)
- cm (Cameroun)
- mg (Madagascar)

Placez ici les fichiers bruts (CSV, JSON, legacy, etc.).
"@
Set-Content -Path "$root/raw/README.md" -Value $readmeRaw

# README Normalized
$readmeNorm = @"
# Datasets Normalisés

Ce dossier contient les CDR après normalisation selon le schéma unifié ORION.
"@
Set-Content -Path "$root/normalized/README.md" -Value $readmeNorm

# README ML
$readmeML = @"
# Datasets ML

Ce dossier contient les datasets pour l'entraînement du modèle ML :
- features dérivées
- labels
- splits train/val/test
"@
Set-Content -Path "$root/ml/README.md" -Value $readmeML

Write-Host "Arborescence datasets ORION (10 pays) créée avec succès."
