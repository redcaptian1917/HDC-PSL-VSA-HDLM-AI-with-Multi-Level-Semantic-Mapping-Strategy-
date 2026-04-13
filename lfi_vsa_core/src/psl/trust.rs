// ============================================================
// Trust Level Hierarchy — The Sovereign Audit Tier
// ============================================================

use serde::{Serialize, Deserialize};

/// Hierarchy of trust for material and neural data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TrustLevel {
    /// Level 0: Adversarial/Blocked.
    Forbidden,
    /// Level 1: Low-confidence/Untrusted.
    Untrusted,
    /// Level 2: High-confidence/Standard data.
    Trusted,
    /// Level 3: Verified material truth.
    Sovereign,
}

impl TrustLevel {
    pub fn permits_execution(&self) -> bool {
        match self {
            TrustLevel::Trusted | TrustLevel::Sovereign => true,
            _ => false,
        }
    }

    /// Whether this trust level requires additional verification before use.
    pub fn needs_verification(&self) -> bool {
        matches!(self, TrustLevel::Untrusted)
    }

    /// Whether this trust level blocks all operations.
    pub fn is_blocked(&self) -> bool {
        matches!(self, TrustLevel::Forbidden)
    }

    /// Human-readable label.
    pub fn label(&self) -> &str {
        match self {
            TrustLevel::Forbidden => "FORBIDDEN",
            TrustLevel::Untrusted => "UNTRUSTED",
            TrustLevel::Trusted => "TRUSTED",
            TrustLevel::Sovereign => "SOVEREIGN",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trust_ordering() {
        assert!(TrustLevel::Sovereign > TrustLevel::Trusted);
        assert!(TrustLevel::Trusted > TrustLevel::Untrusted);
        assert!(TrustLevel::Untrusted > TrustLevel::Forbidden);
    }

    #[test]
    fn test_permits_execution() {
        assert!(TrustLevel::Sovereign.permits_execution());
        assert!(TrustLevel::Trusted.permits_execution());
        assert!(!TrustLevel::Untrusted.permits_execution());
        assert!(!TrustLevel::Forbidden.permits_execution());
    }

    #[test]
    fn test_needs_verification() {
        assert!(TrustLevel::Untrusted.needs_verification());
        assert!(!TrustLevel::Trusted.needs_verification());
        assert!(!TrustLevel::Sovereign.needs_verification());
        assert!(!TrustLevel::Forbidden.needs_verification());
    }

    #[test]
    fn test_is_blocked() {
        assert!(TrustLevel::Forbidden.is_blocked());
        assert!(!TrustLevel::Untrusted.is_blocked());
        assert!(!TrustLevel::Trusted.is_blocked());
        assert!(!TrustLevel::Sovereign.is_blocked());
    }

    #[test]
    fn test_labels() {
        assert_eq!(TrustLevel::Sovereign.label(), "SOVEREIGN");
        assert_eq!(TrustLevel::Trusted.label(), "TRUSTED");
        assert_eq!(TrustLevel::Untrusted.label(), "UNTRUSTED");
        assert_eq!(TrustLevel::Forbidden.label(), "FORBIDDEN");
    }

    #[test]
    fn test_serialization() {
        let level = TrustLevel::Sovereign;
        let json = serde_json::to_string(&level).unwrap();
        let recovered: TrustLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(recovered, TrustLevel::Sovereign);
    }

    #[test]
    fn test_equality() {
        assert_eq!(TrustLevel::Trusted, TrustLevel::Trusted);
        assert_ne!(TrustLevel::Trusted, TrustLevel::Untrusted);
    }
}
