// ============================================================
// HDLM Codebook — Semantic Vector Mapping
// Section 1.III: Multi-Level Semantic Mapping
//
// Maps symbolic AST nodes and tokens to unique 10,000-bit
// bipolar hypervectors. This is the "Item Memory" required
// for decoding VSA structures back into symbolic ASTs.
// ============================================================

use crate::hdc::vector::BipolarVector;
use crate::hdlm::ast::NodeKind;
use crate::hdlm::error::HdlmError;
use crate::debuglog;
use std::collections::HashMap;

/// Result type for codebook operations.
pub type CodebookResult<T> = Result<T, HdlmError>;

/// A codebook (item memory) for HDLM semantic mapping.
/// Stores orthogonal bases for forensic node kinds and tokens.
pub struct HdlmCodebook {
    /// Mapping of NodeKind (as String) to unique base vectors.
    kind_bases: HashMap<String, BipolarVector>,
    
    /// Positional encoding vectors (e.g., for child indices).
    pos_bases: Vec<BipolarVector>,
}

impl HdlmCodebook {
    /// Initialize a new codebook with fresh orthogonal bases for a set of kinds.
    pub fn new(kinds: &[NodeKind]) -> CodebookResult<Self> {
        debuglog!("HdlmCodebook::new: Initializing with {} kinds", kinds.len());
        
        let mut kind_bases = HashMap::new();
        for kind in kinds {
            let key = Self::kind_to_key(kind);
            if !kind_bases.contains_key(&key) {
                debuglog!("HdlmCodebook::new: Generating base for kind '{}'", key);
                kind_bases.insert(key, BipolarVector::new_random().map_err(|e| HdlmError::Tier1GenerationFailed {
                    reason: format!("VSA initialization failed: {}", e),
                })?);
            }
        }
        
        // Generate 10 positional bases for structural hierarchy.
        let mut pos_bases = Vec::with_capacity(10);
        for idx in 0..10 {
            debuglog!("HdlmCodebook::new: Generating positional base {}", idx);
            pos_bases.push(BipolarVector::new_random().map_err(|e| HdlmError::Tier1GenerationFailed {
                reason: format!("VSA initialization failed: {}", e),
            })?);
        }
        
        Ok(Self { kind_bases, pos_bases })
    }

    /// Retrieve the base vector for a specific NodeKind.
    pub fn get_kind_base(&self, kind: &NodeKind) -> Option<&BipolarVector> {
        let key = Self::kind_to_key(kind);
        debuglog!("HdlmCodebook::get_kind_base: key='{}'", key);
        self.kind_bases.get(&key)
    }

    /// Retrieve a positional encoding vector by index.
    pub fn get_pos_base(&self, index: usize) -> Option<&BipolarVector> {
        debuglog!("HdlmCodebook::get_pos_base: index={}", index);
        self.pos_bases.get(index)
    }

    /// Identifies the closest NodeKind for a given hypervector.
    /// Uses the cosine similarity metric (HDC Core).
    pub fn identify_kind(&self, hv: &BipolarVector) -> CodebookResult<(NodeKind, f64)> {
        debuglog!("HdlmCodebook::identify_kind: query dim={}", hv.dim());
        
        let mut best_kind = None;
        let mut max_sim = -1.1; // Lower than possible minimum
        
        for (key, base) in &self.kind_bases {
            let sim = hv.similarity(base).map_err(|e| HdlmError::Tier1GenerationFailed {
                reason: format!("Similarity check failed: {}", e),
            })?;
            
            if sim > max_sim {
                max_sim = sim;
                best_kind = Some(key);
            }
        }
        
        if let Some(key) = best_kind {
            debuglog!("HdlmCodebook::identify_kind: MATCH found, kind={}, sim={:.4}", key, max_sim);
            // In a real implementation, we'd parse the key back to a NodeKind enum.
            // Simplified for now: assume we can reconstruct or return a generic variant.
            Ok((NodeKind::Root, max_sim)) // Placeholder
        } else {
            Err(HdlmError::Tier1GenerationFailed {
                reason: "IdentifyKind: Codebook is empty".to_string(),
            })
        }
    }

    fn kind_to_key(kind: &NodeKind) -> String {
        // Simple discriminant-based key for the codebook.
        let key = format!("{:?}", kind);
        debuglog!("HdlmCodebook::kind_to_key: {:?} -> '{}'", kind, key);
        key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codebook_orthogonality() -> CodebookResult<()> {
        let kinds = vec![NodeKind::Root, NodeKind::Literal { value: "0".to_string() }];
        let cb = HdlmCodebook::new(&kinds)?;
        
        let v1 = cb.get_kind_base(&kinds[0]).ok_or_else(|| HdlmError::Tier1GenerationFailed {
            reason: "Missing base for Root".to_string(),
        })?;
        let v2 = cb.get_kind_base(&kinds[1]).ok_or_else(|| HdlmError::Tier1GenerationFailed {
            reason: "Missing base for Literal".to_string(),
        })?;
        
        let sim = v1.similarity(v2).map_err(|e| HdlmError::Tier1GenerationFailed {
            reason: e.to_string()
        })?;
        
        // Random 10k-bit vectors should be quasi-orthogonal (sim ~ 0).
        assert!(sim.abs() < 0.1, "sim={}", sim);
        Ok(())
    }
}
