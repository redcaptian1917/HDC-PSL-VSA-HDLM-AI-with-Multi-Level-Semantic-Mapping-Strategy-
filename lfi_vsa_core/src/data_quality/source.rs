// ============================================================
// Source Newtype — Validated data source identifiers
// 500-task list item 424: Replace String with validated newtype
//
// PURPOSE: Track provenance of every fact in brain.db. Sources
// are the origin of data (datasets, scrapers, user corrections,
// Ollama generation). Using a newtype ensures consistent naming
// and enables source-level quality filtering.
// ============================================================

use std::fmt;

/// A validated data source identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Source(String);

/// Source trust tier — higher tiers are more reliable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SourceTier {
    /// Unvetted or unknown provenance.
    Unvetted = 0,
    /// Community-contributed, lightly reviewed.
    Community = 1,
    /// Established dataset with known methodology.
    Curated = 2,
    /// Expert-verified, hand-crafted, or peer-reviewed.
    Authoritative = 3,
}

impl Source {
    /// Create a new source identifier, normalizing to lowercase.
    pub fn new(name: &str) -> Self {
        Self(name.to_lowercase().trim().replace(' ', "_"))
    }

    /// Get the source name as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Infer a trust tier from the source name.
    /// BUG ASSUMPTION: This heuristic should be replaced with
    /// a lookup table or database column for production use.
    pub fn infer_tier(&self) -> SourceTier {
        let s = self.0.as_str();
        if s.starts_with("user_correction") || s.starts_with("handcrafted") || s.starts_with("expert") {
            SourceTier::Authoritative
        } else if s.starts_with("oasst2") || s.starts_with("dolly") || s.starts_with("openhermes")
            || s.starts_with("sharegpt") || s.starts_with("stanford_alpaca") || s.starts_with("metamathqa")
        {
            SourceTier::Curated
        } else if s.starts_with("ollama") || s.starts_with("self_play") || s.starts_with("magpie") {
            SourceTier::Community
        } else {
            SourceTier::Unvetted
        }
    }

    /// Suggested quality floor based on source tier.
    pub fn quality_floor(&self) -> f64 {
        match self.infer_tier() {
            SourceTier::Authoritative => 0.85,
            SourceTier::Curated => 0.70,
            SourceTier::Community => 0.50,
            SourceTier::Unvetted => 0.30,
        }
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Source {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for Source {
    fn from(s: String) -> Self {
        Self::new(&s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalization() {
        let s = Source::new("  OpenHermes 2.5  ");
        assert_eq!(s.as_str(), "openhermes_2.5");
    }

    #[test]
    fn test_handcrafted_is_authoritative() {
        assert_eq!(Source::new("handcrafted_identity").infer_tier(), SourceTier::Authoritative);
        assert_eq!(Source::new("user_correction").infer_tier(), SourceTier::Authoritative);
        assert_eq!(Source::new("expert_pentest").infer_tier(), SourceTier::Authoritative);
    }

    #[test]
    fn test_datasets_are_curated() {
        assert_eq!(Source::new("oasst2_en_quality").infer_tier(), SourceTier::Curated);
        assert_eq!(Source::new("dolly_15k").infer_tier(), SourceTier::Curated);
        assert_eq!(Source::new("openhermes_2_5").infer_tier(), SourceTier::Curated);
    }

    #[test]
    fn test_ollama_is_community() {
        assert_eq!(Source::new("ollama_domain_gap").infer_tier(), SourceTier::Community);
        assert_eq!(Source::new("self_play_v2").infer_tier(), SourceTier::Community);
    }

    #[test]
    fn test_unknown_is_unvetted() {
        assert_eq!(Source::new("random_scraper_xyz").infer_tier(), SourceTier::Unvetted);
    }

    #[test]
    fn test_quality_floor_ordering() {
        assert!(Source::new("handcrafted").quality_floor() > Source::new("oasst2").quality_floor());
        assert!(Source::new("oasst2").quality_floor() > Source::new("ollama").quality_floor());
        assert!(Source::new("ollama").quality_floor() > Source::new("unknown").quality_floor());
    }

    #[test]
    fn test_tier_ordering() {
        assert!(SourceTier::Authoritative > SourceTier::Curated);
        assert!(SourceTier::Curated > SourceTier::Community);
        assert!(SourceTier::Community > SourceTier::Unvetted);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Source::new("dolly_15k")), "dolly_15k");
    }

    #[test]
    fn test_from_traits() {
        let s1: Source = "test".into();
        let s2: Source = String::from("test").into();
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_empty_source() {
        let s = Source::new("");
        assert_eq!(s.as_str(), "");
        assert_eq!(s.infer_tier(), SourceTier::Unvetted);
    }
}
