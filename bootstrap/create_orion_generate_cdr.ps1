# create_orion_generate_cdr.ps1
# Génère des CDR multi-pays réalistes dans datasets/raw/<pays>/

param(
    [int]$CountPerCountry = 1000
)

$root = "./datasets/raw"

# Définition des pays et formats
$paysFormats = @{
    "fr" = "csv"
    "be" = "csv"
    "pl" = "csv"
    "ma" = "csv"
    "tn" = "json"
    "eg" = "json"
    "ci" = "legacy"
    "sn" = "legacy"
    "cm" = "csv"
    "mg" = "csv"
}

# RAT possibles
$rats = @("2G", "3G", "4G", "5G")

# Générateur IMSI réaliste
function New-IMSI($mcc) {
    return "$mcc" + (Get-Random -Minimum 1000000000 -Maximum 9999999999)
}

# Générateur MSISDN réaliste
function New-MSISDN($countryCode) {
    return "+$countryCode" + (Get-Random -Minimum 60000000 -Maximum 99999999)
}

# Générateur CellID
function New-CellID($prefix) {
    return "$prefix" + (Get-Random -Minimum 1000 -Maximum 9999)
}

# Génération d'un CDR CSV
function Generate-CSV-CDR($country) {
    $mcc = switch ($country) {
        "fr" { "208" }
        "be" { "206" }
        "pl" { "260" }
        "ma" { "604" }
        "cm" { "624" }
        "mg" { "646" }
        default { "999" }
    }

    $cc = switch ($country) {
        "fr" { "33" }
        "be" { "32" }
        "pl" { "48" }
        "ma" { "212" }
        "cm" { "237" }
        "mg" { "261" }
        default { "999" }
    }

    $imsi = New-IMSI $mcc
    $msisdn = New-MSISDN $cc
    $cell = New-CellID $country.ToUpper()
    $rat = $rats | Get-Random
    $up = Get-Random -Minimum 1000 -Maximum 5000000
    $down = Get-Random -Minimum 1000 -Maximum 5000000
    $ts = (Get-Date).AddMinutes(-1 * (Get-Random -Minimum 0 -Maximum 10000)).ToString("s") + "Z"

    return "$imsi;$msisdn;$cell;$rat;$up;$down;$ts"
}

# Génération d'un CDR JSON
function Generate-JSON-CDR($country) {
    $mcc = switch ($country) {
        "tn" { "605" }
        "eg" { "602" }
        default { "999" }
    }

    $cc = switch ($country) {
        "tn" { "216" }
        "eg" { "20" }
        default { "999" }
    }

    $cdr = @{
        subscriber = @{
            imsi = New-IMSI $mcc
            msisdn = New-MSISDN $cc
        }
        network = @{
            cell = New-CellID $country.ToUpper()
            rat = $rats | Get-Random
        }
        usage = @{
            up = Get-Random -Minimum 1000 -Maximum 5000000
            down = Get-Random -Minimum 1000 -Maximum 5000000
        }
        timestamp = (Get-Date).AddMinutes(-1 * (Get-Random -Minimum 0 -Maximum 10000)).ToString("s") + "Z"
    }

    return ($cdr | ConvertTo-Json -Depth 5)
}

# Génération d'un CDR legacy texte
function Generate-Legacy-CDR($country) {
    $imsi = New-IMSI "999"
    $msisdn = New-MSISDN "999"
    $cell = New-CellID $country.ToUpper()
    $rat = $rats | Get-Random
    $up = Get-Random -Minimum 1000 -Maximum 5000000
    $down = Get-Random -Minimum 1000 -Maximum 5000000
    $ts = (Get-Date).AddMinutes(-1 * (Get-Random -Minimum 0 -Maximum 10000)).ToString("s") + "Z"

    return "IMSI=$imsi|MSISDN=$msisdn|CELL=$cell|RAT=$rat|UP=$up|DOWN=$down|TS=$ts"
}

# Boucle principale
foreach ($country in $paysFormats.Keys) {
    $format = $paysFormats[$country]
    $outputFile = "$root/$country/cdr_$country.txt"

    Write-Host "Génération CDR pour $country ($format)..."

    $lines = @()

    for ($i = 0; $i -lt $CountPerCountry; $i++) {
        switch ($format) {
            "csv"    { $lines += Generate-CSV-CDR $country }
            "json"   { $lines += Generate-JSON-CDR $country }
            "legacy" { $lines += Generate-Legacy-CDR $country }
        }
    }

    Set-Content -Path $outputFile -Value $lines
}

Write-Host "CDR multi-pays générés avec succès."
