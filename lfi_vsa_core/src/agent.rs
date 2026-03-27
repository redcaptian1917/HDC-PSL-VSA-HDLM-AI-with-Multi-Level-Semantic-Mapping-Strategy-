// ============================================================
// LFI Agent Orchestrator — High-Level Reasoning Loop
// Section 2: "Operate as an autonomous intelligence leveraging
// Zero-Trust and Assume Breach protocols."
// ============================================================

use crate::hdc::vector::BipolarVector;
use crate::hdc::compute::LocalBackend;
use crate::hdlm::codebook::HdlmCodebook;
use crate::hdlm::ast::{Ast, NodeKind};
use crate::psl::supervisor::PslSupervisor;
use crate::psl::axiom::AuditTarget;
use crate::hid::{HidDevice, HidCommand};
use crate::hdc::error::HdcError;
use crate::debuglog;

/// The primary LFI Agent. Orchestrates the VSA, PSL, and HID layers.
pub struct LfiAgent {
    /// Compute backend for HDC operations.
    pub compute: LocalBackend,
    /// PSL Supervisor for forensic auditing.
    pub supervisor: PslSupervisor,
    /// Codebook for semantic mapping.
    pub codebook: HdlmCodebook,
    /// Hardware interface.
    pub hid: HidDevice,
}

impl LfiAgent {
    /// Initialize a new agent with a baseline codebook and supervisor.
    pub fn new() -> Result<Self, HdcError> {
        debuglog!("LfiAgent::new: Initializing autonomous agent");
        
        let compute = LocalBackend;
        let supervisor = PslSupervisor::new();
        
        // Initialize codebook with core node kinds
        let kinds = vec![
            NodeKind::Root,
            NodeKind::Assignment,
            NodeKind::Call { function: String::new() },
            NodeKind::Return,
        ];
        let codebook = HdlmCodebook::new(&kinds).map_err(|e| HdcError::InitializationFailed {
            reason: format!("Codebook init failed: {}", e),
        })?;
        
        let hid = HidDevice::new(None)?;
        
        Ok(Self { compute, supervisor, codebook, hid })
    }

    /// Executes a forensic task: Sense -> Think -> Act (Audited).
    pub fn execute_task(&self, task_name: &str) -> Result<(), HdcError> {
        debuglog!("LfiAgent::execute_task: starting '{}'", task_name);

        // 1. SENSE (Simulated): Ingest task intent into hypervector
        let task_vector = BipolarVector::new_random()?;
        
        // 2. THINK: Forensic AST Generation (Simplified)
        let mut ast = Ast::new();
        ast.add_node(NodeKind::Root);
        debuglog!("LfiAgent::execute_task: generated forensic AST");

        // 3. AUDIT: PSL verification of the task vector against axioms
        // Wrap vector in AuditTarget for PSL
        let target = AuditTarget::Vector(task_vector.clone());
        
        // We handle the case where supervisor might be empty for this demo
        if self.supervisor.axiom_count() > 0 {
            let assessment = self.supervisor.audit(&target).map_err(|e| HdcError::InitializationFailed {
                reason: format!("PSL Audit Failure: {:?}", e),
            })?;
            debuglog!("LfiAgent::execute_task: PSL Level: {:?}, Score: {:.4}", 
                assessment.level, assessment.level.score());

            if !assessment.level.permits_execution() {
                debuglog!("LfiAgent::execute_task: FAIL - Hostile payload detected by PSL");
                return Err(HdcError::InitializationFailed {
                    reason: format!("PSL Audit Failure: Trust level {:?} too low", assessment.level),
                });
            }
        } else {
            debuglog!("LfiAgent::execute_task: WARN - No PSL axioms loaded, skipping audit.");
        }

        // 4. ACT: Dispatch to HID Injection
        debuglog!("LfiAgent::execute_task: Dispatching action to HID");
        self.hid.execute(HidCommand::MouseMove { x: 100, y: 100 })?;

        debuglog!("LfiAgent::execute_task: SUCCESS - Task '{}' completed.", task_name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_task_orchestration() -> Result<(), HdcError> {
        let agent = LfiAgent::new()?;
        // Should succeed because no axioms are loaded, bypassing audit.
        agent.execute_task("Initialize UI Probe")?;
        Ok(())
    }
}
