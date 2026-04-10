# PolyMind Personal Agents Specification

| Field | Value |
|-------|-------|
| **Version** | v0.1.0 |
| **Status** | Draft |
| **Lex Namespace** | `polyqlabs/qmind/agents` |
| **Circuit** | `circuits/fl/qmind_agents.fl` |
| **Upstream Dependency** | eStream v0.27.0+ (Containment Proof, AI Agent Architecture), QKit v0.1.0+ |
| **Companion Specs** | [POLYMIND_CE_APP_GRAPH_SPEC.md](POLYMIND_CE_APP_GRAPH_SPEC.md), [POLYMIND_PERSONAL_GROWTH_SPEC.md](POLYMIND_PERSONAL_GROWTH_SPEC.md), CONTAINMENT_PROOF_SPEC.md |
| **Cross-Product Bridge** | Optional bridge to Paragon Aegis v0.1.0+ (enterprise agent governance) |
| **GitHub Epic** | polyquantum/estream#1499 |

---

## 1. Purpose

Q Mind Personal Agents lets users create purpose-built AI agents scoped to specific domains of their life. Not one monolithic AI assistant that knows everything about everything — a constellation of focused agents, each with a clear mandate, provable containment, and the user's explicit consent for every capability it possesses.

A **Project Agent** helps with a home renovation. A **Domain Agent** tracks personal finances. A **Research Agent** deep-dives into a medical topic and reports back. A **Guardian Agent** watches a portfolio overnight. A **Growth Agent** coaches toward Enneagram integration. Each one operates in its own namespace, stores its own state in the user's local lex, and provably cannot escape its scope.

This is the "powerful local AI that the user controls" scenario done right. The user sees exactly what each agent can access, exactly what it has done, and exactly what left the device (which, by default, is nothing). Every agent runs in local WASM/pWASM with no network egress unless the user grants it per-request via SPARK consent. Destroying an agent destroys all its state — no residue, no orphaned data, no ghost threads.

### Why Agents, Not One Big AI

A single monolithic AI accumulates context without boundaries. It knows your finances, your health data, your relationship patterns, your work projects — all in one undifferentiated pool. This creates three problems:

1. **Privacy entanglement**: A query about your startup accidentally surfaces context from your therapy notes because the AI has no internal walls.
2. **Uncontrollable scope**: The user cannot meaningfully reason about what the AI "knows" because it knows everything.
3. **No auditability**: When something goes wrong, there is no way to trace which capability or which data access caused the problem.

Personal agents solve all three. Each agent has a declared scope, a containment proof, and an activity log. The user reasons about agents the way they reason about people they delegate to — "I gave my accountant access to my finances, not my medical records."

---

## 2. Key Differentiator from Paragon

Paragon Aegis manages **enterprise agents** for organizations — compliance bots, knowledge base assistants, automated workflow executors that operate under organizational governance, admin oversight, and compliance audit trails.

Q Mind Personal Agents are for the **individual**:

| Dimension | Q Mind Personal Agents | Paragon Aegis |
|-----------|----------------------|---------------|
| **Owner** | Individual user | Organization admin |
| **Governance** | User self-governance via SPARK consent | Admin RBAC + compliance framework |
| **Scope** | User's personal lex namespace | Organizational lex namespace |
| **Visibility** | Only the user sees agent activity | Admin + compliance officer visibility |
| **Lifecycle** | User creates/destroys at will | Admin provisions, compliance gates termination |
| **Containment model** | Personal containment dashboard | Enterprise containment audit trail |
| **Identity** | SPARK sub-key per agent | Organizational service identity |
| **Data residency** | Local-first, user-controlled | Organizational scatter-CAS policy |

The two can optionally bridge. An employee who uses Q Mind for personal agents and works at a Paragon-enabled organization can opt-in to a bridge that provides organizational context to their personal agents (e.g., a Project Agent for a work project can access the company knowledge base). The bridge is:

- **User-initiated**: The organization cannot force it
- **Scoped**: The user chooses which personal agents get organizational context
- **Revocable**: Disconnect at any time, organizational context is purged from the personal agent
- **Read-only from org**: Personal agent reads organizational data but never writes personal data back to the org namespace

---

## 3. Agent Types

### 3.1 Type Definitions

```fl
type AgentType = enum {
    Project,     // helps with a specific time-bounded project
    Domain,      // ongoing expertise in a persistent life area
    Research,    // deep-dives into a topic and reports back
    Guardian,    // monitors something and alerts on conditions
    Growth,      // integrates with Q Mind growth module
}

type AgentPurpose = struct {
    summary: string,              // user's natural language description of what this agent does
    objectives: list<string>,     // concrete goals the agent should pursue
    success_criteria: string,     // how the user will know the agent has done its job
    time_horizon: u8,             // 0=indefinite, 1=days, 2=weeks, 3=months, 4=years
    estimated_end_tick: option<u64>,  // for time-bounded agents (Project, Research)
}
```

### 3.2 Project Agent

Helps with a specific, time-bounded project: home renovation, thesis, startup launch, wedding planning, move to a new city.

**Characteristics**:
- Has a defined start and expected end
- Accumulates project-specific knowledge over its lifetime
- Can organize tasks, timelines, and reference materials within scope
- On completion, the user can archive the agent (state preserved but agent inactive) or terminate (state destroyed)

**Example**:
> "Help me with my kitchen renovation. You can see my Home Projects folder and my Finances/Home Budget subfolder. You can search the web for product reviews and contractor ratings. You cannot access anything else."

### 3.3 Domain Agent

Ongoing expertise in a persistent area of life: personal finance, health and fitness, cooking and nutrition, parenting, a hobby.

**Characteristics**:
- No expected end date — runs indefinitely
- Builds deep context over months and years
- Learns the user's preferences, patterns, and history within the domain
- ESLM fine-tuning within scope produces increasingly personalized responses

**Example**:
> "Be my personal finance advisor. You can see Finances and Tax Documents. You can read (not write) my calendar for upcoming financial deadlines. You cannot access any other areas."

### 3.4 Research Agent

Deep-dives into a topic and reports back with findings. More autonomous than a Project Agent — the user defines the question, and the Research Agent independently gathers, organizes, and synthesizes information.

**Characteristics**:
- Time-bounded: research has a deadline or completion threshold
- Granted broader read access for the duration of research
- Produces a structured deliverable (report, summary, annotated bibliography)
- Network egress typically required (web search, API access) — each request SPARK-consented
- On completion, deliverable is saved to the user's lex; agent and working state are destroyed

**Example**:
> "Research the best school districts in the Portland metro area for our family. Consider ratings, extracurriculars, commute times from our two candidate neighborhoods. You can access my Family/Schools folder and search the web. Report back in 2 weeks."

### 3.5 Guardian Agent

Monitors something and alerts the user when conditions are met. Runs continuously in the background with minimal resource usage.

