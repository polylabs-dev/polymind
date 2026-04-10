# PolyMind CE + App Graph Specification

| Field | Value |
|-------|-------|
| **Version** | v0.3.0 |
| **Status** | Draft |
| **Lex Namespace** | `polyqlabs/qmind` |
| **App Graph** | `circuits/fl/qmind_app_graph.fl` |
| **CE Meaning** | `circuits/fl/qmind_meaning.fl` |
| **Upstream Dependency** | eStream v0.27.0+ (Intelligence Substrate, Containment Proof), QKit v0.1.0+ |

---

## 1. App Graph — 15 Modules

PolyMind registers 15 FL circuit modules into a single App Graph under `polyqlabs/qmind`:

| Module | Partition | SLA | Description |
|--------|-----------|-----|-------------|
| `qmind_ingest` | Backend | Premium | Content ingestion pipeline — text, audio, image, structured data |
| `qmind_document_ingest` | Backend | Premium | Document-specific ingestion (PDF, DOCX, EPUB, HTML) via PolyDocs bridge |
| `qmind_classify` | Backend | Standard | Content classification — topic, domain, sensitivity, provenance |
| `qmind_eslm` | Backend | Premium | On-device ESLM training + inference (BitNet b1.58 micro-model) |
| `qmind_query` | Head | Premium | Natural-language query engine — retrieval, ranking, synthesis |
| `qmind_insight` | Backend | Standard | Pattern recognition — knowledge gaps, connections, temporal trends |
| `qmind_rbac` | Shared | Standard | Field-level access control for corpus content and legacy policies |
| `qmind_metering` | Backend | Standard | Per-user usage metering (ingest volume, query count, ESLM tokens) |
| `qmind_platform_health` | Shared | Standard | Health probes, corpus integrity checks, ESLM model validation |
| `qmind_guardian` | Backend | Premium | Guardian designation, transfer protocols, dead-man switch |
| `qmind_legacy` | Backend | Premium | Digital estate assembly, time-locked releases, beneficiary routing |
| `knowledge_graph` | Backend | Premium | Stratum CSR property graph — entities, concepts, relationships |
| `legacy_dag` | Backend | Premium | Merkle-linked DAG for legacy artifact provenance and ordering |
| `qmind_growth` | Backend | Standard | Enneagram-centered personal growth tracking, Resonance integration |
| `qmind_agents` | Backend | Premium | Personal agent creation, lifecycle, containment, dashboard (v0.3.0) |

### Intra-Graph Dependencies

```
qmind_query -> qmind_classify, qmind_eslm, knowledge_graph
qmind_insight -> knowledge_graph, qmind_classify
qmind_ingest -> qmind_classify, knowledge_graph
qmind_document_ingest -> qmind_ingest, qmind_classify
qmind_eslm -> qmind_classify, knowledge_graph
qmind_guardian -> qmind_rbac, qmind_legacy
qmind_legacy -> legacy_dag, qmind_rbac, knowledge_graph
qmind_metering -> qmind_rbac
qmind_platform_health -> qmind_metering, qmind_rbac
qmind_growth -> knowledge_graph, qmind_insight, qmind_classify
qmind_agents -> knowledge_graph, qmind_rbac, qmind_growth
```

---

## 2. CE Meaning Domains

Three meaning domains define the signal PolyMind's Cognitive Engine tracks:

### 2.1 `knowledge/corpus_growth`

Tracks the velocity and breadth of knowledge accumulation.

| Signal | Type | Description |
|--------|------|-------------|
| `ingest_velocity` | gauge | Documents ingested per hour (rolling 24h window) |
| `knowledge_coverage` | gauge | Percentage of user-declared topics with ≥10 corpus entries |
| `corpus_diversity_index` | gauge | Shannon entropy across classification domains |
| `stale_entry_ratio` | gauge | Fraction of entries not referenced in >90 days |

### 2.2 `knowledge/query_quality`

Tracks retrieval effectiveness and hallucination risk.

