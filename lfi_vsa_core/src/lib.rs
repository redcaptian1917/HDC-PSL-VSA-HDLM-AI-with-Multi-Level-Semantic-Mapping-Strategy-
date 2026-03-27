#![forbid(unsafe_code)]

// ============================================================
// LFI VSA Core — Sovereign Crate Root
// Section 5: Absolute Memory Safety enforced via forbid(unsafe_code).
// ============================================================

pub mod telemetry;
pub mod hdc;
pub mod psl;
pub mod hdlm;
pub mod hid;
pub mod transducers;
pub mod agent;
pub mod api;

// --------------------------------------------------------
// Re-export core public types for ergonomic access.
// --------------------------------------------------------

// I. HDC Core (Logic & Compute)
pub use hdc::error::HdcError;
pub use hdc::vector::{BipolarVector, HD_DIMENSIONS};
pub use hdc::compute::{ComputeBackend, LocalBackend};
pub use hdc::adaptive::{UiAttributes, UiElement};

// II. PSL Supervisor (The Auditor)
pub use psl::supervisor::PslSupervisor;
pub use psl::trust::{TrustLevel, TrustAssessment};
pub use psl::axiom::{Axiom, AuditTarget, AxiomVerdict};

// III. HDLM (Language & Generation)
pub use hdlm::ast::{Ast, AstNode, NodeKind};
pub use hdlm::tier1_forensic::ForensicGenerator;
pub use hdlm::tier2_decorative::DecorativeExpander;
pub use hdlm::codebook::HdlmCodebook;

// IV. Unified Sensorium & Interaction
pub use hid::{HidDevice, HidCommand};
pub use transducers::BinaryTransducer;

// V. High-Level Orchestration
pub use agent::LfiAgent;

// VI. External Interfaces
pub use api::start_api_server;
