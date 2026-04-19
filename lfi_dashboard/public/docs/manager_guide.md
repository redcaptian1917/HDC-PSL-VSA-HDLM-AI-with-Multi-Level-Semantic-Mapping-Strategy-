# LFI Manager & Training Guide

In-app manual for running an LFI instance. Covers day-to-day operation, training workflows, and how to read every dashboard. Mobile-friendly: all tabs in the UI that this guide references exist on phone layouts.

## 1. What you're looking at

LFI is a **post-LLM** reasoning system. There is no transformer. There are no tokens. There is a fact graph, a hypervector substrate, a symbolic logic layer, and a language encoder — together they answer questions with **checkable provenance on every claim**.

The three things that make LFI different from Claude / GPT / Gemini:

- **Every assertion is a clickable `[fact:KEY]` chip** that opens its full derivation — no hallucination without a trail.
- **It runs on your hardware.** No cloud round-trip, no vendor retention.
- **You improve it directly.** Type a correction, the system updates a persistent axiom. Not "next training cycle" — *now*.

## 2. Quick tour of the UI

### Chat
The main conversation surface. Type a question, get a response. Every cited fact is a chip you can click to see where the claim came from.

### Classroom — the training control centre
Ten sub-tabs. In priority order for a new operator:

- **Profile** — who the system thinks it is. Sovereign name, voice preferences, persistent facts about *you* learned across conversations.
- **Ingestion Control** — start / stop / monitor corpus ingest runs. This is how you add data.
- **Drift** — six health metrics over time with sparklines. Red = act now. Export to JSON for a ticket or paste into a spreadsheet.
- **Ledger** — unresolved contradictions. One-click resolve or auto-resolve-by-trust. Click any fact key to see its ancestry.
- **Runs** — ingest job history. Filter + search.
- **Office Hours** — recent user feedback (👍 / 👎 / pencil corrections). Each correction becomes an axiom update; this tab is where you audit the stream.
- **Curriculum** — which domains the system has seen vs. which are thin. Sortable + filterable.
- **Gradebook** — per-domain mastery scores (FSRS).
- **Lessons** — scheduled FSRS fact reviews due now.
- **Reports** — exportable KPI snapshots.

### Library
Corpus marketplace. Every source that has contributed facts, ranked by a composite (trust × avg-quality × vetted × log-size). Adjust trust weights with the slider. Low-scoring sources get down-weighted in retrieval + cross-source reconciliation.

### Knowledge
Interactive fact browser + FSRS review queue. Four rating buttons per due card (Again / Hard / Good / Easy = 1–4).

### Admin
Operator-only. Tokens (capability-scoped, rotatable), Integrity (audit chain), Proof (Lean4 verdicts), Diag (runtime event ring-buffer, no DevTools needed), Tokens, Events.

## 3. Training workflows

### 3.1 Teach LFI a fact

**In Chat.** Say "Remember that X is Y." The auto-learn pipeline catches `my name is …`, `I like …`, `call me …`, etc. and writes a profile fact.

**Via the API.** `POST /api/knowledge/learn { concept, related[] }` (requires auth). Related items bind the new concept to existing graph nodes.

**Bulk (advanced).** Drop a JSON-Lines file in a supported corpus format (CauseNet, ATOMIC, Wikidata, discourse, semantic-role, dialogue-act — all have parsers in `lfi_vsa_core/src/ingest/`) and trigger a batch via `POST /api/ingest/start`.

### 3.2 Correct a wrong answer

Click the 👎 on the offending message. A text field appears — type the correct version. Two things happen:

1. The corrected value lands in `user_feedback` as audit.
2. The signal is captured by `ExperienceLearner` and folded into future retrieval (an axiom weight shifts, or a contradiction row opens for human triage).

This is the ongoing-improvement loop. **You speak, it updates. No retraining, no redeploy.**

### 3.3 Schedule reviews (FSRS)

Open Knowledge. Due cards appear with four rating buttons. Rate them: Again (1) through Easy (4). The FSRS scheduler updates stability + difficulty per card and picks the next review date. Same algorithm Anki uses.