**Characteristics**:
- Event-driven: mostly idle, activates on triggers
- Minimal scope — only needs read access to what it monitors
- Alert delivery via Q Mind inbox (never external without consent)
- Can monitor external feeds (news, market data) if granted network egress

**Example**:
> "Watch my investment portfolio for any single-day drop exceeding 5% or any position hitting its stop-loss. You can see Finances/Investments. Alert me immediately."

### 3.6 Growth Agent

Integrates with Q Mind's Enneagram-centered personal growth module. A specialized Domain Agent with read access to the user's growth profile and growth actions.

**Characteristics**:
- Enneagram-aware: understands integration/disintegration lines
- Can recommend content aligned with growth directions
- Detects patterns relevant to growth (behavior consistent with integration vs. disintegration)
- Auto-creates growth evidence from agent interactions
- Read access to `polyqlabs/qmind/growth` namespace

**Example**:
> "Coach me on my Enneagram 7→5 integration. Help me practice depth over breadth — flag when I'm context-switching too much, suggest deep-dive content, and help me stick with one thing at a time."

---

## 4. Data Model

### 4.1 PersonalAgent

```fl
type PersonalAgent = struct {
    agent_id: bytes(16),
    user_id: bytes(16),
    agent_type: u8,                  // AgentType
    display_name: string,
    purpose: AgentPurpose,
    containment_profile: ContainmentProfile,
    scope: AgentScope,
    lex_sub_namespace: string,       // e.g. "polyqlabs/qmind/agents/{agent_id}"
    spark_sub_key: bytes(32),        // HKDF-derived sub-key for this agent
    status: u8,                      // AgentStatus
    created_tick: u64,
    last_active_tick: u64,
    total_actions: u64,
    total_data_read_bytes: u64,
    total_data_written_bytes: u64,
    total_external_requests: u32,    // network egress requests (each SPARK-consented)
    eslm_context_size: u64,          // bytes of ESLM context accumulated
    parent_agent_id: option<bytes(16)>,  // if spawned by another agent (with user consent)
    signing_pk: bytes(2592),         // ML-DSA-87 public key for agent attestations
}
```

### 4.2 AgentStatus

```fl
type AgentStatus = enum {
    Created,       // defined but not yet activated (scope review pending)
    Active,        // operating within scope, observed by Cortex
    Paused,        // state frozen, user can resume
    Modifying,     // scope change in progress (expanding requires re-consent)
    Archived,      // state preserved, agent inactive, can be reactivated
    Terminating,   // agent and state destruction in progress
    Terminated,    // agent and all state destroyed, CAS garbage collected
}
```

### 4.3 AgentScope

```fl
type AgentScope = struct {
    readable_namespaces: list<NamespaceGrant>,
    writable_namespaces: list<NamespaceGrant>,
    tool_permissions: list<ToolPermission>,
    network_egress_policy: NetworkEgressPolicy,
    interaction_bounds: InteractionBounds,
    content_type_filter: ContentTypeFilter,
    temporal_bounds: option<TemporalBounds>,
}

type NamespaceGrant = struct {
    namespace: string,               // lex namespace path (e.g. "polyqlabs/qmind/corpus/finances")
    recursive: bool,                 // includes sub-namespaces
    field_restrictions: option<list<string>>,  // if set, only these fields are visible
    granted_tick: u64,
    granted_by_consent_id: bytes(16),  // SPARK consent that authorized this grant
}

type ToolPermission = struct {
    tool_name: string,               // e.g. "web_search", "send_email", "calendar_read"
    permitted: bool,
    rate_limit: option<RateLimit>,   // optional throttle
    requires_per_use_consent: bool,  // if true, each invocation needs SPARK approval
}

type RateLimit = struct {
    max_invocations: u32,
    window_seconds: u32,
}

type NetworkEgressPolicy = enum {
    Blocked,                          // no network access at all
    PerRequestConsent,               // each request requires SPARK consent in real-time
    PreApprovedDomains,              // user pre-approved specific domains
    PreApprovedWithLogging,          // pre-approved domains with full request/response logging
}

type InteractionBounds = struct {
    max_daily_actions: option<u32>,
    max_daily_tokens: option<u64>,
    max_data_read_bytes_per_day: option<u64>,
    max_data_written_bytes_per_day: option<u64>,
    quiet_hours: option<QuietHours>,
}

type QuietHours = struct {
    start_hour: u8,                  // 0-23 local time
    end_hour: u8,
    allow_urgent_alerts: bool,       // Guardian agents can break quiet hours for urgent alerts
}

type ContentTypeFilter = struct {
    allowed_types: list<string>,     // e.g. ["document", "spreadsheet", "note"]
    blocked_types: list<string>,     // e.g. ["voice_recording", "photo"]
}

type TemporalBounds = struct {
    earliest_tick: option<u64>,      // agent can only see data created after this tick
    latest_tick: option<u64>,        // agent can only see data created before this tick
}
```

### 4.4 ContainmentProfile

```fl
type ContainmentProfile = struct {
    profile_id: bytes(16),
    agent_id: bytes(16),
    containment_level: u8,           // ContainmentLevel
    last_attestation_tick: u64,
    attestation_hash: bytes(32),     // hash of the last containment proof
    violation_count: u32,            // total violations detected (should be 0)
    runtime_environment: u8,         // RuntimeEnvironment
    cas_storage_policy: u8,          // CASStoragePolicy
}

type ContainmentLevel = enum {
    Standard,      // local WASM sandbox, namespace isolation, activity logging
    Strict,        // Standard + per-action containment proof verification
    Paranoid,      // Strict + real-time SPARK consent for every data access
}

type RuntimeEnvironment = enum {
    LocalWASM,            // browser/Tauri WASM sandbox
    LocalPWASM,           // pWASM with hardware-backed attestation (T0-enhanced)
    LocalNative,          // native Rust binary in blobjail sandbox
}

type CASStoragePolicy = enum {
    LocalOnly,            // agent state never leaves the device
    EncryptedScatterSync, // encrypted scatter-CAS across user's devices
    EncryptedBackup,      // encrypted backup to user-designated cloud
}
```

### 4.5 AgentActivityLog

