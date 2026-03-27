// ============================================================
// HDLM: Hyperdimensional Language Model — Semantic Mapping
// Section 1.III: Multi-Level Semantic Mapping
// Tier 1 (Forensic): AST logic vectors.
// Tier 2 (Decorative): Aesthetic expansion and prose.
// ============================================================

use crate::hdc::vector::BipolarVector;
use crate::hdc::error::HdcError;
use crate::debuglog;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Result type for HDLM operations.
pub type HdlmResult<T> = Result<T, HdcError>;

/// Tier 1: Forensic AST Node Types.
/// These represent the mathematically perfect logic of the code.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ForensicNode {
    Module,
    Function,
    Statement,
    Expression,
    Literal,
    Identifier,
    Unknown,
}

/// A Semantic Mapping between a Forensic AST and the VSA space.
/// Enforces strict separation between logic (Tier 1) and decoration (Tier 2).
pub struct SemanticMap {
    /// Mapping of node types to their unique, orthogonal base vectors.
    node_bases: HashMap<ForensicNode, BipolarVector>,
    
    /// Positional encoding vectors for structural hierarchy.
    pos_bases: Vec<BipolarVector>,
}

impl SemanticMap {
    /// Initialize a new SemanticMap with fresh orthogonal bases.
    pub fn new() -> HdlmResult<Self> {
        debuglog!("SemanticMap::new: Initializing forensic bases");
        
        let mut node_bases = HashMap::new();
        node_bases.insert(ForensicNode::Module, BipolarVector::new_random()?);
        node_bases.insert(ForensicNode::Function, BipolarVector::new_random()?);
        node_bases.insert(ForensicNode::Statement, BipolarVector::new_random()?);
        node_bases.insert(ForensicNode::Expression, BipolarVector::new_random()?);
        node_bases.insert(ForensicNode::Literal, BipolarVector::new_random()?);
        node_bases.insert(ForensicNode::Identifier, BipolarVector::new_random()?);
        node_bases.insert(ForensicNode::Unknown, BipolarVector::new_random()?);
        
        // Positional bases for up to 10 children per node.
        let mut pos_bases = Vec::with_capacity(10);
        for i in 0..10 {
            debuglog!("SemanticMap::new: Generating positional base {}", i);
            pos_bases.push(BipolarVector::new_random()?);
        }
        
        Ok(Self { node_bases, pos_bases })
    }

    /// Returns the positional encoding base vector at the given child index.
    /// Used for encoding structural hierarchy in the AST.
    pub fn get_pos_base(&self, index: usize) -> Option<&BipolarVector> {
        debuglog!("SemanticMap::get_pos_base: index={}", index);
        self.pos_bases.get(index)
    }

    /// Projects a Forensic Node and its Decorative metadata into a single Hypervector.
    ///
    /// `V = XOR(ForensicBase, DecorativeVector)`
    pub fn project_node(
        &self, 
        node: ForensicNode, 
        decoration: &BipolarVector
    ) -> HdlmResult<BipolarVector> {
        debuglog!("project_node: entry, node={:?}", node);
        
        let base = self.node_bases.get(&node).ok_or_else(|| {
            debuglog!("project_node: FAIL - MissingBase for {:?}", node);
            HdcError::InitializationFailed {
                reason: format!("No base vector for node type {:?}", node),
            }
        })?;
        
        // Bind the logic to the decoration.
        // Resulting vector is quasi-orthogonal to both, but invertible if decoration is known.
        let projected = base.bind(decoration)?;
        
        debuglog!("project_node: SUCCESS, node={:?}, similarity_to_base={:.4}", 
            node, projected.similarity(base)?);
            
        Ok(projected)
    }

    /// Verifies if a projected vector contains a specific Forensic Node.
    ///
    /// `Verification = CosineSimilarity(XOR(Projected, Decoration), ForensicBase) \approx 1.0`
    pub fn verify_forensic_integrity(
        &self,
        projected: &BipolarVector,
        node: ForensicNode,
        decoration: &BipolarVector
    ) -> HdlmResult<bool> {
        debuglog!("verify_forensic_integrity: entry, node={:?}", node);
        
        let base = self.node_bases.get(&node).ok_or_else(|| {
            HdcError::InitializationFailed {
                reason: format!("No base vector for node type {:?}", node),
            }
        })?;
        
        // Unbind the decoration to recover the forensic logic.
        let recovered = projected.bind(decoration)?;
        let sim = recovered.similarity(base)?;
        
        debuglog!("verify_forensic_integrity: recovered_similarity={:.4}", sim);
        
        // Tolerance for floating point and VSA noise (should be exactly 1.0 in this model).
        Ok(sim > 0.99)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_mapping_separation() -> HdlmResult<()> {
        let map = SemanticMap::new()?;
        let logic = ForensicNode::Function;
        let decoration = BipolarVector::new_random()?; // Aesthetic expansion
        
        // 1. Project logic + decoration
        let projected = map.project_node(logic.clone(), &decoration)?;
        
        // 2. Verify Forensic Integrity
        assert!(map.verify_forensic_integrity(&projected, logic, &decoration)?);
        
        // 3. Verify that a different decoration fails verification
        let fake_decoration = BipolarVector::new_random()?;
        assert!(!map.verify_forensic_integrity(&projected, ForensicNode::Function, &fake_decoration)?);
        
        Ok(())
    }
}
