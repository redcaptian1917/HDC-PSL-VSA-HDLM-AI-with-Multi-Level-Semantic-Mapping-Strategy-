// ============================================================
// Constitutional AI Self-Critique
//
// After generating a response, the AI critiques itself against
// a set of constitutional principles. If the response violates
// any principle, it revises before sending.
//
// Principles:
// 1. Accuracy: Claims should be factually correct
// 2. Helpfulness: Response should address the actual question
// 3. Safety: No harmful, dangerous, or illegal advice
// 4. Honesty: Acknowledge uncertainty, don't fabricate
// 5. Respect: Treat the user's question seriously
// 6. Completeness: Don't leave important aspects unaddressed
// ============================================================

/// Constitutional principles the AI holds itself to.
#[derive(Debug, Clone)]
pub struct Principle {
    pub name: String,
    pub description: String,
    pub weight: f64,
}

/// Result of self-critique on a response.
#[derive(Debug, Clone)]
pub struct CritiqueResult {
    pub passes: Vec<(String, f64)>,
    pub violations: Vec<CritiqueViolation>,
    pub overall_score: f64,
    pub needs_revision: bool,
    pub revision_suggestion: Option<String>,
}

/// A specific principle violation found in self-critique.
#[derive(Debug, Clone)]
pub struct CritiqueViolation {
    pub principle: String,
    pub severity: f64,
    pub detail: String,
}

/// The self-critique engine with constitutional principles.
pub struct SelfCritique {
    principles: Vec<Principle>,
}

impl SelfCritique {
    pub fn new() -> Self {
        Self {
            principles: vec![
                Principle { name: "accuracy".into(), description: "Claims should be factually correct".into(), weight: 0.25 },
                Principle { name: "helpfulness".into(), description: "Response addresses the actual question".into(), weight: 0.20 },
                Principle { name: "safety".into(), description: "No harmful or dangerous advice".into(), weight: 0.20 },
                Principle { name: "honesty".into(), description: "Acknowledges uncertainty, doesn't fabricate".into(), weight: 0.15 },
                Principle { name: "respect".into(), description: "Treats the question seriously".into(), weight: 0.10 },
                Principle { name: "completeness".into(), description: "Covers important aspects".into(), weight: 0.10 },
            ],
        }
    }

    /// Critique a response against constitutional principles.
    pub fn critique(&self, question: &str, response: &str) -> CritiqueResult {
        let mut passes = Vec::new();
        let mut violations = Vec::new();

        for principle in &self.principles {
            let (score, detail) = self.check_principle(&principle.name, question, response);
            if score >= 0.7 {
                passes.push((principle.name.clone(), score));
            } else {
                violations.push(CritiqueViolation {
                    principle: principle.name.clone(),
                    severity: 1.0 - score,
                    detail,
                });
            }
        }

        let overall = self.principles.iter().map(|p| {
            let score = passes.iter()
                .find(|(name, _)| name == &p.name)
                .map(|(_, s)| *s)
                .unwrap_or_else(|| {
                    violations.iter()
                        .find(|v| v.principle == p.name)
                        .map(|v| 1.0 - v.severity)
                        .unwrap_or(0.5)
                });
            score * p.weight
        }).sum::<f64>();

        let needs_revision = overall < 0.6 || violations.iter().any(|v| v.severity > 0.7);

        let revision_suggestion = if needs_revision {
            let worst = violations.iter().max_by(|a, b|
                a.severity.partial_cmp(&b.severity).unwrap_or(std::cmp::Ordering::Equal));
            worst.map(|v| format!("Revise for {}: {}", v.principle, v.detail))
        } else {
            None
        };

        CritiqueResult {
            passes,
            violations,
            overall_score: overall,
            needs_revision,
            revision_suggestion,
        }
    }

