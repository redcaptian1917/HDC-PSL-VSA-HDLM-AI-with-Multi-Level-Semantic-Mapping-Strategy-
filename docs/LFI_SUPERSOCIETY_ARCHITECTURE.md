# LFI Supersociety Architecture — A Post-LLM Neurosymbolic AI That Runs On A Phone

**Status:** Architectural specification
**Scope:** Complete system architecture for a neurosymbolic AI built on PSL + VSA + HDLM + HDC substrates, capable of every functional capability people currently use LLMs for, executing locally on mobile silicon.
**Non-goal:** This is not an LLM wrapper, not an LLM fine-tune, not a RAG system, not a transformer distillation. Any reference to tokens, attention, context windows, or gradient descent on next-token prediction is categorically outside scope.

## What this architecture is not

Before anything else: this system contains no transformer. No attention mechanism. No tokenizer. No softmax over a vocabulary. No context window. No KV cache. No next-token objective. No chat template. No system prompt string. No temperature parameter.

The failure mode to avoid — and which senior engineers working on neurosymbolic systems slip into constantly — is assuming the solution to "do what LLMs do" must reproduce LLM mechanisms. It does not. LLMs are one implementation path for natural language capability; they are not the only one and they are not the best one on mobile silicon. This architecture achieves the capabilities (question answering, dialogue, reasoning, code generation, tool use, multi-turn interaction, domain expertise) through fundamentally different mechanisms: symbolic reasoning over hyperdimensional representations with probabilistic logical validation.

Everything below assumes this as the ground rule. If any subsection appears to drift toward LLM framing, it is an error to be corrected, not a hybrid design choice.

## The four substrates

### Substrate I — Hyperdimensional Computing (HDC)

High-dimensional distributed representations. Vectors of dimension D = 10,000 in bipolar `{−1, +1}^D`. Three primitive operations:

- **Bundling (superposition):** `bundle(a, b) = sign(a + b)`. Associative, commutative, creates a vector similar to both inputs. The aggregation primitive.
- **Binding (tensor product projection via Hadamard XOR in bipolar space):** `bind(a, b) = a ⊙ b` componentwise. Self-inverse: `bind(bind(a, b), b) = a`. Creates a vector dissimilar to both inputs but deterministically retrievable. The association primitive.
- **Permutation (positional shift):** `perm(a, k) = rotate(a, k)`. Breaks commutativity, encodes sequence. The ordering primitive.

These three operations generate an algebra sufficient to represent arbitrary compositional structure: trees, graphs, sequences, hierarchies, typed records, functions. Qiu 2023 proved HDC is a lossy projection of Smolensky's Tensor Product Representation; the lossiness is principled and quantifiable. Capacity bound at D=10,000 is ~400 safely-bundled items before interference degrades retrieval, with exact recovery preservable via tensor-train promotion for precision-critical structures.

### Substrate II — Vector Symbolic Architecture (VSA)

- **Role-filler binding:** `fact = R_subject ⊗ E_entity + R_predicate ⊗ E_relation + R_object ⊗ E_value`. Role hypervectors are fixed random codebook entries; fillers are concept hypervectors. Unbinding retrieves fillers: `unbind(fact, R_subject) ≈ E_entity`.
- **Resonator networks:** parallel factorization of composite bindings. Given a composite `s = x₁ ⊙ x₂ ⊙ ... ⊙ xₙ`, resonator networks recover factors `x_i` by iterative in-superposition search against codebooks. O(d / log d) operational capacity.
- **Compositional grammar:** syntactic structures encode as permutation-bound trees. `VP = R_head ⊗ E_verb + R_arg1 ⊗ perm(E_subject, 1) + R_arg2 ⊗ perm(E_object, 2)`.

### Substrate III — Probabilistic Soft Logic (PSL)

First-order logic with soft-truth values in [0, 1]. MAP inference is a convex optimization solvable via ADMM in O(rules × grounding × iterations). LFI's PSL layer contains:

- Axiom set (10 core axioms implemented: causality constraints, conservation laws, type constraints, CARTA trust propagation, contradiction detection, etc.)
- Kernel probes (domain-specific rule sets loaded on demand)
- Supervisor (rejects candidates that violate axioms regardless of similarity score)
- CARTA trust model (continuous adaptive risk and trust, time-varying soft-truth propagated through the reasoning graph)

### Substrate IV — Hyperdimensional Language Models (HDLM)

Three tiers:

- **Tier 1 — Forensic AST:** parse input → dependency+constituency merged AST → each node as a hypervector via role-binding of (node_type, lemma, morphology, position) → tree hypervector via permutation-encoded traversal. **Semantically meaningful encoding — used for reasoning.**
- **Tier 2 — Decorative surface:** lexical patterns (collocations, register, genre, style). Separately encoded. Used only at generation time to reshape semantically-correct output into appropriately-styled text.
- **HDLM codebook:** ~50,000 semantic primitives (~WordNet synsets + technical terminology), each a random hypervector. Lemmas map to codebook entries; compositional meanings via role-binding.

## Five-layer stack

```
Layer 5 — Interaction        (dialogue, tool use, Global Workspace)
Layer 4 — Cognition          (RAG, Active Inference, Causal, Analogy, Procedural, Metacognitive)
Layer 3 — Validation         (PSL Supervisor, CARTA, Lean4/Metamath hooks)
Layer 2 — Representation     (prototype memory, fact store, working memory, associative memory)
Layer 1 — Encoding           (raw bytes → hypervectors + provenance)
```