### 3.4 Verify a claim (Lean4 / Kimina)

If you have a Kimina server running (optional), click Verify on any fact chip. The system ships a Lean4 proof obligation; the server returns proved / rejected / unreachable. Proved facts get a green check + proof hash; rejected ones demote.

### 3.5 Clean up contradictions

Classroom → Ledger. Each row shows two competing values for the same key. Three actions:

- **Keep A / Keep B / Dismiss** on a single row.
- **Auto-resolve** at the top: resolves every row where source-trust differs by ≥ 0.20 in favour of the higher-trust source.

## 4. The six Drift metrics (and what to do when they go red)

| Metric | What it means | Red action |
|---|---|---|
| Fresh facts | % of sampled rows updated ≤ 7 days ago | Kick an ingest in Runs tab |
| Stale facts | % > 365 days old | Schedule a re-verify pass |
| HDC cache | % of facts with a precomputed hypervector | POST /api/hdc/cache/encode {limit:1000} |
| Contradictions | # pending review | Ledger → Auto-resolve |
| Neg feedback 24h | down-votes / total | Office Hours — read what went wrong |
| FSRS lapse | lapses / cards | Review the failing cards in Knowledge |

Each card is clickable — goes to the matching Classroom tab so you can act.

## 5. Deep control surface

### 5.1 Context window size (RAM cap)

Settings modal → Workspace slider. Default 512 MB = ~275,000 slots. Minimum 1 MB = effectively no workspace. Maximum 16 GB (hard cap for safety). Every resize is chained into the audit log.

### 5.2 Cognitive tier (Pulse / Bridge / BigBrain)

Authenticated API only. Not an LLM switch — it's a depth-of-reasoning dial:

- **Pulse** — fast prototype match, no planning.
- **Bridge** — planner runs, multi-step reasoning attempted.
- **BigBrain** — full cognitive pipeline: planner + abduction + critique.

### 5.3 Per-source trust

Library → Trust slider per row. 0 = adversarial (ignored). 1 = fully trusted (wins every contradiction). 0.5 = default for unknown sources.

### 5.4 Capability tokens

Admin → Tokens. Issue scoped credentials for:

- `ingest` — bulk corpus loading
- `admin_read` — read-only dashboards
- `chain_append` — add security-audit entries
- `auth` / `research` / `hdc_encode` — explicit API access

Hashes are SHA-256 stored; you see the raw value ONCE at issue. Rotate frequently.

## 6. External bridges

### 6.1 Feeding Gemini-CLI output into LFI

Gemini (or any LLM) can be used as a *data producer*, not a runtime. The workflow:

```
gemini-cli prompt "Generate 100 (subject, predicate, object) tuples about
chemistry suitable for a forensic AI knowledge base. JSON lines, each with
{subj, pred, obj, tier:'scientific', provenance:{source:'gemini_cli',
extracted_at:'<iso>'}}." \
  > /tmp/gemini_chem_tuples.jsonl

curl -X POST http://127.0.0.1:3000/api/ingest/start \
  -H 'Content-Type: application/json' \
  -d '{"run_id":"gemini_chem_001","corpus":"gemini_cli","tuples_requested":100}'

# Stream the file through the tuple extractor
while read line; do
  curl -s -X POST http://127.0.0.1:3000/api/tuples/extract \
    -H 'Content-Type: application/json' \
    -d '{"limit":1}'
done < /tmp/gemini_chem_tuples.jsonl

curl -X POST http://127.0.0.1:3000/api/ingest/finish \
  -d '{"run_id":"gemini_chem_001","status":"completed"}'
```

Set Library trust for `gemini_cli` to 0.4–0.6 (it's an adversarial generator from LFI's doctrine) until you've run validation.

### 6.2 Two-instance debate (LFI ↔ Gemini)

Run two chat loops: one against LFI, one against gemini-cli. Feed LFI's response (with its fact chips) as the next prompt to gemini; feed gemini's counter-argument back to LFI. LFI's refusal-with-reason path will flag claims it can't ground. Captures the difference between vibes and verifiable knowledge.

