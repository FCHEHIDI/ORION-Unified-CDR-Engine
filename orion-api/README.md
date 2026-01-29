# ORION API

Public REST API for querying CDR data from ScyllaDB.

## Endpoints

### Health Check
```bash
GET /health
```

### Get CDR by ID
```bash
GET /cdr/{id}
```

Response:
```json
{
  "id": "uuid",
  "country_code": "US",
  "timestamp": "2024-01-01T10:00:00Z",
  "duration_seconds": 120,
  "call_type": "voice",
  "msisdn_a": "1234567890",
  "msisdn_b": "0987654321",
  "cell_id": "CELL001",
  "imsi": "123456789012345",
  "is_fraud": false,
  "fraud_score": 0.15
}
```

### Search CDRs
```bash
GET /cdr/search?country_code=US&start_time=2024-01-01T00:00:00Z&end_time=2024-01-02T00:00:00Z&limit=100
```

Query parameters:
- `country_code` (optional): Filter by country code
- `start_time` (optional): ISO 8601 timestamp
- `end_time` (optional): ISO 8601 timestamp
- `limit` (optional): Max results (default: 100)

## Configuration

Environment variables:
```bash
ORION_API_HOST=0.0.0.0
ORION_API_PORT=8080
SCYLLA_NODES=localhost:9042
SCYLLA_KEYSPACE=orion
```

## Development

```bash
cargo build
cargo test
cargo run
```

## Docker

```bash
docker build -t orion-api .
docker run -p 8080:8080 orion-api
```