```fl
type AgentActivityLog = struct {
    log_id: bytes(16),
    agent_id: bytes(16),
    action_type: u8,                // AgentActionType
    description: string,            // plain-language description of what the agent did
    data_accessed: list<DataAccessEntry>,
    data_produced: list<DataProducedEntry>,
    external_communication: option<ExternalComm>,
    tokens_consumed: u64,
    duration_ms: u64,
    containment_proof: option<bytes(32)>,  // proof that this action was within scope
    timestamp: u64,
}

type AgentActionType = enum {
    Query,             // agent queried user data within scope
    Write,             // agent wrote data to writable namespace
    Analyze,           // agent performed analysis on in-scope data
    Alert,             // Guardian agent raised an alert
    Recommend,         // agent produced a recommendation
    WebSearch,         // agent searched the web (with consent)
    APICall,           // agent called an external API (with consent)
    InternalComm,      // agent communicated with another agent (within lex)
    ESLMInference,     // agent ran ESLM inference
    ESLMTraining,      // agent contributed to ESLM fine-tuning within scope
    ScopeCheck,        // agent verified its own containment
}

type DataAccessEntry = struct {
    namespace: string,
    item_id: bytes(16),
    field_names: list<string>,
    bytes_read: u64,
}

type DataProducedEntry = struct {
    namespace: string,
    item_id: bytes(16),
    description: string,
    bytes_written: u64,
}

type ExternalComm = struct {
    direction: u8,         // 0=outbound, 1=inbound_response
    destination: string,   // domain or API endpoint
    request_hash: bytes(32),
    response_hash: bytes(32),
    consent_id: bytes(16), // SPARK consent that authorized this communication
    bytes_sent: u64,
    bytes_received: u64,
}
```

### 4.6 Agent Inbox

```fl
type AgentMessage = struct {
    message_id: bytes(16),
    agent_id: bytes(16),
    direction: u8,                   // MessageDirection
    content_type: u8,                // MessageContentType
    subject: string,
    body: string,
    attachments: list<bytes(16)>,    // CAS references within agent's namespace
    priority: u8,                    // 0=low, 1=normal, 2=high, 3=urgent
    created_tick: u64,
    read_tick: option<u64>,
    action_required: bool,
}

type MessageDirection = enum {
    AgentToUser,       // agent delivers insight, alert, summary, recommendation
    UserToAgent,       // user sends query, task, or instruction
    AgentToAgent,      // inter-agent communication (governed by interaction policy)
}

type MessageContentType = enum {
    Insight,           // pattern recognition result
    Alert,             // Guardian agent trigger
    Summary,           // periodic or on-demand summary
    Recommendation,    // content or action recommendation
    Question,          // agent asks user for clarification
    TaskAssignment,    // user assigns task to agent
    TaskCompletion,    // agent reports task complete
    Report,            // Research agent delivers findings
}
```

---

## 5. Agent Lifecycle

### 5.1 State Machine

```
              ┌──────────┐
              │  Created  │
              └─────┬─────┘
                    │ user reviews scope, consents via SPARK
                    ▼
              ┌──────────┐
         ┌───►│  Active   │◄───┐
         │    └──┬──┬──┬──┘    │
         │       │  │  │       │
    resume│ pause│  │  │modify │ scope change complete
         │       │  │  │       │
         │       ▼  │  ▼       │
    ┌────┴───┐     │  ┌───────┴──┐
    │ Paused │     │  │ Modifying │
    └────────┘     │  └──────────┘
                   │
              archive│ or terminate
                   │
            ┌──────┴──────┐
            ▼             ▼
      ┌──────────┐  ┌─────────────┐
      │ Archived │  │ Terminating │
      └─────┬────┘  └──────┬──────┘
            │              │ CAS garbage collection complete
       reactivate          ▼
            │        ┌────────────┐
            └───────►│ Terminated │
                     └────────────┘
```

### 5.2 Lifecycle Circuits

```fl
circuit create_agent(
    user_id: bytes(16),
    agent_type: u8,
    display_name: string,
    purpose: AgentPurpose,
    scope: AgentScope,
    containment_level: u8,
    runtime_environment: u8,
    cas_storage_policy: u8,
    signing_sk: bytes(4896),
) -> PersonalAgent

circuit activate_agent(
    user_id: bytes(16),
    agent_id: bytes(16),
    spark_consent: bytes(64),       // SPARK biometric consent for the declared scope
    signing_sk: bytes(4896),
) -> bool

circuit pause_agent(
    user_id: bytes(16),
    agent_id: bytes(16),
    signing_sk: bytes(4896),
) -> bool

circuit resume_agent(
    user_id: bytes(16),
    agent_id: bytes(16),
    signing_sk: bytes(4896),
) -> bool

circuit modify_agent_scope(
    user_id: bytes(16),
    agent_id: bytes(16),
    new_scope: AgentScope,
    scope_change_type: u8,          // 0=narrowing, 1=expanding
    spark_consent: option<bytes(64)>,  // required if expanding
    signing_sk: bytes(4896),
) -> bool

circuit archive_agent(
    user_id: bytes(16),
    agent_id: bytes(16),
    signing_sk: bytes(4896),
) -> bool

circuit reactivate_agent(
    user_id: bytes(16),
    agent_id: bytes(16),
    spark_consent: bytes(64),       // re-consent to archived scope
    signing_sk: bytes(4896),
) -> bool

circuit terminate_agent(
    user_id: bytes(16),
    agent_id: bytes(16),
    spark_consent: bytes(64),       // confirm destruction
    signing_sk: bytes(4896),
) -> TerminationReceipt

type TerminationReceipt = struct {
    agent_id: bytes(16),
    terminated_tick: u64,
    data_destroyed_bytes: u64,
    cas_refs_released: u32,
    containment_final_attestation: bytes(32),
    signed_receipt: bytes(4627),    // ML-DSA-87 signed proof of clean termination
}
```

### 5.3 Lifecycle Rules

| Transition | Condition | Notes |
|-----------|-----------|-------|
| Created → Active | SPARK consent for declared scope | User reviews scope visually before consenting |
| Active → Paused | User request or quiet hours | State frozen atomically; no in-flight actions lost |
| Paused → Active | User request | No re-consent needed if scope unchanged |
| Active → Modifying | User initiates scope change | Agent paused during modification |
| Modifying → Active | Scope change applied | Expanding scope requires SPARK re-consent; narrowing is immediate |
| Active → Archived | User request or project completion | State preserved, no resource consumption |
| Archived → Active | User request with SPARK re-consent | Re-consent because scope is re-activated |
| Active → Terminating | User request with SPARK confirmation | Irreversible — all state queued for destruction |
| Archived → Terminating | User request with SPARK confirmation | Can terminate archived agents too |
| Terminating → Terminated | CAS garbage collection complete | TerminationReceipt issued as proof |

---

## 6. Local-First Containment

### 6.1 Execution Model

Every personal agent runs in a local WASM sandbox on the user's device. The sandbox is provided by eStream's SmartCircuit runtime:

