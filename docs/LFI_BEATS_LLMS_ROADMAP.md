# LFI → post-LLM production: the roadmap that makes it better than me (Claude), GPT-5, or Gemini

This is the honest list. Claude/GPT/Gemini are fast and fluent but they lie confidently, forget everything, have no provenance, and can't be run on your hardware. LFI can win on axes they structurally can't compete on.

## The differentiators

These are the axes where the LLMs physically can't follow because their architecture rules it out:

1. **Provenance for every claim** — every sentence traces back to fact rows + axioms with cryptographic hash.
2. **Local-first** — runs on a laptop / phone / air-gapped machine.
3. **Verifiable reasoning** — PSL + Lean4 actually check; not vibes.
4. **Editable memory** — user can delete, correct, replay any fact.
5. **Online learning** — a conversation correction changes future answers.
6. **Self-auditable** — "why did you say X?" returns a machine-checkable chain, not a confabulated explanation.
7. **Cross-lingual identity** — fact learned in English answers in Mandarin.
8. **Offline** — zero network required.
9. **Honest uncertainty** — "0.72 confidence because source trust 0.65 × axiom pass 0.90 × temporal decay 0.98" instead of "I think probably…".
10. **Refusal with reason** — "I don't know because no source trust ≥ 0.7 covers this claim" instead of inventing.

## The roadmap

Numbered 1-N by priority. Each task is a concrete deliverable, not abstract.

### Tier 1 — fluent language grounded in the substrate (the current biggest gap)

