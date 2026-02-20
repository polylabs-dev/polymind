# Poly Mind Architecture

**Version**: 3.0
**Date**: February 2026
**Platform**: eStream v0.8.3
**Upstream**: PolyKit v0.3.0, eStream scatter-cas, graph/DAG constructs
**Build Pipeline**: FastLang (.fl) → ESCIR → Rust/WASM codegen → .escd
**Status**: Concept

---

## Overview

Poly Mind is a personal ESLM corpus and digital legacy system. It builds a private, encrypted, scatter-stored knowledge base from user interactions, documents, and cross-product ingestion — using ESLM's persistent SSM state for incremental compound intelligence that grows with the user. All inference runs on-device. Digital legacy enables graduated transfer to designated guardians via K-of-N PQ threshold cryptography, with classification tiers unlocking over time and Incognito-level data auto-purging.

### What Changed in v3.0

| Area | v2.0 | v3.0 |
|------|------|------|
| Knowledge model | Flat corpus index | `graph knowledge_corpus` with typed overlays |
| Legacy governance | Threshold config blob | `dag legacy_governance` with acyclic enforcement + ML-DSA-87 signing |
| Ingestion state | Implicit | `state_machine ingestion_lifecycle` with classification propagation |
| Circuit format | ESCIR YAML (`circuit.escir.yaml`) | FastLang `.fl` with PolyKit profiles |
| RBAC | Per-circuit annotations | eStream `rbac.fl` composed via PolyKit |
| Platform | eStream v0.8.1 | eStream v0.8.3 |

---

## Zero-Linkage Privacy

Poly Mind operates under the Poly Labs zero-linkage privacy architecture:

- **HKDF context**: `poly-mind-v1` — produces `user_id`, signing key, and encryption key that cannot be correlated with any other Poly product
- **Lex namespace**: `esn/global/org/polylabs/mind` — completely isolated from other product namespaces
- **StreamSight**: Telemetry stays within `polylabs.mind.*` lex paths
- **Metering**: Own `metering_graph` instance under `polylabs.mind.metering` lex
- **Billing**: Tier checked via blinded token status, not cross-product identity

---

## Identity & Authentication

### SPARK Derivation Context

```
SPARK biometric → Secure Enclave/TEE → master_seed (in WASM, never exposed to JS)
                                            │
                                            ▼
                                   HKDF-SHA3-256(master_seed, "poly-mind-v1")
                                            │
                                            ├── ML-DSA-87 signing key pair
                                            │   (corpus manifests, ingestion records, legacy governance)
                                            │
                                            └── ML-KEM-1024 encryption key pair
                                                (corpus encryption, guardian key shares, state checkpoints)
```

### User Identity

```
user_id = SHA3-256(spark_ml_dsa_87_public_key)[0..16]   # 16-byte truncated hash
```

All stream topics, corpus ownership, and legacy configuration reference this SPARK-derived `user_id`. There are no usernames, emails, or phone numbers. This `user_id` is unique to Poly Mind and cannot be linked to identities in other Poly products.

---

## Core Architecture

