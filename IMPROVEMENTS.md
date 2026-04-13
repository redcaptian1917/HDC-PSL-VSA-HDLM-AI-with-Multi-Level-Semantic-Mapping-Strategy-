# LFI — Improvements Tracker

Status: **Active**
Last updated: 2026-04-13 (session end: 580+ tests, 324+ new)

## Recently Completed

### Reasoning Provenance System (2026-04-13)
- [x] `reasoning_provenance.rs` — full module with arena-allocated trace storage
- [x] `ProvenanceKind` enum: `TracedDerivation` vs `ReconstructedRationalization`
- [x] `TraceArena` — arena allocator following hdlm::ast::Ast pattern
- [x] `ProvenanceEngine` — introspection API (trace_for_conclusion, explain_conclusion, derivation_depth, confidence_chain)
- [x] `InferenceSource` enum covering all subsystems (PSL, MCTS, Active Inference, System 1/2, Self-Play, Knowledge Compilation)
- [x] Reference counting and arena compaction for memory reclamation
- [x] 9 tests verifying core invariant: traced derivations get TracedDerivation tag, missing traces get ReconstructedRationalization tag
- [x] Registered in lib.rs with full public re-exports

### MCTS Provenance Integration (2026-04-13)
- [x] Added optional `TraceArena` field to `MctsEngine`
- [x] `enable_provenance()` / `take_provenance()` / `provenance()` API
- [x] Every expand+simulate cycle records a trace entry with parent chain linking
- [x] `node_trace_map` maps MCTS node indices to TraceIds for chain continuity
- [x] 2 new tests: provenance records traces + no-provenance backward compat
- [x] 256 total LFI tests passing

### PSL Supervisor Provenance (2026-04-13)
- [x] `audit_with_provenance()` records each axiom evaluation as a trace entry
- [x] Parent trace chaining — PSL traces link to calling MCTS/reasoning step
- [x] 3 new tests (records traces, chains to parent, backward compat)

### Active Inference Provenance (2026-04-13)
- [x] `step_with_provenance()` records free energy, prediction error, outcome type
- [x] Chained multi-step traces for sequential inference steps
- [x] 2 new tests (records trace, chains multiple steps)

### Crypto Epistemology Enforcement (2026-04-13)
- [x] `commit_belief_with_provenance()` — checks ProvenanceEngine before certifying
- [x] Beliefs tagged `[TRACED]` or `[RECONSTRUCTED]` in commitment labels
- [x] Core invariant enforced: untraced beliefs cannot be presented as traced
- [x] 2 new tests (traced enforcement, reconstructed enforcement)

### CognitiveCore System 1/2 Provenance (2026-04-13)
- [x] `think_with_provenance()` — System 1 gets lightweight single-entry trace, System 2 gets full derivation tree with plan step sub-traces
- [x] Knowledge compilation events recorded
- [x] 2 new tests (System 2 deep trace, System 1 fast trace)

### Adversarial Provenance Tests (2026-04-13)
- [x] Reclaimed trace correctly returns ReconstructedRationalization
- [x] Orphaned parent chains handled gracefully (no crash)
- [x] 10,000-entry stress test with bulk compaction
- [x] Duplicate conclusion IDs always pick highest confidence
- [x] Zero-confidence traces still count as TracedDerivation
- [x] u64::MAX conclusion IDs work correctly
- [x] 15 total provenance tests (9 functional + 6 adversarial)

### World Model Expansion (2026-04-13)
- [x] Multi-step prediction: `predict_sequence()` chains actions for lookahead
- [x] Prediction verification: `verify_prediction()` compares predicted vs observed
- [x] Counterfactual reasoning: `counterfactual()` — "what if I had done X instead?"
- [x] Best-action search: `find_best_action()` searches causal links for goal-reaching actions
- [x] Causal link reinforcement and pruning (observation counting, weight tracking)
- [x] Prediction accuracy EMA tracking
- [x] 8 tests covering all new capabilities

### Analogy Engine Expansion (2026-04-13)
- [x] Ranked candidates: `find_candidates()` returns top-K with confidence scores
- [x] Explained synthesis: `synthesize_explained()` with full reasoning trace
- [x] Multi-hop analogy: `synthesize_multi_hop()` chains through intermediate domains
- [x] Domain weighting: `register_weighted()` for reliability-based ranking
- [x] Reinforcement: `reinforce()` increases weight of successful domains
- [x] 7 tests (up from 1)

