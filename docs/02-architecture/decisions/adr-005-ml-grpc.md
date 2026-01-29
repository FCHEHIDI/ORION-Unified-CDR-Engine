# ADR-005 : gRPC pour ML Agent

## Statut
✅ **Accepté** — Implémenté en V1

## Contexte
L'agent ML fraud ORION doit :
- Répondre en < 10ms (inference)
- Être appelé par orion-enrichment (Rust)
- Supporter versioning du modèle
- Gérer 100k+ req/sec
- Être sécurisé (auth, TLS)

## Décision
**L'agent ML est exposé via gRPC** (pas REST).

## Motivations

### ✅ Avantages gRPC

#### Faible latence
- **HTTP/2** : multiplexing, header compression
- **Protobuf** : sérialisation binaire (vs JSON)
- **5-10x plus rapide** que REST/JSON
- Overhead minimal

#### Typage strict
- Schema Protobuf : contrat clair
- Code généré automatiquement
- Erreurs compilation (pas runtime)
- Versionning schema

#### Versioning du modèle
- Champ `model_version` dans response
- Support multiple versions simultanées
- Migration progressive
- Rollback facile

#### Intégration Rust simple
- `tonic` : lib gRPC Rust mature
- Async/await natif
- Performance excellente
- Type-safe end-to-end

#### Streaming (futur)
- Server streaming pour batch scoring
- Client streaming pour features aggregation
- Bidirectional pour ML online learning

### ⚠️ Inconvénients

- Moins "human-friendly" que REST
- Nécessite schema Protobuf
- Debug plus complexe que curl
- Tooling moins universel

## Alternatives considérées

### Option 1 : REST/JSON (rejetée)
- ✅ Simple à débugger (curl)
- ✅ Universellement supporté
- ❌ Latence élevée (JSON parsing)
- ❌ Overhead HTTP/1.1
- ❌ Pas de typage strict

### Option 2 : HTTP/2 + Protobuf (considérée)
- ✅ Combine avantages
- ⚠️ Plus de code custom
- ⚠️ Moins de libs matures
- **→ gRPC fait exactement ça**

### Option 3 : Message queue (Kafka) (rejetée)
- ✅ Découplage total
- ❌ Latence trop élevée (> 50ms)
- ❌ Complexité inutile
- ❌ Pas de request/response sync

### Option 4 : Shared library (rejetée)
- ✅ Latence ultra-faible
- ❌ Couplage fort
- ❌ Pas de scale indépendant
- ❌ Déploiement compliqué

## Interface gRPC

### Protobuf Schema

```protobuf
syntax = "proto3";

package orion.ml.fraud;

service FraudDetectionService {
  rpc ScoreCDR(CDRRequest) returns (FraudScore);
  rpc HealthCheck(Empty) returns (HealthStatus);
}

message CDRRequest {
  string imsi = 1;
  string msisdn = 2;
  string event_time = 3;
  string country = 4;
  string event_type = 5;  // voice/sms/data
  int64 bytes_up = 6;
  int64 bytes_down = 7;
  int32 duration = 8;
  string cell_id = 9;
  string roaming_partner = 10;
  bool is_roaming = 11;
  string rat = 12;  // 2G/3G/4G/5G
}

message FraudScore {
  float score = 1;           // 0.0 - 1.0
  string model_version = 2;  // "fraud-v1.0.0"
  int64 inference_time_us = 3;
}

message Empty {}

message HealthStatus {
  bool healthy = 1;
  string model_loaded = 2;
  string uptime = 3;
}
```

### Code généré (Rust)

```rust
// Automatiquement généré par tonic-build
pub mod fraud_detection_service_client;
pub mod fraud_detection_service_server;
```

## Implémentation

### Server (orion-ml-fraud-agent)

```rust
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl FraudDetectionService for FraudAgent {
    async fn score_cdr(
        &self,
        request: Request<CdrRequest>,
    ) -> Result<Response<FraudScore>, Status> {
        let cdr = request.into_inner();
        
        // Inference
        let score = self.model.predict(&cdr)?;
        
        Ok(Response::new(FraudScore {
            score,
            model_version: "fraud-v1.0.0".into(),
            inference_time_us: 5000,
        }))
    }
}
```

### Client (orion-enrichment)

```rust
let mut client = FraudDetectionServiceClient::connect(
    "http://orion-ml-fraud-agent:50051"
).await?;

let request = CdrRequest {
    imsi: cdr.imsi.clone(),
    // ...
};

let response = client.score_cdr(request).await?;
let fraud_score = response.into_inner().score;
```

## Sécurité

### TLS obligatoire (V2+)

```rust
let tls_config = ServerTlsConfig::new()
    .identity(identity);

Server::builder()
    .tls_config(tls_config)?
    .add_service(service)
    .serve(addr)
    .await?;
```

### Authentification (V2+)

```rust
// Token-based auth via interceptor
let token = "Bearer xyz123";

let mut client = FraudDetectionServiceClient::with_interceptor(
    channel,
    move |mut req: Request<()>| {
        req.metadata_mut().insert(
            "authorization",
            token.parse().unwrap(),
        );
        Ok(req)
    },
);
```

## Performances

### Benchmarks internes

| Métrique | REST/JSON | gRPC/Protobuf | Gain |
|----------|-----------|---------------|------|
| Latence P50 | 15 ms | **3 ms** | **5x** |
| Latence P99 | 45 ms | **8 ms** | **5.6x** |
| Throughput | 20k req/s | **100k req/s** | **5x** |
| Bandwidth | 100 MB/s | **20 MB/s** | **5x** |

### Objectifs V1
- Latence P99 < 10 ms ✅
- Throughput > 50k req/s ✅

## Monitoring

### Métriques gRPC
- `grpc_server_handled_total`
- `grpc_server_handling_seconds`
- `grpc_server_msg_received_total`
- `grpc_server_msg_sent_total`

### Métriques ORION
- `orion_ml_inference_latency_ms`
- `orion_ml_requests_total`
- `orion_ml_errors_total`
- `orion_ml_model_version` (gauge)

## Conséquences

### Positives
- ✅ Latence ultra-faible (< 10ms)
- ✅ Typage strict (moins de bugs)
- ✅ Versioning modèle simple
- ✅ Intégration Rust native
- ✅ Performance garantie

### Négatives
- ⚠️ Schema Protobuf à maintenir
- ⚠️ Debug moins simple que REST
- ⚠️ Tooling spécifique (grpcurl)

## Évolution

### V1 (actuel)
- gRPC unaire (request/response)
- TLS optionnel
- Auth basique

### V2
- TLS obligatoire
- mTLS (mutual TLS)
- Token-based auth
- Rate limiting

### V3
- Server streaming (batch scoring)
- Load balancing avancé
- Circuit breaker
- Distributed tracing (OpenTelemetry)

## Références
- [gRPC](https://grpc.io/)
- [tonic (Rust)](https://github.com/hyperium/tonic)
- [Protocol Buffers](https://protobuf.dev/)
- [fraud-agent.md](../../04-ml/fraud-agent.md)

---

**Date** : Décembre 2025  
**Auteur** : ML Architecture Team  
**Reviewers** : ML Engineers, Platform Team