```
┌──────────────────────────────────────────────────────────────────────┐
│                      Poly Mind Client                                  │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐            │
│  │ Chat     │  │ Proactive│  │ Ingest   │  │ Legacy   │            │
│  │ Interface│  │ Insights │  │ Manager  │  │ Settings │            │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘            │
│       │              │              │              │                  │
│  ┌────┴──────────────┴──────────────┴──────────────┴──────────────┐ │
│  │  FastLang Circuits (WASM via .escd)                              │ │
│  │                                                                  │ │
│  │  polymind_ingest.fl │ polymind_query.fl │ polymind_legacy.fl     │ │
│  │  polymind_classify.fl │ polymind_metering.fl                     │ │
│  │  (all ML-DSA-87 signed .escd packages, StreamSight-annotated)   │ │
│  └──────────────────────────┬──────────────────────────────────────┘ │
│                              │                                        │
│  ┌──────────────────────────┴──────────────────────────────────────┐ │
│  │  Graph/DAG Layer (WASM, backed by scatter-cas)                    │ │
│  │                                                                   │ │
│  │  graph knowledge_corpus    — documents, concepts, relationships   │ │
│  │  dag   legacy_governance   — guardian governance, transfer edges  │ │
│  │  graph metering_graph      — per-app 8D usage (from PolyKit)     │ │
│  │  graph user_graph          — per-product identity (from PolyKit)  │ │
│  └──────────────────────────┬──────────────────────────────────────┘ │
│                              │                                        │
│  ┌──────────────────────────┴──────────────────────────────────────┐ │
│  │  ESLM Runtime (on-device)                                         │ │
│  │  Persistent SSM State │ Inference Engine │ Embedding Generator    │ │
│  │  Apple Neural Engine / Qualcomm NPU / AVX-512                    │ │
│  └──────────────────────────┬──────────────────────────────────────┘ │
│                              │                                        │
│  ┌──────────────────────────┴──────────────────────────────────────┐ │
│  │  Corpus Store (encrypted, local)                                   │ │
│  │  Vector Index │ Graph Index │ Temporal Index                      │ │
│  └──────────────────────────┬──────────────────────────────────────┘ │
│                              │                                        │
│  ┌──────────────────────────┴──────────────────────────────────────┐ │
│  │  eStream SDK (@estream/sdk-browser or react-native)               │ │
│  │  Wire protocol only: UDP :5000 / WebTransport :4433              │ │
│  └──────────────────────────┬──────────────────────────────────────┘ │
└──────────────────────────────┼───────────────────────────────────────┘
                               │
                        eStream Wire Protocol (QUIC/UDP) — sync only
                               │
┌──────────────────────────────┼───────────────────────────────────────┐
│                        eStream Network                                │
│                               │                                       │
│  ┌────────────────────────────┴────────────────────────────────────┐ │
│  │  Lattice-Hosted Circuits                                          │ │
│  │                                                                   │ │
│  │  polymind_state_sync.fl │ polymind_legacy_relay.fl                │ │
│  │  polymind_metering.fl   │ scatter-cas runtime                    │ │
│  └────┬───────────┬──────────────┬─────────────────────────────────┘ │
│       │           │              │                                    │
│  ┌────┴─────────────────────────────────────────────────────────┐   │
│  │              Scatter Storage Layer (via scatter-cas)             │   │
│  │  AWS │ GCP │ Azure │ Cloudflare │ Hetzner │ Self-host          │   │
│  └────────────────────────────────────────────────────────────────┘   │
└───────────────────────────────────────────────────────────────────────┘
```

All inference runs on-device. The network is used only for scatter-sync of encrypted SSM state checkpoints, corpus index shards, and legacy governance state. No queries, responses, or corpus content are ever sent to any server.

---

## ESLM: Persistent Intelligence

### How It Differs from Cloud LLMs

| Property | Cloud LLM (GPT, Claude) | ESLM (Poly Mind) |
|----------|------------------------|-------------------|
| Memory | Context window only | **Persistent SSM state** |
| Between sessions | Forgets everything | **Remembers everything** |
| Processing | Remote servers | **On-device** |
| Privacy | Provider sees all | **Zero-knowledge** |
| Ownership | Provider's model | **User's corpus** |
| Growth | Static | **Compounds over time** |
| Transfer | Cannot export knowledge | **Digital legacy (K-of-N)** |

### SSM State Management

```
Session 1: User discusses project Alpha
    → SSM state updated with Alpha context
    → State encrypted + scatter-stored as checkpoint

Session 2 (next day): User asks about Alpha
    → SSM state loaded from checkpoint
    → ESLM already knows Alpha context
    → Responds with continuity

Session 100: User asks "What decisions did I make about Alpha?"
    → SSM state + corpus graph retrieval
    → Full temporal awareness of Alpha's evolution
    → Synthesizes answer from accumulated knowledge
```

---

## Graph/DAG Constructs

### Knowledge Corpus Graph (`polymind_knowledge_graph.fl`)

The knowledge base is modeled as a typed graph. Documents, concepts, and relationships are nodes; references, derivations, and contradictions are edges. Overlays provide real-time corpus state (relevance, access patterns, confidence scores) without mutating the base graph.