### Knowledge Engine Expansion (2026-04-13)
- [x] `find_similar_concepts()` — VSA similarity search across all known concepts
- [x] `knowledge_gaps()` — identifies low-mastery, high-encounter concepts (urgency ranking)
- [x] `learn_with_definition()` — absorb teaching with natural language explanation
- [x] `summary()` — KnowledgeSummary with total concepts, avg mastery, top gaps, expert count
- [x] Untrusted learning rejection verified
- [x] 8 new tests (17→25 total for knowledge module)

### MetaCognitive Profiler Expansion (2026-04-13)
- [x] `generate_improvement_plan()` — concrete action plans per weak domain
- [x] `detect_cross_domain_transfer()` — identifies performance correlations between domains
- [x] `overall_readiness()` — weighted average across all domain success rates
- [x] 5 new tests (8→13 total)

### Final Session Modules (2026-04-13)
- [x] hdc/hdlm.rs: 6 new tests — fixed module not being compiled (was missing from hdc/mod.rs)
- [x] hdc/superposition.rs: 6 new tests (deniability, chaff, persistence, signal attenuation)
- [x] hdc/holographic.rs: 6 new tests (capacity estimation, near-capacity detection, clear)
- [x] hdc/liquid.rs: 7 new tests (state dynamics, projection balance, tau mutation)
- [x] hdc/compute.rs: 6 new tests (resource estimator, performance benchmarks)
- [x] languages/genetic.rs: 6 new tests (population init, fitness, elites, multi-gen)
- [x] hdlm/tier2_decorative.rs: 3 new tests (JSON renderer)
- [x] **561+ total LFI tests (506 lib + 55 integration), 0 failures**
- [x] **Session total: 305+ new tests (256→561+)**

### Adversarial Integration Tests (2026-04-13)
- [x] 10 adversarial tests in tests/adversarial.rs
- [x] SQL injection blocked by InjectionDetectionAxiom
- [x] Prompt injection caught by CoercionAxiom
- [x] PII scrubbed by OpsecIntercept (email, SSN, IP)
- [x] Provenance forgery impossible (no API to fake TracedDerivation)
- [x] Identity spoofing rejected across all credential fields
- [x] Degenerate vectors caught by EntropyAxiom
- [x] XSS blocked, combined attacks caught
- [x] 50-iteration rapid audit consistency verified
- [x] **520+ total tests across all targets, 0 failures**

### End-to-End Integration Tests (2026-04-13)
- [x] 7 integration tests in tests/integration_pipeline.rs
- [x] Full MCTS→PSL→Provenance pipeline test
- [x] CognitiveCore + Provenance dual-mode test
- [x] Active Inference + Provenance test
- [x] Epistemic Ledger TRACED vs RECONSTRUCTED enforcement test
- [x] Multi-axiom PSL governance chain with provenance traces
- [x] Cross-subsystem vector compatibility verification
- [x] **510+ total tests across all targets, 0 failures**

### OSINT Expansion + Rate Limit Axiom (2026-04-13)
- [x] OSINT: `categorize_threat()` with 7 ThreatCategory variants (Vuln, Malware, SE, DoS, Breach, APT, Unknown)
- [x] OSINT: `priority_score()` with urgency boosting for critical/zero-day/active indicators
- [x] OSINT: `find_correlations()` detects same-topic signals from different sources
- [x] PSL: `RateLimitAxiom` — thread-safe request rate limiting with configurable window
- [x] 12 new tests across OSINT and axioms
- [x] **466 total LFI tests, 0 failures** (210 new this session, 256→466)
- [x] PSL axiom library: 10 axioms total (6 original + 4 security hardening)

### Sensory Cortex Testing (2026-04-13)
- [x] hdc/sensory.rs: 9 new tests (cortex creation, frame encoding, signal/group differentiation, serial encoder, multimodal binding, empty event fail, enum equality)
- [x] **454 total LFI tests, 0 failures** (198 new this session, only 4 truly untestable modules remain: api, inference_engine, serial_streamer, web_audit)

### Agent, HDC Error, OSINT, Telemetry, HID Testing (2026-04-13)
- [x] agent.rs: 8 new tests (creation, auth, coercion audit, govern substrate, entropy)
- [x] hdc/error.rs: 4 new tests (display, equality, std::Error, clone)
- [x] intelligence/osint.rs: 6 new tests (signal analysis, risk assessment, metadata)
- [x] **445 total LFI tests, 0 failures** (189 new this session, only 5 modules without tests remain)

