// ============================================================
// Proactive Suggestion Engine — Anticipate user needs
//
// Analyzes conversation patterns to offer relevant suggestions:
// - Repeated topic → offer a study plan or deeper exploration
// - Multiple errors → suggest debugging approach
// - Domain shift → suggest related resources
// - Idle/new session → suggest conversation starters
//
// SUPERSOCIETY: The difference between reactive and proactive AI.
// Don't wait to be asked — notice patterns and offer value.
// ============================================================

use std::collections::HashMap;

/// A proactive suggestion for the user.
#[derive(Debug, Clone)]
pub struct Suggestion {
    pub text: String,
    pub category: SuggestionCategory,
    pub relevance: f64,
    pub action: Option<String>,
}

/// Categories of suggestions.
#[derive(Debug, Clone, PartialEq)]
pub enum SuggestionCategory {
    /// User keeps asking about this topic — offer to go deeper
    DeepDive,
    /// User seems stuck — offer alternative approaches
    Unstuck,
    /// Related topic the user might find interesting
    RelatedTopic,
    /// Conversation starter for new/idle sessions
    Starter,
    /// User frequently asks about this domain — offer a learning path
    LearningPath,
    /// Follow-up on a previous conversation
    FollowUp,
}

/// Tracks conversation patterns to generate suggestions.
pub struct SuggestionEngine {
    /// Topic frequency counts from recent messages.
    topic_counts: HashMap<String, usize>,
    /// Recent query intents.
    recent_intents: Vec<String>,
    /// Domains the user engages with most.
    domain_affinity: HashMap<String, usize>,
    /// Total messages processed.
    message_count: usize,
}

impl SuggestionEngine {
    pub fn new() -> Self {
        Self {
            topic_counts: HashMap::new(),
            recent_intents: Vec::new(),
            domain_affinity: HashMap::new(),
            message_count: 0,
        }
    }

    /// Feed a user message to update pattern tracking.
    pub fn observe(&mut self, input: &str, domain: Option<&str>) {
        self.message_count += 1;

        // Track topic keywords
        let stopwords = ["the", "and", "for", "are", "but", "not", "you", "all",
            "can", "was", "has", "how", "who", "what", "this", "that", "with"];
        let stop_set: std::collections::HashSet<&str> = stopwords.iter().copied().collect();

        for word in input.split_whitespace() {
            let clean = word.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect::<String>();
            if clean.len() >= 5 && !stop_set.contains(clean.as_str()) {
                *self.topic_counts.entry(clean).or_default() += 1;
            }
        }

        // Track domain affinity
        if let Some(d) = domain {
            *self.domain_affinity.entry(d.to_string()).or_default() += 1;
        }

        // Keep recent intents (last 20)
        if self.recent_intents.len() > 20 {
            self.recent_intents.remove(0);
        }
        self.recent_intents.push(input.chars().take(100).collect());
    }

    /// Generate suggestions based on observed patterns.
    pub fn suggest(&self) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        // 1. DEEP DIVE: Topics mentioned 3+ times
        let mut hot_topics: Vec<(&String, &usize)> = self.topic_counts.iter()
            .filter(|(_, c)| **c >= 3)
            .collect();
        hot_topics.sort_by(|a, b| b.1.cmp(a.1));

        for (topic, count) in hot_topics.iter().take(2) {
            suggestions.push(Suggestion {
                text: format!("You've been exploring '{}' — want me to create a comprehensive guide or study plan?", topic),
                category: SuggestionCategory::DeepDive,
                relevance: (**count as f64 / self.message_count.max(1) as f64).min(1.0),
                action: Some(format!("deep_dive:{}", topic)),
            });
        }

        // 2. LEARNING PATH: Dominant domain
        if let Some((top_domain, count)) = self.domain_affinity.iter().max_by_key(|(_, c)| *c) {
            if *count >= 5 {
                suggestions.push(Suggestion {
                    text: format!("I notice you're really into {}. Want me to design a structured learning path with progressive difficulty?", top_domain),
                    category: SuggestionCategory::LearningPath,
                    relevance: 0.7,
                    action: Some(format!("learning_path:{}", top_domain)),
                });
            }
        }

        // 3. RELATED TOPIC: Cross-domain bridges
        if self.domain_affinity.len() >= 2 {
            let domains: Vec<&String> = self.domain_affinity.keys().collect();
            if domains.len() >= 2 {
                suggestions.push(Suggestion {
                    text: format!("Interesting combo — you're exploring both {} and {}. Want me to find connections between them?", domains[0], domains[1]),
                    category: SuggestionCategory::RelatedTopic,
                    relevance: 0.6,
                    action: Some(format!("cross_domain:{}:{}", domains[0], domains[1])),
                });
            }
        }

        // 4. STARTERS: When no messages yet
        if self.message_count == 0 {
            let starters = [
                ("What's the latest in cybersecurity threats?", "cybersecurity"),
                ("Explain quantum computing in simple terms", "physics"),
                ("Help me design a Rust web API", "programming"),
                ("What's happening in AI research?", "technology"),
                ("Teach me about network protocols", "networking"),
                ("Analyze the current economic trends", "economics"),
            ];
            for (text, domain) in &starters {
                suggestions.push(Suggestion {
                    text: text.to_string(),
                    category: SuggestionCategory::Starter,
                    relevance: 0.5,
                    action: Some(format!("starter:{}", domain)),
                });
            }
        }

        // Sort by relevance
        suggestions.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap_or(std::cmp::Ordering::Equal));
        suggestions.truncate(5);
        suggestions
    }

    /// Get conversation statistics.
    pub fn stats(&self) -> SuggestionStats {
        SuggestionStats {
            messages_observed: self.message_count,
            unique_topics: self.topic_counts.len(),
            domains_explored: self.domain_affinity.len(),
            top_topics: self.topic_counts.iter()
                .map(|(k, v)| (k.clone(), *v))
                .collect::<Vec<_>>()
                .into_iter()
                .take(10)
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct SuggestionStats {
    pub messages_observed: usize,
    pub unique_topics: usize,
    pub domains_explored: usize,
    pub top_topics: Vec<(String, usize)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starter_suggestions() {
        let engine = SuggestionEngine::new();
        let suggestions = engine.suggest();
        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().all(|s| s.category == SuggestionCategory::Starter));
    }

    #[test]
    fn test_deep_dive_detection() {
        let mut engine = SuggestionEngine::new();
        for _ in 0..5 {
            engine.observe("Tell me more about quantum entanglement and quantum computing", Some("physics"));
        }
        let suggestions = engine.suggest();
        assert!(suggestions.iter().any(|s| s.category == SuggestionCategory::DeepDive));
    }

    #[test]
    fn test_learning_path() {
        let mut engine = SuggestionEngine::new();
        for _ in 0..6 {
            engine.observe("How does Rust handle memory?", Some("programming"));
        }
        let suggestions = engine.suggest();
        assert!(suggestions.iter().any(|s| s.category == SuggestionCategory::LearningPath));
    }

    #[test]
    fn test_cross_domain() {
        let mut engine = SuggestionEngine::new();
        engine.observe("Explain encryption", Some("cybersecurity"));
        engine.observe("Explain prime numbers", Some("mathematics"));
        let suggestions = engine.suggest();
        assert!(suggestions.iter().any(|s| s.category == SuggestionCategory::RelatedTopic));
    }

    #[test]
    fn test_stats() {
        let mut engine = SuggestionEngine::new();
        engine.observe("Test message about quantum physics", Some("physics"));
        let stats = engine.stats();
        assert_eq!(stats.messages_observed, 1);
        assert!(stats.unique_topics > 0);
    }
}
