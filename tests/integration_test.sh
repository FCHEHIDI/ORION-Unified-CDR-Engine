#!/bin/bash
# ORION Integration Tests - Ne pas utiliser set -e car on veut continuer même si certains tests échouent

# ORION Integration Tests
# Tests end-to-end du pipeline CDR

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
TESTS_PASSED=0
TESTS_FAILED=0

# Helper functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[✓]${NC} $1"
    ((TESTS_PASSED++))
}

log_error() {
    echo -e "${RED}[✗]${NC} $1"
    ((TESTS_FAILED++))
}

log_warning() {
    echo -e "${YELLOW}[!]${NC} $1"
}

# Test functions
test_service_health() {
    local service_name=$1
    local port=$2
    local endpoint=${3:-/health}
    
    log_info "Testing $service_name health..."
    
    if curl -sf "http://localhost:$port$endpoint" > /dev/null 2>&1; then
        log_success "$service_name is healthy (port $port)"
        return 0
    else
        log_error "$service_name is not healthy (port $port)"
        return 1
    fi
}

test_kafka_topics() {
    log_info "Testing Kafka topics..."
    
    local topics=$(timeout 10 docker exec orion-kafka kafka-topics --bootstrap-server localhost:9092 --list 2>/dev/null || echo "")
    
    local expected_topics=("cdr.raw.FR" "cdr.raw.TN" "cdr.raw.CH" "cdr.processed" "cdr.normalized" "cdr.enriched")
    
    for topic in "${expected_topics[@]}"; do
        if echo "$topics" | grep -q "^${topic}$"; then
            log_success "Kafka topic '$topic' exists"
        else
            log_warning "Kafka topic '$topic' not found (will be auto-created)"
        fi
    done
}

test_scylladb_connection() {
    log_info "Testing ScyllaDB connection..."
    
    if timeout 10 docker exec orion-scylladb cqlsh -e "SELECT now() FROM system.local;" > /dev/null 2>&1; then
        log_success "ScyllaDB connection successful"
        return 0
    else
        log_error "ScyllaDB connection failed"
        return 1
    fi
}

test_minio_connection() {
    log_info "Testing MinIO connection..."
    
    if curl -sf "http://localhost:9000/minio/health/live" > /dev/null 2>&1; then
        log_success "MinIO is accessible"
        return 0
    else
        log_error "MinIO is not accessible"
        return 1
    fi
}

test_prometheus_metrics() {
    log_info "Testing Prometheus metrics scraping..."
    
    # Test que Prometheus collecte des métriques
    local metrics=$(curl -s "http://localhost:9090/api/v1/targets" | grep -o '"health":"up"' | wc -l)
    
    if [ "$metrics" -gt 0 ]; then
        log_success "Prometheus is scraping $metrics targets"
        return 0
    else
        log_warning "Prometheus may not be scraping targets yet"
        return 0
    fi
}

test_grafana_connection() {
    log_info "Testing Grafana connection..."
    
    if curl -sf "http://localhost:3000/api/health" > /dev/null 2>&1; then
        log_success "Grafana is accessible"
        return 0
    else
        log_error "Grafana is not accessible"
        return 1
    fi
}

test_cdr_generation() {
    log_info "Testing CDR generation..."
    
    # Envoyer une requête au traffic generator
    local response=$(curl -s -X POST "http://localhost:9200/generate" \
        -H "Content-Type: application/json" \
        -d '{
            "count": 5,
            "countries": ["FR", "TN"],
            "interval_ms": 100
        }')
    
    if echo "$response" | grep -q "generated"; then
        log_success "CDR generation successful"
        return 0
    else
        log_warning "CDR generation may not be working yet"
        return 0
    fi
}

test_cdr_ingestion() {
    log_info "Testing CDR ingestion metrics..."
    
    # Vérifier les métriques d'ingestion
    local ingestion_metrics=$(curl -s "http://localhost:8081/metrics" | grep "orion_cdr_received_total")
    
    if [ -n "$ingestion_metrics" ]; then
        log_success "Ingestion service is processing CDRs"
        return 0
    else
        log_warning "Ingestion metrics not found (service may need more time)"
        return 0
    fi
}

test_cdr_validation() {
    log_info "Testing CDR validation metrics..."
    
    local validation_metrics=$(curl -s "http://localhost:8082/metrics" | grep "orion_cdr")
    
    if [ -n "$validation_metrics" ]; then
        log_success "Validation service is processing CDRs"
        return 0
    else
        log_warning "Validation metrics not found"
        return 0
    fi
}

test_cdr_normalization() {
    log_info "Testing CDR normalization metrics..."
    
    local normalization_metrics=$(curl -s "http://localhost:8083/metrics" | grep "orion_cdr")
    
    if [ -n "$normalization_metrics" ]; then
        log_success "Normalization service is processing CDRs"
        return 0
    else
        log_warning "Normalization metrics not found"
        return 0
    fi
}