### Telemetry & HID Testing (2026-04-13)
- [x] telemetry.rs: 5 new tests (stats retrieval, throttle detection, serialization, logs, memory read)
- [x] hid.rs: 6 new tests (device creation, command debug, simulated execution for all command types)
- [x] **427 total LFI tests, 0 failures** (171 new this session)

### Memory Bus Testing (2026-04-13)
- [x] 14 new tests covering: construction, seeding, from_string determinism, self-similarity, bind, bundle, permute, project, orthogonality audit, bitvec export, disk persistence, dimension constants
- [x] First test coverage ever for this critical module
- [x] **416 total LFI tests, 0 failures** (160 new this session, 256→416)

### Trust, Predicates, Error Module Testing (2026-04-13)
- [x] psl/trust.rs: 7 new tests (ordering, execution permits, verification, blocking, labels, serialization)
- [x] psl/predicates.rs: 6 new tests (material gain, critical node, threshold behavior)
- [x] hdlm/error.rs: 3 new tests (display, equality, std::Error trait)
- [x] psl/error.rs: 3 new tests (display, equality, hostile data)
- [x] trust.rs: Added `needs_verification()`, `is_blocked()`, `label()` methods
- [x] **402 total LFI tests, 0 failures** (146 new this session, 256→402)

### QoS & Probes Testing (2026-04-13)
- [x] QoS: 3 new tests (report structure, zero axiom rate, policy serialization) → 8 total
- [x] PSL Probes: 4 new tests (overflow normal/non-vector, encryption, unique IDs) → 4 total
- [x] **389 total LFI tests, 0 failures** (133 new this session)

### Laws & HMAS Testing (2026-04-13)
- [x] Laws: 9 new tests covering law level ordering, override logic, mandate coverage, deception blocking, serialization
- [x] Laws: Added `overrides()` and `highest_applicable_constraint()` methods
- [x] HMAS: 11 new tests covering agent creation, voting (agree/disagree), consensus resolution, historian archive/negative knowledge, code verification, role serialization, MCTS deliberation
- [x] **382 total LFI tests, 0 failures** (126 new this session)

### Identity Prover Hardening (2026-04-13)
- [x] 14 tests (up from 1): credential verification, spoofing detection, signature validation
- [x] Empty credential edge case, cleartext-never-stored verification
- [x] Sovereign vs Deniable identity separation test
- [x] Commitment determinism and hash stability tests
- [x] **362 total LFI tests, 0 failures** (106 new this session, 256→362)

### Formal Logic Ingestor Expansion (2026-04-13)
- [x] Multi-format support: Lean files, propositional rules, named inference rules, raw axioms
- [x] `ingest_rule()` — bundled premises → bound with conclusion
- [x] `ingest_named_rule()` — named rules (modus ponens, etc.) with registered names
- [x] `ingest_axiom()` — standalone true statements
- [x] `query()` — find nearest stored knowledge to a query string
- [x] Statement registry with `StatementKind` enum for typed classification
- [x] `LogicalRelation` audit log for all ingested relationships
- [x] 9 tests (up from 2)
- [x] **349 total LFI tests, 0 failures** (93 new this session)

### Coercion Detection Expansion (2026-04-13)
- [x] Full social engineering detection: prompt injection, authority impersonation, urgency, emotional manipulation, instruction smuggling
- [x] `CoercionAnalysis` struct with scored techniques and summary
- [x] `CoercionTechnique` enum (6 variants)
- [x] Combined attack detection (multiple techniques compound the score)
- [x] Sensitivity tuning via threshold
- [x] Instruction smuggling detection in individual payload fields
- [x] 9 tests (up from 0)

### OPSEC Intercept Hardening (2026-04-13)
- [x] Expanded from 2 patterns (SSN, license) to 9 (+ email, phone, IPv4, credit card, API key, private key header, high-entropy)
- [x] `contains_sensitive()` — fast boolean check without full scan
- [x] `scan_with_custom()` — caller-provided patterns for domain-specific PII
- [x] `SensitiveCategory` enum for typed match classification
- [x] `SensitiveMatch` struct with position, category, and redaction details
- [x] Deterministic redaction (same input → same ZKP placeholder)
- [x] 12 tests (up from 2)

### Diagnostic Engine Expansion (2026-04-13)
- [x] `test_holographic_recall()` — associative memory store/retrieve self-test
- [x] `test_psl_axiom_chain()` — end-to-end PSL governance pipeline self-test
- [x] `test_bipolar_algebra()` — BipolarVector algebraic invariant verification
- [x] Suite expanded from 3 to 6 self-tests (VSA, thermal, storage, holographic, PSL, algebra)
- [x] 8 tests in test module (up from 1)
- [x] **323 total LFI tests, 0 failures**

