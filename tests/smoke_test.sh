#!/bin/bash
# Quick smoke test - vérifie que tous les services sont up

set -e

GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

echo "🔍 ORION Smoke Test"
echo ""

check_container() {
    local name=$1
    if docker ps --filter "name=$name" --filter "status=running" | grep -q "$name"; then
        echo -e "${GREEN}✓${NC} $name is running"
        return 0
    else
        echo -e "${RED}✗${NC} $name is NOT running"
        return 1
    fi
}

FAILED=0

# Infrastructure
check_container "orion-kafka" || ((FAILED++))
check_container "orion-zookeeper" || ((FAILED++))
check_container "orion-scylladb" || ((FAILED++))
check_container "orion-minio" || ((FAILED++))
check_container "orion-prometheus" || ((FAILED++))
check_container "orion-grafana" || ((FAILED++))

echo ""

# Services
check_container "orion-ingestion" || ((FAILED++))
check_container "orion-validation" || ((FAILED++))
check_container "orion-normalization" || ((FAILED++))
check_container "orion-enrichment" || ((FAILED++))
check_container "orion-ml-fraud-agent" || ((FAILED++))
check_container "orion-traffic-generator" || ((FAILED++))
check_container "orion-storage-hot" || ((FAILED++))
check_container "orion-storage-cold" || ((FAILED++))
check_container "orion-observability" || ((FAILED++))
check_container "orion-api" || ((FAILED++))

echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✓ All services are running!${NC}"
    exit 0
else
    echo -e "${RED}✗ $FAILED service(s) are not running${NC}"
    exit 1
fi
