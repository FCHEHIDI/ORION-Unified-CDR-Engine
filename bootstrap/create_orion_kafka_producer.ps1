# create_orion_kafka_producer.ps1
# Envoie les CDR multi-pays dans Kafka via kcat

param(
    [string]$KafkaBrokers = "localhost:9092",
    [int]$DelayMs = 10
)

$root = "./datasets/raw"

# Liste réaliste des pays
$pays = @(
    "fr","be","pl","ma","tn",
    "eg","ci","sn","cm","mg"
)

foreach ($country in $pays) {

    $file = "$root/$country/cdr_$country.txt"

    if (-Not (Test-Path $file)) {
        Write-Host "Aucun fichier trouvé pour $country, on saute."
        continue
    }

    Write-Host "Envoi des CDR pour $country vers Kafka..."

    $topic = "cdr.raw.$country"

    Get-Content $file | ForEach-Object {
        $line = $_

        # Envoi dans Kafka
        echo $line | kcat -P -b $KafkaBrokers -t $topic

        Start-Sleep -Milliseconds $DelayMs
    }

    Write-Host "Pays $country terminé."
}

Write-Host "Tous les CDR ont été envoyés dans Kafka."