```
┌──────────────────────────────────────────────────────┐
│ User's Device                                        │
│                                                      │
│  ┌─────────────────────────────────────────────────┐ │
│  │ Q Mind Runtime                                  │ │
│  │                                                 │ │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐     │ │
│  │  │ Agent A  │  │ Agent B  │  │ Agent C  │     │ │
│  │  │ (WASM)   │  │ (WASM)   │  │ (WASM)   │     │ │
│  │  │          │  │          │  │          │     │ │
│  │  │ scope: A │  │ scope: B │  │ scope: C │     │ │
│  │  │ lex: /a  │  │ lex: /b  │  │ lex: /c  │     │ │
│  │  └────┬─────┘  └────┬─────┘  └────┬─────┘     │ │
│  │       │              │              │           │ │
│  │  ┌────▼──────────────▼──────────────▼────────┐ │ │
│  │  │        Containment Enforcement Layer       │ │ │
│  │  │  namespace isolation · scope verification  │ │ │
│  │  │  activity logging · containment proofs     │ │ │
│  │  └────────────────────┬──────────────────────┘ │ │
│  │                       │                         │ │
│  │  ┌────────────────────▼──────────────────────┐ │ │
│  │  │             User's Local Lex               │ │ │
│  │  │  polyqlabs/qmind/agents/a                  │ │ │
│  │  │  polyqlabs/qmind/agents/b                  │ │ │
│  │  │  polyqlabs/qmind/agents/c                  │ │ │
│  │  └───────────────────────────────────────────┘ │ │
│  └─────────────────────────────────────────────────┘ │
│                                                      │
│  ┌─────────────────────────────────────────────────┐ │
│  │ Network Egress Gate (SPARK consent per request) │ │
│  └─────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────┘
```

### 6.2 Containment Guarantees

| Guarantee | Mechanism |
|-----------|-----------|
| **Namespace isolation** | Each agent has its own lex sub-namespace. The Containment Enforcement Layer verifies every read/write against the agent's `AgentScope`. |
| **No ambient network** | WASM sandbox has no raw socket access. All network goes through the Network Egress Gate which requires SPARK consent. |
| **No cross-agent leakage** | Agents share no memory, no CAS refs, no lex paths unless explicitly granted by user-defined interaction policy. |
| **Auditable activity** | Every action is logged to `AgentActivityLog` in the user's lex. Logs are append-only and ML-DSA-87 signed. |
| **Verifiable containment** | Containment proofs (hash chains of scope + actions) can be independently verified. |
| **Clean termination** | Terminating an agent destroys all state in its lex sub-namespace. `TerminationReceipt` proves clean destruction. |

### 6.3 Containment Proof Circuit

```fl
circuit generate_containment_proof(
    agent_id: bytes(16),
    scope: AgentScope,
    activity_log_hash: bytes(32),   // Merkle root of all activity log entries
    previous_proof: option<bytes(32)>,
    signing_sk: bytes(4896),
) -> ContainmentProof

type ContainmentProof = struct {
    proof_id: bytes(16),
    agent_id: bytes(16),
    scope_hash: bytes(32),         // hash of the current AgentScope
    activity_hash: bytes(32),      // Merkle root of activity log since last proof
    previous_proof_hash: option<bytes(32)>,
    chain_length: u64,             // proof chain position
    generated_tick: u64,
    signature: bytes(4627),        // ML-DSA-87 signature
}

circuit verify_containment_proof(
    proof: ContainmentProof,
    scope: AgentScope,
    activity_log_entries: list<AgentActivityLog>,
    signing_pk: bytes(2592),
) -> ContainmentVerification

type ContainmentVerification = struct {
    valid: bool,
    chain_intact: bool,            // all proofs chain correctly
    scope_violations: list<ScopeViolation>,
    verified_tick: u64,
}

type ScopeViolation = struct {
    log_entry_id: bytes(16),
    violation_type: u8,            // 0=namespace, 1=tool, 2=network, 3=temporal, 4=content_type
    description: string,
    severity: u8,                  // 0=warning, 1=violation, 2=critical
}
```

### 6.4 External Verification

The user can verify containment at any time. The verification circuit takes the agent's scope, its full activity log, and the containment proof chain, and returns whether every action was within the declared scope. This is not a trust-me assertion — it is a cryptographic proof that can be independently checked.

```fl
circuit user_verify_agent_containment(
    user_id: bytes(16),
    agent_id: bytes(16),
    signing_pk: bytes(2592),
) -> ContainmentReport

type ContainmentReport = struct {
    agent_id: bytes(16),
    agent_name: string,
    verification_result: ContainmentVerification,
    total_actions_verified: u64,
    data_accessed_summary: DataAccessSummary,
    network_egress_summary: NetworkEgressSummary,
    report_generated_tick: u64,
}

type DataAccessSummary = struct {
    namespaces_accessed: list<string>,
    total_bytes_read: u64,
    total_bytes_written: u64,
    unique_items_accessed: u32,
}

type NetworkEgressSummary = struct {
    total_requests: u32,
    unique_destinations: list<string>,
    total_bytes_sent: u64,
    total_bytes_received: u64,
    all_consented: bool,           // every request had SPARK consent
}
```

---

## 7. Agent Scoping

Users define what each agent can and cannot access through four scoping dimensions. All four dimensions compose conjunctively — an agent must satisfy all applicable dimensions to access any data.

### 7.1 Namespace Scoping

The primary access control. Users grant agents read and/or write access to specific lex namespaces.

```fl
circuit set_agent_namespace_grants(
    user_id: bytes(16),
    agent_id: bytes(16),
    readable: list<NamespaceGrant>,
    writable: list<NamespaceGrant>,
    spark_consent: bytes(64),
    signing_sk: bytes(4896),
) -> bool
```

**Examples**:
- `polyqlabs/qmind/corpus/finances` — agent can see financial documents
- `polyqlabs/qmind/corpus/projects/renovation` — agent can see renovation project files
- `polyqlabs/qmind/growth` — Growth Agent can see growth profile (read-only)

Namespace grants support recursive access (a grant to a parent namespace includes all children) and field restrictions (a grant can limit which fields within a namespace are visible).

### 7.2 Content-Type Scoping

Restricts agents by the type of content they can access, regardless of namespace.

```fl
circuit set_agent_content_filter(
    user_id: bytes(16),
    agent_id: bytes(16),
    filter: ContentTypeFilter,
    signing_sk: bytes(4896),
) -> bool
```

**Examples**:
- Agent can read documents and notes but not voice recordings
- Agent can access spreadsheets but not photos
- Agent can read text-based content but not multimedia

### 7.3 Temporal Scoping

Restricts agents to data within a time window. Useful for Research Agents investigating a specific period or Project Agents that should only see recent data.

```fl
circuit set_agent_temporal_bounds(
    user_id: bytes(16),
    agent_id: bytes(16),
    bounds: TemporalBounds,
    signing_sk: bytes(4896),
) -> bool
```

**Examples**:
- Research Agent can only access documents from the last 12 months
- Project Agent can only see data created after the project start date
- Guardian Agent has no temporal bounds (needs historical context for trend analysis)

### 7.4 Tool Scoping

Controls which tools (capabilities) an agent can use.

```fl
circuit set_agent_tool_permissions(
    user_id: bytes(16),
    agent_id: bytes(16),
    permissions: list<ToolPermission>,
    spark_consent: bytes(64),
    signing_sk: bytes(4896),
) -> bool
```

**Built-in tools**:

