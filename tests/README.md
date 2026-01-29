# ORION Integration Tests

Suite de tests pour valider le bon fonctionnement du moteur CDR unifié ORION.

## Scripts de test disponibles

### 1. Smoke Test (`smoke_test.sh`)
Test rapide qui vérifie que tous les conteneurs sont en cours d'exécution.

```bash
cd tests
./smoke_test.sh
```

**Vérifie :**
- Tous les conteneurs Docker sont running
- Infrastructure (Kafka, ScyllaDB, MinIO, Prometheus, Grafana)
- Tous les microservices ORION

### 2. Integration Test (`integration_test.sh`)
Test complet qui valide l'ensemble du pipeline CDR.

```bash
cd tests
./integration_test.sh
```

**Vérifie :**
- Santé de l'infrastructure (Kafka topics, ScyllaDB, MinIO)
- Health endpoints de tous les services
- Métriques Prometheus
- Génération de CDR
- Pipeline end-to-end (ingestion → validation → normalisation → enrichissement)
- Service de détection de fraude

### 3. Load Test (`load_test.sh`)
Test de charge qui génère un volume important de CDR.

```bash
cd tests
# Générer 1000 CDR avec intervalle de 10ms
./load_test.sh 1000 10

# Générer 10000 CDR avec intervalle de 5ms
./load_test.sh 10000 5
```

**Effectue :**
- Génération massive de CDR
- Distribution entre plusieurs pays
- Affichage des statistiques du pipeline

## Prérequis

1. Tous les services ORION doivent être démarrés :
```bash
docker compose up -d
```

2. Attendre que les services soient healthy (environ 30-60 secondes)

3. Vérifier que les ports sont accessibles :
   - 8081: orion-ingestion
   - 8082: orion-validation
   - 8083: orion-normalization
   - 8084: orion-enrichment
   - 8090: orion-ml-fraud-agent
   - 9200: orion-traffic-generator
   - 9100: orion-observability
   - 9090: Prometheus
   - 3000: Grafana

## Exécution des tests

### Test complet (recommandé)
```bash
cd tests
chmod +x *.sh
./smoke_test.sh && ./integration_test.sh
```

### Test de charge
```bash
cd tests
./load_test.sh 5000 20
```

## Interprétation des résultats

### Smoke Test
- ✓ (vert) : Service running
- ✗ (rouge) : Service arrêté ou en erreur

### Integration Test
- **Passed** : Test réussi
- **Failed** : Test échoué (vérifier les logs)
- **Warning** : Test non critique ou besoin de plus de temps

### Load Test
- Affiche le nombre de CDR traités à chaque étape
- Compare avec le nombre généré pour détecter les pertes

## Troubleshooting

### Services not healthy
```bash
docker compose ps
docker logs <service-name>
```

### Tests failing
1. Vérifier que tous les services sont up :
```bash
docker compose ps
```

2. Vérifier les logs du service en erreur :
```bash
docker logs orion-<service-name>
```

3. Redémarrer les services si nécessaire :
```bash
docker compose restart
```

### Kafka topics not created
Les topics sont créés automatiquement lors du premier message. 
Pour les créer manuellement :
```bash
docker exec orion-kafka kafka-topics --create \
  --bootstrap-server localhost:9092 \
  --topic cdr.raw.FR \
  --partitions 3 \
  --replication-factor 1
```

## Métriques et monitoring

Accéder aux interfaces :
- **Prometheus** : http://localhost:9090
- **Grafana** : http://localhost:3000 (admin/admin)
- **Metrics endpoints** : http://localhost:808X/metrics

## CI/CD Integration

Ces tests peuvent être intégrés dans un pipeline CI/CD :

```yaml
test:
  script:
    - docker compose up -d
    - sleep 60  # Attendre que les services soient prêts
    - cd tests && ./smoke_test.sh
    - cd tests && ./integration_test.sh
    - cd tests && ./load_test.sh 1000 10
```

## Contribution

Pour ajouter de nouveaux tests :
1. Créer un nouveau script dans `tests/`
2. Suivre le format des scripts existants
3. Utiliser les fonctions helpers (log_info, log_success, log_error)
4. Documenter dans ce README
