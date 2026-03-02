# Poly Mind

Personal ESLM corpus and digital legacy system built on eStream v0.8.3 and PolyKit v0.3.0. Status: Concept.

## Overview

Poly Mind is a personal AI that grows with the user. It ingests conversations, documents, notes, and cross-product data — building a private, encrypted, scatter-stored knowledge base using ESLM's persistent SSM state for incremental compound intelligence. All inference runs on-device. Digital legacy enables graduated transfer to designated guardians via K-of-N PQ threshold cryptography, with classification tiers unlocking over time and Incognito-level data auto-purging.

## Key Patterns

- **Zero-linkage**: HKDF context `poly-mind-v1`, lex `esn/global/org/polylabs/mind`, isolated StreamSight + metering + billing
- **Graph model**: `graph knowledge_corpus` (DocumentNode, ConceptNode, RelationshipNode) with CSR tiered storage
- **DAG model**: `dag legacy_governance` (GovernanceNode with guardian designation, TransferEdge with k-of-n threshold) — acyclic enforcement, ML-DSA-87 signed
- **State machines**: `ingestion_lifecycle` (PENDING → CLASSIFYING → INDEXED → ACTIVE → STALE → PURGED), `legacy_lifecycle` (ACTIVE → PENDING_VERIFICATION → GRADUATED_TRANSFER → ARCHIVED)
- **Overlays**: relevance_score, last_accessed_ns, citation_count, confidence (knowledge_corpus); guardian_status, transfer_phase (legacy_governance)
- **ai_feed**: corpus_insight on knowledge_corpus (ESLM-powered knowledge synthesis, pattern detection, contradiction surfacing)
- **Build**: FastLang `.fl` → ESCIR → Rust/WASM → `.escd`
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
| ESLM Engine | `crates/poly-mind-core/` | On-device inference with persistent SSM state |
| Ingestion Pipeline | `crates/ingestion/` | Ingest from Poly products + external sources |
| Knowledge Graph Engine | `crates/knowledge-graph/` | Entity/relationship/temporal graph operations |
| Desktop App | `apps/desktop/` | Tauri-based personal AI interface |
| Mobile App | `apps/mobile/` | React Native with Rust FFI |

## No REST API

All sync uses the eStream Wire Protocol (QUIC/UDP). No REST/HTTP endpoints. All inference is on-device.

## Pricing

| Tier | Corpus | Price |
|------|--------|-------|
| Premium | 1 GB (persistent SSM, scatter-sync) | $4.99/mo |
| Pro | 10 GB (+ external ingestion, API, cross-product) | $9.99/mo |
| Enterprise | Custom (+ team knowledge bases, RBAC) | Per-seat |

## Platform

- eStream v0.8.3
- PolyKit v0.3.0
- ESLM (eStream Small Language Model) with persistent SSM
- ML-KEM-1024, ML-DSA-87, SHA3-256
- 8-Dimension metering
- Blinded billing tokens

## Cross-Repo Coordination

This repo is part of the [polylabs-dev](https://github.com/polylabs-dev) organization, coordinated through the **AI Toolkit hub** at `toddrooke/ai-toolkit/`.

For cross-repo context, strategic priorities, and the master work queue:
- `toddrooke/ai-toolkit/CLAUDE-CONTEXT.md` — org map and priorities
- `toddrooke/ai-toolkit/scratch/BACKLOG.md` — master backlog
- `toddrooke/ai-toolkit/repos/polylabs-dev.md` — this org's status summary