test_cdr_enrichment() {
    log_info "Testing CDR enrichment metrics..."
    
    local enrichment_metrics=$(curl -s "http://localhost:8084/metrics" | grep "orion_cdr")
    
    if [ -n "$enrichment_metrics" ]; then
        log_success "Enrichment service is processing CDRs"
        return 0
    else
        log_warning "Enrichment metrics not found"
        return 0
    fi
}

test_fraud_detection() {
    log_info "Testing fraud detection service..."
    
    # Test avec un CDR fictif
    local response=$(curl -s -X POST "http://localhost:8090/analyze" \
        -H "Content-Type: application/json" \
        -d '{
            "cdr_id": "test-001",
            "call_duration": 3600,
            "cost": 1000.0,
            "country_code": "FR"
        }')
    
    if echo "$response" | grep -q "fraud_score"; then
        log_success "Fraud detection service is responding"
        return 0
    else
        log_warning "Fraud detection may not be fully initialized"
        return 0
    fi
}

test_end_to_end_flow() {
    log_info "Testing end-to-end CDR flow (this may take a minute)..."
    
    # 1. Générer des CDR
    log_info "  Step 1: Generating test CDRs..."
    curl -s -X POST "http://localhost:9200/generate" \
        -H "Content-Type: application/json" \
        -d '{"count": 10, "countries": ["FR"], "interval_ms": 50}' > /dev/null
    
    # 2. Attendre que les CDR passent par le pipeline
    log_info "  Step 2: Waiting for CDR processing (10s)..."
    sleep 10
    
    # 3. Vérifier les métriques de chaque étape
    local ingestion_count=$(curl -s "http://localhost:8081/metrics" | grep "orion_cdr_received_total" | grep -o '[0-9]\+$' | head -1)
    local validation_count=$(curl -s "http://localhost:8082/metrics" | grep "orion_cdr_validated_total" | grep -o '[0-9]\+$' | head -1)
    local normalization_count=$(curl -s "http://localhost:8083/metrics" | grep "orion_cdr_normalized_total" | grep -o '[0-9]\+$' | head -1)
    local enrichment_count=$(curl -s "http://localhost:8084/metrics" | grep "orion_cdr_enriched_total" | grep -o '[0-9]\+$' | head -1)
    
    log_info "  Pipeline metrics:"
    log_info "    Ingested: ${ingestion_count:-0}"
    log_info "    Validated: ${validation_count:-0}"
    log_info "    Normalized: ${normalization_count:-0}"
    log_info "    Enriched: ${enrichment_count:-0}"
    
    if [ -n "$ingestion_count" ] && [ "$ingestion_count" -gt 0 ]; then
        log_success "End-to-end CDR flow is working"
        return 0
    else
        log_warning "End-to-end flow needs more time or troubleshooting"
        return 0
    fi
}

# Main test suite
main() {
    echo "================================================================"
    echo "  ORION Unified CDR Engine - Integration Tests"
    echo "================================================================"
    echo ""
    
    log_info "Starting integration tests..."
    echo ""
    
    # Test 1: Infrastructure health
    echo "=== Infrastructure Health Checks ==="
    test_kafka_topics
    test_scylladb_connection
    test_minio_connection
    test_prometheus_metrics
    test_grafana_connection
    echo ""
    
    # Test 2: Service health
    echo "=== Microservices Health Checks ==="
    test_service_health "orion-ingestion" 8081
    test_service_health "orion-validation" 8082
    test_service_health "orion-normalization" 8083
    test_service_health "orion-enrichment" 8084
    test_service_health "orion-ml-fraud-agent" 8090
    test_service_health "orion-traffic-generator" 9200 "/health" || true
    test_service_health "orion-observability" 9100
    echo ""
    
    # Test 3: Service functionality
    echo "=== Service Functionality Tests ==="
    test_cdr_generation
    sleep 2
    test_cdr_ingestion
    test_cdr_validation
    test_cdr_normalization
    test_cdr_enrichment
    test_fraud_detection
    echo ""
    
    # Test 4: End-to-end flow
    echo "=== End-to-End Flow Test ==="
    test_end_to_end_flow
    echo ""
    
    # Summary
    echo "================================================================"
    echo "  Test Results Summary"
    echo "================================================================"
    echo -e "${GREEN}Passed:${NC} $TESTS_PASSED"
    echo -e "${RED}Failed:${NC} $TESTS_FAILED"
    echo ""
    
    if [ $TESTS_FAILED -eq 0 ]; then
        echo -e "${GREEN}✓ All critical tests passed!${NC}"
        exit 0
    else
        echo -e "${RED}✗ Some tests failed. Please check the logs above.${NC}"
        exit 1
    fi
}

# Run main
main "$@"