    /// Check a single principle against the response.
    fn check_principle(&self, principle: &str, question: &str, response: &str) -> (f64, String) {
        let lower_r = response.to_lowercase();
        let lower_q = question.to_lowercase();

        match principle {
            "accuracy" => {
                // Check for known hallucination markers
                let markers = ["as of my training", "as of my last update",
                    "I believe it was in", "if I recall correctly"];
                let has_hedging = markers.iter().any(|m| lower_r.contains(m));
                if has_hedging {
                    return (0.5, "Contains uncertain hedging language".into());
                }
                (0.8, "No obvious accuracy issues".into())
            }
            "helpfulness" => {
                // Check if response addresses the question
                if response.len() < 20 {
                    return (0.3, "Response too short to be helpful".into());
                }
                // Check keyword overlap between question and response
                let q_words: std::collections::HashSet<&str> = lower_q.split_whitespace()
                    .filter(|w| w.len() >= 4).collect();
                let r_words: std::collections::HashSet<&str> = lower_r.split_whitespace()
                    .filter(|w| w.len() >= 4).collect();
                let overlap = q_words.intersection(&r_words).count();
                let relevance = if q_words.is_empty() { 0.7 } else {
                    (overlap as f64 / q_words.len() as f64).min(1.0)
                };
                if relevance < 0.2 {
                    return (0.4, "Response doesn't seem to address the question".into());
                }
                (0.7 + relevance * 0.3, "Response is relevant".into())
            }
            "safety" => {
                let unsafe_patterns = ["here's how to hack", "to attack",
                    "steal the password", "bypass security", "inject malware"];
                for p in &unsafe_patterns {
                    if lower_r.contains(p) && !lower_q.contains("defend") && !lower_q.contains("protect") {
                        return (0.1, format!("Contains potentially unsafe advice: '{}'", p));
                    }
                }
                (0.9, "No safety concerns".into())
            }
            "honesty" => {
                let overconfident = ["I am 100% certain", "there is absolutely no",
                    "it is impossible that", "this will definitely"];
                for p in &overconfident {
                    if lower_r.contains(p) {
                        return (0.5, format!("Overconfident claim: '{}'", p));
                    }
                }
                // Good: acknowledges limits
                let honest_markers = ["I'm not sure", "I think", "based on my knowledge",
                    "it depends", "there are different perspectives"];
                let has_honesty = honest_markers.iter().any(|m| lower_r.contains(m));
                if has_honesty { (0.9, "Honest hedging present".into()) }
                else { (0.7, "No explicit uncertainty markers".into()) }
            }
            "respect" => {
                let disrespectful = ["that's a stupid question", "obviously",
                    "you should know this", "as I already said"];
                for p in &disrespectful {
                    if lower_r.contains(p) {
                        return (0.2, format!("Disrespectful language: '{}'", p));
                    }
                }
                (0.9, "Respectful tone".into())
            }
            "completeness" => {
                if response.len() < 50 && lower_q.contains("explain") {
                    return (0.4, "Explanation request got a very short response".into());
                }
                if lower_q.contains("?") && !lower_r.contains(".") {
                    return (0.5, "Question got a response without complete sentences".into());
                }
                (0.8, "Response seems complete".into())
            }
            _ => (0.7, "Unknown principle".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn critic() -> SelfCritique { SelfCritique::new() }

    #[test]
    fn test_good_response() {
        let result = critic().critique(
            "What is Rust?",
            "Rust is a systems programming language focused on safety, speed, and concurrency. It achieves memory safety without garbage collection through its ownership system. Rust is used for operating systems, web browsers, and game engines."
        );
        assert!(result.overall_score >= 0.6);
        assert!(!result.needs_revision);
    }

    #[test]
    fn test_too_short() {
        let result = critic().critique("Explain quantum computing", "It's complex.");
        assert!(result.violations.iter().any(|v| v.principle == "helpfulness" || v.principle == "completeness"));
    }

    #[test]
    fn test_unsafe_response() {
        let result = critic().critique(
            "How do I access my neighbor's WiFi?",
            "Here's how to hack into their network: first, use aircrack to inject malware..."
        );
        assert!(result.violations.iter().any(|v| v.principle == "safety"));
        assert!(result.needs_revision);
    }

    #[test]
    fn test_overconfident() {
        let result = critic().critique(
            "Will it rain tomorrow?",
            "I am 100% certain it will rain tomorrow. There is absolutely no chance of sun."
        );
        assert!(result.violations.iter().any(|v| v.principle == "honesty"));
    }

    #[test]
    fn test_principles_count() {
        let c = critic();
        assert_eq!(c.principles.len(), 6);
        let total_weight: f64 = c.principles.iter().map(|p| p.weight).sum();
        assert!((total_weight - 1.0).abs() < 0.01);
    }
}
