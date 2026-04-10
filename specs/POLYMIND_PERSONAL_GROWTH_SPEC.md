# PolyMind Personal Growth Specification

| Field | Value |
|-------|-------|
| **Version** | v0.1.0 |
| **Status** | Draft |
| **Lex Namespace** | `polyqlabs/qmind/growth` |
| **Circuit** | `circuits/fl/qmind_growth.fl` |
| **Upstream Dependency** | eStream v0.27.0+ (Intelligence Substrate: Resonance, Sage), QKit v0.1.0+ |
| **Companion Specs** | [POLYMIND_CE_APP_GRAPH_SPEC.md](POLYMIND_CE_APP_GRAPH_SPEC.md), [INTELLIGENCE_SUBSTRATE_SPEC.md](estream specs) |
| **Cross-Product Bridge** | Optional bridge to Paragon Foundation v0.3.0+ (organizational aggregation) |
| **GitHub Epic** | polyquantum/estream#1496 (Intelligence Substrate) |

---

## 1. Purpose

Q Mind Personal Growth integrates multiple personality and behavioral assessment frameworks into the personal knowledge corpus, enabling the ESLM to understand the user as a whole person — not just what they know, but how they think, where they're growing, and where they have blind spots.

The Enneagram is the primary growth framework because it uniquely provides **directional growth paths** (integration/disintegration lines) rather than static categories. Other frameworks (MBTI, CliftonStrengths, DISC, Working Genius, Belbin, Kolbe) provide complementary lenses that cross-validate and enrich the Enneagram core.

### Key Differentiator from Paragon

Paragon Foundation stores personality profiles for **organizational** context — team composition, C-suite management, board alignment. Q Mind Personal Growth is about the **individual's self-development journey**: self-awareness, intentional growth, content curation for development, and long-term trajectory tracking.

The two can optionally bridge: Q Mind growth data flows to Paragon (with user consent) for organizational context, while Paragon's organizational frameworks provide external context for individual growth.

---

## 2. Enneagram Growth Model

### 2.1 Core Enneagram Structure

Each Enneagram type has a rich internal structure that maps to concrete growth actions:

| Component | Description | Growth Relevance |
|-----------|-------------|-----------------|
| **Core Type** (1-9) | Primary motivation and fear pattern | Defines the growth direction |
| **Wing** | Adjacent type that flavors behavior | Expands growth surface area |
| **Integration Line** | Type you move toward in growth/health | Primary growth target |
| **Disintegration Line** | Type you move toward under stress | Early warning indicator |
| **Instinctual Variant** | SP (Self-Preservation), SX (Sexual/Intimate), SO (Social) | Shapes which domain growth manifests in |
| **Tritype** | Head/Heart/Gut triad combination | Provides nuanced growth priorities |
| **Health Level** (1-9) | Riso-Hudson levels of development | Measures overall growth state |

### 2.2 Integration / Disintegration Lines

```
1 → 7 (integration) / 1 → 4 (disintegration)
2 → 4 (integration) / 2 → 8 (disintegration)
3 → 6 (integration) / 3 → 9 (disintegration)
4 → 1 (integration) / 4 → 2 (disintegration)
5 → 8 (integration) / 5 → 7 (disintegration)
6 → 9 (integration) / 6 → 3 (disintegration)
7 → 5 (integration) / 7 → 1 (disintegration)
8 → 2 (integration) / 8 → 5 (disintegration)
9 → 3 (integration) / 9 → 6 (disintegration)
```

### 2.3 Growth Actions by Type

Each Enneagram type has specific growth areas. These are tracked as `GrowthAction` items with evidence from user behavior and self-reporting:

| Type | Integration Direction | Key Growth Actions |
|------|----------------------|-------------------|
| **1 (Reformer)** | → 7: Spontaneity, joy | Practice accepting imperfection, allow play, explore without evaluating |
| **2 (Helper)** | → 4: Self-awareness, boundaries | Identify own needs without others, practice receiving, creative self-expression |
| **3 (Achiever)** | → 6: Authenticity, loyalty | Value process over results, build genuine relationships, vulnerability |
| **4 (Individualist)** | → 1: Discipline, objectivity | Follow through on commitments, balance emotion with action, consistent routines |
| **5 (Investigator)** | → 8: Confidence, engagement | Take decisive action, share knowledge boldly, physical engagement |
| **6 (Loyalist)** | → 9: Inner peace, trust | Practice stillness, trust own judgment, release worst-case scenarios |
| **7 (Enthusiast)** | → 5: Depth, focus | Stay with one thing, process difficult emotions, depth over breadth |
| **8 (Challenger)** | → 2: Vulnerability, tenderness | Show softer side, listen without controlling, nurture others |
| **9 (Peacemaker)** | → 3: Self-assertion, action | State preferences, pursue own goals, healthy conflict engagement |

