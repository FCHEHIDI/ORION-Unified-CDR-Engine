# create_orion_k8s_ingress.ps1
# Génère un Ingress Kubernetes pour exposer ORION API, Grafana et Prometheus

$root = "./k8s"
New-Item -ItemType Directory -Force -Path $root | Out-Null

$ingress = @"
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: orion-ingress
  namespace: orion
  annotations:
    kubernetes.io/ingress.class: nginx
    nginx.ingress.kubernetes.io/rewrite-target: /\$1
spec:
  rules:
    - host: api.orion.local
      http:
        paths:
          - path: /(.*)
            pathType: Prefix
            backend:
              service:
                name: orion-api
                port:
                  number: 9100

    - host: grafana.orion.local
      http:
        paths:
          - path: /(.*)
            pathType: Prefix
            backend:
              service:
                name: grafana
                port:
                  number: 3000

    - host: prometheus.orion.local
      http:
        paths:
          - path: /(.*)
            pathType: Prefix
            backend:
              service:
                name: prometheus
                port:
                  number: 9090
"@

Set-Content -Path "$root/ingress.yml" -Value $ingress

Write-Host "Ingress Kubernetes ORION généré avec succès."
