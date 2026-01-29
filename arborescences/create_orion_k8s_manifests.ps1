# create_orion_k8s_manifests.ps1
# Génère les manifests Kubernetes pour tous les microservices ORION

$root = "./k8s"
New-Item -ItemType Directory -Force -Path $root | Out-Null

# Liste des services ORION
$services = @(
    "orion-ingestion",
    "orion-validation",
    "orion-normalization",
    "orion-enrichment",
    "orion-ml-fraud-agent",
    "orion-storage-hot",
    "orion-storage-cold",
    "orion-api",
    "orion-observability"
)

# ---- Namespace ----
$ns = @"
apiVersion: v1
kind: Namespace
metadata:
  name: orion
"@
Set-Content -Path "$root/namespace.yml" -Value $ns

# ---- ConfigMap (env partagé) ----
$config = @"
apiVersion: v1
kind: ConfigMap
metadata:
  name: orion-config
  namespace: orion
data:
  KAFKA_BROKERS: "kafka:9092"
  SCYLLA_HOSTS: "scylla"
  SCYLLA_PORT: "9042"
  SCYLLA_KEYSPACE: "orion"
  CEPH_ENDPOINT: "http://minio:9000"
  CEPH_BUCKET: "orion"
  ML_ENDPOINT: "http://orion-ml-fraud-agent:50051"
  API_PORT: "8080"
  ORION_COUNTRIES: "fr,be,pl,ma,tn,eg,ci,sn,cm,mg"
"@
Set-Content -Path "$root/configmap-orion.yml" -Value $config

# ---- Template Deployment + Service ----
foreach ($svc in $services) {

$manifest = @"
apiVersion: apps/v1
kind: Deployment
metadata:
  name: $svc
  namespace: orion
spec:
  replicas: 1
  selector:
    matchLabels:
      app: $svc
  template:
    metadata:
      labels:
        app: $svc
    spec:
      containers:
        - name: $svc
          image: $svc:latest
          imagePullPolicy: IfNotPresent
          envFrom:
            - configMapRef:
                name: orion-config
          ports:
            - containerPort: 9100
          readinessProbe:
            httpGet:
              path: /health
              port: 9100
            initialDelaySeconds: 5
            periodSeconds: 5
          livenessProbe:
            httpGet:
              path: /health
              port: 9100
            initialDelaySeconds: 10
            periodSeconds: 10

---
apiVersion: v1
kind: Service
metadata:
  name: $svc
  namespace: orion
spec:
  selector:
    app: $svc
  ports:
    - port: 9100
      targetPort: 9100
"@

    Set-Content -Path "$root/$svc.yml" -Value $manifest
}

Write-Host "Manifests Kubernetes ORION générés avec succès."
