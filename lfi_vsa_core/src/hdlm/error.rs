// ============================================================
// HDLM Error Types — Multi-Level Semantic Mapping errors.
// ============================================================

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum HdlmError {
    /// AST node could not be constructed from the given input.
    MalformedAst {
        reason: String,
    },
    /// Tier 1 forensic generation failed.
    Tier1GenerationFailed {
        reason: String,
    },
    /// Tier 2 decorative expansion failed.
    Tier2ExpansionFailed {
        reason: String,
    },
    /// Attempted to traverse an empty AST.
    EmptyAst,
    /// Vector-to-AST decoding encountered an unmapped symbol.
    UnmappedSymbol {
        symbol_id: usize,
    },
}

impl fmt::Display for HdlmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MalformedAst { reason } => {
                write!(f, "HDLM MalformedAst: {}", reason)
            }
            Self::Tier1GenerationFailed { reason } => {
                write!(f, "HDLM Tier1GenerationFailed: {}", reason)
            }
            Self::Tier2ExpansionFailed { reason } => {
                write!(f, "HDLM Tier2ExpansionFailed: {}", reason)
            }
            Self::EmptyAst => {
                write!(f, "HDLM EmptyAst: cannot operate on empty tree")
            }
            Self::UnmappedSymbol { symbol_id } => {
                write!(f, "HDLM UnmappedSymbol: id={}", symbol_id)
            }
        }
    }
}

impl std::error::Error for HdlmError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let e = HdlmError::MalformedAst { reason: "bad node".into() };
        assert!(format!("{}", e).contains("bad node"));

        let e2 = HdlmError::EmptyAst;
        assert!(format!("{}", e2).contains("empty"));

        let e3 = HdlmError::UnmappedSymbol { symbol_id: 42 };
        assert!(format!("{}", e3).contains("42"));
    }

    #[test]
    fn test_error_equality() {
        assert_eq!(HdlmError::EmptyAst, HdlmError::EmptyAst);
        assert_ne!(
            HdlmError::MalformedAst { reason: "a".into() },
            HdlmError::MalformedAst { reason: "b".into() }
        );
    }

    #[test]
    fn test_error_is_std_error() {
        let e: Box<dyn std::error::Error> = Box::new(HdlmError::EmptyAst);
        assert!(format!("{}", e).contains("empty"));
    }
}
