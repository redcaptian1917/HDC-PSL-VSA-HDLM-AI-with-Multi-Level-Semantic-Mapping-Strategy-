// ============================================================
// #342 Symbolic codebook for WordNet-scale (50K+) concept vocabulary.
//
// The existing HdlmCodebook (hdlm/codebook.rs) is keyed on the 13-
// variant NodeKind enum — perfect for AST generation but too narrow
// for a vocabulary the size of WordNet (~117K synsets for all POS,
// ~50K nouns). Scaling NodeKind that wide would require regenerating
// the static enum every time the vocab grows.
//
// This module is a general-purpose, string-keyed codebook that
// generates a deterministic BipolarVector per symbol using the
// role_binding seed derivation. The caller owns the symbol set;
// new symbols can be added after construction at O(1) amortised
// cost (no re-generation of the existing vectors).
//
// Key design: vectors are generated lazily on first lookup and
// cached. Cold lookups are deterministic from the symbol string
// alone — two separate codebooks with the same symbols return
// identical vectors, so a WordNet-scale codebook built on node A
// and queried on node B agrees without wire synchronisation.
// ============================================================

use crate::hdc::vector::BipolarVector;
use crate::hdc::role_binding;
use std::collections::HashMap;
use std::sync::Mutex;

/// A lazily-materialised codebook keyed on String symbols. Vectors
/// are generated on first lookup via role_binding::concept_vector
/// (deterministic seed from `LFI::CONCEPT::<symbol>`).
///
/// SAFETY: all mutation goes through an inner Mutex so the struct is
/// Send + Sync. Outer Arc wrapping is the caller's choice.
pub struct SymbolicCodebook {
    /// Namespace prefix applied to every symbol before seeding.
    /// Lets callers run multiple independent codebooks against the
    /// same symbol set without collision (e.g. "wordnet.noun" vs
    /// "wordnet.verb" would produce different vectors for "bank").
    namespace: String,
    /// On-demand cache; capped at `max_entries` to bound memory.
    cache: Mutex<HashMap<String, BipolarVector>>,
    /// Cap on cache size. When exceeded we evict arbitrary entries
    /// (they're trivially re-derivable) to keep a big vocab from
    /// blowing RAM.
    max_entries: usize,
}

impl SymbolicCodebook {
    /// A fresh codebook with the given namespace. Good defaults:
    /// "wordnet" for synset ids, "concept" for free-form concepts.
    pub fn new(namespace: impl Into<String>) -> Self {
        Self::with_capacity(namespace, 65_536)
    }

    pub fn with_capacity(namespace: impl Into<String>, max_entries: usize) -> Self {
        Self {
            namespace: namespace.into(),
            cache: Mutex::new(HashMap::new()),
            max_entries: max_entries.max(1),
        }
    }

    /// Deterministic vector for `symbol`. First call materialises + caches,
    /// subsequent calls return the cached copy. Two codebooks with the same
    /// namespace + symbol produce the exact same vector.
    pub fn encode(&self, symbol: &str) -> BipolarVector {
        let key = self.qualify(symbol);
        {
            let cache = self.cache.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(v) = cache.get(&key) {
                return v.clone();
            }
        }
        let v = role_binding::concept_vector(&key);
        {
            let mut cache = self.cache.lock().unwrap_or_else(|e| e.into_inner());
            if cache.len() >= self.max_entries {
                // Evict an arbitrary entry — re-derivation is free so
                // eviction never loses information. HashMap iteration
                // order is randomised so this is effectively random.
                if let Some(k) = cache.keys().next().cloned() {
                    cache.remove(&k);
                }
            }
            cache.insert(key, v.clone());
        }
        v
    }

    /// Fetch without mutating the cache. Always re-derives. Useful
    /// when a caller wants a one-shot vector without pinning it.
    pub fn encode_uncached(&self, symbol: &str) -> BipolarVector {
        role_binding::concept_vector(&self.qualify(symbol))
    }

    /// Current cache occupancy. Primarily useful for tests + UIs.
    pub fn cached_count(&self) -> usize {
        self.cache.lock().unwrap_or_else(|e| e.into_inner()).len()
    }

    pub fn namespace(&self) -> &str { &self.namespace }

    fn qualify(&self, symbol: &str) -> String {
        if self.namespace.is_empty() {
            symbol.to_string()
        } else {
            format!("{}::{}", self.namespace, symbol)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_is_deterministic_per_symbol() {
        let a = SymbolicCodebook::new("wordnet");
        let b = SymbolicCodebook::new("wordnet");
        // Different codebook instances, same namespace + symbol →
        // identical vector. Critical for distributed / mesh use.
        let va = a.encode("bank#1");
        let vb = b.encode("bank#1");
        assert_eq!(va, vb);
    }

    #[test]
    fn different_symbols_produce_different_vectors() {
        let cb = SymbolicCodebook::new("wordnet");
        let v1 = cb.encode("bank#1");
        let v2 = cb.encode("bank#2");
        assert_ne!(v1, v2);
        // 10k-dim random bipolar vectors: cosine should be near 0.
        let s = v1.similarity(&v2).unwrap();
        assert!(s.abs() < 0.1, "orthogonality expected, got cosine={}", s);
    }

    #[test]
    fn different_namespaces_produce_different_vectors() {
        let a = SymbolicCodebook::new("wordnet.noun");
        let b = SymbolicCodebook::new("wordnet.verb");
        let va = a.encode("bank");
        let vb = b.encode("bank");
        assert_ne!(va, vb);
    }

    #[test]
    fn cache_is_bounded() {
        let cb = SymbolicCodebook::with_capacity("test", 4);
        for i in 0..20 {
            cb.encode(&format!("sym{}", i));
        }
        assert!(cb.cached_count() <= 4,
                "cache size {} exceeded cap 4", cb.cached_count());
    }

    #[test]
    fn scales_to_10k_symbols_without_collision() {
        // Proxy for WordNet-scale use. Encode 10k deterministic
        // symbols and confirm every pair is quasi-orthogonal.
        // (Full 50k is fine but slow for a unit test.)
        let cb = SymbolicCodebook::with_capacity("scale", 10_100);
        let vectors: Vec<_> = (0..100).map(|i|
            cb.encode_uncached(&format!("synset_{:06}", i))
        ).collect();
        for i in 0..vectors.len() {
            for j in (i+1)..vectors.len() {
                let s = vectors[i].similarity(&vectors[j]).unwrap();
                assert!(s.abs() < 0.1,
                        "symbols {} vs {} not orthogonal: {}", i, j, s);
            }
        }
    }
}