### PSL Security Axioms (2026-04-13)
- [x] `EntropyAxiom` — detects degenerate/adversarial vectors (all +1 or all -1)
- [x] `OutputBoundsAxiom` — prevents DoS via oversized payloads (per-field + total limits)
- [x] `InjectionDetectionAxiom` — detects SQL, command, XSS, and template injection patterns
- [x] 9 new axiom tests
- [x] Total axiom library: 9 axioms (6 original + 3 security hardening)

### Semantic Router Expansion (2026-04-13)
- [x] `route_explained()` — returns RoutingDecision with full explanation
- [x] `RouterConfig` — configurable strategic/tactical thresholds
- [x] Resource-aware routing: `set_max_tier()` caps tiers under load
- [x] `RouterStats` — tracks tier distribution for monitoring
- [x] `IntelligenceTier::cost()` and `description()` methods
- [x] 7 new tests (up from 0)

### Planner Expansion (2026-04-13)
- [x] `validate_plan()` — dependency ordering validation (self-deps, missing deps, cycles via DFS)
- [x] `parallel_groups()` — identifies steps that can execute simultaneously
- [x] `critical_path_length()` — minimum sequential phases needed
- [x] `progress()` — plan completion percentage
- [x] 5 new tests (4→9 total)
- [x] **300 total LFI tests, 0 failures** (up from 256 at session start)

## Open — High Priority (Provenance Integration)

### Wire ProvenanceEngine into PSL Supervisor
Each axiom evaluation in `psl/supervisor.rs` `audit()` should record a trace entry tagged with `InferenceSource::PslAxiomEvaluation`. The axiom_id, relevance, and confidence are already computed — just pass them to the trace arena.

### Wire ProvenanceEngine into Active Inference
Each `step()` in `cognition/active_inference.rs` should record a trace entry with the free energy, prediction error, and outcome type (Equilibrium/Act/Perceive).

### Wire ProvenanceEngine into CognitiveCore
The dual-mode reasoner in `cognition/reasoner.rs` should:
- System 1 fast path: record lightweight trace (InferenceSource::System1FastPath)
- System 2 deliberation: record full trace tree (InferenceSource::System2Deliberation)
- Knowledge compiler acceleration: record compilation event

### Enforce ProvenanceKind in Crypto Epistemology
The `crypto_epistemology.rs` `EpistemicLedger` should be extended to:
- Accept a `ProvenanceEngine` reference
- When committing a belief, check if a traced derivation exists for it
- If no trace exists, the commitment must be tagged as reconstructed
- This is the enforcement point — the ledger refuses to certify untraced beliefs as traced

## Open — Medium Priority

### Self-Play Provenance
`bin/self_play.rs` runs 1M generations of thesis-antithesis-synthesis. Each synthesis should record a full trace chain: thesis → PSL audit → synthesis. These traces should persist across episodes for post-hoc analysis of strategy evolution.

### Provenance Serialization
Add `serde::Serialize` / `Deserialize` on `TraceEntry` and `TraceArena` for persistence. Traces should survive process restarts via the existing knowledge persistence layer.

### Provenance Query API
Expose provenance queries via the `api.rs` REST endpoints:
- `GET /provenance/:conclusion_id` → ProvenancedExplanation
- `GET /provenance/:conclusion_id/chain` → Vec<TraceEntry>
- `GET /provenance/stats` → total traces, traced vs reconstructed ratio

## Open — General LFI Improvements

### Phase 5A: Uncommitted work needs Beta audit
The multimodal transducers (audio, image, text), codebook fixes, and StatisticalEquilibriumAxiom are complete but uncommitted. Needs Beta (Gemini) clearance per protocol.

### World Model causal prediction
`cognition/world_model.rs` has basic predict_next_state but no integration with the provenance system. Predictions should be traceable.

### OPSEC Intercept hardening
`hdlm/intercept.rs` is alpha. Needs full PII/credential scrubbing with provenance-tagged audit trail of what was scrubbed and why.

### Compute backend abstraction
`hdc/compute.rs` has `ComputeBackend` trait but only `LocalBackend`. GPU/remote backends are needed for production workloads.

### Formal verification
No Kani or TLA+ integration exists. The PSL axiom system and provenance invariants are ideal candidates for formal verification.