| Signal | Type | Description |
|--------|------|-------------|
| `retrieval_precision` | gauge | Top-5 retrieval relevance score (user feedback + auto-eval) |
| `hallucination_rate` | counter | Synthesized answers flagged as ungrounded per 1000 queries |
| `query_latency_p99_ms` | gauge | 99th percentile query response latency |
| `eslm_confidence_mean` | gauge | Mean ESLM confidence score across queries |

### 2.3 `knowledge/legacy_governance`

Tracks guardian transfer patterns and digital estate health.

| Signal | Type | Description |
|--------|------|-------------|
| `guardian_transfer_patterns` | event | Guardian designation changes, dead-man switch activations |
| `digital_estate_health` | gauge | Completeness score: beneficiary coverage, artifact freshness, policy currency |
| `legacy_artifact_count` | gauge | Total time-locked and released legacy artifacts |
| `policy_violation_count` | counter | Guardian policy violations detected per evaluation cycle |

### 2.4 `knowledge/personal_growth` (v0.2.0)

Tracks personality-aware growth trajectory and content alignment.

| Signal | Type | Description |
|--------|------|-------------|
| `growth_action_velocity` | gauge | Growth actions progressed per month |
| `integration_progress` | gauge | Progress toward Enneagram integration type (0-10000 bps) |
| `disintegration_warning` | gauge | Stress indicator — movement toward disintegration type |
| `resonance_growth_link_rate` | gauge | Fraction of Resonance captures relevant to growth actions |
| `health_level_trajectory` | gauge | Direction of health level change (smoothed) |
| `framework_cross_validation` | gauge | Agreement between frameworks on growth areas |

### 2.5 `knowledge/personal_agents` (v0.3.0)

Tracks personal agent lifecycle, containment, and effectiveness.

| Signal | Type | Description |
|--------|------|-------------|
| `active_agent_count` | gauge | Number of currently active personal agents |
| `agent_action_velocity` | gauge | Agent actions per day (rolling 24h window) |
| `containment_violations` | counter | Containment boundary violations detected |
| `scope_utilization_mean` | gauge | Mean utilization of granted scope across agents (0-10000 bps) |
| `agent_effectiveness` | gauge | Ratio of agent actions that led to user-acknowledged value |
| `external_comm_requests` | counter | External communication requests (all should be SPARK-consented) |

---

## 3. Noise Filter

### Suppression Rules

| Rule | Pattern | Reason |
|------|---------|--------|
| `suppress_reindex` | `ingest_velocity` spike during scheduled re-index windows | Re-indexing is maintenance, not organic growth |
| `suppress_cache_refresh` | `query_latency_p99_ms` transient spikes during ESLM cache warmup | Cache refresh is expected after model updates |
| `suppress_bulk_import` | `corpus_diversity_index` drop during single-source bulk imports | Diversity recovers as import is classified |

### Signal Amplification

| Rule | Pattern | Reason |
|------|---------|--------|
| `amplify_knowledge_gap` | `knowledge_coverage` drops below 60% for any declared topic | Indicates corpus blind spot requiring user attention |
| `amplify_guardian_violation` | Any `policy_violation_count` increment | Guardian policy violations are always actionable |
| `amplify_hallucination_spike` | `hallucination_rate` exceeds 50 per 1000 queries | Retrieval pipeline degradation requires investigation |

---

## 4. SME Panels

### 4.1 Knowledge Graph Quality Panel

Evaluates the structural health of the knowledge graph.

- **Metrics observed**: `corpus_diversity_index`, `stale_entry_ratio`, `knowledge_coverage`
- **Trigger**: Weekly scheduled + on-demand when `knowledge_coverage` drops >10% in 24h
- **Output**: Quality score (0–100), recommended re-classification actions, orphan node report

### 4.2 ESLM Training Effectiveness Panel

Evaluates whether on-device ESLM training is converging and useful.

- **Metrics observed**: `eslm_confidence_mean`, `retrieval_precision`, `hallucination_rate`
- **Trigger**: After every ESLM training epoch + on-demand when `hallucination_rate` spikes
- **Output**: Training convergence assessment, recommended corpus curation actions, model rollback recommendation if regression detected

### 4.3 Legacy Governance Compliance Panel

Evaluates whether the digital estate is well-governed and transfer-ready.

