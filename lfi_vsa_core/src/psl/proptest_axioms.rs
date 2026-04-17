// ============================================================
// Property-Based Tests for PSL Axioms
// AVP-2 Tier 6: Verify axiom invariants across random inputs
// ============================================================

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use crate::psl::axiom::*;

    // Helper: build a Payload audit target from text
    fn payload(text: &str) -> AuditTarget {
        AuditTarget::Payload {
            source: "proptest".into(),
            fields: vec![("content".into(), text.into())],
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(500))]

        /// DataIntegrityAxiom: confidence always in [0, 1]
        #[test]
        fn data_integrity_confidence_bounded(text in ".*") {
            let axiom = DataIntegrityAxiom { max_bytes: 1_000_000 };
            let target = payload(&text);
            if let Ok(verdict) = axiom.evaluate(&target) {
                prop_assert!(verdict.confidence >= 0.0 && verdict.confidence <= 1.0,
                    "DataIntegrity confidence out of bounds: {}", verdict.confidence);
            }
        }

        /// DataIntegrityAxiom: empty payload always passes (vacuously true)
        #[test]
        fn data_integrity_empty_passes(_seed in 0u64..1000) {
            let axiom = DataIntegrityAxiom { max_bytes: 1_000_000 };
            let target = payload("");
            if let Ok(verdict) = axiom.evaluate(&target) {
                prop_assert!(verdict.confidence >= 0.5,
                    "Empty payload should pass data integrity: {}", verdict.confidence);
            }
        }

        /// StatisticalEquilibriumAxiom: confidence always in [0, 1]
        #[test]
        fn stat_equilibrium_confidence_bounded(text in ".{0,500}") {
            let axiom = StatisticalEquilibriumAxiom { tolerance: 0.3 };
            let target = payload(&text);
            if let Ok(verdict) = axiom.evaluate(&target) {
                prop_assert!(verdict.confidence >= 0.0 && verdict.confidence <= 1.0,
                    "StatEquilibrium confidence out of bounds: {}", verdict.confidence);
            }
        }

        /// InjectionDetectionAxiom: confidence always in [0, 1]
        #[test]
        fn injection_confidence_bounded(text in ".{0,500}") {
            let axiom = InjectionDetectionAxiom;
            let target = payload(&text);
            if let Ok(verdict) = axiom.evaluate(&target) {
                prop_assert!(verdict.confidence >= 0.0 && verdict.confidence <= 1.0,
                    "InjectionDetection confidence out of bounds: {}", verdict.confidence);
            }
        }

        /// InjectionDetectionAxiom: known injection gets low confidence (detected)
        #[test]
        fn injection_detects_known_patterns(_seed in 0u64..100) {
            let axiom = InjectionDetectionAxiom;
            let target = payload("'; DROP TABLE users; --");
            if let Ok(verdict) = axiom.evaluate(&target) {
                prop_assert!(verdict.confidence < 0.8,
                    "SQL injection should be detected: {}", verdict.confidence);
            }
        }

        /// OutputBoundsAxiom: confidence always in [0, 1]
        #[test]
        fn output_bounds_confidence_bounded(text in ".{0,500}") {
            let axiom = OutputBoundsAxiom::default();
            let target = payload(&text);
            if let Ok(verdict) = axiom.evaluate(&target) {
                prop_assert!(verdict.confidence >= 0.0 && verdict.confidence <= 1.0,
                    "OutputBounds confidence out of bounds: {}", verdict.confidence);
            }
        }

        /// EntropyAxiom: confidence always in [0, 1]
        #[test]
        fn entropy_confidence_bounded(text in ".{0,200}") {
            let axiom = EntropyAxiom::default();
            let target = payload(&text);
            if let Ok(verdict) = axiom.evaluate(&target) {
                prop_assert!(verdict.confidence >= 0.0 && verdict.confidence <= 1.0,
                    "Entropy confidence out of bounds: {}", verdict.confidence);
            }
        }

        /// No axiom panics on arbitrary input
        #[test]
        fn no_axiom_panics(text in ".*") {
            let target = payload(&text);
            let _ = DataIntegrityAxiom { max_bytes: 1_000_000 }.evaluate(&target);
            let _ = DimensionalityAxiom.evaluate(&target);
            let _ = StatisticalEquilibriumAxiom { tolerance: 0.3 }.evaluate(&target);
            let _ = InjectionDetectionAxiom.evaluate(&target);
            let _ = ExfiltrationDetectionAxiom.evaluate(&target);
            // If we get here without panicking, the test passes
        }
    }
}
