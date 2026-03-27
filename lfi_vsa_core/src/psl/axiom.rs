// ============================================================
// PSL Axiom Trait — The Verification Interface
// Section 1.II: Material axioms (physics, logic, security).
//
// ALPHA BOUNDARY: This file defines the trait interface only.
// Actual axiom implementations (logic rules) are defined by
// Workflow Beta (The Auditor). Alpha is strictly prohibited
// from inventing logic rules (Section 4).
// ============================================================

use crate::hdc::vector::BipolarVector;
use crate::psl::error::PslError;
use crate::debuglog;

/// The target of a PSL axiom verification.
/// Wraps the various data types the supervisor may audit.
#[derive(Debug, Clone)]
pub enum AuditTarget {
    /// A hypervector output from the HDC core.
    Vector(BipolarVector),
    /// Raw bytes from an external source (GPU return, file ingestion).
    RawBytes {
        source: String,
        data: Vec<u8>,
    },
    /// A numeric computation result.
    Scalar {
        label: String,
        value: f64,
    },
    /// A structured key-value payload (e.g., API response).
    Payload {
        source: String,
        fields: Vec<(String, String)>,
    },
}

/// Result of a single axiom check against a target.
#[derive(Debug, Clone, PartialEq)]
pub struct AxiomVerdict {
    /// Unique identifier of the axiom that was checked.
    pub axiom_id: String,
    /// Whether the axiom passed.
    pub passed: bool,
    /// Soft truth value in [0.0, 1.0] (PSL probabilistic weight).
    /// 1.0 = full satisfaction, 0.0 = total violation.
    pub truth_value: f64,
    /// Explanation of the verdict.
    pub detail: String,
}

impl AxiomVerdict {
    /// Construct a passing verdict.
    pub fn pass(axiom_id: String, truth_value: f64, detail: String) -> Self {
        let tv = truth_value.clamp(0.0, 1.0);
        debuglog!("AxiomVerdict::pass [{}]: tv={:.4}", axiom_id, tv);
        Self {
            axiom_id,
            passed: true,
            truth_value: tv,
            detail,
        }
    }

    /// Construct a failing verdict.
    pub fn fail(axiom_id: String, truth_value: f64, detail: String) -> Self {
        let tv = truth_value.clamp(0.0, 1.0);
        debuglog!("AxiomVerdict::fail [{}]: tv={:.4}", axiom_id, tv);
        Self {
            axiom_id,
            passed: false,
            truth_value: tv,
            detail,
        }
    }
}

/// Trait that all PSL axioms must implement.
///
/// Beta (The Auditor) defines concrete implementations.
/// Alpha provides only the structural interface.
pub trait Axiom: Send + Sync {
    /// Unique identifier for this axiom (e.g., "Axiom:Dimensionality_Constraint").
    fn id(&self) -> &str;

    /// Human-readable description of what this axiom verifies.
    fn description(&self) -> &str;

    /// Execute the axiom check against the given target.
    /// Returns Ok(AxiomVerdict) on successful evaluation,
    /// or Err(PslError) if the audit itself fails structurally.
    fn verify(&self, target: &AuditTarget) -> Result<AxiomVerdict, PslError>;
}

// ============================================================
// Built-in structural axioms — these verify framework invariants,
// not domain logic. Safe for Alpha to define.
// ============================================================

/// Verifies that a Vector target has exactly HD_DIMENSIONS bits.
pub struct DimensionalityAxiom;

impl Axiom for DimensionalityAxiom {
    fn id(&self) -> &str {
        "Axiom:Dimensionality_Constraint"
    }

    fn description(&self) -> &str {
        "Verifies vector targets have exactly 10,000 dimensions"
    }

    fn verify(&self, target: &AuditTarget) -> Result<AxiomVerdict, PslError> {
        debuglog!("DimensionalityAxiom::verify");
        match target {
            AuditTarget::Vector(v) => {
                let dim = v.dim();
                if dim == crate::hdc::vector::HD_DIMENSIONS {
                    Ok(AxiomVerdict::pass(
                        self.id().to_string(),
                        1.0,
                        format!("Vector dim={}, matches HD_DIMENSIONS", dim),
                    ))
                } else {
                    Ok(AxiomVerdict::fail(
                        self.id().to_string(),
                        0.0,
                        format!(
                            "Vector dim={}, expected {}",
                            dim,
                            crate::hdc::vector::HD_DIMENSIONS
                        ),
                    ))
                }
            }
            _ => Err(PslError::InvalidAuditTarget {
                reason: "DimensionalityAxiom requires a Vector target".to_string(),
            }),
        }
    }
}

/// Verifies that raw bytes from an external source are non-empty
/// and within a sane size bound. Hostile data guard.
pub struct DataIntegrityAxiom {
    pub max_bytes: usize,
}

impl Axiom for DataIntegrityAxiom {
    fn id(&self) -> &str {
        "Axiom:Data_Integrity"
    }

    fn description(&self) -> &str {
        "Verifies external data is non-empty and within size bounds"
    }

    fn verify(&self, target: &AuditTarget) -> Result<AxiomVerdict, PslError> {
        debuglog!("DataIntegrityAxiom::verify, max_bytes={}", self.max_bytes);
        match target {
            AuditTarget::RawBytes { source, data } => {
                if data.is_empty() {
                    return Ok(AxiomVerdict::fail(
                        self.id().to_string(),
                        0.0,
                        format!("Empty payload from source '{}'", source),
                    ));
                }
                if data.len() > self.max_bytes {
                    return Ok(AxiomVerdict::fail(
                        self.id().to_string(),
                        0.2,
                        format!(
                            "Payload from '{}' exceeds limit: {} > {}",
                            source,
                            data.len(),
                            self.max_bytes
                        ),
                    ));
                }
                Ok(AxiomVerdict::pass(
                    self.id().to_string(),
                    1.0,
                    format!(
                        "Payload from '{}': {} bytes, within limit",
                        source,
                        data.len()
                    ),
                ))
            }
            _ => Err(PslError::InvalidAuditTarget {
                reason: "DataIntegrityAxiom requires a RawBytes target".to_string(),
            }),
        }
    }
}
