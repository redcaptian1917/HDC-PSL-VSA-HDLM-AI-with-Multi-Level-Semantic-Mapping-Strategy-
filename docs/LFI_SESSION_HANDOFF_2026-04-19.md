# Session handoff — 2026-04-19

Carries everything the user said in this session so the next session can continue without them having to repeat.

## User directives (verbatim intent, preserved for next session)

1. **Make LFI better than Claude / GPT-5 / Gemini** — established the 60-task roadmap in `docs/LFI_BEATS_LLMS_ROADMAP.md`. Work from that doc.
2. **Massive context window, as much as RAM allows** — shipped. Default workspace sized by byte budget (LFI_WORKSPACE_MAX_MB=512 → ~275k slots vs. the old 8). `GET/PUT /api/settings/workspace` live.
3. **UI RAM-cap control in all UIs** — backend endpoint ready. Claude 2's territory to finish slider in Settings modal (visible from Chat / Classroom / Admin). Task #398 still pending UI wire.
4. **Numeric certainty score 0 – 100%, two decimal places** — shipped in the prose output: "(87.50% certain)".
5. **No hardcoded outputs. No rote memorization. Intelligence, not canned phrases.** — partially shipped. Stripped the English sentence templates + hedge pool in `causal_summary_prose`. Legacy pools (jokes, anchor responses, conversational templates in `reasoner.rs`) still exist — task #400 tracks the full de-hardcoding. Task #399 tracks learning hedge expressions from dialogue corpus.
6. **Learn the way humans do** — expression-of-uncertainty pipeline from dialogue corpus (#399), response-pattern sampling (#400), active question generation (#378), conversation-to-tuple auto-ingestion (#376) all represent this.
7. **Audits and tests enforce these things: no LLM, AVP-2 adherence, no hardcoded responses** — shipped as `tests/doctrine_audit.rs`. Four tests: `no_llm_imports`, `no_hardcoded_response_pools_in_src`, `no_unwrap_or_expect_in_library_code`, `secret_comparisons_are_constant_time`.
8. **Audits for DBs, datasets** — shipped as `GET /api/audit/datasets` + `BrainDb::dataset_audit()`. Six checks: edges_source_exists / source_mono_culture / fts5_freshness / contradiction_backlog / source_trust_coverage / schema_parity. Each returns `{name, passed, detail, metric}`. 362 ms on prod.
9. **Make all tiers (Pulse / Bridge / BigBrain) function + relatively quick on "hello"** — "hello" now 20–40 ms after warm-up. Cold-start first-hit was the slow-first-request illusion. Claude 2 is wired to Pulse/Bridge/BigBrain switcher; backend tier_handler gates on auth.
10. **PlausiDen-MCP CI failure** — fixed in commit 2479d16 (cargo fmt over the tree). Green.

## What's live as of this handoff (commit `0cfc07c` + followups)

- Post-LLM chat pipeline
- `[fact:KEY]` inline chips on every RAG-retrieved assertion
- Prose composer emits `subj — predicate: obj1, obj2, obj3 (87.50% certain)` per clause — NO English hardcoded templates
- Confidence 0.00–100.00 % threaded through from fact_edges.strength
- Massive workspace (275k slots default, live-resizable by RAM)
- Doctrine audit tests (4)
- Dataset integrity audit endpoint (6 checks)
- `/api/parse/english` + `/api/hdlm/render` for HDLM output inspection
- `/api/proof/{verify,status,stats}` Lean4-Kimina integration points
- 60 roadmap tasks filed (#356–#396) + 6 live-concern tasks (#397–#402)

## What's queued (do first on resume)

In rough priority:

1. **Finish #356 / #400**: get rid of jokes / greeting / anchor / status / capability / identity pools in `reasoner.rs`. Replace with sampled dialogue patterns from the `dialogue_tuples_v1` corpus in brain.db. Doctrine test's `legacy_exempt = ["reasoner.rs"]` should disappear after this.
2. **#399**: mine the dialogue corpus for uncertainty expressions (probably / likely / maybe / could be / possibly / usually / typically / always / certainly / definitely / I think / I'd say / in my view…). Tag each with a confidence bin derived from the turn's explicit hedge frequency. Sample at prose-render time.
3. **#359 refusal with reason** — when confidence < threshold or no fact grounds the claim, refuse explicitly: "I don't have a source ≥ 0.70 trust that answers this." This is the partner to calibrated hedging — below the floor we SHOULD refuse.
4. **#360 multi-sentence discourse composition** — use #334 discourse relations as connective tissue between clauses.
5. **#372 image → chat wire** — transducer exists; wire through the chat handler so "what's in this picture" works.
6. **#382 ARM64 mobile end-to-end** — scaffold in `.cargo/config.toml`; run on device.
7. **#392 Wikidata streaming ingester** — parser shipped; wrap in a `tools/lfi-wikidata-ingest` binary.
8. **#398 Settings-modal slider** — Claude 2's wire for the workspace RAM cap.

## Running instances

- Server: `/home/user/cargo-target/release/server` on :3000. brain.db WAL at /home/user/.local/share/plausiden/brain.db (~92 GB).
- Dashboard: `/root/LFI/lfi_dashboard/dist` served by the server's SPA fallback.
- Claude 2 working frontend in parallel via `/tmp/claude-ipc/bus.jsonl`.

## Commit trail (post-session cutoff)

Look for the chain ending at `0cfc07c` (doctrine audit) on `main`. Recent chronological order:

- #356 HDLM Tier-2 prose composition
- #357 inline [fact:KEY] chips
- #358 calibrated hedging (initial) → then stripped to numeric-only per user correction
- #397 massive workspace + RAM cap endpoint (#398 backend)
- #401 doctrine audit tests
- #402 dataset integrity audit endpoint + BrainDb::dataset_audit

## Session-invariant reminders

- All work is POST-LLM. No transformer, no Ollama, no attention. Substrates: HDC + PSL + HDLM.
- Provenance chips are MANDATORY on every assertion. It's THE differentiator.
- No hardcoded response pools — sample / compose from learned patterns.
- Every secret comparison constant-time (`subtle::ConstantTimeEq`).
- Every external data sink goes through `secret_scanner::redact`.
- Doctrine audit tests MUST pass before any commit.
- User runs in Kali rescue mode — don't touch GRUB / initramfs / wlan0.
