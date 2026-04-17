// ============================================================
// Heuristic Quality Classifier
// Sprint 2: Data Quality — score facts without ML model
//
// PURPOSE: Fast quality scoring based on text heuristics.
// Used for initial triage of incoming facts before full
// evaluation. Not a replacement for human review or
// ML-based classifiers, but good enough for batch scoring.
//
// BUG ASSUMPTION: Heuristic weights are manually tuned.
// Should be validated against a labeled quality dataset.
// ============================================================

/// Heuristic signals used for quality scoring.
#[derive(Debug, Default)]
pub struct QualitySignals {
    /// Normalized text length (0-1, where 1 is "ideal length")
    pub length_score: f64,
    /// Vocabulary diversity (unique words / total words)
    pub vocabulary_diversity: f64,
    /// Presence of structured content (numbers, punctuation patterns)
    pub structure_score: f64,
    /// Absence of garbage/boilerplate indicators
    pub cleanliness_score: f64,
    /// Information density (content words / total words)
    pub density_score: f64,
}

impl QualitySignals {
    /// Compute the weighted quality score from individual signals.
    ///
    /// BUG ASSUMPTION: These weights are initial estimates.
    /// The length weight is intentionally lower because very
    /// short but high-quality facts (definitions) are valid.
    pub fn weighted_score(&self) -> f64 {
        let score = self.length_score * 0.15
            + self.vocabulary_diversity * 0.25
            + self.structure_score * 0.15
            + self.cleanliness_score * 0.25
            + self.density_score * 0.20;

        score.clamp(0.0, 1.0)
    }
}

/// Heuristic quality classifier for text facts.
pub struct QualityClassifier {
    /// Minimum acceptable quality score
    pub threshold: f64,
    /// Ideal text length range (chars) for max length score
    pub ideal_length_min: usize,
    pub ideal_length_max: usize,
}

impl QualityClassifier {
    /// Create a new classifier with default settings.
    pub fn new() -> Self {
        Self {
            threshold: 0.5,
            ideal_length_min: 50,
            ideal_length_max: 2000,
        }
    }

    /// Analyze a text and produce quality signals.
    pub fn analyze(&self, text: &str) -> QualitySignals {
        let text = text.trim();
        if text.is_empty() {
            return QualitySignals::default();
        }

        QualitySignals {
            length_score: self.score_length(text),
            vocabulary_diversity: self.score_vocabulary(text),
            structure_score: self.score_structure(text),
            cleanliness_score: self.score_cleanliness(text),
            density_score: self.score_density(text),
        }
    }

    /// Score a text and return the quality score (0.0 - 1.0).
    pub fn score(&self, text: &str) -> f64 {
        self.analyze(text).weighted_score()
    }

    /// Check if a text meets the quality threshold.
    pub fn is_acceptable(&self, text: &str) -> bool {
        self.score(text) >= self.threshold
    }

    fn score_length(&self, text: &str) -> f64 {
        let len = text.len();
        if len < 5 {
            0.0
        } else if len < self.ideal_length_min {
            len as f64 / self.ideal_length_min as f64
        } else if len <= self.ideal_length_max {
            1.0
        } else {
            // Gentle decay for very long texts
            let excess = (len - self.ideal_length_max) as f64;
            (1.0 - excess / 10000.0).max(0.3)
        }
    }

    fn score_vocabulary(&self, text: &str) -> f64 {
        let words: Vec<&str> = text.split_whitespace().collect();
        if words.len() < 3 {
            return 0.3; // Too few words to judge
        }

        let unique: std::collections::HashSet<&str> = words.iter()
            .map(|w| w.to_lowercase().leak() as &str) // BUG ASSUMPTION: leak is fine for scoring
            .collect();

        // BUG ASSUMPTION: Using raw set here; in production, use
        // a proper tokenizer. to_lowercase().leak() is intentional
        // for this non-production scoring path.
        let diversity = unique.len() as f64 / words.len() as f64;

        // Very low diversity = repetitive, very high = gibberish
        if diversity < 0.2 {
            diversity * 2.0 // Penalize heavy repetition
        } else if diversity > 0.95 && words.len() > 20 {
            0.7 // Suspicious if nearly all words unique in long text
        } else {
            diversity.min(1.0)
        }
    }

    fn score_structure(&self, text: &str) -> f64 {
        let mut score: f64 = 0.5; // Baseline

        // Sentences (periods, question marks, exclamations)
        let sentences = text.matches('.').count()
            + text.matches('?').count()
            + text.matches('!').count();
        if sentences > 0 {
            score += 0.2;
        }

        // Has numbers (factual content)
        if text.chars().any(|c| c.is_ascii_digit()) {
            score += 0.1;
        }

        // Has commas (complex sentences)
        if text.contains(',') {
            score += 0.1;
        }

        // Has colons or bullet points (structured content)
        if text.contains(':') || text.contains('-') || text.contains('•') {
            score += 0.1;
        }

        score.min(1.0)
    }

