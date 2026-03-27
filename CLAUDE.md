# CLAUDE.md — Workflow Alpha State Document

**Agent:** Claude Code (Workflow Alpha — The Architect)
**Last Updated:** 2026-03-26
**Protocol Version:** 5.6

---

## Role Boundaries

- **Responsible for:** Structural engineering, Rust/C++/ASM, Web API, transducer bridging
- **Prohibited from:** Inventing logic rules (Beta's domain)
- **Test mandate:** Exhaustive unit tests proving mathematical properties before integration
- **Telemetry mandate:** `debuglog!` in every function, every branch, every edge case

---

## Completed Phases

### Phase 1: HDC Core (Commit 827ee4e)
- `BipolarVector`: 10,000-dim bipolar hypervectors via bitvec
- Three operations: Bind (XOR), Bundle (Sum+Clip), Permute (Cyclic Shift)
- Cosine similarity + Hamming distance
- `ComputeBackend` trait + `LocalBackend`
- 48 tests, all passing
- **Beta cleared:** 5 axioms verified (Commit 15e4678)

### Phase 2: PSL Supervisor + HDLM AST (Commit d12feb3)
- PSL: `Axiom` trait, `PslSupervisor`, `TrustLevel` (CARTA), `AuditTarget`, `AxiomVerdict`
- Built-in structural axioms: `DimensionalityAxiom`, `DataIntegrityAxiom`
- HDLM: `Ast` arena, `NodeKind` (13 variants), `ForensicGenerator`, `DecorativeExpander`
- Tier 1: `ArithmeticGenerator` (prefix -> AST)
- Tier 2: `InfixRenderer`, `SExprRenderer` (read-only on AST)
- 89 tests total, all passing
- **Beta audit:** Pending

---

## Current Git State

```
448319a Merge origin/main (dependabot config) into master
d12feb3 PHASE2: PSL Supervisor + HDLM AST Generation Infrastructure
15e4678 AUDIT: Phase 1 VSA Core cleared by Beta (Gemini)
827ee4e INIT: VSA Core Baseline — Ground Zero Protocol v5.6
```

Branches: `master` and `main` synced to same HEAD.

---

## Conventions

- `#![forbid(unsafe_code)]` — never removed
- All ops return `Result<T, E>` — no `.unwrap()`, `.expect()`, `panic!()`
- `debuglog!` in every function, every branch
- Tests use `-> Result<(), ErrorType>` with `?` operator
- Forensic commit messages with full change summary
- Push to both `master` and `main` after every commit
- Write `lfi_bus.json` payload after every phase completion
- Yield to Beta after every phase for audit

---

## Next Steps (Pending Beta Clearance of Phase 2)

1. **Phase 3:** HDC Item Memory / Codebook
   - Symbol<->Vector mapping
   - Nearest-neighbor query
   - Codebook training from vector pairs
   - Enables `generate_from_vector()` in HDLM

2. **Phase 4:** Unified Sensorium
   - Audio/video/image transducers
   - Binary file projection into VSA space

3. **Phase 5:** axum Web API
   - REST/WebSocket backend daemon
   - Hardened endpoints with PSL audit on all inputs
