# create_orion_helm_chart.ps1
# Génère un chart Helm complet pour ORION

$root = "./helm/orion"
$templates = "$root/templates"

# Create directories
New-Item -ItemType Directory -Force -Path $root | Out-Null
New-Item -ItemType Directory -Force -Path $templates | Out-Null

# ---- Chart.yaml ----
$chart = @"
apiVersion: v2
name: orion
description: ORION Unified CDR Engine - Helm Chart
type: application
version: 0.1.0
appVersion: "1.0.0"
"@
Set-Content -Path "$root/Chart.yaml" -Value $chart

# ---- values.yaml ----
$values = @"
replicaCount: 1

image:
  repository: orion
  tag: latest
  pullPolicy: IfNotPresent

countries: "fr,be,pl,ma,tn,eg,ci,sn,cm,mg"

kafka:
  brokers: "kafka:9092"

scylla:
  hosts: "scylla"
  port: 9042
  keyspace: "orion"

minio:
  endpoint: "http://minio:9000"
  bucket: "orion"

ml:
  endpoint: "http://orion-ml-fraud-agent:50051"

api:
  port: 8080
"@
Set-Content -Path "$root/values.yaml" -Value $values

# ---- namespace.yaml ----
$namespace = @"
apiVersion: v1
kind: Namespace
metadata:
  name: orion
"@
Set-Content -Path "$templates/namespace.yaml" -Value $namespace

# ---- configmap.yaml ----
$configmap = @"
apiVersion: v1
kind: ConfigMap
metadata:
  name: orion-config
  namespace: orion
data:
  KAFKA_BROKERS: {{ .Values.kafka.brokers }}
  SCYLLA_HOSTS: {{ .Values.scylla.hosts }}
  SCYLLA_PORT: "{{ .Values.scylla.port }}"
  SCYLLA_KEYSPACE: "{{ .Values.scylla.keyspace }}"
  CEPH_ENDPOINT: "{{ .Values.minio.endpoint }}"
  CEPH_BUCKET: "{{ .Values.minio.bucket }}"
  ML_ENDPOINT: "{{ .Values.ml.endpoint }}"
  API_PORT: "{{ .Values.api.port }}"
  ORION_COUNTRIES: "{{ .Values.countries }}"
"@
Set-Content -Path "$templates/configmap.yaml" -Value $configmap

# ---- Services list ----
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

# ---- Deployment + Service templates ----
foreach ($svc in $services) {

$deployment = @"
apiVersion: apps/v1
kind: Deployment
metadata:
  name: $svc
  namespace: orion
spec:
  replicas: {{ .Values.replicaCount }}
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
          image: "{{ .Values.image.repository }}/$svc:{{ .Values.image.tag }}"
          imagePullPolicy: {{ .Values.image.pullPolicy }}
          envFrom:
            - configMapRef:
                name: orion-config
          ports:
            - containerPort: 9100
"@
Set-Content -Path "$templates/deployment-$svc.yaml" -Value $deployment

$service = @"
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
Set-Content -Path "$templates/service-$svc.yaml" -Value $service
}

# ---- ingress.yaml ----
$ingress = @"
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: orion-ingress
  namespace: orion
  annotations:
    kubernetes.io/ingress.class: nginx
spec:
  rules:
    - host: api.orion.local
      http:
        paths:
          - path: /
            pathType: Prefix
            backend:
              service:
                name: orion-api
                port:
                  number: 9100
"@
Set-Content -Path "$templates/ingress.yaml" -Value $ingress

Write-Host "Helm chart ORION généré avec succès."
