// ============================================================
// Active Learning Loop — Model picks what to learn next
//
// After each training batch or interaction period, identifies
// weakest domains and prioritizes them for the next batch.
// Closed-loop self-improvement cycle.
//
// Pipeline:
// 1. Analyze: score each domain by accuracy + coverage + freshness
// 2. Rank: sort domains by learning priority (worst first)
// 3. Generate: create Magpie prompts for top-priority domains
// 4. Train: ingest new pairs, update scores
// 5. Repeat
// ============================================================

use std::collections::HashMap;
use std::sync::Arc;
use crate::persistence::BrainDb;

/// Priority score for a domain that needs more learning.
#[derive(Debug, Clone)]
pub struct LearningPriority {
    pub domain: String,
    pub priority_score: f64,
    pub fact_count: i64,
    pub avg_quality: f64,
    pub reason: String,
}

/// The active learning engine.
pub struct ActiveLearner {
    db: Arc<BrainDb>,
}

impl ActiveLearner {
    pub fn new(db: Arc<BrainDb>) -> Self {
        Self { db }
    }

    /// Analyze all domains and rank by learning priority.
    /// Higher priority = needs more attention.
    pub fn prioritize(&self) -> Vec<LearningPriority> {
        let conn = self.db.conn.lock().unwrap_or_else(|e| e.into_inner());

        let mut stmt = match conn.prepare(
            "SELECT domain, COUNT(*) as cnt, AVG(COALESCE(quality_score, 0.5)) as avg_q \
             FROM facts WHERE domain IS NOT NULL GROUP BY domain"
        ) {
            Ok(s) => s,
            Err(_) => return vec![],
        };

        let domains: Vec<(String, i64, f64)> = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, f64>(2)?,
            ))
        }).unwrap_or_else(|_| panic!("query_map")).filter_map(|r| r.ok()).collect();

        drop(stmt);
        drop(conn);

        // Also check training accuracy per domain
        let conn2 = self.db.conn.lock().unwrap_or_else(|e| e.into_inner());
        let training_accuracy: HashMap<String, f64> = conn2.prepare(
            "SELECT domain, AVG(accuracy) FROM training_results GROUP BY domain"
        ).ok().map(|mut s| {
            s.query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
            }).unwrap_or_else(|_| panic!("query_map")).filter_map(|r| r.ok()).collect()
        }).unwrap_or_default();
        drop(conn2);

        let max_count = domains.iter().map(|(_, c, _)| *c).max().unwrap_or(1);

        let mut priorities: Vec<LearningPriority> = domains.iter().map(|(domain, count, avg_q)| {
            let mut priority = 0.0f64;
            let mut reasons = Vec::new();

            // Factor 1: Low coverage (fewer facts = higher priority)
            let coverage_score = 1.0 - (*count as f64 / max_count as f64).min(1.0);
            if *count < 1000 {
                priority += 0.4;
                reasons.push(format!("only {} facts", count));
            } else if *count < 10000 {
                priority += 0.2 * coverage_score;
                reasons.push("moderate coverage".into());
            }

            // Factor 2: Low quality (lower quality = higher priority)
            if *avg_q < 0.6 {
                priority += 0.3;
                reasons.push(format!("low quality ({:.2})", avg_q));
            } else if *avg_q < 0.75 {
                priority += 0.15;
                reasons.push("mediocre quality".into());
            }

            // Factor 3: Low training accuracy
            if let Some(acc) = training_accuracy.get(domain) {
                if *acc < 0.5 {
                    priority += 0.3;
                    reasons.push(format!("poor accuracy ({:.0}%)", acc * 100.0));
                } else if *acc < 0.7 {
                    priority += 0.15;
                }
            } else {
                // Never trained on this domain
                priority += 0.1;
                reasons.push("never evaluated".into());
            }

            let reason = if reasons.is_empty() {
                "well covered".into()
            } else {
                reasons.join(", ")
            };

            LearningPriority {
                domain: domain.clone(),
                priority_score: priority.min(1.0),
                fact_count: *count,
                avg_quality: *avg_q,
                reason,
            }
        }).collect();

        priorities.sort_by(|a, b| b.priority_score.partial_cmp(&a.priority_score).unwrap_or(std::cmp::Ordering::Equal));
        priorities
    }

    /// Get top N domains that need the most learning.
    pub fn top_priorities(&self, n: usize) -> Vec<LearningPriority> {
        self.prioritize().into_iter().take(n).collect()
    }

    /// Generate Magpie prompts for priority domains.
    pub fn generate_learning_plan(&self, max_prompts: usize) -> Vec<(String, String)> {
        let priorities = self.top_priorities(5);
        let mut prompts = Vec::new();

        for p in &priorities {
            if prompts.len() >= max_prompts { break; }
            let per_domain = (max_prompts / priorities.len().max(1)).max(1);
            for i in 0..per_domain {
                if prompts.len() >= max_prompts { break; }
                let difficulty = match i {
                    0 => "basic",
                    1 => "intermediate",
                    _ => "advanced",
                };
                prompts.push((
                    p.domain.clone(),
                    format!("Ask a {} question about {} and provide a thorough answer.\nQ:", difficulty, p.domain),
                ));
            }
        }

        prompts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_learner() -> ActiveLearner {
        let id = std::process::id();
        let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let path = PathBuf::from(format!("/tmp/plausiden_test_al_{}_{}.db", id, ts));
        let db = Arc::new(BrainDb::open(&path).unwrap());
        // Seed domains
        let conn = db.conn.lock().unwrap();
        conn.execute("ALTER TABLE facts ADD COLUMN domain TEXT", []).ok();
        conn.execute("ALTER TABLE facts ADD COLUMN quality_score REAL", []).ok();
        for i in 0..100 {
            conn.execute("INSERT INTO facts (key,value,source,confidence,domain,quality_score) VALUES (?,?,'test',0.8,'cybersecurity',0.85)",
                rusqlite::params![format!("c{}", i), format!("Cyber fact {}", i)]).ok();
        }
        for i in 0..5 {
            conn.execute("INSERT INTO facts (key,value,source,confidence,domain,quality_score) VALUES (?,?,'test',0.4,'philosophy',0.4)",
                rusqlite::params![format!("p{}", i), format!("Phil fact {}", i)]).ok();
        }
        drop(conn);
        ActiveLearner::new(db)
    }

    #[test]
    fn test_prioritize() {
        let learner = test_learner();
        let priorities = learner.prioritize();
        assert!(priorities.len() >= 2);
        // Philosophy should be higher priority (fewer facts, lower quality)
        let phil = priorities.iter().find(|p| p.domain == "philosophy");
        let cyber = priorities.iter().find(|p| p.domain == "cybersecurity");
        assert!(phil.is_some());
        assert!(cyber.is_some());
        assert!(phil.unwrap().priority_score > cyber.unwrap().priority_score);
    }

    #[test]
    fn test_learning_plan() {
        let learner = test_learner();
        let plan = learner.generate_learning_plan(10);
        assert!(!plan.is_empty());
        assert!(plan.len() <= 10);
    }

    #[test]
    fn test_top_priorities() {
        let learner = test_learner();
        let top = learner.top_priorities(3);
        assert!(top.len() <= 3);
        // Should be sorted by priority descending
        for i in 1..top.len() {
            assert!(top[i-1].priority_score >= top[i].priority_score);
        }
    }
}
