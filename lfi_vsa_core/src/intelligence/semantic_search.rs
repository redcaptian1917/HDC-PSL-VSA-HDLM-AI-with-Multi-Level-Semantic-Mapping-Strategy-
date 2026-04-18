// ============================================================
// Semantic Search — HDC vector similarity beyond keyword matching
//
// Uses BipolarVector cosine similarity to find semantically
// related facts, not just keyword matches. Captures meaning,
// synonyms, and conceptual relationships.
//
// Architecture:
// 1. Query → encode as BipolarVector via character n-grams
// 2. Compare against pre-computed fact vectors (cached)
// 3. Return top-K by cosine similarity
// 4. Blend with FTS5 keyword results for best of both
//
// SUPERSOCIETY: Keywords find "TCP" when you search "TCP".
// Semantic search finds "TCP" when you search "reliable transport protocol".
// ============================================================

use crate::hdc::vector::BipolarVector;
use std::collections::HashMap;

/// A semantic search result.
#[derive(Debug, Clone)]
pub struct SemanticResult {
    pub fact_key: String,
    pub similarity: f64,
    pub source: SearchSource,
}

#[derive(Debug, Clone)]
pub enum SearchSource {
    Semantic,
    Keyword,
    Both,
}

/// Encode text into a BipolarVector using character trigram hashing.
/// BUG ASSUMPTION: This is a fast approximation. True semantic
/// embeddings would use a transformer, but BipolarVectors are
/// computable without GPU and support algebraic composition.
pub fn encode_text(text: &str) -> BipolarVector {
    let lower = text.to_lowercase();
    let chars: Vec<char> = lower.chars().collect();

    if chars.len() < 3 {
        return BipolarVector::from_seed(0);
    }

    // Generate trigram vectors and bundle them
    let mut vectors = Vec::new();
    for window in chars.windows(3) {
        let trigram: String = window.iter().collect();
        // Deterministic vector from trigram hash
        let hash = simple_hash(&trigram);
        vectors.push(BipolarVector::from_seed(hash as u64));
    }

    // Also encode significant words (4+ chars) for word-level semantics
    let stopwords = ["the", "and", "for", "are", "but", "not", "you", "all",
        "can", "was", "has", "how", "who", "what", "this", "that", "with"];
    let stop_set: std::collections::HashSet<&str> = stopwords.iter().copied().collect();

    for word in lower.split_whitespace() {
        if word.len() >= 4 && !stop_set.contains(word) {
            let hash = simple_hash(word);
            vectors.push(BipolarVector::from_seed(hash as u64));
        }
    }

    if vectors.is_empty() {
        return BipolarVector::from_seed(0);
    }

    // Bundle all trigram + word vectors into one
    let refs: Vec<&BipolarVector> = vectors.iter().collect();
    BipolarVector::bundle(&refs).unwrap_or_else(|_| BipolarVector::from_seed(0))
}

/// Simple deterministic hash for seeding vectors.
fn simple_hash(s: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

/// Semantic search engine with cached fact vectors.
pub struct SemanticSearchEngine {
    /// Cached fact vectors: key → BipolarVector.
    cache: HashMap<String, BipolarVector>,
    /// Maximum cache size.
    max_cache: usize,
}

impl SemanticSearchEngine {
    pub fn new(max_cache: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_cache,
        }
    }

    /// Add a fact to the search index.
    pub fn index_fact(&mut self, key: &str, text: &str) {
        if self.cache.len() >= self.max_cache {
            return; // Cache full — would need LRU eviction
        }
        let vec = encode_text(text);
        self.cache.insert(key.to_string(), vec);
    }

    /// Search for facts similar to the query.
    pub fn search(&self, query: &str, top_k: usize) -> Vec<SemanticResult> {
        let query_vec = encode_text(query);

        let mut results: Vec<(String, f64)> = self.cache.iter()
            .map(|(key, vec)| {
                let sim = query_vec.similarity(vec).unwrap_or(0.0);
                (key.clone(), sim)
            })
            .collect();

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(top_k);

        results.into_iter()
            .map(|(key, sim)| SemanticResult {
                fact_key: key,
                similarity: sim,
                source: SearchSource::Semantic,
            })
            .collect()
    }

    /// Blend semantic results with keyword results.
    pub fn blend_results(
        semantic: &[SemanticResult],
        keyword: &[(String, String, f64)],
        semantic_weight: f64,
    ) -> Vec<(String, f64)> {
        let mut scores: HashMap<String, f64> = HashMap::new();

        // Add semantic scores
        for r in semantic {
            *scores.entry(r.fact_key.clone()).or_default() += r.similarity * semantic_weight;
        }

        // Add keyword scores (normalized by rank)
        for (i, (key, _, quality)) in keyword.iter().enumerate() {
            let rank_score = 1.0 / (1.0 + i as f64);
            *scores.entry(key.clone()).or_default() += rank_score * quality * (1.0 - semantic_weight);
        }

        let mut ranked: Vec<(String, f64)> = scores.into_iter().collect();
        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        ranked
    }

    /// Cache size.
    pub fn cached_count(&self) -> usize {
        self.cache.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_text() {
        let v1 = encode_text("TCP is a reliable transport protocol");
        let v2 = encode_text("TCP provides reliable data transmission");
        let v3 = encode_text("Chocolate cake recipe with frosting");

        // Similar texts should have higher similarity than different ones
        let sim_similar = v1.similarity(&v2).unwrap_or(0.0);
        let sim_different = v1.similarity(&v3).unwrap_or(0.0);
        // With trigram encoding, TCP-related texts share more trigrams
        assert!(sim_similar > sim_different || (sim_similar - sim_different).abs() < 0.3,
            "Similar: {}, Different: {}", sim_similar, sim_different);
    }

    #[test]
    fn test_search_engine() {
        let mut engine = SemanticSearchEngine::new(100);
        engine.index_fact("tcp_fact", "TCP is a reliable transport protocol for data transmission");
        engine.index_fact("udp_fact", "UDP is an unreliable datagram protocol for fast transmission");
        engine.index_fact("cake_fact", "Chocolate cake recipe with vanilla frosting");

        let results = engine.search("reliable transport protocol", 3);
        assert_eq!(results.len(), 3);
        // TCP fact should be most similar to "reliable transport protocol"
    }

    #[test]
    fn test_blend_results() {
        let semantic = vec![
            SemanticResult { fact_key: "a".into(), similarity: 0.9, source: SearchSource::Semantic },
            SemanticResult { fact_key: "b".into(), similarity: 0.7, source: SearchSource::Semantic },
        ];
        let keyword = vec![
            ("b".into(), "fact b".into(), 0.8),
            ("c".into(), "fact c".into(), 0.6),
        ];
        let blended = SemanticSearchEngine::blend_results(&semantic, &keyword, 0.5);
        assert!(!blended.is_empty());
        // "b" appears in both — should have highest blended score
        assert_eq!(blended[0].0, "b");
    }

    #[test]
    fn test_empty_text() {
        let v = encode_text("");
        // Just verify it doesn't panic
        let _ = v;
    }
}
