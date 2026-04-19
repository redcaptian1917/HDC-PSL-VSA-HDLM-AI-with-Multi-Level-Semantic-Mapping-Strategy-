// Formal-verification adapters. Ships the wire + types;
// an actual verifier server is an optional runtime dependency
// (Kimina / Lean4 / Coq), so callers that can't reach one must
// treat Transport errors as "unknown", not "rejected".

pub mod lean4;

pub use lean4::{Lean4Client, ProofRequest, ProofResponse, VerifyVerdict, LeanClientError};
