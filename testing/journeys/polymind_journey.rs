//! Poly Mind Journey Tests
//!
//! End-to-end journey for Poly Mind: document ingestion, corpus classification,
//! knowledge graph queries, insight generation, legacy snapshots,
//! and blind telemetry — following the eStream Convoy pattern.

use estream_test::{
    Journey, JourneyParty, JourneyStep, StepAction, JourneyMetrics,
    assert_metric_emitted, assert_blinded, assert_povc_witness,
};
use estream_test::convoy::{ConvoyContext, ConvoyResult};
use estream_test::stratum::{StratumVerifier, CsrTier, SeriesMerkleChain};
use estream_test::cortex::{CortexVisibility, RedactPolicy, ObfuscatePolicy};

pub struct PolymindJourney;

impl Journey for PolymindJourney {
    fn name(&self) -> &str {
        "polymind_e2e"
    }

    fn description(&self) -> &str {
        "End-to-end journey for PolyMind: document ingestion, classification, querying, insight generation, legacy snapshot, knowledge graph and blind telemetry"
    }

    fn parties(&self) -> Vec<JourneyParty> {
        vec![
            JourneyParty::new("alice")
                .with_spark_context("poly-mind-v1")
                .with_role("knowledge_owner"),
            JourneyParty::new("bob")
                .with_spark_context("poly-mind-v1")
                .with_role("authorized_querier"),
            JourneyParty::new("eslm_engine")
                .with_spark_context("poly-mind-v1")
                .with_role("inference_engine"),
        ]
    }

