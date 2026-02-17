# Poly Mind

Personal ESLM corpus and digital legacy built on eStream v0.8.1.

## Overview

Poly Mind is a personal AI that grows with the user. It ingests conversations, documents, notes, and emails -- building a private, encrypted, scatter-stored knowledge base that uses ESLM's persistent SSM state for incremental compound intelligence. Digital legacy transfer via K-of-N PQ threshold cryptography.

## Architecture

```
User Query
    |
    v
ESLM Inference (on-device)
    |
    +-- Persistent SSM State (encrypted, scatter-synced)
    +-- Personal Corpus Index (encrypted, local-first)
    +-- Classification Filter (prevents restricted data leaking)
    +-- Cross-Product Integration (Mail, Data, Messenger, Calendar)
    |
    v
Response (never leaves device)
```

## Key Components

| Component | Location | Purpose |
|-----------|----------|---------|
| ESLM Engine | crates/poly-mind-core/ | On-device inference with persistent state |
| Ingestion Pipeline | crates/ingestion/ | Ingest from Poly products + external sources |
| Knowledge Graph | crates/knowledge-graph/ | Entity/relationship/temporal graph |
| Legacy Governance | circuits/ | K-of-N digital legacy with graduated transfer |
| Desktop App | apps/desktop/ | Tauri-based personal AI interface |
| Mobile App | apps/mobile/ | React Native with Rust FFI |

## Key Concepts

- **Persistent SSM State**: Unlike cloud LLMs that forget between sessions, ESLM maintains persistent state that compounds over time
- **Classification-Aware**: Every ingested piece inherits a classification (PUBLIC through SOVEREIGN) that controls when/how it can be referenced
- **Digital Legacy**: K-of-N guardian governance for graduated transfer after death/incapacity
- **Zero-Knowledge**: All inference on-device; no queries, responses, or corpus data sent to any server

## The Ultimate Moat

Every day a user interacts with Poly Mind, the switching cost increases. After years of use, the accumulated personal knowledge graph is irreplaceable.

## Platform

- eStream v0.8.1
- ESLM (eStream Small Language Model) with persistent SSM
- ESCIR SmartCircuits
- ML-KEM-1024, ML-DSA-87, SHA3-256
- 8-Dimension metering