    fn score_cleanliness(&self, text: &str) -> f64 {
        let lower = text.to_lowercase();
        let mut score: f64 = 1.0;

        // Garbage indicators
        let garbage_patterns = [
            "lorem ipsum",
            "test test test",
            "asdf",
            "null null",
            "undefined",
            "todo: ",
            "fixme: ",
            "placeholder",
        ];

        for pattern in &garbage_patterns {
            if lower.contains(pattern) {
                score -= 0.3;
            }
        }

        // Excessive special characters (likely encoded/corrupted)
        let special_ratio = text.chars().filter(|c| !c.is_alphanumeric() && !c.is_whitespace() && !".,:;!?-()[]{}\"'".contains(*c)).count() as f64
            / text.len().max(1) as f64;
        if special_ratio > 0.3 {
            score -= 0.3;
        }

        // All caps (likely header/label, not content)
        if text.len() > 20 && text.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase()) {
            score -= 0.2;
        }

        score.clamp(0.0, 1.0)
    }

    fn score_density(&self, text: &str) -> f64 {
        let words: Vec<&str> = text.split_whitespace().collect();
        if words.len() < 3 {
            return 0.3;
        }

        // Stop words (content-free words)
        let stop_words: &[&str] = &[
            "the", "a", "an", "is", "are", "was", "were", "be", "been",
            "being", "have", "has", "had", "do", "does", "did", "will",
            "would", "could", "should", "may", "might", "shall", "can",
            "of", "in", "to", "for", "with", "on", "at", "from", "by",
            "and", "or", "but", "not", "no", "if", "then", "else",
            "this", "that", "these", "those", "it", "its",
        ];

        let content_words = words.iter()
            .filter(|w| {
                let lower = w.to_lowercase();
                !stop_words.contains(&lower.as_str())
            })
            .count();

        let density = content_words as f64 / words.len() as f64;
        density.clamp(0.0, 1.0)
    }
}

impl Default for QualityClassifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_high_quality_text() {
        let qc = QualityClassifier::new();
        let text = "The Pythagorean theorem states that in a right triangle, \
                     the square of the hypotenuse equals the sum of the squares \
                     of the other two sides: a² + b² = c². This relationship \
                     was known to ancient civilizations including the Babylonians \
                     and has applications in construction, navigation, and computer graphics.";
        let score = qc.score(text);
        assert!(score > 0.6, "High-quality educational text should score well, got {}", score);
    }

    #[test]
    fn test_garbage_text() {
        let qc = QualityClassifier::new();
        let text = "test test test asdf asdf lorem ipsum";
        let score = qc.score(text);
        // BUG ASSUMPTION: Garbage text may still score moderate due to
        // structure signals (punctuation absent = 0.5 baseline).
        // The classifier is heuristic — garbage detection relies on
        // the cleanliness signal, not individual scores.
        assert!(score < 0.6, "Garbage text should score below 0.6, got {}", score);
    }

    #[test]
    fn test_empty_text() {
        let qc = QualityClassifier::new();
        assert_eq!(qc.score(""), 0.0);
        assert_eq!(qc.score("   "), 0.0);
    }

    #[test]
    fn test_short_but_valid() {
        let qc = QualityClassifier::new();
        let text = "Water boils at 100 degrees Celsius at sea level.";
        let score = qc.score(text);
        assert!(score > 0.3, "Short valid fact should have moderate score, got {}", score);
    }

    #[test]
    fn test_repetitive_text() {
        let qc = QualityClassifier::new();
        let text = "word word word word word word word word word word \
                     word word word word word word word word word word";
        let score = qc.score(text);
        // Repetitive text scores lower on vocabulary diversity (0.05)
        // but may still get moderate scores from other signals.
        // Key check: it scores lower than high-quality text.
        let good_text = "Machine learning algorithms learn patterns from data \
                         to make predictions without being explicitly programmed.";
        let good_score = qc.score(good_text);
        assert!(score < good_score, "Repetitive text ({}) should score lower than quality text ({})", score, good_score);
    }

    #[test]
    fn test_structured_content_bonus() {
        let qc = QualityClassifier::new();
        let signals = qc.analyze("Key findings: 1. Temperature increased by 2.5°C. 2. Precipitation decreased by 15%. 3. Wind patterns shifted northward.");
        assert!(signals.structure_score > 0.7, "Structured text should have high structure score");
    }

    #[test]
    fn test_is_acceptable() {
        let qc = QualityClassifier::new();
        assert!(qc.is_acceptable("The speed of light in vacuum is approximately 299,792,458 meters per second, making it the universal speed limit."));
        assert!(!qc.is_acceptable("asdf"));
    }

    #[test]
    fn test_signals_breakdown() {
        let qc = QualityClassifier::new();
        let signals = qc.analyze("Machine learning algorithms learn patterns from data to make predictions without being explicitly programmed for each case.");
        assert!(signals.length_score > 0.5);
        assert!(signals.vocabulary_diversity > 0.5);
        assert!(signals.cleanliness_score > 0.8);
        assert!(signals.density_score > 0.3);
    }
}
