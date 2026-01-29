# ORION Storage Cold

CDR archival service with S3-compatible storage (MinIO/Ceph) and Parquet columnar format.

## Features

- **S3-Compatible**: Works with MinIO (dev) and Ceph RGW (production)
- **Parquet Format**: Efficient columnar storage with Snappy compression
- **Smart Partitioning**: `year=YYYY/month=MM/day=DD/country=CC/` structure
- **Batch Processing**: Accumulates 1000 records or 30s timeout before archiving
- **Prometheus Metrics**: Records archived, files uploaded, errors

## Architecture

```
Kafka (cdr-enriched) → Archive Service → Parquet Writer → S3 Upload → MinIO/Ceph
                                    ↓
                              Partition: year=2024/month=01/day=15/country=US/cdr_timestamp.parquet
```

## Configuration

Environment variables:
```bash
# Server
ORION_STORAGE_COLD_HOST=0.0.0.0
ORION_STORAGE_COLD_PORT=9400

# Kafka (future integration - currently polling mode on Windows)
KAFKA_BROKERS=localhost:9092
KAFKA_TOPIC_ENRICHED=cdr-enriched
KAFKA_GROUP_ID=orion-storage-cold

# S3 (MinIO or Ceph RGW)
S3_ENDPOINT=http://localhost:9000
S3_REGION=us-east-1
S3_BUCKET=orion-cdr-archive
S3_ACCESS_KEY=minioadmin
S3_SECRET_KEY=minioadmin
S3_PATH_STYLE=true  # true for MinIO/Ceph, false for AWS
```

## Endpoints

### Health Check
```bash
GET /health
```

### Statistics
```bash
GET /stats
```

Response:
```json
{
  "total_archived": 15000,
  "total_uploaded": 15,
  "total_errors": 0
}
```

## Development

```bash
cargo build
cargo test
cargo run
```

## Production Notes

### MinIO vs Ceph

**Development**: MinIO
- Single container deployment
- S3-compatible API
- Fast iteration

**Production**: Ceph RGW (RADOS Gateway)
- Enterprise-grade distributed storage
- CRUSH algorithm for data placement
- Same S3 API (seamless migration)

### Kafka Integration

⚠️ **Windows Limitation**: `rdkafka-sys` fails to compile on Windows (requires native librdkafka build)

**Current**: HTTP polling mode stub
**Production (Linux)**: Enable rdkafka in Cargo.toml:
```toml
rdkafka = { version = "0.37", features = ["tokio"] }
```

See [scripts/deploy-ceph-docker.sh](../scripts/deploy-ceph-docker.sh) for Ceph cluster setup.

## Parquet Schema

```
id: String
country_code: String
timestamp: Timestamp(ms)
duration_seconds: Int32
call_type: String
msisdn_a: String
msisdn_b: String
cell_id: String (nullable)
imsi: String (nullable)
is_fraud: Boolean
fraud_score: Float64 (nullable)
```

## Docker

```bash
docker build -t orion-storage-cold .
docker run -p 9400:9400 \
  -e S3_ENDPOINT=http://minio:9000 \
  -e KAFKA_BROKERS=kafka:9092 \
  orion-storage-cold
```
