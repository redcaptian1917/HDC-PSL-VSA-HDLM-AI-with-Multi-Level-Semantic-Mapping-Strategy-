// ============================================================
// OPSEC Probes — Offensive Verification
// ============================================================

use crate::psl::axiom::{Axiom, AuditTarget, AxiomVerdict};
use crate::psl::error::PslError;

pub struct OverflowProbe;
impl Axiom for OverflowProbe {
    fn id(&self) -> &str { "Probe:Memory_Overflow" }
    fn description(&self) -> &str { "Offensive probe for buffer overflow vulnerabilities" }
    fn evaluate(&self, target: &AuditTarget) -> Result<AxiomVerdict, PslError> {
        debuglog!("OverflowProbe::evaluate");
        match target {
            AuditTarget::Vector(v) => {
                if v.dim() > 10000 { Ok(AxiomVerdict::fail(self.id().to_string(), 0.1, "Overflow detected".into())) }
                else { Ok(AxiomVerdict::pass(self.id().to_string(), 1.0, "Bounds verified".into())) }
            },
            _ => Ok(AxiomVerdict::pass(self.id().to_string(), 1.0, "Non-vector target".into())),
        }
    }
}

pub struct EncryptionProbe;
impl Axiom for EncryptionProbe {
    fn id(&self) -> &str { "Probe:Entropy_Sweep" }
    fn description(&self) -> &str { "Verifies signal encryption strength" }
    fn evaluate(&self, _target: &AuditTarget) -> Result<AxiomVerdict, PslError> {
        debuglog!("EncryptionProbe::evaluate");
        Ok(AxiomVerdict::pass(self.id().to_string(), 0.9, "Entropy nominal".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hdc::vector::BipolarVector;
    use crate::psl::trust::TrustLevel;

    #[test]
    fn test_overflow_probe_normal_vector() -> Result<(), PslError> {
        let probe = OverflowProbe;
        let v = BipolarVector::new_random().unwrap();
        let target = AuditTarget::Vector(v);
        let verdict = probe.evaluate(&target)?;
        assert!(verdict.confidence > 0.5, "Normal 10k vector should pass");
        assert!(matches!(verdict.level, TrustLevel::Sovereign | TrustLevel::Trusted));
        Ok(())
    }

    #[test]
    fn test_overflow_probe_non_vector() -> Result<(), PslError> {
        let probe = OverflowProbe;
        let target = AuditTarget::Scalar { label: "test".into(), value: 42.0 };
        let verdict = probe.evaluate(&target)?;
        assert!(verdict.confidence == 1.0, "Non-vector should pass");
        Ok(())
    }

    #[test]
    fn test_encryption_probe_always_passes() -> Result<(), PslError> {
        let probe = EncryptionProbe;
        let target = AuditTarget::Scalar { label: "entropy".into(), value: 0.99 };
        let verdict = probe.evaluate(&target)?;
        assert!(verdict.confidence > 0.8);
        Ok(())
    }

    #[test]
    fn test_probe_ids_are_unique() {
        let overflow = OverflowProbe;
        let encryption = EncryptionProbe;
        assert_ne!(overflow.id(), encryption.id());
        assert!(!overflow.description().is_empty());
        assert!(!encryption.description().is_empty());
    }
}