```fastlang
type DocumentNode = struct {
    document_id: bytes(16),
    content_hash: bytes(32),
    classification: u8,
    source_product: u8,
    source_id: bytes(16),
    mime_type: u16,
    title_hash: bytes(32),
    embedding_hash: bytes(32),
    word_count: u32,
    ingested_at: u64,
    updated_at: u64,
}

type ConceptNode = struct {
    concept_id: bytes(16),
    embedding_hash: bytes(32),
    topic: bytes(256),
    entity_type: u8,
    first_seen_at: u64,
    last_updated_at: u64,
}

type RelationshipNode = struct {
    relationship_id: bytes(16),
    relationship_type: u8,
    subject_id: bytes(16),
    object_id: bytes(16),
    strength: u16,
    temporal_start: u64,
    temporal_end: u64,
}

type ReferencesEdge = struct {
    context: bytes(256),
    reference_type: u8,
    created_at: u64,
}

type DerivedFromEdge = struct {
    derivation_method: u8,
    confidence: u16,
    derived_at: u64,
}

type ContradictEdge = struct {
    contradiction_type: u8,
    severity: u8,
    detected_at: u64,
    resolved: u8,
}

graph knowledge_corpus {
    node DocumentNode
    node ConceptNode
    node RelationshipNode
    edge ReferencesEdge
    edge DerivedFromEdge
    edge ContradictEdge

    overlay relevance_score: u16 curate delta_curate
    overlay last_accessed_ns: u64 bitmask delta_curate
    overlay citation_count: u32 bitmask delta_curate
    overlay confidence: u16 curate delta_curate

    storage csr {
        hot @bram,
        warm @ddr,
        cold @nvme,
    }

    ai_feed corpus_insight

    observe knowledge_corpus: [relevance_score, citation_count, confidence] threshold: {
        anomaly_score 0.8
        baseline_window 300
    }
}

series corpus_series: knowledge_corpus
    merkle_chain true
    lattice_imprint true
    witness_attest true
```

The `ai_feed corpus_insight` drives ESLM-powered knowledge synthesis: detecting emerging patterns across documents, surfacing contradictions between ingested sources, and generating proactive insights. The `observe` block flags corpus anomalies (e.g., sudden relevance shifts, confidence degradation, contradictions accumulating in a topic area).

Key circuits: `ingest_document`, `create_concept`, `link_reference`, `detect_contradiction`, `query_corpus`, `update_relevance`, `prune_stale`.

### Legacy Governance DAG (`polymind_legacy_dag.fl`)

Digital legacy governance is modeled as a DAG. Guardian designations and transfer authorizations are nodes; threshold-gated transfer edges connect them. The DAG enforces acyclicity — a completed transfer phase cannot be rolled back. Classification tiers unlock over time, and Incognito-level data auto-purges on trigger.

```fastlang
type GovernanceNode = struct {
    governance_id: bytes(16),
    guardian_id: bytes(16),
    guardian_pubkey: bytes(2592),
    guardian_kem_pubkey: bytes(1568),
    designation_type: u8,
    share_index: u8,
    designated_at: u64,
    last_confirmed_at: u64,
}

type TransferEdge = struct {
    transfer_id: bytes(16),
    threshold_k: u8,
    threshold_n: u8,
    classification_tier: u8,
    trigger_type: u8,
    trigger_evidence_hash: bytes(32),
    initiated_at: u64,
    completed_at: u64,
    signatures_collected: u8,
}

type PurgeEdge = struct {
    purge_id: bytes(16),
    classification_tier: u8,
    purge_reason: u8,
    initiated_at: u64,
    confirmed_at: u64,
}

dag legacy_governance {
    node GovernanceNode
    edge TransferEdge
    edge PurgeEdge

    enforce acyclic
    sign ml_dsa_87

    overlay guardian_status: u8 curate delta_curate
    overlay transfer_phase: u8 curate delta_curate

    storage csr {
        hot @bram,
        warm @ddr,
        cold @nvme,
    }

    observe legacy_governance: [guardian_status, transfer_phase] threshold: {
        anomaly_score 0.95
        baseline_window 600
    }
}

series legacy_series: legacy_governance
    merkle_chain true
    lattice_imprint true
    witness_attest true
```

