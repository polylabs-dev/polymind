# Poly Mind

Personal ESLM corpus and digital legacy system built on eStream v0.22.0 and PolyKit v0.3.0. 100% FastLang. No hand-written Rust. Status: Active.

## Overview

Poly Mind is a personal AI that grows with the user. It ingests conversations, documents, notes, and cross-product data — building a private, encrypted, scatter-stored knowledge base using ESLM's persistent SSM state for incremental compound intelligence. All inference runs on-device. Digital legacy enables graduated transfer to designated guardians via K-of-N PQ threshold cryptography, with classification tiers unlocking over time and Incognito-level data auto-purging.

## Key Patterns

- **Zero-linkage**: HKDF context `poly-mind-v1`, lex `esn/global/org/polylabs/mind`, isolated StreamSight + metering + billing
- **Graph model**: `graph knowledge_corpus` (DocumentNode, ConceptNode, RelationshipNode) with CSR tiered storage
- **DAG model**: `dag legacy_governance` (GovernanceNode with guardian designation, TransferEdge with k-of-n threshold) — acyclic enforcement, ML-DSA-87 signed
- **State machines**: `ingestion_lifecycle` (PENDING → CLASSIFYING → INDEXED → ACTIVE → STALE → PURGED), `legacy_lifecycle` (ACTIVE → PENDING_VERIFICATION → GRADUATED_TRANSFER → ARCHIVED)
- **Overlays**: relevance_score, last_accessed_ns, citation_count, confidence (knowledge_corpus); guardian_status, transfer_phase (legacy_governance)
- **ai_feed**: corpus_insight on knowledge_corpus (ESLM-powered knowledge synthesis, pattern detection, contradiction surfacing)
- **Build**: FastLang `.fl` → FLIR → Rust/WASM → `.escd`
- **RBAC**: eStream `rbac.fl` composed via PolyKit profiles
- **On-device**: All inference local; network used only for scatter-sync of encrypted state

## Architecture

See `docs/ARCHITECTURE.md` for full specification including graph/DAG constructs, FastLang circuits, ESLM persistent SSM, digital legacy governance, and ingestion pipeline design.

## Key Components

| Component | Location | Purpose |
|-----------|----------|---------|
| Knowledge Graph | `circuits/fl/graphs/polymind_knowledge_graph.fl` | Document/concept/relationship corpus as typed graph |
| Legacy DAG | `circuits/fl/graphs/polymind_legacy_dag.fl` | Guardian governance with k-of-n threshold transfer |
| Ingest Circuit | `circuits/fl/polymind_ingest.fl` | Content ingestion, classification, embedding |
| Query Circuit | `circuits/fl/polymind_query.fl` | Corpus query, vector search, graph traversal |
| Classify Circuit | `circuits/fl/polymind_classify.fl` | Classification propagation, sensitivity analysis |
| Legacy Circuit | `circuits/fl/polymind_legacy.fl` | Guardian designation, trigger verification, tier unlock |
| Insight Circuit | `circuits/fl/polymind_insight.fl` | Proactive synthesis, contradiction detection |
| Metering Circuit | `circuits/fl/polymind_metering.fl` | Per-product 8D metering (isolated) |
| Platform Health | `circuits/fl/polymind_platform_health.fl` | Blind relay telemetry for corpus operations |
| ESLM Circuit | `circuits/fl/polymind_eslm.fl` | ESLM model management, checkpoints, fine-tuning, inference |
| Guardian Transfer | `circuits/fl/polymind_guardian.fl` | K-of-N graduated digital legacy transfer |
| RBAC Circuit | `circuits/fl/polymind_rbac.fl` | Role-based access control graph for enterprise corpora |

> **Note**: `crates/` is legacy scaffolding superseded by FLIR codegen. All logic lives in FastLang circuits.

## No REST API

All sync uses the eStream Wire Protocol (QUIC/UDP). No REST/HTTP endpoints. All inference is on-device.

## Pricing

| Tier | Corpus | Price |
|------|--------|-------|
| Premium | 1 GB (persistent SSM, scatter-sync) | $4.99/mo |
| Pro | 10 GB (+ external ingestion, API, cross-product) | $9.99/mo |
| Enterprise | Custom (+ team knowledge bases, RBAC) | Per-seat |

## Platform

- eStream v0.22.0
- PolyKit v0.3.0
- ESLM (eStream Small Language Model) with persistent SSM
- FLIR codegen (FastLang → FLIR → Rust/WASM)
- ML-KEM-1024, ML-DSA-87, SHA3-256
- 8-Dimension metering
- Blinded billing tokens

## Commit Convention

Commit to the GitHub issue or epic the work was done under.
