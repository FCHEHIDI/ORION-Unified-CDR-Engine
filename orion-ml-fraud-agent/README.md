# 🤖 ORION ML Fraud Agent

**Machine Learning-based fraud detection inference service for real-time CDR analysis**

## 🎯 Purpose

High-performance ONNX inference server that detects fraudulent telecommunication activities in real-time. Provides sub-10ms predictions using trained ML models with fallback rule-based detection.

## 🏗️ Architecture

```
┌─────────────────────────────────────────┐
│     orion-ml-fraud-agent                │
├─────────────────────────────────────────┤
│                                         │
│  HTTP Server (Axum)                     │
│    ↓                                    │
│  Feature Extraction                     │
│    ↓                                    │
│  ONNX Runtime Inference                 │
│    ↓                                    │
│  Fraud Score + Classification           │
│    ↓                                    │
│  JSON Response                          │
│                                         │
└─────────────────────────────────────────┘
```

## 🔌 API Endpoints

### Health Check
```bash
GET /health
```

Response:
```json
{
  "status": "healthy",
  "service": "orion-ml-fraud-agent",
  "version": "0.1.0"
}
```

### Single Prediction
```bash
POST /predict
Content-Type: application/json

{
  "features": {
    "cdr_id": "CDR-FR-20260129-123456",
    "duration_seconds": 3600.0,
    "is_international": 1.0,
    "is_premium": 0.0,
    "is_roaming": 1.0,
    "hour_of_day": 2.0,
    "day_of_week": 6.0,
    "is_weekend": 1.0,
    "is_night_call": 1.0,
    "daily_call_count": 50.0,
    "daily_call_duration": 5000.0,
    "unique_destinations_count": 30.0,
    "call_frequency_per_hour": 5.0,
    "cell_tower_changes": 10.0,
    "signal_strength": 0.1,
    "duration_zscore": 3.5,
    "cost_zscore": 4.0
  }
}
```

Response:
```json
{
  "cdr_id": "CDR-FR-20260129-123456",
  "fraud_score": 0.85,
  "is_fraud": true,
  "confidence": 0.85,
  "inference_time_ms": 2.3
}
```

### Batch Prediction
```bash
POST /predict/batch
Content-Type: application/json

{
  "features_batch": [
    { ... },
    { ... }
  ]
}
```

### Model Information
```bash
GET /model/info
```

Response:
```json
{
  "threshold": 0.5,
  "batch_size": 32,
  "feature_count": 16,
  "model_type": "ONNX (fallback: rule-based)"
}
```

### Metrics (Prometheus)
```bash
GET /metrics
```

## 📊 Features

The model expects **16 features** per CDR:

| Feature | Type | Range | Description |
|---------|------|-------|-------------|
| `duration_seconds` | float | 0+ | Call duration in seconds |
| `is_international` | binary | 0/1 | International call flag |
| `is_premium` | binary | 0/1 | Premium number flag |
| `is_roaming` | binary | 0/1 | Roaming call flag |
| `hour_of_day` | float | 0-23 | Hour of call |
| `day_of_week` | float | 0-6 | Day of week (0=Monday) |
| `is_weekend` | binary | 0/1 | Weekend call flag |
| `is_night_call` | binary | 0/1 | Night call (22h-6h) |
| `daily_call_count` | float | 0+ | User's calls today |
| `daily_call_duration` | float | 0+ | User's total duration today |
| `unique_destinations_count` | float | 0+ | Unique destinations today |
| `call_frequency_per_hour` | float | 0+ | Calls per hour rate |
| `cell_tower_changes` | float | 0+ | Tower handoffs during call |
| `signal_strength` | float | 0-1 | Normalized signal strength |
| `duration_zscore` | float | any | Z-score of duration |
| `cost_zscore` | float | any | Z-score of cost |

## ⚙️ Configuration

Environment variables:

```bash
# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=8090

# Model
MODEL_PATH=./models/fraud_detector.onnx
FRAUD_THRESHOLD=0.5
MODEL_BATCH_SIZE=32
ENABLE_CUDA=false

# Logging
RUST_LOG=info
```

