# Epic: PolyMind CE + App Graph Integration

| Field | Value |
|-------|-------|
| **Status** | Planned |
| **Priority** | P0 |
| **Product** | PolyMind |
| **Lex** | `polyqlabs/qmind` |
| **Spec** | `specs/POLYMIND_CE_APP_GRAPH_SPEC.md` |
| **Target** | v0.1.0 |

---

## Summary

Register PolyMind's 13 FL circuit modules into the platform App Graph, wire intra-graph REQUIRES edges and cross-graph BRIDGE_TO edges (QKit ESLM classify, PolyDocs document ingest), and configure the Cognitive Engine with 3 meaning domains, noise filter, and 3 SME panels.

---

## Tasks

### App Graph

- [ ] Implement `make_qmind_module` helper with PolyMind-common defaults
- [ ] Define 13 module circuits (ingest, document_ingest, classify, eslm, query, insight, rbac, metering, platform_health, guardian, legacy, knowledge_graph, legacy_dag)
- [ ] Implement `qmind_app_graph_register` with 19+ REQUIRES edges
- [ ] Implement `qmind_register_bridge_edges` (QKit classify, PolyDocs parser)
- [ ] Add golden tests for graph registration, bridge edges, module lookup

### CE Meaning

- [ ] Define `knowledge/corpus_growth` domain (4 signals)
- [ ] Define `knowledge/query_quality` domain (4 signals)
- [ ] Define `knowledge/legacy_governance` domain (4 signals)
- [ ] Implement noise filter (3 suppress + 3 amplify rules)
- [ ] Define 3 SME panels (knowledge graph quality, ESLM training effectiveness, legacy governance compliance)
- [ ] Implement `qmind_register_ce` orchestrator
- [ ] Add golden tests for domains, noise filter, panels, registration

### Integration

- [ ] Verify bilateral bridge confirmation with QKit eslm_classify
- [ ] Verify bilateral bridge confirmation with PolyDocs parser
- [ ] Register PolyMind in platform inventory (`estream/circuits/services/capabilities/`)
- [ ] End-to-end smoke test: graph register -> CE register -> bridge confirm

---

## Acceptance Criteria

1. `qmind_app_graph_register` adds exactly 13 modules and >=19 edges
2. `qmind_register_bridge_edges` adds 2 BRIDGE_TO cross-graph edges
3. `qmind_register_ce` registers 3 domains, 6 noise rules, 3 SME panels
4. All golden tests pass
5. Platform inventory updated with PolyMind module count
