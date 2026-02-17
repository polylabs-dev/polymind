# Poly Mind Architecture

**Version**: 1.0
**Last Updated**: February 2026
**Platform**: eStream v0.8.1
**Status**: Concept

---

## Overview

Poly Mind is a personal AI that builds a private, encrypted, scatter-stored knowledge corpus using ESLM's persistent SSM state. All inference runs on-device. The corpus is scatter-synced across devices for continuity. Digital legacy enables graduated transfer to designated guardians via PQ threshold cryptography.

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Poly Mind Client                           │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │ Chat     │  │ Proactive│  │ Ingest   │  │ Legacy   │   │
│  │ Interface│  │ Insights │  │ Manager  │  │ Settings │   │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘   │
│       │              │              │              │         │
│  ┌────┴──────────────┴──────────────┴──────────────┴─────┐  │
│  │              poly-mind-core (Rust)                      │  │
│  │  ESLM Engine | Knowledge Graph | Classification        │  │
│  └──────────────────────┬────────────────────────────────┘  │
│                          │                                   │
│  ┌──────────────────────┴────────────────────────────────┐  │
│  │              ESLM Runtime                              │  │
│  │  SSM State (persistent) | Inference (on-device)       │  │
│  │  Apple Neural Engine / Qualcomm NPU / AVX-512         │  │
│  └──────────────────────┬────────────────────────────────┘  │
│                          │                                   │
│  ┌──────────────────────┴────────────────────────────────┐  │
│  │              Corpus Store (encrypted, local)           │  │
│  │  Vector Index | Graph Index | Temporal Index          │  │
│  └──────────────────────┬────────────────────────────────┘  │
└─────────────────────────┼───────────────────────────────────┘
                          │
                   eStream Wire Protocol (sync only)
                          │
              Scatter Storage (SSM checkpoints, corpus index)
```

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
    -> SSM state updated with Alpha context
    -> State encrypted + scatter-stored as checkpoint

Session 2 (next day): User asks about Alpha
    -> SSM state loaded from checkpoint
    -> ESLM already knows Alpha context
    -> Responds with continuity

Session 100: User asks "What decisions did I make about Alpha?"
    -> SSM state + corpus index retrieval
    -> Full temporal awareness of Alpha's evolution
    -> Synthesizes answer from accumulated knowledge
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
└── Poly Pass: (never ingested -- too sensitive)

Manual Input:
├── Journal entries
├── Notes
├── Voice memos (transcribed)
└── Explicit "remember this" commands

External (optional):
├── Browser bookmarks / highlights
├── Photos (ESLM captioned)
├── Imported documents
└── RSS / newsletter digests
```

### Processing

```
Raw Content
    |
    v
Classification Assignment
    |  (PUBLIC, INTERNAL, SENSITIVE, RESTRICTED, SOVEREIGN, EPHEMERAL)
    |
    v
ESLM Embedding (on-device)
    |  Vector representation for similarity search
    |
    v
Knowledge Graph Update
    |  Entities, relationships, temporal links
    |
    v
SSM State Integration
    |  Persistent state update
    |
    v
Corpus Store (encrypted)
    |
    v
Scatter Sync (encrypted checkpoints)
```

---

## Knowledge Graph

```
Entities:
├── People (name, relationship, context, last interaction)
├── Projects (status, deadlines, dependencies, decisions)
├── Organizations (role, contacts, context)
├── Topics (knowledge areas, expertise level)
├── Decisions (what, when, why, outcome, revisit date)
├── Preferences (food, travel, tools, workflows)
├── Health (RESTRICTED by default)
├── Financial (RESTRICTED by default)
└── Temporal Events (what happened when)

Relationships:
├── Person -> Project (role, contribution)
├── Person -> Person (relationship type)
├── Project -> Decision (when, context)
├── Decision -> Outcome (result, satisfaction)
└── Topic -> Topic (related, prerequisite, conflicts)
```

---

## Digital Legacy

### Configuration