---

## 3. Data Model

### 3.1 Enneagram Profile

```fl
type EnneagramType = enum {
    Reformer,     // 1
    Helper,       // 2
    Achiever,     // 3
    Individualist, // 4
    Investigator, // 5
    Loyalist,     // 6
    Enthusiast,   // 7
    Challenger,   // 8
    Peacemaker,   // 9
}

type InstinctualVariant = enum {
    SelfPreservation,   // SP
    Sexual,             // SX (One-to-One)
    Social,             // SO
}

type EnneagramProfile = struct {
    core_type: u8,                   // 1-9
    wing: u8,                        // adjacent type
    integration_type: u8,            // growth direction
    disintegration_type: u8,         // stress direction
    tritype: list<u8>,               // head/heart/gut (3 types)
    instinctual_variant: u8,         // SP=0, SX=1, SO=2
    instinctual_stack: list<u8>,     // ordered [primary, secondary, tertiary]
    health_level: u8,                // 1-9 (1=healthiest)
    assessment_tick: u64,
    confidence_bps: u16,             // self-report vs. professional assessment
}
```

### 3.2 Complementary Frameworks

```fl
type ComplementaryProfile = struct {
    mbti_type: option<string>,           // "INTJ", etc.
    mbti_cognitive_stack: option<list<string>>,  // ["Ni", "Te", "Fi", "Se"]
    clifton_top_5: option<list<string>>,
    clifton_domains: option<CliftonDomains>,
    disc_profile: option<DISCScores>,
    working_genius: option<WorkingGeniusSet>,
    belbin_roles: option<list<string>>,
    kolbe_scores: option<KolbeScores>,
}

type CliftonDomains = struct {
    executing_bps: u16,
    influencing_bps: u16,
    relationship_building_bps: u16,
    strategic_thinking_bps: u16,
}

type DISCScores = struct {
    dominance_bps: u16,
    influence_bps: u16,
    steadiness_bps: u16,
    conscientiousness_bps: u16,
    natural_style: string,
    adapted_style: string,
}

type WorkingGeniusSet = struct {
    genius_pair: list<string>,
    competency_pair: list<string>,
    frustration_pair: list<string>,
}

type KolbeScores = struct {
    fact_finder: u8,     // 1-10
    follow_thru: u8,
    quick_start: u8,
    implementor: u8,
}
```

### 3.3 Growth Tracking

```fl
type GrowthDomain = enum {
    IntegrationDirection,   // moving toward integration type
    WingDevelopment,        // developing wing qualities
    InstinctualBalance,    // balancing instinctual variants
    HealthLevel,           // overall health level progression
    EmotionalIntelligence,
    RelationalCapacity,
    SelfAwareness,
    StressManagement,
    PurposeClarity,
    CliftonStrengthsDev,
    WorkingGeniusGrowth,
    Custom,
}

type GrowthAction = struct {
    action_id: bytes(16),
    domain: u8,                // GrowthDomain
    description: string,
    source_framework: string,  // "enneagram", "clifton", "disc", etc.
    priority_bps: u16,
    status: u8,                // 0=suggested, 1=accepted, 2=in_progress, 3=completed, 4=declined
    evidence_count: u32,
    created_tick: u64,
    last_evidence_tick: u64,
    progress_bps: u16,        // 0-10000
}

type GrowthEvidence = struct {
    evidence_id: bytes(16),
    action_id: bytes(16),
    evidence_type: u8,        // 0=self_report, 1=behavioral, 2=resonance, 3=insight, 4=feedback
    description: string,
    resonance_id: option<bytes(32)>,  // link to Resonance if from consumed content
    insight_id: option<bytes(16)>,     // link to Insight if auto-detected
    significance_bps: u16,
    captured_tick: u64,
}

type GrowthSnapshot = struct {
    snapshot_id: bytes(16),
    snapshot_tick: u64,
    health_level: u8,
    active_actions: u32,
    completed_actions: u32,
    domain_progress: list<DomainProgress>,
    integration_progress_bps: u16,  // progress toward integration type qualities
    stress_indicator_bps: u16,      // movement toward disintegration (lower=better)
}

type DomainProgress = struct {
    domain: u8,
    progress_bps: u16,
    trend: u8,                // 0=declining, 1=stable, 2=growing
    evidence_count: u32,
}
```

