// ============================================================
// HDC Error Types — Forensic Fault Handling
// ============================================================

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum HdcError {
    DimensionMismatch { expected: usize, actual: usize },
    InitializationFailed { reason: String },
    MemoryFull,
    InvalidBipolarValue,
    PersistenceFailure { detail: String },
    LogicFault { reason: String },
    EmptyBundle,
}

impl fmt::Display for HdcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HdcError::DimensionMismatch { expected, actual } => 
                write!(f, "Dimension Mismatch: expected {}, got {}", expected, actual),
            HdcError::InitializationFailed { reason } => 
                write!(f, "Initialization Failed: {}", reason),
            HdcError::MemoryFull => write!(f, "Holographic Memory Full"),
            HdcError::InvalidBipolarValue => write!(f, "Values must be strictly -1 or 1"),
            HdcError::PersistenceFailure { detail } => 
                write!(f, "Failed to commit state: {}", detail),
            HdcError::LogicFault { reason } => 
                write!(f, "Material Logic Fault: {}", reason),
            HdcError::EmptyBundle => write!(f, "Attempted to bundle zero vectors"),
        }
    }
}

impl std::error::Error for HdcError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_messages() {
        assert!(format!("{}", HdcError::DimensionMismatch { expected: 10000, actual: 5000 }).contains("10000"));
        assert!(format!("{}", HdcError::InitializationFailed { reason: "no mem".into() }).contains("no mem"));
        assert!(format!("{}", HdcError::MemoryFull).contains("Full"));
        assert!(format!("{}", HdcError::InvalidBipolarValue).contains("-1 or 1"));
        assert!(format!("{}", HdcError::PersistenceFailure { detail: "disk".into() }).contains("disk"));
        assert!(format!("{}", HdcError::LogicFault { reason: "bad axiom".into() }).contains("bad axiom"));
        assert!(format!("{}", HdcError::EmptyBundle).contains("zero"));
    }

    #[test]
    fn test_error_equality() {
        assert_eq!(HdcError::MemoryFull, HdcError::MemoryFull);
        assert_eq!(HdcError::EmptyBundle, HdcError::EmptyBundle);
        assert_ne!(HdcError::MemoryFull, HdcError::EmptyBundle);
    }

    #[test]
    fn test_error_is_std_error() {
        let e: Box<dyn std::error::Error> = Box::new(HdcError::MemoryFull);
        assert!(format!("{}", e).contains("Full"));
    }

    #[test]
    fn test_error_clone() {
        let original = HdcError::LogicFault { reason: "test".into() };
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }
}
