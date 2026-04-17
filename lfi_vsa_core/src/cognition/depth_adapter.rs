// ============================================================
// Explanation Depth Adapter — Match user expertise level
//
// Tracks per-domain expertise from conversation history.
// Beginner gets simple analogies, expert gets technical depth.
//
// Expertise levels:
//   Novice:     Simple language, analogies, step-by-step
//   Intermediate: Some jargon OK, moderate detail
//   Advanced:   Full technical depth, assume prerequisites
//   Expert:     Peer-level discussion, skip basics entirely
// ============================================================

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ExpertiseLevel {
    Novice,
    Intermediate,
    Advanced,
    Expert,
}

impl ExpertiseLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Novice => "novice",
            Self::Intermediate => "intermediate",
            Self::Advanced => "advanced",
            Self::Expert => "expert",
        }
    }

    /// Prompt modifier for Ollama system prompt.
    pub fn prompt_guidance(&self) -> &'static str {
        match self {
            Self::Novice => "Explain like teaching a beginner. Use simple analogies, avoid jargon. Break into numbered steps. Define any technical term you use.",
            Self::Intermediate => "User has basic understanding. Use some technical terms but explain complex ones. Give practical examples.",
            Self::Advanced => "User is knowledgeable. Use technical vocabulary freely. Focus on nuances, edge cases, and deeper implications.",
            Self::Expert => "Peer-level discussion. Skip fundamentals entirely. Discuss trade-offs, recent developments, and advanced techniques. Be concise.",
        }
    }

    fn from_score(score: f64) -> Self {
        match score {
            s if s >= 0.8 => Self::Expert,
            s if s >= 0.5 => Self::Advanced,
            s if s >= 0.2 => Self::Intermediate,
            _ => Self::Novice,
        }
    }
}

/// Tracks user expertise per domain and adapts explanation depth.
pub struct DepthAdapter {
    /// Per-domain expertise score (0.0 = novice, 1.0 = expert).
    domain_scores: HashMap<String, f64>,
    /// Per-domain interaction count.
    domain_interactions: HashMap<String, usize>,
    /// Default level for unknown domains.
    default_level: ExpertiseLevel,
}

impl DepthAdapter {
    pub fn new() -> Self {
        Self {
            domain_scores: HashMap::new(),
            domain_interactions: HashMap::new(),
            default_level: ExpertiseLevel::Intermediate,
        }
    }

    /// Observe a user message to update expertise estimates.
    pub fn observe(&mut self, input: &str, domain: Option<&str>) {
        let domain_key = domain.unwrap_or("general").to_string();
        *self.domain_interactions.entry(domain_key.clone()).or_default() += 1;

        let lower = input.to_lowercase();
        let current = self.domain_scores.entry(domain_key).or_insert(0.3);

        // Signals that increase estimated expertise
        let expert_signals = [
            "specifically", "technically", "implementation", "architecture",
            "trade-off", "optimize", "benchmark", "latency", "throughput",
            "complexity", "algorithm", "protocol", "kernel", "syscall",
            "concurrency", "deadlock", "mutex", "atomic",
        ];
        let novice_signals = [
            "what is", "explain like", "simple", "basic", "beginner",
            "don't understand", "confused", "eli5", "in simple terms",
            "step by step", "how do i start", "what does",
        ];

        let expert_hits = expert_signals.iter().filter(|s| lower.contains(*s)).count();
        let novice_hits = novice_signals.iter().filter(|s| lower.contains(*s)).count();

        // Adjust score: expert signals push up, novice push down
        if expert_hits > 0 {
            *current = (*current + 0.05 * expert_hits as f64).min(1.0);
        }
        if novice_hits > 0 {
            *current = (*current - 0.1 * novice_hits as f64).max(0.0);
        }

        // Gradual drift toward intermediate with many interactions
        let interactions = self.domain_interactions.get(&lower).copied().unwrap_or(0);
        if interactions > 10 && *current < 0.3 {
            *current += 0.02; // Assume they're learning
        }
    }

    /// Get the estimated expertise level for a domain.
    pub fn level(&self, domain: Option<&str>) -> ExpertiseLevel {
        let key = domain.unwrap_or("general");
        match self.domain_scores.get(key) {
            Some(score) => ExpertiseLevel::from_score(*score),
            None => self.default_level.clone(),
        }
    }

    /// Get prompt guidance for the current domain context.
    pub fn guidance(&self, domain: Option<&str>) -> &'static str {
        self.level(domain).prompt_guidance()
    }

    /// Get all domain expertise levels.
    pub fn all_levels(&self) -> Vec<(String, ExpertiseLevel, f64)> {
        self.domain_scores.iter()
            .map(|(d, s)| (d.clone(), ExpertiseLevel::from_score(*s), *s))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_intermediate() {
        let adapter = DepthAdapter::new();
        assert_eq!(adapter.level(Some("physics")), ExpertiseLevel::Intermediate);
    }

    #[test]
    fn test_expert_detection() {
        let mut adapter = DepthAdapter::new();
        for _ in 0..5 {
            adapter.observe("I need to optimize the concurrency model and reduce mutex contention in the kernel implementation", Some("programming"));
        }
        let level = adapter.level(Some("programming"));
        assert!(level == ExpertiseLevel::Advanced || level == ExpertiseLevel::Expert);
    }

    #[test]
    fn test_novice_detection() {
        let mut adapter = DepthAdapter::new();
        adapter.observe("What is a variable? I don't understand programming at all. Explain like I'm a beginner", Some("programming"));
        assert_eq!(adapter.level(Some("programming")), ExpertiseLevel::Novice);
    }

    #[test]
    fn test_prompt_guidance() {
        assert!(ExpertiseLevel::Novice.prompt_guidance().contains("beginner"));
        assert!(ExpertiseLevel::Expert.prompt_guidance().contains("Peer-level"));
    }

    #[test]
    fn test_multi_domain() {
        let mut adapter = DepthAdapter::new();
        adapter.observe("Explain quantum entanglement in simple terms", Some("physics"));
        adapter.observe("I need to benchmark the throughput of this protocol implementation", Some("programming"));
        assert_eq!(adapter.level(Some("physics")), ExpertiseLevel::Novice);
        assert!(adapter.level(Some("programming")) >= ExpertiseLevel::Intermediate);
    }
}