| Tool | Description | Default |
|------|-------------|---------|
| `web_search` | Search the web via privacy-preserving relay | Blocked |
| `web_fetch` | Fetch a specific URL | Blocked |
| `api_call` | Call an external API endpoint | Blocked |
| `send_notification` | Send alert to Q Mind inbox | Permitted |
| `write_document` | Create/edit documents in writable namespaces | Permitted |
| `eslm_inference` | Run ESLM inference on in-scope data | Permitted |
| `eslm_training` | Contribute to ESLM fine-tuning within scope | Permitted |
| `calendar_read` | Read Q Calendar events | Blocked |
| `calendar_write` | Create Q Calendar events | Blocked |
| `task_create` | Create Q Tasks items | Blocked |
| `knowledge_graph_write` | Add/modify knowledge graph nodes | Permitted |

### 7.5 Scope Change Semantics

| Direction | User Experience | Consent |
|-----------|----------------|---------|
| **Narrowing** | Immediate — agent loses access instantly | No re-consent needed |
| **Expanding** | Requires SPARK biometric consent | User sees visual diff of old vs. new scope |
| **Lateral** | Treated as expand + narrow | Requires SPARK consent for the expansion portion |

---

## 8. Personal Containment Dashboard

The dashboard is the user's non-technical view into what their agents are doing. No log files, no JSON — visual metaphors that anyone can understand.

### 8.1 Visual Scope Map

The scope map uses a "rooms in a house" metaphor. The user's data namespaces are rooms. Each agent is a colored icon that shows which rooms it can enter.

```fl
type ScopeMapRoom = struct {
    namespace: string,
    display_name: string,           // user-friendly name (e.g., "Finances", "Health")
    icon: string,
    agents_with_read: list<bytes(16)>,
    agents_with_write: list<bytes(16)>,
}

type ScopeMapView = struct {
    rooms: list<ScopeMapRoom>,
    agents: list<AgentScopeMapEntry>,
}

type AgentScopeMapEntry = struct {
    agent_id: bytes(16),
    display_name: string,
    color: string,                  // hex color for visual distinction
    agent_type: u8,
    status: u8,
    rooms_readable: list<string>,
    rooms_writable: list<string>,
}

circuit get_scope_map(
    user_id: bytes(16),
) -> ScopeMapView
```

**User sees**: A visual grid where each cell shows whether an agent has access. Green for read, blue for read+write, gray for no access. At a glance, the user can see that "Finance Advisor" can see the Finances room but not the Health room.

### 8.2 Activity Timeline

Plain-language activity log showing what each agent has done:

```fl
type ActivityTimelineEntry = struct {
    agent_name: string,
    action_summary: string,         // e.g., "Read 3 documents from Finances"
    detail: string,                 // e.g., "Analyzed Q3 tax documents for deduction opportunities"
    data_flow: DataFlowSummary,
    timestamp: u64,
}

type DataFlowSummary = struct {
    data_in_description: string,    // "Read 3 documents (12 KB)"
    data_out_description: string,   // "Wrote 1 summary (2 KB) to Finances/Reports"
    external_description: string,   // "No external communication" or "Searched web (2 queries, SPARK-consented)"
}

circuit get_activity_timeline(
    user_id: bytes(16),
    agent_id: option<bytes(16)>,    // filter to specific agent, or all agents
    limit: u32,
    offset: u32,
) -> list<ActivityTimelineEntry>
```

### 8.3 Containment Attestation Card

A simple card showing the containment status for each agent:

```fl
type ContainmentCard = struct {
    agent_id: bytes(16),
    agent_name: string,
    status_label: string,           // "Verified Clean", "Pending Verification", "Violation Detected"
    last_verified_tick: u64,
    proof_chain_length: u64,
    total_actions_since_creation: u64,
    violations_detected: u32,       // should be 0
    next_auto_verification_tick: u64,
}

circuit get_containment_cards(
    user_id: bytes(16),
) -> list<ContainmentCard>
```

**User sees**: A green shield icon with "Verified Clean — last checked 2 hours ago" or a red warning icon with details.

### 8.4 Data Flow Visualization

Shows what went in, what came out, and confirms nothing leaked:

```fl
type DataFlowVisualization = struct {
    agent_id: bytes(16),
    period: u8,                     // 0=today, 1=this_week, 2=this_month, 3=all_time
    total_data_in_bytes: u64,
    total_data_out_bytes: u64,      // data written to user's lex
    total_external_bytes_sent: u64, // should be 0 unless web search granted
    total_external_bytes_received: u64,
    namespaces_accessed: list<NamespaceAccessSummary>,
    external_destinations: list<ExternalDestinationSummary>,
}

type NamespaceAccessSummary = struct {
    namespace: string,
    display_name: string,
    bytes_read: u64,
    bytes_written: u64,
    items_accessed: u32,
}

type ExternalDestinationSummary = struct {
    destination: string,
    request_count: u32,
    bytes_sent: u64,
    bytes_received: u64,
    all_consented: bool,
}

circuit get_data_flow_visualization(
    user_id: bytes(16),
    agent_id: bytes(16),
    period: u8,
) -> DataFlowVisualization
```

---

## 9. Agent Communication

### 9.1 Agent-to-User Communication

Agents deliver insights, alerts, summaries, and recommendations to the user's Q Mind inbox. The inbox is a unified view across all agents, filterable by agent and by message type.

```fl
circuit agent_send_message(
    agent_id: bytes(16),
    subject: string,
    body: string,
    content_type: u8,              // MessageContentType
    priority: u8,
    attachments: list<bytes(16)>,  // CAS refs within agent's writable namespace
    signing_sk: bytes(4896),
) -> AgentMessage

circuit get_agent_inbox(
    user_id: bytes(16),
    agent_filter: option<bytes(16)>,
    type_filter: option<u8>,
    unread_only: bool,
    limit: u32,
    offset: u32,
) -> list<AgentMessage>
```

### 9.2 User-to-Agent Communication

Users interact with agents through natural language queries and task assignments.

```fl
circuit user_query_agent(
    user_id: bytes(16),
    agent_id: bytes(16),
    query: string,
    signing_sk: bytes(4896),
) -> AgentMessage

circuit user_assign_task(
    user_id: bytes(16),
    agent_id: bytes(16),
    task_description: string,
    deadline_tick: option<u64>,
    signing_sk: bytes(4896),
) -> AgentMessage
```

### 9.3 Agent-to-Agent Communication

Agents can communicate with each other, but only under user-defined interaction policies. All inter-agent messages stay within the user's lex — no agent can cause another agent to breach its containment.