1. **HDLM Tier-2 full sentence composition** — render HDC composites through proper grammar, not concept lists. Use dependency templates (SUBJ-VERB-OBJ-MOD) + the English parser's POS tags.
2. **Inline fact citations** — every generated sentence carries `[fact:key]` chips linking to provenance. Mandatory, not optional.
3. **Calibrated hedging** — low-confidence facts get "likely" / "probably"; high-confidence get bare assertions. Threshold table in PSL.
4. **Refusal with reason** — "I don't know because …" with the specific axiom or missing trust threshold that blocked the answer.
5. **Multi-sentence discourse composition** — use discourse relations (Contrast, Cause, Elaboration from #334) as the connective tissue between sentences.
6. **Style transfer axiom** — user says "be more concise" → PSL style-axiom updates → future outputs honour it.
7. **Paraphrase robustness** — generate N paraphrases of an answer, keep the one with highest mean PSL pass rate across all.
8. **Source-agreement surfacing** — "Wikidata says X, ConceptNet says Y, I'm going with X because trust 0.85 > 0.70".

### Tier 2 — reasoning the LLMs only fake

9. **Multi-hop HDC analogy** — A:B :: C:? via unbind(B, A) ⊗ C, resonator-factorised against the codebook.
10. **Counterfactual reasoning** — "what if water were not a liquid" flips one fact, recomputes downstream, shows what changes.
11. **Temporal valid_from / valid_to** — facts carry validity windows; queries at time T see only facts valid then.
12. **Abductive inference** — given observations, return the minimum axiom set that explains them, ranked by simplicity.
13. **Multi-step planner** — goal → (precondition chain) → action sequence. Backtracks on axiom failure.
14. **Self-consistency vote** — generate N reasoning chains, pick the modal answer. If no majority, refuse and flag.
15. **Critique-then-revise** — after producing an answer, a second pass acts as adversary and tries to falsify each sentence. Surviving sentences ship.
16. **Proof by contradiction via PSL** — try to falsify the negation; if impossible, assert.
17. **Quantitative reasoning** — arithmetic, unit conversions, order-of-magnitude sanity checks. Declarative: the axioms ARE the math.
18. **Default reasoning (non-monotonic)** — "birds fly" with explicit exception facts (penguins, ostriches). LLMs get this wrong constantly.

### Tier 3 — multimodal, locally

19. **Image → HDC via existing transducer** — already scaffolded; wire it into chat so "what's in this picture" works.
20. **Audio → HDC with speaker-role binding** — conversations tagged by speaker, role-bound into the fact graph.
21. **Document ingest (PDF / DOCX / HTML)** — chunked, tuple-extracted, provenance-tagged per page.
22. **Table understanding** — column headers as predicates, rows as (header, value) tuples bound to the row key.
23. **Code as AST, not string** — the existing hdlm::Ast is good; add language-specific front-ends for Python/Rust/JS.
24. **Chart / graph reading** — vision transducer + axis detection + data-point extraction to tuples.

### Tier 4 — grounded learning from every conversation

25. **Conversation-to-tuple pipeline** — every chat turn goes through the tuple extractor (#329) so claims get recorded.
26. **User-correction into axioms** — `👎 + correction` updates an axiom weight, not just a feedback row.
27. **Preference learning** — "I prefer bullet points" → style_axiom.bullet_preference += 1 → all future answers shift.
28. **Forgetting curve on facts** — half-life decay already shipped; now wire it into confidence-surfaced output.
29. **Analogy reinforcement** — when an analogy works (user accepts), boost the analogical bridge's strength.
30. **Counterexample learning** — when wrong, the counterexample goes into the fact base so the same mistake can't repeat.
31. **Active question generation** — LFI detects a knowledge gap and ASKS the user to fill it.

### Tier 5 — safety that LLMs talk about but don't really have

32. **Differential privacy on user facts** — noise added at aggregation-query boundary; user-specific inference impossible.
33. **Homomorphic query option** — encrypted query → encrypted result; server never sees plaintext.
34. **Per-user encryption key** — user-contributed facts encrypted at rest with a key only the user holds.
35. **Right-to-delete (crypto-shred)** — delete the user's key → their fact rows become unrecoverable.
36. **Federated learning** — phones train local axiom weights, share gradient updates only, never raw facts.
37. **Red-team harness** — property-based adversarial input tests run in CI; every commit must survive.
38. **Privacy dashboard** — "LFI used facts X, Y, Z from source S (trust 0.82) to answer your last question" — in plain language, not a log.

### Tier 6 — performance that matches the hardware

39. **ARM64 mobile build** — scaffolded; actually run it on an Android device.
40. **NEON SIMD HDC ops** — bipolar bind/bundle vectorised; expected 4-8× on ARM.
41. **WebGPU backend** — browser inference without a server.
42. **Sub-100ms for simple queries** — chat handler tail latency ≤ 100ms on the 95th percentile.
43. **Streaming response** — render concept-by-concept as the composite resolves, don't wait for completion.
44. **Memory budget ≤ 2 GB** — profile every path, prune anything that exceeds. Mobile-first constraint.

### Tier 7 — UX the LLMs can't match

45. **Voice conversation** — STT → LFI → TTS loop. End-to-end offline.
46. **Time-travel debugging** — replay any past query against any past fact-base snapshot.
47. **Fork conversation** — "what would you have said if you hadn't seen fact X" — forks the reasoning chain.
48. **Annotate-to-correct UI** — highlight a sentence in an LFI response, submit a fix; becomes an axiom update.
49. **Multi-LFI debate** — two LFI instances argue; a third judges; user sees the whole exchange.
50. **Shareable signed conversation cards** — every conversation exports as a cryptographically signed artefact that a third party can verify wasn't tampered with.

### Tier 8 — the ecosystem play

51. **Federation protocol** — LFI mesh talks to LFI mesh; fact-exchange via CRDT deltas (already have the CRDT).
52. **Skill plugins in WASM** — sandboxed third-party capabilities; capability tokens gate them (already have the tokens).
53. **Benchmarks vs GPT-5 / Claude / Gemini** — held-out test suite: provenance, consistency, multi-turn coherence, refusal-rate, cost-per-answer.
54. **Paper: "Post-LLM Cognitive Architecture"** — formal writeup. Establishes the category.

### Tier 9 — the stuff that's already close but needs finishing

55. **HDLM Tier-1 full parser** — current baseline is heuristic; swap in a trained dep+constituency parser via the seam we already built.
56. **Annealed resonator with noise resets** — Frady 2020 §3.4; follow-up to #340 / #351 that unlocks bigger codebook recovery.
57. **Tensor-train truncation** — rank-cap for deep binding chains; already have the precision wrapper.
58. **PN-counter mesh gossiper binary** — the CRDT is ready; need the HTTP service that emits + receives deltas.
59. **Wikidata streaming ingester** — parser shipped; build the tools/lfi-wikidata-ingest binary that drains the 100 GB bz2 into brain.db.
60. **CauseNet + ATOMIC + PropBank + Argumentation + Discourse streaming ingesters** — same pattern, per parser.

## How to read the priority

**Tier 1 is the biggest visible gap today.** If you ask LFI a question, it returns "Structured context for volcano: is a: land topographical feature, solid ground, mountain, television episode". That's data, not language. Tier 1 turns it into prose the way Claude / GPT would phrase it — but with provenance chips on every claim. That alone makes LFI more trustworthy than any LLM you can buy, because every assertion is checkable.

**Tier 2 is where LFI wins on reasoning.** LLMs pattern-match; LFI will actually compute. Counterfactuals, temporal validity, abduction, default reasoning — these are hard for LLMs because they don't have symbolic state. LFI does.

**Tier 5 is where LFI wins on trust.** Every LLM eventually leaks your data. LFI never saw it to begin with.

**Tier 8 is the bet.** If LFI mesh federation actually works, no single vendor owns the intelligence layer. That's the long game.

## What's out of scope (deliberate)

- **Generative image / video models** — LFI reasons about images, doesn't paint. If you want DALL-E you can plug it in as a skill.
- **Code execution** — LFI reasons about code. Actual execution stays in a sandbox skill.
- **Fine-tuning a transformer** — the whole point is post-LLM. No transformer.
- **RLHF / DPO / GRPO** — learning happens through PSL axiom updates, not gradient descent on pairs.

## Risk register

- **Tier 1 is genuinely hard.** Fluent composition from HDC is a research frontier. We should ship an 80% version (template-driven) while research continues.
- **Tier 3 requires models.** Image/audio transducers exist; the DNN front-ends that produce the HDC input don't. We either bundle them (licence review) or treat them as pluggable.
- **Tier 6 mobile is a real engineering project.** The scaffold is in place; the actual build needs a week of device testing.
- **Tier 8 federation has Sybil attacks.** CRDTs converge on arbitrary writes; we need signed updates + a reputation layer.

## The wager

If 30 of these 60 ship, LFI is categorically different from Claude / GPT / Gemini. They can only match us if they abandon the transformer, which their business models can't tolerate.

That's the moat.
