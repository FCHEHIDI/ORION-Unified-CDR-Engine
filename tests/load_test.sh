#!/bin/bash
# Load test - génère du trafic CDR pour tester la performance

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Configuration
CDR_COUNT=${1:-1000}
COUNTRIES=("FR" "TN" "CH" "FN")
INTERVAL_MS=${2:-10}

echo "📊 ORION Load Test"
echo "  CDR Count: $CDR_COUNT"
echo "  Interval: ${INTERVAL_MS}ms"
echo ""

# Fonction pour générer des CDR
generate_cdrs() {
    local country=$1
    local count=$2
    
    echo "Generating $count CDRs for country: $country..."
    
    curl -s -X POST "http://localhost:9200/generate" \
        -H "Content-Type: application/json" \
        -d "{
            \"count\": $count,
            \"countries\": [\"$country\"],
            \"interval_ms\": $INTERVAL_MS
        }" | jq -r '.message'
}

# Distribuer les CDR entre les pays
per_country=$((CDR_COUNT / ${#COUNTRIES[@]}))

echo "Generating CDRs across ${#COUNTRIES[@]} countries..."
for country in "${COUNTRIES[@]}"; do
    generate_cdrs "$country" "$per_country" &
done

# Attendre que tous les générateurs finissent
wait

echo ""
echo "✓ CDR generation completed"
echo ""
echo "Waiting 30s for processing..."
sleep 30

# Afficher les statistiques
echo ""
echo "=== Pipeline Statistics ==="
echo ""

get_metric() {
    local port=$1
    local metric=$2
    curl -s "http://localhost:$port/metrics" | grep "$metric" | grep -o '[0-9]\+$' | head -1
}

echo "Ingestion:      $(get_metric 8081 'orion_cdr_received_total') CDRs"
echo "Validation:     $(get_metric 8082 'orion_cdr_validated_total') CDRs"
echo "Normalization:  $(get_metric 8083 'orion_cdr_normalized_total') CDRs"
echo "Enrichment:     $(get_metric 8084 'orion_cdr_enriched_total') CDRs"

echo ""
echo "✓ Load test completed"
