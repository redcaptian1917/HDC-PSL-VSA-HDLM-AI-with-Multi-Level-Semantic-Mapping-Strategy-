// ============================================================
// Reasoning Chain Verifier — Validate step-by-step logic
//
// When the AI produces multi-step reasoning, verify each step
// is consistent with previous steps. If step 3 contradicts
// step 1, flag and suggest regeneration.
// ============================================================

/// Result of verifying a reasoning chain.
#[derive(Debug, Clone)]
pub struct ChainVerification {
    pub steps: Vec<StepCheck>,
    pub is_consistent: bool,
    pub inconsistencies: Vec<String>,
    pub overall_confidence: f64,
}

/// Verification result for a single step.
#[derive(Debug, Clone)]
pub struct StepCheck {
    pub step_number: usize,
    pub text: String,
    pub consistent_with_previous: bool,
    pub confidence: f64,
}

/// Verify a chain of reasoning steps for internal consistency.
pub fn verify_chain(steps: &[String]) -> ChainVerification {
    if steps.is_empty() {
        return ChainVerification {
            steps: vec![],
            is_consistent: true,
            inconsistencies: vec![],
            overall_confidence: 1.0,
        };
    }

    let mut checks = Vec::new();
    let mut inconsistencies = Vec::new();
    let mut total_conf = 0.0f64;

    // First step is always consistent (baseline)
    checks.push(StepCheck {
        step_number: 1,
        text: steps[0].clone(),
        consistent_with_previous: true,
        confidence: 0.9,
    });
    total_conf += 0.9;

    for i in 1..steps.len() {
        let current = &steps[i].to_lowercase();
        let mut consistent = true;
        let mut conf = 0.8f64;

        // Check for contradictions with ALL previous steps
        for j in 0..i {
            let prev = &steps[j].to_lowercase();

            // Negation patterns
            let negation_pairs = [
                ("is", "is not"), ("can", "cannot"), ("will", "will not"),
                ("does", "does not"), ("has", "has no"), ("true", "false"),
                ("increases", "decreases"), ("more", "less"),
                ("always", "never"), ("all", "none"),
            ];

            for (pos, neg) in &negation_pairs {
                // If previous step claims X and current claims not-X
                if prev.contains(pos) && current.contains(neg) {
                    // Check if they're about the same subject (share keywords)
                    let prev_words: std::collections::HashSet<&str> = prev.split_whitespace()
                        .filter(|w| w.len() >= 4).collect();
                    let curr_words: std::collections::HashSet<&str> = current.split_whitespace()
                        .filter(|w| w.len() >= 4).collect();
                    let overlap = prev_words.intersection(&curr_words).count();

                    if overlap >= 2 {
                        consistent = false;
                        conf = 0.3;
                        inconsistencies.push(format!(
                            "Step {} may contradict step {}: '{}' vs '{}'",
                            i + 1, j + 1, pos, neg
                        ));
                    }
                }
            }

            // Check for numeric contradictions
            let prev_nums: Vec<f64> = extract_numbers(prev);
            let curr_nums: Vec<f64> = extract_numbers(current);
            if !prev_nums.is_empty() && !curr_nums.is_empty() {
                // If same quantity mentioned with very different values
                for pn in &prev_nums {
                    for cn in &curr_nums {
                        if (*pn - *cn).abs() > (*pn).abs().max(cn.abs()) * 0.5 && *pn != 0.0 {
                            // Only flag if the context overlaps
                            let prev_words: std::collections::HashSet<&str> = prev.split_whitespace()
                                .filter(|w| w.len() >= 4).collect();
                            let curr_words: std::collections::HashSet<&str> = current.split_whitespace()
                                .filter(|w| w.len() >= 4).collect();
                            if prev_words.intersection(&curr_words).count() >= 2 {
                                conf = conf.min(0.5);
                                inconsistencies.push(format!(
                                    "Possible numeric inconsistency: {} vs {} in steps {}/{}", pn, cn, j+1, i+1
                                ));
                            }
                        }
                    }
                }
            }
        }

        checks.push(StepCheck {
            step_number: i + 1,
            text: steps[i].clone(),
            consistent_with_previous: consistent,
            confidence: conf,
        });
        total_conf += conf;
    }

    let overall = total_conf / steps.len() as f64;

    ChainVerification {
        steps: checks,
        is_consistent: inconsistencies.is_empty(),
        inconsistencies,
        overall_confidence: overall,
    }
}

/// Extract numbers from text.
fn extract_numbers(text: &str) -> Vec<f64> {
    text.split(|c: char| !c.is_numeric() && c != '.' && c != '-')
        .filter(|s| !s.is_empty() && s.len() < 15)
        .filter_map(|s| s.parse::<f64>().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consistent_chain() {
        let steps = vec![
            "TCP provides reliable data transmission".to_string(),
            "It uses acknowledgments to confirm delivery".to_string(),
            "Lost packets are retransmitted automatically".to_string(),
        ];
        let result = verify_chain(&steps);
        assert!(result.is_consistent);
        assert!(result.overall_confidence > 0.7);
    }

    #[test]
    fn test_contradictory_chain() {
        let steps = vec![
            "The algorithm always produces correct results".to_string(),
            "Due to rounding errors, the algorithm never produces correct results".to_string(),
        ];
        let result = verify_chain(&steps);
        assert!(!result.is_consistent);
        assert!(!result.inconsistencies.is_empty());
    }

    #[test]
    fn test_empty_chain() {
        let result = verify_chain(&[]);
        assert!(result.is_consistent);
        assert_eq!(result.overall_confidence, 1.0);
    }

    #[test]
    fn test_extract_numbers() {
        let nums = extract_numbers("The speed is 300000 km/s or about 3.0e8");
        assert!(nums.contains(&300000.0));
        assert!(nums.contains(&3.0));
    }
}