Key circuits: `designate_guardian`, `revoke_guardian`, `submit_trigger`, `collect_signature`, `unlock_tier`, `execute_purge`, `verify_alive_proof`.

### Ingestion Lifecycle State Machine (`polymind_ingestion_lifecycle.fl`)

Every ingested item follows a strict lifecycle with classification propagation and anomaly detection.

```fastlang
state_machine ingestion_lifecycle {
    initial PENDING
    persistence wal
    terminal [PURGED]
    li_anomaly_detection true

    PENDING -> CLASSIFYING when content_received guard source_authorized
    CLASSIFYING -> INDEXED when classification_assigned guard classification_valid
    CLASSIFYING -> REJECTED when classification_failed
    INDEXED -> ACTIVE when embedding_complete guard corpus_updated
    ACTIVE -> STALE when relevance_decayed guard decay_threshold_met
    ACTIVE -> ACTIVE when re_referenced guard relevance_refreshed
    ACTIVE -> PURGED when user_deleted
    ACTIVE -> PURGED when legacy_purge guard classification_ephemeral
    STALE -> ACTIVE when re_referenced guard relevance_refreshed
    STALE -> PURGED when prune_expired
}
```

State transitions update the `relevance_score` and `last_accessed_ns` overlays on `knowledge_corpus`. The `observe` block flags anomalies (e.g., mass ingestion from unexpected source, unusual classification distribution, rapid relevance decay).

### Legacy Transfer State Machine (`polymind_legacy_lifecycle.fl`)

```fastlang
state_machine legacy_lifecycle {
    initial ACTIVE
    persistence wal
    terminal [ARCHIVED, PARTIALLY_ARCHIVED]
    li_anomaly_detection true

    ACTIVE -> PENDING_VERIFICATION when trigger_submitted guard trigger_valid
    PENDING_VERIFICATION -> GRADUATED_TRANSFER when threshold_met guard k_of_n_signatures
    PENDING_VERIFICATION -> ACTIVE when alive_proof_submitted guard spark_verified
    GRADUATED_TRANSFER -> GRADUATED_TRANSFER when tier_unlocked guard schedule_reached
    GRADUATED_TRANSFER -> GRADUATED_TRANSFER when purge_executed guard classification_sovereign_or_ephemeral
    GRADUATED_TRANSFER -> ARCHIVED when transfer_complete guard all_tiers_processed
    GRADUATED_TRANSFER -> PARTIALLY_ARCHIVED when purge_complete guard sovereign_purged
}
```

---

## Ingestion Pipeline

### Sources

```
Poly Products (with user consent):
├── Poly Messenger: Conversation summaries
├── Poly Mail: Email headers + body
├── Poly Data: Document content
├── Poly Calendar: Events, meetings, notes
└── Poly Pass: (never ingested — too sensitive)

Manual Input:
├── Journal entries
├── Notes
├── Voice memos (transcribed on-device)
└── Explicit "remember this" commands

External (optional, Pro tier):
├── Browser bookmarks / highlights
├── Photos (ESLM captioned on-device)
├── Imported documents
└── RSS / newsletter digests
```

Each ingested item retains the classification of its source product. Poly Messenger content inherits its conversation's classification. Poly Mail content is classified based on sensitivity analysis. Users can override classification upward (more restricted) but not downward.

### Processing

```
Raw Content
    │
    ▼
Classification Assignment
    │  (PUBLIC, INTERNAL, SENSITIVE, RESTRICTED, SOVEREIGN, EPHEMERAL)
    │
    ▼
ESLM Embedding (on-device)
    │  Vector representation for similarity search
    │
    ▼
Knowledge Graph Update
    │  DocumentNode + ConceptNode + edges in knowledge_corpus
    │
    ▼
SSM State Integration
    │  Persistent state update (incremental, not full retrain)
    │
    ▼
Corpus Store (encrypted)
    │
    ▼
Scatter Sync (encrypted checkpoints)
```

---

## Digital Legacy

### Configuration