```yaml
legacy:
  guardians:
    - spark:did:spouse        # Guardian 1
    - spark:did:attorney      # Guardian 2
    - spark:did:pastor        # Guardian 3
    - spark:did:sibling       # Guardian 4
    - spark:did:trusted_friend # Guardian 5
  
  threshold: 3-of-5          # K-of-N requirement
  
  trigger:
    - death_certificate       # Verified document
    - court_order             # Legal authority
    - inactivity: 12_months   # Automatic after 12 months no auth
  
  transfer_schedule:
    day_0:                    # Trigger verified
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
    K -> (S1, S2, S3, S4, S5)    # 5 shares
    Any 3 shares -> reconstruct K  # 3-of-5 threshold

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
    |
    v
3-of-5 Guardians sign trigger verification
    |  (ML-DSA-87 threshold signatures)
    |
    v
Verify trigger document (death certificate, etc.)
    |
    v
Day 0: PUBLIC + INTERNAL corpus unlocked
    |  Guardians receive decrypted general knowledge
    |
    v
Day 30: SENSITIVE corpus unlocked
    |  Financial, business content accessible
    |
    v
Day 90: RESTRICTED corpus unlocked
    |  Private, personal content accessible
    |
    v
SOVEREIGN + EPHEMERAL: Auto-purged
    Scatter storage operators receive purge commands
    All Incognito-related data destroyed
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
- Classification EPHEMERAL for in-session-only knowledge

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

- Size: ~2-7B parameters (INT4/INT8 quantized)
- RAM: 2-4GB for inference
- Storage: ~2-4GB for model weights
- Inference: 20-50 tokens/second on Apple M2

---

## ESCIR Circuits

### Ingestion Circuit

```yaml
escir: "0.8.1"
name: poly-mind-ingest
version: "1.0.0"
lex: polylabs.mind

stream:
  - topic: "polylabs.mind.{user_id}.ingest"
    pattern: scatter
    retention: permanent
    hash_chain: true
    signature_required: true

  - topic: "polylabs.mind.{user_id}.sync.state"
    pattern: scatter
    retention: permanent
    signature_required: true

  - topic: "polylabs.mind.{user_id}.sync.corpus"
    pattern: scatter
    retention: permanent
    signature_required: true
```

### Legacy Governance Circuit

```yaml
escir: "0.8.1"
name: poly-mind-legacy
version: "1.0.0"
lex: polylabs.mind.legacy

stream:
  - topic: "polylabs.mind.{user_id}.legacy.trigger"
    pattern: two_phase
    two_phase: true
    signature_required: true

  - topic: "polylabs.mind.{user_id}.legacy.unlock.{tier}"
    pattern: scatter
    retention: ephemeral
    signature_required: true

fsm:
  initial_state: active
  states:
    active:
      transitions:
        - event: trigger_submitted
          target: pending_verification
    pending_verification:
      transitions:
        - event: threshold_met
          target: graduated_transfer
        - event: user_alive_proof
          target: active
    graduated_transfer:
      transitions:
        - event: tier_unlocked
          target: graduated_transfer
        - event: transfer_complete
          target: archived
        - event: purge_complete
          target: partially_archived
```

---

## Roadmap

### Phase 1: Foundation (Q1 2027)
- Basic personal corpus (text only)
- Conversational memory with persistent SSM
- Classification-aware storage
- Scatter-stored state sync
- Desktop app (Tauri)

### Phase 2: Intelligence (Q2-Q3 2027)
- Cross-product integration (Mail, Data, Messenger)
- Knowledge graph construction
- Proactive insights
- Mobile apps

### Phase 3: Legacy (Q4 2027)
- Digital legacy (K-of-N threshold)
- Graduated transfer
- Guardian management
- Emergency purge

### Phase 4: Compound (2028+)
- Voice and image ingestion
- Team knowledge bases (enterprise)
- Poly Vault integration (SOVEREIGN tier)
- FPGA-accelerated inference

---

## Related Documents

- [polylabs/business/POLY_MIND_CONCEPT.md] -- Detailed concept document
- [polylabs/business/PRODUCT_FAMILY.md] -- Product specifications
- [polylabs/business/STRATEGY.md] -- Overall strategy