## 🚀 Usage

### Development
```bash
# Build
cargo build --release

# Run tests
cargo test

# Run service
cargo run
```

### Docker
```bash
# Build image
docker build -t orion-ml-fraud-agent .

# Run container
docker run -p 8090:8090 \
  -e MODEL_PATH=/models/fraud_detector.onnx \
  -e FRAUD_THRESHOLD=0.5 \
  orion-ml-fraud-agent
```

### Test the API
```bash
# Health check
curl http://localhost:8090/health

# Test prediction
curl -X POST http://localhost:8090/predict \
  -H "Content-Type: application/json" \
  -d '{
    "features": {
      "cdr_id": "test-123",
      "duration_seconds": 120.0,
      "is_international": 0.0,
      "is_premium": 0.0,
      "is_roaming": 0.0,
      "hour_of_day": 14.0,
      "day_of_week": 2.0,
      "is_weekend": 0.0,
      "is_night_call": 0.0,
      "daily_call_count": 5.0,
      "daily_call_duration": 600.0,
      "unique_destinations_count": 3.0,
      "call_frequency_per_hour": 0.5,
      "cell_tower_changes": 1.0,
      "signal_strength": 0.8,
      "duration_zscore": 0.2,
      "cost_zscore": 0.1
    }
  }'
```

## 📈 Metrics

Prometheus metrics exposed at `/metrics`:

- `ml_fraud_predictions_total` - Total predictions made
- `ml_fraud_detections_total` - Fraud cases detected
- `ml_fraud_prediction_duration_seconds` - Inference latency
- `ml_fraud_score_distribution` - Fraud score distribution
- `ml_fraud_model_loads_total` - Model load attempts
- `ml_fraud_model_load_errors_total` - Model load failures
- `ml_fraud_feature_extraction_errors_total` - Feature errors
- `ml_fraud_feature_extraction_duration_seconds` - Feature extraction time

## 🧪 Testing

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration

# Load testing (TODO)
# wrk -t4 -c100 -d30s http://localhost:8090/predict
```

## 🔬 Model Development

The service expects an ONNX model at `MODEL_PATH`. To train a model:

1. **Prepare training data** in [orion-ml-notebooks](../notebooks/)
2. **Train model** (XGBoost, RandomForest, Neural Network)
3. **Export to ONNX** using `skl2onnx` or `pytorch.onnx.export()`
4. **Place in** `./models/fraud_detector.onnx`
5. **Restart service**

Example model export (Python):
```python
import onnx
from skl2onnx import convert_sklearn
from skl2onnx.common.data_types import FloatTensorType

# Train your model
model = XGBClassifier()
model.fit(X_train, y_train)

# Convert to ONNX
initial_type = [('float_input', FloatTensorType([None, 16]))]
onnx_model = convert_sklearn(model, initial_types=initial_type)

# Save
with open("fraud_detector.onnx", "wb") as f:
    f.write(onnx_model.SerializeToString())
```

## 🎯 Performance Targets

- **Latency**: < 10ms per prediction (p99)
- **Throughput**: > 1000 predictions/sec
- **Accuracy**: > 95% (to be validated with real model)
- **False Positive Rate**: < 5%

## 🔄 Integration

Called by `orion-enrichment` service:

```rust
let client = reqwest::Client::new();
let response = client
    .post("http://orion-ml-fraud-agent:8090/predict")
    .json(&features)
    .send()
    .await?;
let prediction: FraudPrediction = response.json().await?;
```

## 📝 Notes

- **Fallback Mode**: When ONNX model is unavailable, uses rule-based scoring
- **GPU Support**: Enable with `ENABLE_CUDA=true` (requires CUDA-compatible GPU)
- **Model Updates**: Hot-reload not yet supported - requires service restart
- **Thread Safety**: Model inference is thread-safe via Arc<RwLock<Session>>

## 🚧 Future Enhancements

- [ ] Model hot-reloading without service restart
- [ ] A/B testing support for multiple models
- [ ] Feature importance explanations (SHAP values)
- [ ] Model drift detection
- [ ] AutoML pipeline integration
- [ ] Federated learning support