```yaml
legacy:
  guardians:
    - spark:did:spouse
    - spark:did:attorney
    - spark:did:pastor
    - spark:did:sibling
    - spark:did:trusted_friend

  threshold: 3-of-5

  trigger:
    - death_certificate
    - court_order
    - inactivity: 12_months

  transfer_schedule:
    day_0:
      unlock: [PUBLIC, INTERNAL]
      description: "Personal notes, general knowledge, preferences"

    day_30:
      unlock: [SENSITIVE]
      description: "Financial records, business documents"

    day_90:
      unlock: [RESTRICTED]
      description: "Private conversations, health records"

    never:
      purge: [SOVEREIGN, EPHEMERAL]
      description: "Auto-purged on trigger event"
      note: "Incognito-related data always purged"
```

### PQ Threshold Cryptography

```
User's corpus key: K

Shamir Secret Sharing (lattice-based, PQ-safe):
    K → (S1, S2, S3, S4, S5)    # 5 shares
    Any 3 shares → reconstruct K  # 3-of-5 threshold

Each share encrypted with guardian's ML-KEM-1024 key:
    E(S1, spouse_pub)
    E(S2, attorney_pub)
    E(S3, pastor_pub)
    E(S4, sibling_pub)
    E(S5, friend_pub)

Shares scatter-stored (not held by guardians until trigger)
```

### Graduated Transfer Process

```
Trigger Event Submitted
    │
    ▼
3-of-5 Guardians sign trigger verification
    │  (ML-DSA-87 threshold signatures via legacy_governance DAG)
    │
    ▼
Verify trigger document (death certificate, etc.)
    │
    ▼
Day 0: PUBLIC + INTERNAL corpus unlocked
    │  Guardians receive decrypted general knowledge
    │  transfer_phase overlay → TIER_0_UNLOCKED
    │
    ▼
Day 30: SENSITIVE corpus unlocked
    │  Financial, business content accessible
    │  transfer_phase overlay → TIER_1_UNLOCKED
    │
    ▼
Day 90: RESTRICTED corpus unlocked
    │  Private, personal content accessible
    │  transfer_phase overlay → TIER_2_UNLOCKED
    │
    ▼
SOVEREIGN + EPHEMERAL: Auto-purged
    Scatter storage operators receive purge commands via PurgeEdge
    All Incognito-related data destroyed
    transfer_phase overlay → PURGE_COMPLETE
```

---

## Privacy Model

### What Poly Labs Cannot Know

| Aspect | Protection |
|--------|-----------|
| Queries | On-device inference; never sent anywhere |
| Responses | Generated locally; never logged |
| Corpus content | PQ-encrypted; scatter-stored |
| SSM state | PQ-encrypted checkpoints |
| Knowledge graph | Encrypted; local + scatter |
| Legacy config | Encrypted; user-controlled |
| Usage patterns | No telemetry on Mind interactions |

### Incognito Integration

For users in oppressive environments:
- Poly Mind can be hidden (no visible app presence)
- Corpus encrypted with deniable encryption (hidden volume)
- Traffic mimicry for state sync
- Emergency purge: SPARK biometric + panic gesture = instant wipe
- Classification EPHEMERAL for in-session-only knowledge (auto-purges on legacy trigger)

---

## scatter-cas Integration

Poly Mind builds on eStream's `scatter-cas` runtime for corpus and state storage. Classification-driven k-of-n erasure coding distributes encrypted data across providers.

### Storage Layers

```
scatter-cas ObjectStore
  ├── PackStore      (local ESLite, offline cache — primary for on-device inference)
  └── ScatterStore   (distributed k-of-n erasure coded — sync + backup)
        ├── k-of-n scatter per data classification:
        │   PUBLIC:        3-of-5, 2+ jurisdictions
        │   INTERNAL:      3-of-5, 2+ jurisdictions
        │   SENSITIVE:     5-of-7, 3+ jurisdictions
        │   RESTRICTED:    7-of-9, 3+ jurisdictions
        │   SOVEREIGN:     user-hosted only (never on shared infrastructure)
        │   EPHEMERAL:     local only, no scatter (in-session, auto-purge)
        └── Providers: AWS, GCP, Azure, Cloudflare, Hetzner, self-host
```

---

## FastLang Circuits

