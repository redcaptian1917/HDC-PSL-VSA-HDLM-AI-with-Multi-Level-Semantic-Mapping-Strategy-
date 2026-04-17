// ============================================================
// Temporal Decay Module — Domain-Specific Fact Freshness
// Sprint 1: Quality Ceiling — weight facts by recency
//
// PURPOSE: Not all knowledge decays at the same rate.
// Cybersecurity CVEs decay fast (months). Historical facts
// decay slowly (decades). Math never decays.
//
// BUG ASSUMPTION: Domain half-life values are estimates and
// should be calibrated against benchmark performance over time.
// ============================================================

use std::collections::HashMap;

/// Domain-specific temporal decay calculator.
///
/// Uses exponential decay: score = base_quality * 2^(-age / half_life)
/// where age is in days and half_life is domain-specific.
pub struct TemporalDecay {
    /// Domain -> half-life in days
    half_lives: HashMap<String, f64>,
    /// Default half-life for unknown domains
    default_half_life: f64,
}

impl TemporalDecay {
    /// Create a new TemporalDecay with domain-specific half-lives.
    ///
    /// BUG ASSUMPTION: These half-lives are initial estimates.
    /// They should be tuned based on training loss per-domain
    /// to optimize for model accuracy, not just freshness.
    pub fn new() -> Self {
        let mut half_lives = HashMap::new();

        // Rapidly-changing domains (months)
        half_lives.insert("cybersecurity".into(), 90.0);   // CVEs, patches, new threats
        half_lives.insert("pentesting".into(), 120.0);     // Tools, techniques evolve
        half_lives.insert("technology".into(), 180.0);     // Software, hardware cycles

        // Moderately-changing domains (1-3 years)
        half_lives.insert("politics".into(), 365.0);       // Elections, policies
        half_lives.insert("economics".into(), 365.0);      // Market conditions
        half_lives.insert("finance".into(), 270.0);        // Regulations, products
        half_lives.insert("legal".into(), 540.0);          // Laws change slowly
        half_lives.insert("code".into(), 365.0);           // Libraries, frameworks

        // Slowly-changing domains (5+ years)
        half_lives.insert("science".into(), 1825.0);       // 5 years
        half_lives.insert("philosophy".into(), 3650.0);    // 10 years
        half_lives.insert("history".into(), 7300.0);       // 20 years
        half_lives.insert("mathematics".into(), 36500.0);  // 100 years (effectively never)
        half_lives.insert("reasoning".into(), 36500.0);    // Logic is timeless

        // General/conversational
        half_lives.insert("conversational".into(), 730.0); // 2 years
        half_lives.insert("general".into(), 730.0);

        Self {
            half_lives,
            default_half_life: 730.0, // 2 years default
        }
    }

    /// Get the half-life for a domain in days.
    pub fn half_life_for(&self, domain: &str) -> f64 {
        self.half_lives
            .get(domain)
            .copied()
            .unwrap_or(self.default_half_life)
    }

    /// Calculate the decay-adjusted quality score for a fact.
    ///
    /// # Arguments
    /// * `base_quality` - Original quality score (0.0 to 1.0)
    /// * `domain` - Domain of the fact
    /// * `age_days` - Age of the fact in days
    ///
    /// # Returns
    /// Decay-adjusted quality score (always between 0.0 and base_quality)
    pub fn adjusted_quality(&self, base_quality: f64, domain: &str, age_days: f64) -> f64 {
        if age_days <= 0.0 {
            return base_quality;
        }

        let half_life = self.half_life_for(domain);
        let decay_factor = 2.0_f64.powf(-age_days / half_life);

        // Clamp to [0.0, base_quality]
        (base_quality * decay_factor).clamp(0.0, base_quality)
    }

    /// Calculate the minimum age (in days) at which a fact's quality
    /// drops below a given threshold.
    ///
    /// Useful for pruning: "how old must a cybersecurity fact be
    /// before it's no longer trustworthy?"
    pub fn age_at_threshold(&self, base_quality: f64, domain: &str, threshold: f64) -> f64 {
        if threshold >= base_quality || threshold <= 0.0 {
            return 0.0;
        }

        let half_life = self.half_life_for(domain);
        // Solve: threshold = base * 2^(-age / half_life)
        // age = -half_life * log2(threshold / base)
        -half_life * (threshold / base_quality).log2()
    }

    /// Add or update a domain's half-life.
    pub fn set_half_life(&mut self, domain: &str, half_life_days: f64) {
        self.half_lives.insert(domain.to_string(), half_life_days);
    }

    /// List all configured domains and their half-lives.
    pub fn domain_half_lives(&self) -> &HashMap<String, f64> {
        &self.half_lives
    }
}

impl Default for TemporalDecay {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fresh_fact_no_decay() {
        let td = TemporalDecay::new();
        let score = td.adjusted_quality(0.9, "cybersecurity", 0.0);
        assert!((score - 0.9).abs() < f64::EPSILON);
    }

    #[test]
    fn test_half_life_halves_quality() {
        let td = TemporalDecay::new();
        // Cybersecurity: 90 day half-life
        let score = td.adjusted_quality(1.0, "cybersecurity", 90.0);
        assert!(
            (score - 0.5).abs() < 0.01,
            "At half-life, quality should be ~0.5, got {}",
            score
        );
    }

    #[test]
    fn test_math_barely_decays() {
        let td = TemporalDecay::new();
        // Math: 100 year half-life, check at 1 year
        let score = td.adjusted_quality(1.0, "mathematics", 365.0);
        assert!(
            score > 0.99,
            "Math facts should barely decay in a year, got {}",
            score
        );
    }

    #[test]
    fn test_old_cyber_fact_is_stale() {
        let td = TemporalDecay::new();
        // 2 years old cybersecurity fact
        let score = td.adjusted_quality(0.9, "cybersecurity", 730.0);
        assert!(
            score < 0.01,
            "2-year-old cyber fact should be nearly worthless, got {}",
            score
        );
    }

    #[test]
    fn test_age_at_threshold() {
        let td = TemporalDecay::new();
        // How many days until a 0.9 quality cyber fact drops below 0.5?
        let age = td.age_at_threshold(0.9, "cybersecurity", 0.5);
        // Should be approximately 75 days (90 * log2(0.9/0.5) = 90 * 0.848 = ~76)
        assert!(
            (age - 76.3).abs() < 1.0,
            "Threshold age should be ~76 days, got {}",
            age
        );
    }

    #[test]
    fn test_unknown_domain_uses_default() {
        let td = TemporalDecay::new();
        let score = td.adjusted_quality(1.0, "unknown_domain", 730.0);
        // Default half-life is 730 days, so at 730 days quality = 0.5
        assert!(
            (score - 0.5).abs() < 0.01,
            "Unknown domain at default half-life should be ~0.5, got {}",
            score
        );
    }

    #[test]
    fn test_set_custom_half_life() {
        let mut td = TemporalDecay::new();
        td.set_half_life("custom", 30.0);
        let score = td.adjusted_quality(1.0, "custom", 30.0);
        assert!(
            (score - 0.5).abs() < 0.01,
            "Custom 30-day half-life at 30 days should be ~0.5, got {}",
            score
        );
    }

    #[test]
    fn test_negative_age_returns_base() {
        let td = TemporalDecay::new();
        let score = td.adjusted_quality(0.8, "science", -10.0);
        assert!((score - 0.8).abs() < f64::EPSILON);
    }
}