    fn steps(&self) -> Vec<JourneyStep> {
        vec![
            // Step 1: Alice ingests a document into her personal corpus
            JourneyStep::new("alice_ingests_document")
                .party("alice")
                .action(StepAction::Execute(|ctx: &mut ConvoyContext| {
                    let document_content = ctx.generate_test_payload(1024 * 128); // 128 KiB
                    let ingest_result = ctx.polymind().ingest(
                        "journal-2025.md",
                        &document_content,
                        "markdown",
                        &[("classification", "personal"), ("sensitivity", "high")],
                    )?;

                    ctx.set("document_id", &ingest_result.document_id);
                    ctx.set("corpus_id", &ingest_result.corpus_id);

                    assert!(!ingest_result.document_id.is_empty());
                    assert!(ingest_result.encrypted_at_rest);
                    assert!(ingest_result.chunks_created >= 1);
                    assert!(ingest_result.embeddings_generated);

                    assert_metric_emitted!(ctx, "polymind.document.ingested", {
                        "format" => "markdown",
                        "chunks" => &ingest_result.chunks_created.to_string(),
                        "encrypted" => "true",
                    });

                    assert_povc_witness!(ctx, "polymind.ingest", {
                        witness_type: "document_ingestion",
                        document_id: &ingest_result.document_id,
                    });

                    assert_blinded!(ctx, "polymind.document.ingested", {
                        field: "document_content",
                        blinding: "absent",
                    });

                    assert_blinded!(ctx, "polymind.document.ingested", {
                        field: "owner_id",
                        blinding: "hmac_sha3",
                    });

                    Ok(())
                }))
                .timeout_ms(15_000),

            // Step 2: Corpus classified by ESLM
            JourneyStep::new("corpus_classified")
                .party("eslm_engine")
                .depends_on(&["alice_ingests_document"])
                .action(StepAction::Execute(|ctx: &mut ConvoyContext| {
                    let corpus_id = ctx.get::<String>("corpus_id");
                    let document_id = ctx.get::<String>("document_id");

                    let classification = ctx.polymind().classify_corpus(&corpus_id)?;

                    assert!(!classification.categories.is_empty());
                    assert!(classification.confidence >= 0.5);
                    assert!(classification.on_device);

                    ctx.set("classification_id", &classification.classification_id);

                    let topic_graph = ctx.polymind().build_topic_graph(&corpus_id)?;
                    assert!(topic_graph.node_count >= 1);
                    assert!(topic_graph.edge_count >= 0);

                    ctx.set("graph_id", &topic_graph.graph_id);

                    assert_metric_emitted!(ctx, "polymind.corpus.classified", {
                        "categories" => &classification.categories.len().to_string(),
                        "on_device" => "true",
                    });

                    assert_povc_witness!(ctx, "polymind.classify", {
                        witness_type: "corpus_classification",
                        corpus_id: &corpus_id,
                    });

                    assert_blinded!(ctx, "polymind.corpus.classified", {
                        field: "category_content",
                        blinding: "hmac_sha3",
                    });

                    Ok(())
                }))
                .timeout_ms(20_000),

            // Step 3: Bob queries the corpus (authorized)
            JourneyStep::new("bob_queries_corpus")
                .party("bob")
                .depends_on(&["corpus_classified"])
                .action(StepAction::Execute(|ctx: &mut ConvoyContext| {
                    let corpus_id = ctx.get::<String>("corpus_id");
                    let alice_id = ctx.party_id("alice");

                    let grant = ctx.polymind().request_query_access(
                        &alice_id,
                        &corpus_id,
                        &["semantic_search"],
                    )?;
                    assert!(grant.access_granted);

                    let query_result = ctx.polymind().query(
                        &corpus_id,
                        "What were the key decisions in 2025?",
                        5, // top-k
                    )?;

                    ctx.set("query_id", &query_result.query_id);

                    assert!(!query_result.results.is_empty());
                    assert!(query_result.results.len() <= 5);
                    assert!(query_result.relevance_scores_valid());

                    assert_metric_emitted!(ctx, "polymind.query.executed", {
                        "result_count" => &query_result.results.len().to_string(),
                        "query_type" => "semantic_search",
                    });

                    assert_blinded!(ctx, "polymind.query.executed", {
                        field: "query_text",
                        blinding: "hmac_sha3",
                    });

                    assert_blinded!(ctx, "polymind.query.executed", {
                        field: "querier_id",
                        blinding: "hmac_sha3",
                    });

                    assert_povc_witness!(ctx, "polymind.query", {
                        witness_type: "corpus_query",
                        corpus_id: &corpus_id,
                        query_id: &query_result.query_id,
                    });

                    Ok(())
                }))
                .timeout_ms(12_000),

            // Step 4: Insight generated by ESLM
            JourneyStep::new("insight_generated")
                .party("eslm_engine")
                .depends_on(&["bob_queries_corpus"])
                .action(StepAction::Execute(|ctx: &mut ConvoyContext| {
                    let corpus_id = ctx.get::<String>("corpus_id");
                    let query_id = ctx.get::<String>("query_id");

                    let insight = ctx.polymind().generate_insight(
                        &corpus_id,
                        &query_id,
                        "summarize_key_themes",
                    )?;

                    ctx.set("insight_id", &insight.insight_id);

                    assert!(!insight.insight_id.is_empty());
                    assert!(!insight.content.is_empty());
                    assert!(insight.source_attribution_count >= 1);
                    assert!(insight.hallucination_check_passed);
                    assert!(insight.on_device || insight.server_encrypted);

                    assert_metric_emitted!(ctx, "polymind.insight.generated", {
                        "type" => "summarize_key_themes",
                        "source_citations" => &insight.source_attribution_count.to_string(),
                    });

                    assert_povc_witness!(ctx, "polymind.insight", {
                        witness_type: "insight_generation",
                        insight_id: &insight.insight_id,
                    });

                    assert_blinded!(ctx, "polymind.insight.generated", {
                        field: "insight_content",
                        blinding: "absent",
                    });

                    Ok(())
                }))
                .timeout_ms(20_000),

            // Step 5: Legacy snapshot captured
            JourneyStep::new("legacy_snapshot")
                .party("alice")
                .depends_on(&["insight_generated"])
                .action(StepAction::Execute(|ctx: &mut ConvoyContext| {
                    let corpus_id = ctx.get::<String>("corpus_id");
                    let graph_id = ctx.get::<String>("graph_id");

                    let snapshot = ctx.polymind().create_legacy_snapshot(
                        &corpus_id,
                        &graph_id,
                        "digital_legacy_v1",
                    )?;

                    ctx.set("snapshot_id", &snapshot.snapshot_id);

                    assert!(!snapshot.snapshot_id.is_empty());
                    assert!(snapshot.corpus_included);
                    assert!(snapshot.graph_included);
                    assert!(snapshot.encrypted);
                    assert!(snapshot.recovery_key_escrowed);

                    assert_metric_emitted!(ctx, "polymind.legacy.snapshot_created", {
                        "version" => "digital_legacy_v1",
                        "encrypted" => "true",
                    });

                    assert_povc_witness!(ctx, "polymind.legacy_snapshot", {
                        witness_type: "legacy_capture",
                        snapshot_id: &snapshot.snapshot_id,
                    });

                    assert_blinded!(ctx, "polymind.legacy.snapshot_created", {
                        field: "recovery_key",
                        blinding: "absent",
                    });

                    Ok(())
                }))
                .timeout_ms(15_000),

            // Step 6: Verify knowledge graph and Stratum storage
            JourneyStep::new("verify_knowledge_graph")
                .party("alice")
                .depends_on(&["legacy_snapshot"])
                .action(StepAction::Execute(|ctx: &mut ConvoyContext| {
                    let corpus_id = ctx.get::<String>("corpus_id");
                    let graph_id = ctx.get::<String>("graph_id");

                    let graph = ctx.polymind().get_graph(&graph_id)?;
                    assert!(graph.node_count >= 1);
                    assert!(graph.integrity_valid);
                    assert!(graph.encrypted_at_rest);

                    let stratum = StratumVerifier::new(ctx);

                    let csr_report = stratum.verify_csr_tiers(&corpus_id)?;
                    assert!(csr_report.tier_matches(CsrTier::Warm));
                    assert!(csr_report.encryption_at_rest);

                    let merkle = stratum.verify_series_merkle_chain(&corpus_id)?;
                    assert!(merkle.chain_intact);
                    assert!(merkle.root_hash_valid);
                    assert!(merkle.series_count >= 1);

                    let cortex = CortexVisibility::new(ctx);
                    cortex.assert_redacted("polymind.corpus", RedactPolicy::ContentFields)?;
                    cortex.assert_obfuscated("polymind.corpus", ObfuscatePolicy::PartyIdentifiers)?;

                    assert_metric_emitted!(ctx, "polymind.stratum.verified", {
                        "csr_tier" => "warm",
                        "chain_intact" => "true",
                    });

                    Ok(())
                }))
                .timeout_ms(12_000),

            // Step 7: Verify blind telemetry and namespace isolation
            JourneyStep::new("verify_blind_telemetry")
                .party("alice")
                .depends_on(&["verify_knowledge_graph"])
                .action(StepAction::Execute(|ctx: &mut ConvoyContext| {
                    let telemetry = ctx.streamsight().drain_telemetry("poly-mind-v1");

                    for event in &telemetry {
                        assert_blinded!(ctx, &event.event_type, {
                            field: "user_id",
                            blinding: "hmac_sha3",
                        });

                        assert_blinded!(ctx, &event.event_type, {
                            field: "document_content",
                            blinding: "absent",
                        });

                        assert_blinded!(ctx, &event.event_type, {
                            field: "query_text",
                            blinding: "hmac_sha3",
                        });

                        assert_blinded!(ctx, &event.event_type, {
                            field: "insight_content",
                            blinding: "absent",
                        });
                    }

                    let cortex = CortexVisibility::new(ctx);
                    cortex.assert_redacted("polymind", RedactPolicy::ContentFields)?;
                    cortex.assert_obfuscated("polymind", ObfuscatePolicy::PartyIdentifiers)?;

                    assert!(telemetry.len() >= 6, "Expected at least 6 telemetry events");

                    let namespaces: Vec<&str> = telemetry
                        .iter()
                        .map(|e| e.namespace.as_str())
                        .collect();
                    for ns in &namespaces {
                        assert!(
                            ns.starts_with("poly-mind-v1"),
                            "Telemetry must stay within poly-mind-v1 namespace, found: {}",
                            ns,
                        );
                    }

                    Ok(())
                }))
                .timeout_ms(5_000),
        ]
    }

    fn metrics(&self) -> JourneyMetrics {
        JourneyMetrics {
            expected_events: vec![
                "polymind.document.ingested",
                "polymind.corpus.classified",
                "polymind.query.executed",
                "polymind.insight.generated",
                "polymind.legacy.snapshot_created",
                "polymind.stratum.verified",
            ],
            max_duration_ms: 100_000,
            required_povc_witnesses: 6,
            lex_namespace: "poly-mind-v1",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use estream_test::convoy::ConvoyRunner;

    #[tokio::test]
    async fn run_polymind_journey() {
        let runner = ConvoyRunner::new()
            .with_streamsight("poly-mind-v1")
            .with_stratum()
            .with_cortex()
            .with_eslm();

        runner.run(PolymindJourney).await.expect("PolyMind journey failed");
    }
}