```fl
type AgentInteractionPolicy = struct {
    policy_id: bytes(16),
    source_agent_id: bytes(16),
    target_agent_id: bytes(16),
    direction: u8,                 // 0=one_way, 1=bidirectional
    allowed_content_types: list<u8>,
    max_messages_per_day: u32,
    requires_user_approval: bool,  // if true, user must approve each inter-agent message
    granted_tick: u64,
    granted_by_consent_id: bytes(16),
}

circuit set_agent_interaction_policy(
    user_id: bytes(16),
    policy: AgentInteractionPolicy,
    spark_consent: bytes(64),
    signing_sk: bytes(4896),
) -> bool

circuit agent_send_to_agent(
    source_agent_id: bytes(16),
    target_agent_id: bytes(16),
    subject: string,
    body: string,
    content_type: u8,
    signing_sk: bytes(4896),
) -> AgentMessage
```

**Example interaction**: A Guardian Agent monitoring investments detects a significant portfolio change. It sends a message to the Domain Agent (Personal Finance), which then synthesizes the investment change with the user's overall financial picture and delivers a contextualized summary to the user.

### 9.4 No Agent-to-External Without Consent

The absolute rule: no agent can communicate externally without explicit SPARK consent per request. This applies even if the agent has `PreApprovedDomains` network egress — the pre-approval only covers the domain list, not arbitrary destinations.

```fl
circuit request_external_communication(
    agent_id: bytes(16),
    destination: string,
    request_body_hash: bytes(32),  // agent shows what it wants to send
    purpose: string,               // plain-language explanation
) -> ExternalCommRequest

type ExternalCommRequest = struct {
    request_id: bytes(16),
    agent_id: bytes(16),
    destination: string,
    request_body_hash: bytes(32),
    purpose: string,
    status: u8,                    // 0=pending_consent, 1=approved, 2=denied
    consent_id: option<bytes(16)>,
}

circuit approve_external_communication(
    user_id: bytes(16),
    request_id: bytes(16),
    spark_consent: bytes(64),
    signing_sk: bytes(4896),
) -> bool
```

---

## 10. Growth Agent Integration

The Growth Agent is a specialized agent type that bridges Q Mind Personal Agents with Q Mind's Enneagram-centered personal growth module (see [POLYMIND_PERSONAL_GROWTH_SPEC.md](POLYMIND_PERSONAL_GROWTH_SPEC.md)).

### 10.1 Growth Agent Scope