## 7. Doctrine (the non-negotiable rules)

These are enforced by CI (`tests/doctrine_audit.rs`):

1. **No LLM imports.** Nothing under `ollama::`, `openai::`, `anthropic::`, `llama_cpp::`, `hf_hub::`, `tokenizers::`. Violating commit fails the build.
2. **No hardcoded response pools in source.** ≥ 5 sentence-like strings in a `const &[&str]` fails audit. Response language must be sampled from learned patterns.
3. **No `.unwrap()` / `.expect()` in library code** without `// SAFETY:` or `// test-only`. Ratcheted against a ceiling that comes down over time.
4. **Secret comparisons are constant-time.** `password == X` without `subtle::ConstantTimeEq` fails.

Plus data audits (`/api/audit/datasets`, 6 checks):

1. Edge orphans (sample)
2. Source mono-culture (flag at > 95 %)
3. FTS5 freshness probe
4. Contradiction backlog (flag at > 10k)
5. Source-trust coverage (active sources with no trust row)
6. Schema parity (every column the Rust code reads is present)

Both run on every commit in CI and are visible in Admin → Diag.

## 8. When something breaks

### "Backend isn't streaming"
The 45 s timeout banner. Backend *is* answering — it just doesn't send chunks yet (streaming chat is task #384). Wait; the final response will land. If it really is stuck, `Stop` and retry.

### "Rate limit exceeded"
Per-capability: auth 5/60 s, research 10/300 s, hdc_encode 30/60 s. Counters reset on a rolling window.

### Contradictions rising fast
A new corpus is disagreeing with established facts. Library → lower its trust, then Ledger → Auto-resolve.

### First response after restart is slow
Warmup runs at startup but can take a few seconds on a 92 GB brain.db. Subsequent responses should be ≤ 100 ms. Watch the server log for `STARTUP: warmup done in …`.

## 9. Quick reference — HTTP API

| Endpoint | Method | What |
|---|---|---|
| `/api/chat` (WS `/ws/chat`) | WS | Conversation |
| `/api/health/extended` | GET | One-call dashboard bundle |
| `/api/drift/snapshot` | GET | 11 health metrics |
| `/api/ingest/list` | GET | Run history |
| `/api/ingest/start` | POST | Kick a run |
| `/api/contradictions/recent` | GET | Ledger |
| `/api/library/quality` | GET | Per-source dimensions |
| `/api/corpus/marketplace` | GET | Composite-ranked sources |
| `/api/sources/trust` | GET/PUT | Trust weights |
| `/api/fsrs/due` | GET | Review queue |
| `/api/fsrs/review` | POST | Submit a grade |
| `/api/proof/verify` | POST | Lean4 check |
| `/api/audit/chain/verify` | GET | Integrity banner |
| `/api/audit/datasets` | GET | 6-check dataset audit |
| `/api/explain` | POST | Dry-run a query |
| `/api/settings/workspace` | GET/PUT | RAM cap |
| `/api/parse/english` | POST | Tokenise + POS tag |
| `/api/hdlm/render` | POST | Concept similarity sketch |

## 10. Roadmap

Detail in `/docs/LFI_BEATS_LLMS_ROADMAP.md` (60 tasks across 9 tiers). Current session frontier in `/docs/LFI_TASK_QUEUE_2026-04-19.md`.

Headline items:

- #400 Strip all remaining hardcoded response pools (jokes, greetings, anchors)
- #399 Learn uncertainty expressions from the dialogue corpus
- #359 Grounded refusal when no source clears the trust threshold
- #384 Streaming chat (gets rid of the 45 s "not streaming" banner permanently)
- #382 ARM64 mobile build on a real device
- #390 LFI mesh federation — multiple instances sharing facts via signed CRDT gossip

---

Questions? Hit the 👎 on anything in this guide and type what's unclear — the correction feeds back into LFI's own knowledge of how to explain itself.