- **Metrics observed**: `guardian_transfer_patterns`, `digital_estate_health`, `policy_violation_count`, `legacy_artifact_count`
- **Trigger**: Monthly scheduled + on-demand when guardian designation changes
- **Output**: Estate readiness score (0–100), policy gap analysis, beneficiary coverage report

### 4.4 Personal Growth Quality Panel (v0.2.0)

Evaluates personal growth trajectory and content alignment with growth directions.

- **Metrics observed**: `growth_action_velocity`, `integration_progress`, `disintegration_warning`, `resonance_growth_link_rate`, `health_level_trajectory`
- **Trigger**: Monthly scheduled + on-demand when `disintegration_warning` exceeds threshold
- **Output**: Growth trajectory assessment, recommended focus shifts, integration milestone recognition, stress management suggestions, cross-framework agreement analysis

### 4.5 Personal Agent Health Panel (v0.3.0)

Evaluates agent containment integrity and effectiveness.

- **Metrics observed**: `active_agent_count`, `containment_violations`, `scope_utilization_mean`, `agent_effectiveness`, `external_comm_requests`
- **Trigger**: Daily scheduled + immediate on any `containment_violations` increment
- **Output**: Containment integrity score (0-100), scope optimization recommendations, agent effectiveness ranking, containment violation forensics

---

## 5. Bridge Edges

### 5.1 QKit ESLM Classify Bridge

| Field | Value |
|-------|-------|
| **Source** | `polyqlabs/qmind` → `qmind_classify` |
| **Target** | `polyqlabs/qkit` → `qkit_eslm_classify` |
| **Scope** | Platform |
| **Shared Fields** | `classification_result`, `domain_tags`, `confidence_score` |
| **Direction** | Bilateral — PolyMind sends content, QKit returns classification |

### 5.2 eStream Resonance Growth Bridge (v0.2.0)

| Field | Value |
|-------|-------|
| **Source** | `polyqlabs/qmind` → `qmind_growth` |
| **Target** | `esn/global/intelligence/ai` → `resonance` |
| **Scope** | Platform |
| **Shared Fields** | `resonance_id`, `topic_sentiments`, `overall_impact_bps` |
| **Direction** | Inbound — eStream Resonance events trigger growth action linking |

### 5.3 eStream Containment Agent Bridge (v0.3.0)

| Field | Value |
|-------|-------|
| **Source** | `polyqlabs/qmind` → `qmind_agents` |
| **Target** | `esn/global/containment` → `behavioral_envelope` |
| **Scope** | Platform |
| **Shared Fields** | `agent_containment_profile`, `behavioral_envelope`, `boundary_events` |
| **Direction** | Bilateral — Q Mind agents consume eStream containment profiles, boundary events flow back for dashboard |

### 5.4 PolyDocs Document Ingest Bridge

| Field | Value |
|-------|-------|
| **Source** | `polyqlabs/qmind` → `qmind_document_ingest` |
| **Target** | `polyqlabs/qdocs` → `qdocs_parser` |
| **Scope** | Platform |
| **Shared Fields** | `parsed_document`, `extracted_text`, `structural_metadata` |
| **Direction** | Bilateral — PolyMind sends raw documents, PolyDocs returns parsed content |

---

## 6. Strategic Grant Configuration

### 6.1 eStream Grant

PolyMind consumes the following eStream platform primitives under the PolyQ Labs commercial license:

- `scatter-cas` — Corpus content storage with erasure coding
- `SPARK` — Per-product biometric identity (HKDF context: `q-mind-v1`)
- `StreamSight` — Observability within `polyqlabs/qmind` lex namespace
- `ESLM` — On-device SSM micro-inference (BitNet b1.58)
- `ML-KEM-1024` / `ML-DSA-87` — PQ encryption + signatures for legacy artifacts

### 6.2 Paragon Grant

PolyMind may optionally bridge to Paragon Foundation for family office deployments:

- `paragon/foundation` → Entity graph for beneficiary resolution
- `paragon/foundation` → Compliance framework for legacy regulatory requirements
- `paragon/aegis` → Enterprise agent governance bridge (employees can have personal agents governed by enterprise policy)
- Bridge is opt-in per deployment; PolyMind operates independently without Paragon