A Growth Agent automatically receives read access to `polyqlabs/qmind/growth` (the growth module's namespace) in addition to any user-defined scope. It cannot write to the growth namespace directly — growth evidence is created through the growth module's circuits, with the Growth Agent providing context.

```fl
type GrowthAgentConfig = struct {
    agent_id: bytes(16),
    enneagram_type: u8,            // user's core Enneagram type
    integration_type: u8,          // growth direction type
    disintegration_type: u8,       // stress direction type
    active_growth_actions: list<bytes(16)>,  // GrowthAction IDs to track
    coaching_style: u8,            // CoachingStyle
    check_in_frequency: u8,        // 0=daily, 1=weekly, 2=biweekly, 3=monthly
    content_recommendations: bool, // whether to recommend content aligned with growth
}

type CoachingStyle = enum {
    Gentle,            // supportive, non-confrontational
    Direct,            // clear, straightforward feedback
    Socratic,          // questions that lead to self-discovery
    Challenging,       // push the user beyond comfort zone
    Adaptive,          // ESLM learns the user's preferred style over time
}

circuit create_growth_agent(
    user_id: bytes(16),
    config: GrowthAgentConfig,
    additional_scope: AgentScope,  // user's additional scope grants beyond growth namespace
    signing_sk: bytes(4896),
) -> PersonalAgent
```

### 10.2 Growth Pattern Detection

The Growth Agent monitors user behavior within its scope for patterns relevant to Enneagram growth:

```fl
type GrowthPatternDetection = struct {
    detection_id: bytes(16),
    agent_id: bytes(16),
    pattern_type: u8,              // GrowthPatternType
    description: string,
    enneagram_relevance: string,   // how this pattern relates to the user's type
    direction: u8,                 // 0=integration, 1=disintegration, 2=neutral
    confidence_bps: u16,
    evidence_refs: list<bytes(16)>,
    detected_tick: u64,
}

type GrowthPatternType = enum {
    IntegrationBehavior,   // behavior consistent with integration direction
    DisintegrationSign,    // behavior consistent with stress/disintegration
    WingExpression,        // wing type qualities emerging
    StrengthApplication,   // applying a strength in a growth-relevant way
    AvoidancePattern,      // avoiding growth areas (type-specific)
    BreakthroughMoment,    // significant shift toward healthier expression
}

circuit detect_growth_patterns(
    agent_id: bytes(16),
    user_behavior: list<AgentActivityLog>,  // recent behavior within scope
    enneagram_profile: EnneagramProfile,
    active_actions: list<GrowthAction>,
) -> list<GrowthPatternDetection>
```

### 10.3 Content Alignment

The Growth Agent can recommend content that aligns with the user's active growth directions, drawing from the user's corpus and (with consent) external sources:

```fl
circuit recommend_growth_aligned_content(
    agent_id: bytes(16),
    active_actions: list<GrowthAction>,
    enneagram_profile: EnneagramProfile,
    corpus_namespace: string,      // where to search for content
    include_external: bool,        // search external sources (requires network consent)
) -> list<GrowthContentRecommendation>
```

### 10.4 Growth Evidence Auto-Creation

When the Growth Agent detects behavior or interactions relevant to active growth actions, it can auto-create growth evidence entries (pending user confirmation):

```fl
circuit auto_create_growth_evidence(
    agent_id: bytes(16),
    action_id: bytes(16),
    pattern: GrowthPatternDetection,
    signing_sk: bytes(4896),
) -> GrowthEvidence  // created with evidence_type = behavioral, pending user confirmation
```

---

## 11. CE Meaning Domain

### 11.1 `knowledge/personal_agents`

New meaning domain for the Cognitive Engine, added alongside the existing four domains:

| Signal | Type | Description |
|--------|------|-------------|
| `agent_count` | gauge | Active personal agents (status = Active) |
| `agent_action_velocity` | gauge | Total agent actions per day across all agents (rolling 24h) |
| `containment_violations` | counter | Containment boundary violations detected (should always be 0) |
| `scope_utilization` | gauge | Fraction of granted scope each agent actually uses (mean across agents) |
| `agent_communication_volume` | gauge | Inter-agent messages per day |
| `external_egress_rate` | gauge | External network requests per day (all agents combined) |
| `user_interaction_rate` | gauge | User-to-agent interactions per day |
| `eslm_context_growth` | gauge | ESLM context bytes accumulated across all agents (rolling 7d) |
| `containment_proof_freshness` | gauge | Mean age of most recent containment proof across agents (seconds) |
| `terminated_agents_total` | counter | Total agents terminated (lifecycle health indicator) |

### 11.2 Noise Filter

#### Suppression Rules

| Rule | Pattern | Reason |
|------|---------|--------|
| `suppress_guardian_heartbeat` | `agent_action_velocity` spikes from Guardian Agent periodic checks | Heartbeat checks are maintenance, not meaningful activity |
| `suppress_scope_check` | `agent_action_velocity` includes routine containment self-checks | Containment verification is overhead, not user-facing work |
| `suppress_bulk_ingest` | `scope_utilization` spike during initial agent onboarding | New agents read heavily during initial context building |

#### Signal Amplification

| Rule | Pattern | Reason |
|------|---------|--------|
| `amplify_containment_violation` | Any `containment_violations` increment | Containment violations are always critical |
| `amplify_scope_dormancy` | `scope_utilization` drops below 10% for 30+ days | Agent may be abandoned — suggest archive or termination |
| `amplify_egress_anomaly` | `external_egress_rate` exceeds 3x rolling 7d average | Unusual external communication warrants investigation |

### 11.3 SME Panel: Agent Ecosystem Health

Evaluates the overall health and effectiveness of the user's personal agent ecosystem.

- **Metrics observed**: `agent_count`, `agent_action_velocity`, `containment_violations`, `scope_utilization`, `containment_proof_freshness`
- **Trigger**: Weekly scheduled + on-demand when `containment_violations` > 0
- **Output**: Ecosystem health score (0–100), agent utilization report, containment compliance status, recommendations for agent consolidation or termination

---

## 12. App Graph Integration

### 12.1 Module Registration

`qmind_agents` registers as module 15 in the Q Mind App Graph (updating module count from 14 to 15).

| Module | Partition | SLA | Description |
|--------|-----------|-----|-------------|
| `qmind_agents` | Backend | Premium | Personal agent lifecycle, containment, scoping, communication |

### 12.2 Updated App Graph — 15 Modules

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
| `qmind_agents` | Backend | Premium | Personal agent lifecycle, containment, scoping, communication |

### 12.3 Intra-Graph Dependencies

```
qmind_agents -> knowledge_graph, qmind_rbac, qmind_growth
qmind_agents -> qmind_eslm, qmind_classify, qmind_insight
qmind_agents -> qmind_metering
```

- **knowledge_graph**: Agent state, activity logs, and scope maps are nodes in the KG
- **qmind_rbac**: Agent namespace grants are enforced through the existing RBAC layer
- **qmind_growth**: Growth Agent reads growth profile and creates growth evidence
- **qmind_eslm**: Agents use ESLM for inference and contribute to fine-tuning within scope
- **qmind_classify**: Agent-ingested data is classified for scope enforcement and content filtering
- **qmind_insight**: Agents feed patterns to the insight engine; insights inform agent recommendations
- **qmind_metering**: Agent resource consumption is metered per-user

### 12.4 Updated Dependency Graph

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
qmind_agents -> knowledge_graph, qmind_rbac, qmind_growth, qmind_eslm, qmind_classify, qmind_insight, qmind_metering
```

### 12.5 Bridge Edge

```
qmind_agents -> paragon/aegis/agent_governance (BRIDGE_TO, optional)
```

Optional bridge to Paragon Aegis for enterprise agent governance context. See Section 13 for details.

---

## 13. Privacy and Governance

### 13.1 Zero-Linkage Compliance

Personal agent data is strictly within the user's `polyqlabs/qmind/agents` lex namespace. The HKDF context `q-mind-agents-v1` derives a separate key hierarchy for agent data encryption:

```
HKDF("q-mind-agents-v1", user_master_key)
  └── agent_{id}_key = HKDF("agent-{agent_id}", agents_root_key)
       ├── agent_{id}_state_key    // encrypts agent state and configuration
       ├── agent_{id}_log_key      // encrypts activity logs
       ├── agent_{id}_comm_key     // encrypts agent messages
       └── agent_{id}_proof_key    // signs containment proofs
```

Each agent has its own derived key. Destroying an agent's key material makes all its encrypted data irrecoverable — this is the cryptographic guarantee behind clean termination.

### 13.2 Cross-Product Isolation

Personal agents cannot access data from other PolyQ Labs products (Q Docs, Q Messenger, Q Files, etc.) unless the user explicitly bridges the products. Even then, the bridge grants read-only access to specific namespaces within the other product, subject to that product's own zero-linkage constraints.

### 13.3 Paragon Bridge (Opt-In)

If the user belongs to a Paragon Aegis-enabled organization, they can opt-in to a bridge that provides bidirectional context:

**Inbound (Org → Personal)**:
- Personal agents can read organizational knowledge bases and policy documents
- Read-only access, scoped to what the user's organizational role already permits
- No organizational data is copied into the personal agent's lex — it is accessed in-place via the bridge

**Outbound (Personal → Org)**:
- Only if the user explicitly opts-in per agent
- Only aggregate signals are shared (e.g., "this user's Project Agent is working on topic X")
- No raw personal data, activity logs, or agent messages flow to the organization
- `RedactionPolicy` strips all personal context before bridge transmission

**Bridge Controls**:

```fl
type ParagonAgentBridge = struct {
    bridge_id: bytes(16),
    user_id: bytes(16),
    agent_id: bytes(16),
    org_id: bytes(16),
    inbound_namespaces: list<string>,  // organizational namespaces this agent can read
    outbound_signals: list<string>,    // which aggregate signals to share with org
    bridge_status: u8,                 // 0=inactive, 1=active, 2=suspended
    created_tick: u64,
    consent_id: bytes(16),
}

circuit create_paragon_bridge(
    user_id: bytes(16),
    agent_id: bytes(16),
    org_id: bytes(16),
    inbound_namespaces: list<string>,
    outbound_signals: list<string>,
    spark_consent: bytes(64),
    signing_sk: bytes(4896),
) -> ParagonAgentBridge

circuit revoke_paragon_bridge(
    user_id: bytes(16),
    bridge_id: bytes(16),
    signing_sk: bytes(4896),
) -> bool
```

### 13.4 Subpoena Surface

Agent data is encrypted at rest with per-agent keys derived from the user's SPARK identity. A subpoena targeting the user's device would require the user's biometric to decrypt. A subpoena targeting the server would yield nothing — agent state is local-first and encrypted with keys the server never sees.

If the user has opted-in to encrypted scatter-sync across devices, the synced data is encrypted with keys derived from the user's SPARK. No relay node or sync intermediary can read agent state.

### 13.5 Data Retention and Garbage Collection

```fl
type AgentRetentionPolicy = struct {
    activity_log_retention_days: u32,  // how long to keep detailed activity logs
    message_retention_days: u32,       // how long to keep agent messages
    containment_proof_retention: u8,   // 0=keep_all, 1=keep_last_100, 2=keep_last_year
    auto_archive_after_inactive_days: option<u32>,  // auto-archive if agent is inactive
    auto_terminate_after_archived_days: option<u32>, // auto-terminate archived agents
}

circuit set_agent_retention_policy(
    user_id: bytes(16),
    agent_id: bytes(16),
    policy: AgentRetentionPolicy,
    signing_sk: bytes(4896),
) -> bool

circuit run_agent_garbage_collection(
    user_id: bytes(16),
    agent_id: bytes(16),
    signing_sk: bytes(4896),
) -> GarbageCollectionReport

type GarbageCollectionReport = struct {
    agent_id: bytes(16),
    logs_purged: u32,
    messages_purged: u32,
    proofs_purged: u32,
    bytes_reclaimed: u64,
    gc_tick: u64,
}
```

---

## 14. Circuit Inventory

### 14.1 Full Circuit List

| Circuit | Purpose |
|---------|---------|
| `create_agent` | Create a new personal agent with type, purpose, scope, and containment profile |
| `activate_agent` | Activate a created agent after SPARK consent |
| `pause_agent` | Freeze agent state (resumable) |
| `resume_agent` | Resume a paused agent |
| `modify_agent_scope` | Change agent scope (narrowing immediate, expanding requires consent) |
| `archive_agent` | Archive an agent (state preserved, agent inactive) |
| `reactivate_agent` | Reactivate an archived agent with SPARK re-consent |
| `terminate_agent` | Destroy agent and all its state |
| `set_agent_namespace_grants` | Configure namespace read/write grants |
| `set_agent_content_filter` | Configure content-type restrictions |
| `set_agent_temporal_bounds` | Configure time-window restrictions |
| `set_agent_tool_permissions` | Configure tool/capability grants |
| `generate_containment_proof` | Generate a cryptographic containment proof |
| `verify_containment_proof` | Verify a containment proof against scope and activity |
| `user_verify_agent_containment` | Full containment report for a user's agent |
| `agent_send_message` | Agent sends a message to the user |
| `get_agent_inbox` | Retrieve messages from agent inbox |
| `user_query_agent` | User sends a query to an agent |
| `user_assign_task` | User assigns a task to an agent |
| `set_agent_interaction_policy` | Define inter-agent communication rules |
| `agent_send_to_agent` | Agent sends a message to another agent |
| `request_external_communication` | Agent requests network egress |
| `approve_external_communication` | User approves an external communication request |
| `create_growth_agent` | Create a Growth Agent with growth-specific config |
| `detect_growth_patterns` | Detect Enneagram growth patterns from behavior |
| `recommend_growth_aligned_content` | Recommend content aligned with growth directions |
| `auto_create_growth_evidence` | Auto-create growth evidence from pattern detection |
| `get_scope_map` | Generate visual scope map for dashboard |
| `get_activity_timeline` | Retrieve plain-language activity timeline |
| `get_containment_cards` | Retrieve containment status cards for all agents |
| `get_data_flow_visualization` | Generate data flow visualization for an agent |
| `set_agent_retention_policy` | Configure data retention and auto-archive/terminate |
| `run_agent_garbage_collection` | Execute garbage collection for an agent |
| `create_paragon_bridge` | Create optional bridge to Paragon Aegis |
| `revoke_paragon_bridge` | Revoke Paragon Aegis bridge |
| `get_agent_dashboard` | Aggregate all agent data for main dashboard view |
| `get_agent_ecosystem_health` | Compute ecosystem health score for SME panel |

### 14.2 Dashboard Aggregation

```fl
circuit get_agent_dashboard(
    user_id: bytes(16),
) -> AgentDashboard

type AgentDashboard = struct {
    agents: list<AgentSummary>,
    scope_map: ScopeMapView,
    recent_activity: list<ActivityTimelineEntry>,
    containment_cards: list<ContainmentCard>,
    ecosystem_health: EcosystemHealth,
    unread_messages: u32,
    pending_consent_requests: u32,
}

type AgentSummary = struct {
    agent_id: bytes(16),
    display_name: string,
    agent_type: u8,
    status: u8,
    purpose_summary: string,
    actions_today: u32,
    last_active_tick: u64,
    containment_status: string,
    scope_utilization_bps: u16,
}

type EcosystemHealth = struct {
    score: u16,                    // 0-10000 bps
    active_agents: u32,
    total_agents: u32,
    containment_compliant: bool,
    dormant_agents: u32,           // active but unused for 30+ days
    recommendations: list<string>, // plain-language health recommendations
}

circuit get_agent_ecosystem_health(
    user_id: bytes(16),
    agents: list<PersonalAgent>,
    activity_logs: list<AgentActivityLog>,
    containment_proofs: list<ContainmentProof>,
) -> EcosystemHealth
```

---

## 15. Strategic Grant Configuration

### 15.1 eStream Grant

Q Mind Personal Agents consumes the following eStream platform primitives under the PolyQ Labs commercial license:

- `scatter-cas` — Agent state storage with per-agent key encryption
- `SPARK` — Per-agent sub-key derivation (HKDF context: `q-mind-agents-v1`)
- `StreamSight` — Observability within `polyqlabs/qmind/agents` lex namespace
- `ESLM` — On-device SSM micro-inference for agent reasoning (BitNet b1.58)
- `ML-KEM-1024` / `ML-DSA-87` — PQ encryption + signatures for agent state, proofs, and messages
- `Containment Proof` — Cryptographic proof framework for agent scope verification
- `AI Agent Architecture` — SmartCircuit agent runtime (WASM sandbox, lifecycle management)
- `Cortex` — AI advisor observing agent behavior for anomaly detection

### 15.2 Paragon Grant

Q Mind Personal Agents may optionally bridge to Paragon Aegis for enterprise context:

- `paragon/aegis` → Enterprise agent governance framework
- `paragon/aegis` → Organizational knowledge base access (read-only, role-scoped)
- Bridge is opt-in per agent per user; Q Mind Personal Agents operates fully independently without Paragon

---

## 16. Future Considerations

- **Agent Templates**: Pre-configured agent templates for common use cases (Financial Advisor, Research Assistant, Health Coach) that users can instantiate with one click
- **Agent Marketplace**: Community-created agent templates (scope definitions, not data) shared via eStream Marketplace with containment-verified badges
- **Multi-Device Agent Migration**: Seamlessly move an agent's execution context between devices while maintaining containment guarantees
- **Agent Collaboration Patterns**: Formalized patterns for multi-agent workflows (e.g., Research Agent feeds Guardian Agent which alerts Project Agent)
- **Voice Interface**: Natural language voice interaction with agents via Q Phone bridge
- **T0 Hardware Containment**: When Companion SoC is available, agents can run in hardware TEE with pWASM attestation — containment is physically enforced, not just cryptographically proven
- **Family Agent Sharing**: Share a read-only view of an agent's outputs (not its scope or data) with family members via Q Messenger, for collaborative use cases like shared project planning
- **Agent Memory Compression**: Long-running Domain Agents accumulate context; periodic ESLM-driven compression reduces storage while preserving key patterns
- **Regulatory Compliance Agents**: Specialized agent templates that implement regulatory monitoring (tax deadline tracking, compliance calendar) with pre-defined scope and containment