### 3.4 Resonance-Growth Integration

Content Resonance (from the Intelligence Substrate) feeds directly into growth tracking. When a user captures a Resonance for consumed content, Q Mind evaluates whether the content is relevant to their active growth actions:

```fl
type ResonanceGrowthLink = struct {
    resonance_id: bytes(32),
    action_id: bytes(16),
    relevance_bps: u16,
    growth_direction: u8,      // 0=supports_integration, 1=supports_wing, 2=general, 3=warns_disintegration
    auto_detected: bool,
    user_confirmed: bool,
}
```

For example:
- A **Type 7** captures a Resonance for a book about meditation (depth/focus). Q Mind auto-detects this is relevant to their **7→5 integration direction** and creates a growth evidence entry.
- A **Type 1** captures a Resonance for a podcast about spontaneity and play. Q Mind links it to their **1→7 integration direction**.
- A **Type 8** reads a book about empathetic leadership. Q Mind links it to their **8→2 integration direction**.

### 3.5 Content Recommendations

```fl
type GrowthContentRecommendation = struct {
    recommendation_id: bytes(16),
    action_id: bytes(16),
    media_type: u8,            // from Resonance MediaContentType
    title: string,
    author: string,
    relevance_bps: u16,
    growth_domain: u8,
    reason: string,
    recommended_tick: u64,
    user_response: u8,         // 0=pending, 1=consumed, 2=saved, 3=dismissed
}
```

---

## 4. CE Meaning Domain

### 4.1 `knowledge/personal_growth`

New meaning domain for the Cognitive Engine, added alongside the existing three domains:

| Signal | Type | Description |
|--------|------|-------------|
| `growth_action_velocity` | gauge | Growth actions progressed per month |
| `integration_progress` | gauge | Progress toward integration type (0-10000 bps) |
| `disintegration_warning` | gauge | Stress indicator — movement toward disintegration type |
| `resonance_growth_link_rate` | gauge | Fraction of Resonance captures relevant to growth actions |
| `health_level_trajectory` | gauge | Direction of health level change (smoothed) |
| `framework_cross_validation` | gauge | Agreement between frameworks on growth areas |

### 4.2 SME Panel: Personal Growth Quality

- **Metrics observed**: `growth_action_velocity`, `integration_progress`, `disintegration_warning`, `health_level_trajectory`
- **Trigger**: Monthly scheduled + on-demand when `disintegration_warning` exceeds threshold
- **Output**: Growth trajectory assessment, recommended focus shifts, integration milestone recognition, stress management suggestions

---

## 5. Growth Insight Integration

The existing `qmind_insight.fl` module's insight engine is extended with growth-aware patterns:

| Insight Type | Source | Growth Action |
|-------------|--------|---------------|
| **Growth Opportunity** | Resonance patterns showing deepening interest in integration-aligned topics | Suggest concrete growth actions aligned with integration direction |
| **Stress Warning** | Behavioral patterns matching disintegration type characteristics | Surface stress management techniques specific to user's type |
| **Cross-Framework Confirmation** | Multiple frameworks agree on a growth area | Elevate priority of growth actions confirmed by 2+ frameworks |
| **Strength Application** | CliftonStrengths/Working Genius alignment with growth domain | Suggest applying existing strengths to growth challenges |
| **Wing Development** | Content or behavior patterns showing wing exploration | Encourage wing development as secondary growth path |

---

## 6. Privacy and Governance

### 6.1 Zero-Linkage Compliance

Growth data is strictly within the user's `polyqlabs/qmind` lex namespace. No growth data flows to any other product without explicit user consent. The HKDF context `q-mind-growth-v1` derives a separate key for growth data encryption.

### 6.2 Paragon Bridge (Opt-In)

If the user belongs to a Paragon-enabled organization, they can opt-in to share growth data with Paragon Foundation for organizational context:

- Only aggregate growth domain progress is shared (not raw evidence or self-reports)
- Sharing uses the `UserContextBridge` from the Intelligence Substrate
- `RedactionPolicy` strips all self-report text, resonance details, and health levels
- Bridge is user-initiated and immediately revocable

### 6.3 Assessment Source Classification

