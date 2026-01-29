# ADR-001 : Rust Only (V1)

## Statut
✅ **Accepté** — V1 implémentée

## Contexte
Pour le prototype ORION V1, nous devons choisir le langage principal du pipeline CDR.

## Décision
**La V1 d'ORION est 100% Rust** pour l'ensemble du pipeline :
- Ingestion Kafka
- Validation & normalisation
- Enrichissement
- ML fraud agent
- Storage hot/cold
- API interne
- Observabilité

## Motivations

### ✅ Avantages

#### Performance CPU
- Latence ultra-faible (< 1 ms per CDR)
- Zero-cost abstractions
- Pas de GC (vs Java/Go)
- Optimisations compilateur agressives

#### Sécurité mémoire
- Memory-safe par design
- Pas de null pointer dereferences
- Pas de data races
- Ownership model garantit la sûreté

#### Cohérence du prototype
- Un seul langage = moins de complexité
- Stack homogène = build/deploy simplifiés
- Équipe focalisée sur une technologie

#### Démonstration claire
- Showcase des capacités Rust pour télécom
- Argument fort pour adoption en production
- Démonstration cohérente du prototype

### ⚠️ Inconvénients

- Courbe d'apprentissage Rust
- Moins de librairies I/O vs Go/Python
- Compilation plus lente que Go
- Recrutement plus difficile

## Alternatives considérées

### Option 1 : Rust + Python (rejetée)
- ❌ Python trop lent pour le pipeline
- ❌ Overhead sérialisation Rust ↔ Python
- ✅ Python OK pour ML training (hors pipeline)

### Option 2 : Rust + Go (retenue pour V2)
- ✅ Go excellent pour I/O-bound (API gateway, orchestrateur)
- ✅ Go plus simple à recruter
- ⚠️ Ajoute complexité en V1 (éviter pour prototype)
- **→ Retenu pour V2**

### Option 3 : Go only (rejetée)
- ❌ Moins performant que Rust pour compute-bound
- ❌ GC peut causer latence unpredictable
- ❌ Moins sécurisé (nil pointers, data races possibles)

### Option 4 : Java/Scala (rejetée)
- ❌ JVM overhead
- ❌ GC pauses inacceptables pour temps réel
- ❌ Empreinte mémoire élevée

## Conséquences

### Positives
- ✅ Pipeline ultra-performant
- ✅ Sécurité garantie
- ✅ Prototype cohérent et démontrable
- ✅ Base solide pour V2

### Négatives
- ⚠️ Formation Rust nécessaire
- ⚠️ Temps de build plus long
- ⚠️ Quelques libs manquantes (compensé par qualité)

## Évolution
**V2** : Introduction de Go pour services I/O-bound :
- API gateway publique
- Orchestrateur
- Storage gateways
- Connecteurs legacy

**Pipeline core restera Rust** (compute-bound).

## Références
- [Why Rust?](https://www.rust-lang.org/)
- Benchmarks Rust vs Go vs Java (internes)
- [architecture-globale.md](../architecture-globale.md)

---

**Date** : Décembre 2025  
**Auteur** : Architecture Team  
**Reviewers** : CTO, Lead Architects