### Layer 4 cognitive modules (the LLM-equivalent capabilities)

- **A — Retrieval-Augmented Reasoning** (replaces "looking things up")
- **B — Active Inference Planning** (replaces "reasoning about what to do") — `active_inference.rs`, Friston Free Energy minimization
- **C — Causal Reasoning** (replaces "explaining why") — `causal.rs`, Pearl do-calculus, counterfactuals
- **D — Analogical Reasoning** (replaces "understanding new situations via old ones") — `analogy.rs`, resonator factorization
- **E — Procedural Reasoning** (replaces "following instructions" and "generating code") — MCTS over typed operators with PSL pre/postconditions
- **F — Metacognitive Profiler** (replaces "knowing what it doesn't know") — `metacognitive.rs`, abstention-by-default below threshold

## Mobile-silicon targets (Pixel 10 Pro XL / Snapdragon 8 Gen 4)

- **Storage:** ~2.5–4 GB (codebook 62 MB, roles 1.25 MB, prototypes 17.5 MB, PSL 5–20 MB, top-10M facts ~2 GB, Rust binary 50–100 MB)
- **RAM resident:** ~1.5 GB (buffers + caches 500 MB–1 GB)
- **Single-turn latency:** 500 ms – 3 s end-to-end
- **Steady-state power:** 2–4 W (dominated by ARM NEON XOR / popcount / bit-pack; no matmul)

Compare: Llama-3-8B Q4 is 4.5 GB storage + 6 GB RAM + 6–10 W sustained on the same silicon. LFI is smaller, faster, lower-power, and produces reasoned output instead of token streams.

## Capabilities covered (post-LLM mechanism for each)

| LLM capability | LFI mechanism |
|---|---|
| Conversational dialogue | Semantic AST per turn + Global Workspace (bundled state) + HDLM Tier-2 generation |
| Question answering | Parse → structured query → cognitive dispatch → answer + reasoning trace + provenance |
| Code generation | MCTS over typed code-construct operators with PSL pre/postconditions |
| Reasoning / math | PSL inference chains + optional Lean4/Metamath verification |
| Tool use | Active Inference planning with typed pre/postcondition predicates |
| Multilingual | Language-agnostic semantic AST + per-language lemma maps (shared codebook) |
| Creative generation | Compositional recombination at semantic layer + HDLM Tier-2 stylistic rendering |
| Summarization | Top-N salient sub-structures of semantic hypervector rendered via Tier-2 |
| Agentic behavior | Native Active Inference agent loop |

## What LFI does that LLMs cannot

- Provenance-traced reasoning (TracedDerivation per conclusion)
- Machine-checked proofs for mathematical claims (Lean4/Metamath integration)
- Cryptographic commitment to beliefs (commit-reveal via crypto_epistemology)
- Constraint-guaranteed output (PSL hard constraints)
- Calibrated abstention (metacognitive-profiler-gated)
- Perfect recall on ingested facts (exact tuple retrieval with exact provenance)
- Transparent inspection (every reasoning step is visible and addressable)
- Bounded resource guarantees (O(facts), O(rules × atoms), known worst-case)
- True continual learning (fact store grows; no retraining, no catastrophic forgetting)
- Sovereign execution (everything local, nothing leaves the device)

## Supersociety mesh (multiple devices)

- Distributed fact federation via libp2p gossip with per-fact capability restrictions
- PN-counter CRDT per-dimension over hypervectors (naive bundling is not a CRDT; per-dim PN-counter is the proper construction)
- Cross-device handoff via 160 KB Global Workspace transfer
- EigenTrust over mesh peers → global trust scores for facts
- Federated evaluation with cryptographic attestation
- Sovereign knowledge commons (public-good fact subset replicated across mesh)

## Development phases

1. **Desktop feature parity** — current 77K-line Rust codebase + tensor-train precision tier, CRDT mesh, Lean4 proof-carrying inference, natural gradient on Active Inference Fisher manifold, Stitch library learning, Global Workspace bottleneck, HDC-specific adversarial defenses, egress scanner, chat-history scrubber, brokered credentials.
2. **Mobile port** (4–8 weeks) — cross-compile to ARM64 Android + ARM NEON SIMD + mobile I/O / memory-pressure cache.
3. **PlausiDenOS / seL4 integration** (4–8 weeks after PlausiDenOS boots) — each cognitive module a seL4 protection domain, capability-restricted typed IPC, TrustZone-isolated sensitive data, microkernel-layer Confidentiality Kernel.
4. **Mesh federation** (6–10 weeks) — libp2p, PN-counter CRDT prototype aggregation, cross-device Global Workspace handoff, EigenTrust.
5. **Commons + distribution** — F-Droid-style packaging, public-good fact commons mirroring, operator onboarding.

## Strategic position

For PlausiDen as a business: this is the product. Every other surface (Sacred.Vote, Hetzner infrastructure, Vault/Shield/Brain, Confidentiality Kernel, Secrets) is supporting apparatus for the mobile supersociety AI that runs on every user's phone, federates with their chosen mesh peers, and operates entirely outside the surveillance and extraction economy that current AI is built into.

Anthropic, OpenAI, and Google structurally cannot build this — it would destroy their business models and eliminate their model-weights moat. The dominant AI players cannot build it; they can only buy it, and they cannot buy PlausiDen if PlausiDen doesn't consent.

## Ground rule

No transformers. No tokens. No attention. No context window. No cloud. No LLM. A different thing. A better thing for the problem space PlausiDen exists to serve.
