// ============================================================
// PSL Supervisor Error Types
// Section 1.II: Zero-Hallucination enforcement errors.
// ============================================================

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum PslError {
    /// An axiom was violated during audit.
    AxiomViolation {
        axiom_id: String,
        detail: String,
    },
    /// A structural failure occurred during axiom execution.
    AxiomFailure {
        axiom_id: String,
        reason: String,
    },
    /// The audited data had insufficient dimensionality or format.
    InvalidAuditTarget {
        reason: String,
    },
    /// Confidence score fell below the trust threshold.
    TrustThresholdBreached {
        required: f64,
        actual: f64,
    },
    /// Remote GPU return failed integrity check.
    HostileDataDetected {
        source: String,
        reason: String,
    },
    /// No axioms were loaded into the supervisor.
    EmptyAxiomSet,
}

impl fmt::Display for PslError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AxiomViolation { axiom_id, detail } => {
                write!(f, "PSL AxiomViolation [{}]: {}", axiom_id, detail)
            }
            Self::AxiomFailure { axiom_id, reason } => {
                write!(f, "PSL AxiomFailure [{}]: {}", axiom_id, reason)
            }
            Self::InvalidAuditTarget { reason } => {
                write!(f, "PSL InvalidAuditTarget: {}", reason)
            }
            Self::TrustThresholdBreached { required, actual } => {
                write!(
                    f,
                    "PSL TrustThresholdBreached: required {:.4}, actual {:.4}",
                    required, actual
                )
            }
            Self::HostileDataDetected { source, reason } => {
                write!(f, "PSL HostileDataDetected [{}]: {}", source, reason)
            }
            Self::EmptyAxiomSet => {
                write!(f, "PSL EmptyAxiomSet: no axioms loaded for audit")
            }
        }
    }
}

impl std::error::Error for PslError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_psl_error_display() {
        let e = PslError::AxiomViolation { axiom_id: "DimAxiom".into(), detail: "wrong dim".into() };
        let msg = format!("{}", e);
        assert!(msg.contains("DimAxiom") && msg.contains("wrong dim"));

        let e2 = PslError::TrustThresholdBreached { required: 0.75, actual: 0.5 };
        let msg2 = format!("{}", e2);
        assert!(msg2.contains("0.75") && msg2.contains("0.5"));

        let e3 = PslError::EmptyAxiomSet;
        assert!(format!("{}", e3).contains("no axioms"));
    }

    #[test]
    fn test_psl_error_equality() {
        assert_eq!(PslError::EmptyAxiomSet, PslError::EmptyAxiomSet);
        assert_ne!(
            PslError::AxiomViolation { axiom_id: "a".into(), detail: "b".into() },
            PslError::AxiomFailure { axiom_id: "a".into(), reason: "b".into() }
        );
    }

    #[test]
    fn test_hostile_data_error() {
        let e = PslError::HostileDataDetected {
            source: "remote_gpu".into(),
            reason: "integrity mismatch".into(),
        };
        let msg = format!("{}", e);
        assert!(msg.contains("remote_gpu") && msg.contains("integrity"));
    }
}
