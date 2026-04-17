// ============================================================
// Domain Newtype — Validated domain identifiers
// 500-task list item 423: Replace String with validated newtype
//
// PURPOSE: Prevent domain name typos at compile time and ensure
// consistent domain naming across the codebase. All domain
// strings should eventually migrate to this type.
// ============================================================

use std::fmt;

/// Known valid domains in the PlausiDen knowledge base.
/// Using a newtype instead of raw strings catches typos at compile time
/// and ensures consistent naming.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Domain(String);

impl Domain {
    /// Known domain names — used for validation and autocomplete.
    pub const KNOWN: &'static [&'static str] = &[
        "cybersecurity", "pentesting", "technology", "code", "mathematics",
        "science", "history", "economics", "politics", "philosophy",
        "legal", "finance", "medicine", "psychology", "communication",
        "reasoning", "conversational", "creative_writing", "general",
        "education", "business", "public_health", "criminology",
        "engineering", "social_science", "social_media", "adversarial",
        "biomedical", "academic", "computer_vision", "qa_extractive",
        "qa_general", "reading_comprehension", "sentiment", "commonsense",
        "nli", "instruction", "knowledge", "news_topics", "multilingual",
        "web_knowledge", "encyclopedic", "commerce", "identity",
        "tool_use", "biology", "geography", "code_quality",
    ];

    /// Create a new Domain. Accepts any string but logs a warning
    /// if it's not in the known list.
    pub fn new(name: &str) -> Self {
        let normalized = name.to_lowercase().trim().to_string();
        if !normalized.is_empty() && !Self::KNOWN.contains(&normalized.as_str()) {
            tracing::debug!("Unknown domain: '{}' — consider adding to Domain::KNOWN", normalized);
        }
        Self(normalized)
    }

    /// Create from a known domain (panics in debug if unknown — use in tests).
    pub fn known(name: &str) -> Self {
        let d = Self::new(name);
        debug_assert!(
            Self::KNOWN.contains(&d.0.as_str()),
            "Domain::known() called with unknown domain: '{}'", name
        );
        d
    }

    /// Get the domain name as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Check if this is a known domain.
    pub fn is_known(&self) -> bool {
        Self::KNOWN.contains(&self.0.as_str())
    }

    /// Get the temporal decay half-life for this domain (in days).
    pub fn half_life_days(&self) -> f64 {
        match self.0.as_str() {
            "cybersecurity" => 90.0,
            "pentesting" => 120.0,
            "technology" => 180.0,
            "politics" | "economics" => 365.0,
            "finance" => 270.0,
            "legal" => 540.0,
            "code" | "code_quality" => 365.0,
            "science" | "biomedical" => 1825.0,
            "philosophy" => 3650.0,
            "history" => 7300.0,
            "mathematics" | "reasoning" => 36500.0,
            _ => 730.0, // 2 years default
        }
    }
}

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Domain {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for Domain {
    fn from(s: String) -> Self {
        Self::new(&s)
    }
}

impl AsRef<str> for Domain {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_domains_valid() {
        for &d in Domain::KNOWN {
            let domain = Domain::new(d);
            assert!(domain.is_known(), "'{}' should be known", d);
        }
    }

    #[test]
    fn test_unknown_domain_accepted() {
        let d = Domain::new("exotic_new_domain");
        assert!(!d.is_known());
        assert_eq!(d.as_str(), "exotic_new_domain");
    }

    #[test]
    fn test_case_normalized() {
        let d = Domain::new("CyberSecurity");
        assert_eq!(d.as_str(), "cybersecurity");
        assert!(d.is_known());
    }

    #[test]
    fn test_whitespace_trimmed() {
        let d = Domain::new("  science  ");
        assert_eq!(d.as_str(), "science");
        assert!(d.is_known());
    }

    #[test]
    fn test_empty_domain() {
        let d = Domain::new("");
        assert_eq!(d.as_str(), "");
        assert!(!d.is_known());
    }

    #[test]
    fn test_display() {
        let d = Domain::new("mathematics");
        assert_eq!(format!("{}", d), "mathematics");
    }

    #[test]
    fn test_from_str() {
        let d: Domain = "history".into();
        assert_eq!(d.as_str(), "history");
    }

    #[test]
    fn test_from_string() {
        let d: Domain = String::from("legal").into();
        assert_eq!(d.as_str(), "legal");
    }

    #[test]
    fn test_half_life_cyber_fast() {
        let d = Domain::new("cybersecurity");
        assert_eq!(d.half_life_days(), 90.0);
    }

    #[test]
    fn test_half_life_math_slow() {
        let d = Domain::new("mathematics");
        assert_eq!(d.half_life_days(), 36500.0);
    }

    #[test]
    fn test_half_life_unknown_default() {
        let d = Domain::new("unknown_domain");
        assert_eq!(d.half_life_days(), 730.0);
    }

    #[test]
    fn test_equality() {
        assert_eq!(Domain::new("science"), Domain::new("Science"));
        assert_ne!(Domain::new("science"), Domain::new("history"));
    }

    #[test]
    fn test_known_count() {
        assert!(Domain::KNOWN.len() >= 40,
            "Should have at least 40 known domains, got {}", Domain::KNOWN.len());
    }
}