All circuits are written in FastLang `.fl` using PolyKit profiles. The build pipeline is:

```bash
estream-dev build-wasm-client --from-fl circuits/fl/ --sign key.pem --enforce-budget
```

### Client-Side Circuits (compiled to `.escd` WASM)

| Circuit | File | Purpose | Size Budget |
|---------|------|---------|-------------|
| `polymind_ingest` | `polymind_ingest.fl` | Content ingestion, classification assignment, embedding | ≤256 KB |
| `polymind_query` | `polymind_query.fl` | Corpus query, vector search, graph traversal | ≤256 KB |
| `polymind_classify` | `polymind_classify.fl` | Classification propagation, sensitivity analysis | ≤128 KB |
| `polymind_legacy` | `polymind_legacy.fl` | Guardian designation, trigger verification, tier unlock | ≤128 KB |
| `polymind_insight` | `polymind_insight.fl` | Proactive insight generation, contradiction detection | ≤256 KB |

All circuits compose PolyKit:
```fastlang
circuit polymind_ingest(user_id: bytes(16), content: bytes, source: u8, classification: u8) -> bytes(16)
    profile poly_framework_sensitive
    composes: [polykit_identity, polykit_metering, polykit_sanitize]
    lex esn/global/org/polylabs/mind/ingest
    constant_time false
    observe metrics: [ingest_ops, corpus_size_bytes, latency_ns]
{
    classify_and_embed(content, source, classification)
}
```

### Server-Side Circuits (lattice-hosted)

| Circuit | File | Purpose |
|---------|------|---------|
| `polymind_state_sync` | `polymind_state_sync.fl` | SSM checkpoint scatter policy enforcement, cross-device sync |
| `polymind_legacy_relay` | `polymind_legacy_relay.fl` | Guardian notification relay, trigger verification relay |
| `polymind_metering` | `polymind_metering.fl` | Per-product 8D metering (isolated) |

---

## Hardware Requirements

### Minimum

| Platform | Requirements |
|----------|-------------|
| iPhone | iPhone 15+ (Neural Engine) |
| Android | 8GB+ RAM, Snapdragon 8 Gen 2+ (Hexagon NPU) |
| Mac | Apple Silicon (M1+) |
| Windows | 16GB RAM, AVX-512 support |
| Linux | 16GB RAM, AVX-512 or CUDA |

### ESLM Model

- Size: ~2–7B parameters (INT4/INT8 quantized)
- RAM: 2–4GB for inference
- Storage: ~2–4GB for model weights
- Inference: 20–50 tokens/second on Apple M2

---

## StreamSight Observability

Per-product isolated telemetry within the `polylabs.mind.*` lex namespace.

### Telemetry Stream Paths

```
lex://estream/apps/polylabs.mind/telemetry
lex://estream/apps/polylabs.mind/telemetry/sli
lex://estream/apps/polylabs.mind/metrics/baseline
lex://estream/apps/polylabs.mind/metrics/deviations
lex://estream/apps/polylabs.mind/incidents
lex://estream/apps/polylabs.mind/eslm/corpus_insight
```

No telemetry path references any other Poly product. StreamSight baseline gate learns per-operation latency distributions and flags deviations. Mind-specific telemetry is minimized — no content or query telemetry, only operational metrics (sync latency, corpus size, ingestion rate).

---

## Console Widgets

| Widget ID | Category | Description |
|-----------|----------|-------------|
| `polymind-corpus-health` | observability | Corpus size, document count, classification distribution |
| `polymind-sync-latency` | observability | SSM checkpoint sync latency gauge |
| `polymind-ingestion-rate` | observability | Ingestion pipeline throughput and status |
| `polymind-deviation-feed` | observability | StreamSight baseline deviation feed |
| `polymind-corpus-insight` | observability | ESLM corpus_insight synthesis feed |
| `polymind-legacy-status` | governance | Guardian status, transfer phase, alive-proof recency |
| `polymind-classification-dist` | governance | Classification tier distribution across corpus |

---

## Enterprise

### Team Knowledge Bases

Enterprise tier enables shared team knowledge bases where organizational knowledge is collaboratively built:

- Each team member's contributions are individually signed (ML-DSA-87)
- Team corpus has its own `knowledge_corpus` graph instance
- RBAC controls who can ingest, query, and administer the team corpus
- Individual personal corpora remain completely separate (zero-linkage preserved)

### Lex Bridge (Opt-In)

Enterprise admins can opt-in to cross-product visibility via an explicit lex bridge between `esn/global/org/polylabs/mind` and the enterprise admin namespace. The bridge is gated by **k-of-n admin witness attestation** and is revocable.

```
Enterprise admin namespace ←──lex bridge──→ polylabs.mind.{org_id}.*
                              │
                              └── gated by k-of-n witness attestation
                              └── org-level aggregates only (no individual corpus content)
                              └── revocable
```

Even with the bridge, individual user corpus content and queries are never exposed — only org-level aggregates (team corpus size, ingestion rates, classification compliance) flow across the bridge.

---

## Pricing

| Tier | Corpus | Features | Price |
|------|--------|----------|-------|
| Premium | 1 GB | Persistent SSM, conversational memory, scatter-sync, classification | $4.99/mo |
| Pro | 10 GB | + External ingestion, API access, cross-product integration, proactive insights | $9.99/mo |
| Enterprise | Custom | + Team knowledge bases, admin console, RBAC, compliance | Per-seat |

Tier enforcement via PolyKit `metering_graph` + `subscription_lifecycle` state machine. Billing uses blinded payment tokens — backend cannot correlate which SPARK identity subscribes to which tier.

---

## Directory Structure

```
polymind/
├── circuits/fl/
│   ├── polymind_ingest.fl
│   ├── polymind_query.fl
│   ├── polymind_classify.fl
│   ├── polymind_legacy.fl
│   ├── polymind_insight.fl
│   ├── polymind_state_sync.fl
│   ├── polymind_legacy_relay.fl
│   ├── polymind_metering.fl
│   └── graphs/
│       ├── polymind_knowledge_graph.fl
│       └── polymind_legacy_dag.fl
├── crates/
│   ├── poly-mind-core/
│   ├── ingestion/
│   └── knowledge-graph/
├── apps/
│   ├── desktop/            Tauri-based personal AI interface
│   └── mobile/             React Native with Rust FFI
├── packages/
│   └── sdk/
├── apps/console/
│   └── src/widgets/
├── docs/
│   └── ARCHITECTURE.md
├── CLAUDE.md
└── package.json
```

---

## Roadmap

### Phase 1: Foundation (Q1 2027)
- `knowledge_corpus` graph with typed overlays
- `ingestion_lifecycle` state machine
- Basic personal corpus (text only, manual input)
- Conversational memory with persistent SSM
- Classification-aware storage
- Scatter-stored state sync
- Desktop app (Tauri)
- StreamSight L0 metrics
- Premium tier

### Phase 2: Intelligence (Q2–Q3 2027)
- Cross-product integration (Mail, Data, Messenger — with consent)
- `ai_feed corpus_insight` for proactive synthesis
- `polymind_insight.fl` contradiction detection
- External ingestion sources (Pro tier)
- Mobile apps (iOS, Android)
- API access for Pro tier
- Console widgets (7 widgets)

### Phase 3: Legacy (Q4 2027)
- `legacy_governance` DAG with ML-DSA-87 signing
- `legacy_lifecycle` state machine
- K-of-N guardian designation and governance
- Graduated transfer with classification tiers
- SOVEREIGN/EPHEMERAL auto-purge
- Emergency purge (panic gesture)
- Guardian management UI

### Phase 4: Compound (2028+)
- Voice and image ingestion (on-device transcription/captioning)
- Team knowledge bases (Enterprise tier)
- Lex bridge for enterprise cross-product visibility (opt-in, k-of-n gated)
- Poly Vault HSM integration (SOVEREIGN tier corpus keys in hardware)
- FPGA-accelerated on-device inference
- ESN-AI optimization recommendations

---

## Related Documents

- [polylabs/business/POLY_MIND_CONCEPT.md] — Detailed concept document
- [polylabs/business/PRODUCT_FAMILY.md] — Product specifications
- [polylabs/business/STRATEGY.md] — Overall strategy
