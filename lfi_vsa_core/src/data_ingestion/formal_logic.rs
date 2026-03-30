// NODE 014: Formal Logic Ingestor (Lean Mathlib)
// STATUS: ALPHA - Material Ingestion Active
// PROTOCOL: First-Principles-Vectorization / Lean-VSA-Binding
// QoS: Assume Broken - Exhaustive Telemetry & Tests Mandated

use std::fs::File;
use std::io::{BufRead, BufReader};
use tracing::{info, debug, warn};
use crate::memory_bus::{HyperMemory, DIM_PROLETARIAT};

pub struct FormalLogicIngestor {
    pub memory: HyperMemory,
}

impl FormalLogicIngestor {
    pub fn new() -> Self {
        debug!("FormalLogicIngestor::new: Initializing logic substrate.");
        let memory = HyperMemory::load_from_disk(".vsa_logic_memory.bin")
            .unwrap_or_else(|_| HyperMemory::new(DIM_PROLETARIAT));
        Self { memory }
    }

    /// INGEST: Processes Lean (.lean) files and binds theorem-proof pairs into VSA.
    pub fn ingest_lean_module(&mut self, path: &str) -> Result<usize, Box<dyn std::error::Error>> {
        info!("// AUDIT: Ingesting formal logic from: {}", path);
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut associations = 0;

        let mut current_theorem = String::new();
        
        for line in reader.lines() {
            let line = line?;
            debug!("FormalLogicIngestor: Scanning line: {}", line);

            if line.trim().starts_with("theorem") {
                current_theorem = line.clone();
            } else if line.trim().starts_with("proof") && !current_theorem.is_empty() {
                debug!("FormalLogicIngestor: Binding Theorem -> Proof pair.");
                let theorem_hv = HyperMemory::from_string(&current_theorem, DIM_PROLETARIAT);
                let proof_hv = HyperMemory::from_string(&line, DIM_PROLETARIAT);
                
                let binding = theorem_hv.bind(&proof_hv)?;
                self.memory = HyperMemory::bundle(&[self.memory.clone(), binding])?;
                
                associations += 1;
                current_theorem.clear();
            }
        }

        if associations > 0 {
            info!("// AUDIT: Committed {} logic associations to disk.", associations);
            self.memory.commit_to_disk(".vsa_logic_memory.bin")?;
        } else {
            warn!("// AUDIT: Zero logic associations extracted from {}. Verify Lean syntax parser.", path);
        }

        Ok(associations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lean_ingestion_logic() {
        let mut ingestor = FormalLogicIngestor::new();
        // Synthetic Lean mock
        let content = "theorem t1 : a = b\nproof : reflexivity";
        let path = "/tmp/test_logic.lean";
        std::fs::write(path, content).unwrap();
        
        let result = ingestor.ingest_lean_module(path).unwrap();
        assert_eq!(result, 1, "Should have ingested 1 theorem-proof pair.");
    }

    #[test]
    fn test_vsa_logic_retrieval() {
        let mut ingestor = FormalLogicIngestor::new();
        let theorem = "theorem fermat : x^n + y^n = z^n";
        let proof = "proof : by contradiction";
        
        let theorem_hv = HyperMemory::from_string(theorem, DIM_PROLETARIAT);
        let proof_hv = HyperMemory::from_string(proof, DIM_PROLETARIAT);
        let binding = theorem_hv.bind(&proof_hv).unwrap();
        
        // Bundle into memory
        ingestor.memory = HyperMemory::bundle(&[ingestor.memory.clone(), binding]).unwrap();
        
        // Prove retrieval: Probing with Theorem should yield Proof similarity
        let result = ingestor.memory.bind(&theorem_hv).unwrap();
        let sim = result.similarity(&proof_hv);
        assert!(sim > 0.5, "Theorem probe should yield high similarity to Proof. Sim={:.4}", sim);
    }
}