| Source | Confidence Multiplier | Description |
|--------|----------------------|-------------|
| Professional Assessment | 1.0x | Certified Enneagram practitioner, CliftonStrengths coach |
| Self-Assessment (guided) | 0.7x | Structured questionnaire within Q Mind |
| Self-Assessment (declared) | 0.5x | User self-identifies type |
| Behavioral Inference | 0.3x | ESLM infers type from usage patterns (never overrides explicit) |

---

## 7. Circuits

### 7.1 Circuit Inventory

| Circuit | Purpose |
|---------|---------|
| `set_enneagram_profile` | Capture or update Enneagram profile from assessment |
| `set_complementary_profile` | Capture complementary framework results |
| `generate_growth_actions` | Generate type-specific growth actions from profile |
| `record_growth_evidence` | Record evidence of growth (self-report, behavioral, resonance-linked) |
| `link_resonance_to_growth` | Auto-detect and link Resonance captures to active growth actions |
| `recommend_growth_content` | Recommend content aligned with active growth directions |
| `compute_growth_snapshot` | Compute periodic growth trajectory snapshot |
| `detect_stress_pattern` | Detect movement toward disintegration type |
| `cross_validate_frameworks` | Find agreement/conflict between frameworks on growth areas |
| `get_integration_progress` | Query progress toward integration type qualities |
| `get_growth_dashboard` | Aggregate growth data for UI display |

### 7.2 Key Signatures

```fl
circuit set_enneagram_profile(
    user_id: bytes(16),
    profile: EnneagramProfile,
    assessment_source: u8,
    signing_sk: bytes(4896),
) -> bool

circuit generate_growth_actions(
    user_id: bytes(16),
    enneagram: EnneagramProfile,
    complementary: ComplementaryProfile,
) -> list<GrowthAction>

circuit record_growth_evidence(
    user_id: bytes(16),
    action_id: bytes(16),
    evidence_type: u8,
    description: string,
    resonance_id: option<bytes(32)>,
    signing_sk: bytes(4896),
) -> GrowthEvidence

circuit link_resonance_to_growth(
    user_id: bytes(16),
    resonance_id: bytes(32),
    media_title: string,
    topic_sentiments: list<TopicSentiment>,
    active_actions: list<GrowthAction>,
) -> list<ResonanceGrowthLink>

circuit compute_growth_snapshot(
    user_id: bytes(16),
    actions: list<GrowthAction>,
    evidence: list<GrowthEvidence>,
    enneagram: EnneagramProfile,
) -> GrowthSnapshot

circuit detect_stress_pattern(
    user_id: bytes(16),
    recent_behavior: list<bytes>,
    enneagram: EnneagramProfile,
) -> u16  // stress_indicator_bps
```

---

## 8. App Graph Integration

### 8.1 Module Registration

`qmind_growth` registers as module 14 in the Q Mind App Graph (updating module count from 13 to 14).

### 8.2 Intra-Graph Dependencies

```
qmind_growth -> knowledge_graph, qmind_insight, qmind_classify
```

- **knowledge_graph**: Growth actions, evidence, and snapshots are nodes in the KG
- **qmind_insight**: Growth-aware insights feed from and inform growth tracking
- **qmind_classify**: Resonance content is classified for growth domain relevance

### 8.3 Bridge Edge

```
qmind_growth -> estream/core/intelligence/ai/resonance (BRIDGE_TO)
```

Resonance data from the Intelligence Substrate feeds into growth action linking.

---

## 9. UX Considerations

### 9.1 Growth Dashboard

The growth dashboard surfaces:
- Current Enneagram profile with integration/disintegration arrows
- Active growth actions with progress bars
- Recent evidence timeline
- Resonance-growth connections (which content supported growth)
- Integration progress gauge
- Stress indicator with historical trend
- Cross-framework agreement map

### 9.2 Content Curation

Q Mind can curate the user's reading/listening queue to prioritize content aligned with their active growth directions. This is not prescriptive — it surfaces relevance, not mandates.

### 9.3 Periodic Check-Ins

Optional periodic prompts (configurable frequency) that ask the user to reflect on growth actions. Reflections become growth evidence entries. The ESLM learns the user's reflection style over time and adapts prompts accordingly.

---

## 10. Future Considerations

- **Group dynamics**: When multiple Q Mind users in a team/family opt-in, analyze complementary personality dynamics (via Paragon bridge)
- **AI coaching**: ESLM-powered coaching conversations that reference the user's growth actions and evidence
- **Temporal analysis**: Track how personality expression shifts over months/years (health level changes, wing development)
- **Spiritual growth**: For users who opt-in, integrate Enneagram's spiritual growth traditions (contemplative practices, virtue development)
