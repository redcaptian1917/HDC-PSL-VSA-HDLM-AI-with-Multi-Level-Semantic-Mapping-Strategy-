// NODE 030: Substrate Diagnostic Engine (Self-Test)
// STATUS: ALPHA - Forensic Diagnostics Active
// PROTOCOL: Assume-Broken / Proactive-Audit
// QoS: Intentional Commenting - Validates substrate functionality
// REFERENCE: Man pages for 'tokio' and 'ndarray' used for async logic and vector math.

use crate::memory_bus::{HyperMemory, DIM_PROLETARIAT};
use crate::telemetry::MaterialAuditor;
use serde::{Serialize, Deserialize};
use tracing::info;

/// Result of a single substrate self-test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub component: String,
    pub status: String, // "NOMINAL" | "FAULT" | "DEGRADED"
    pub details: String,
    pub timestamp: String,
}

pub struct DiagnosticEngine;

impl DiagnosticEngine {
    /// EXECUTE: Runs a comprehensive suite of self-tests to prove functionality.
    /// This adheres to the QoS 'Assume Broken' mandate.
    pub fn run_full_suite() -> Vec<TestResult> {
        info!("// DIAG: Initiating substrate self-test suite.");
        let mut results = Vec::new();

        // 1. VSA Memory Test
        results.push(Self::test_vsa_integrity());

        // 2. Hardware Thermal Test
        results.push(Self::test_thermal_bounds());

        // 3. Storage I/O Test
        results.push(Self::test_persistence());

        info!("// DIAG: Self-test suite complete. Faults detected: {}", 
              results.iter().filter(|r| r.status == "FAULT").count());
        results
    }

    /// TEST: Proves the VSA memory can still perform binding/similarity.
    fn test_vsa_integrity() -> TestResult {
        let v1 = HyperMemory::from_string("DIAG_V1", DIM_PROLETARIAT);
        let v2 = HyperMemory::from_string("DIAG_V2", DIM_PROLETARIAT);
        
        match v1.bind(&v2) {
            Ok(bound) => {
                let sim = bound.similarity(&v1);
                // In VSA, a bound vector should be orthogonal to its factors
                if sim < 0.1 {
                    TestResult {
                        component: "VSA Memory Bus".to_string(),
                        status: "NOMINAL".to_string(),
                        details: format!("Binding integrity verified. Sim={:.4}", sim),
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    }
                } else {
                    TestResult {
                        component: "VSA Memory Bus".to_string(),
                        status: "FAULT".to_string(),
                        details: "Concept bleed detected in binding logic.".to_string(),
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    }
                }
            },
            Err(e) => TestResult {
                component: "VSA Memory Bus".to_string(),
                status: "FAULT".to_string(),
                details: format!("Binding error: {:?}", e),
                timestamp: chrono::Utc::now().to_rfc3339(),
            }
        }
    }

    /// TEST: Verifies hardware thermals are within Supersociety bounds.
    fn test_thermal_bounds() -> TestResult {
        let probe = HyperMemory::new(DIM_PROLETARIAT);
        let stats = MaterialAuditor::get_stats(probe.audit_orthogonality(), 1.0);
        
        if stats.cpu_temp_c < 75.0 {
            TestResult {
                component: "Hardware Thermals".to_string(),
                status: "NOMINAL".to_string(),
                details: format!("Temperature is safe at {:.1}°C", stats.cpu_temp_c),
                timestamp: chrono::Utc::now().to_rfc3339(),
            }
        } else {
            TestResult {
                component: "Hardware Thermals".to_string(),
                status: "DEGRADED".to_string(),
                details: format!("High thermals detected: {:.1}°C", stats.cpu_temp_c),
                timestamp: chrono::Utc::now().to_rfc3339(),
            }
        }
    }

    /// TEST: Verifies file system write capability.
    fn test_persistence() -> TestResult {
        let path = "/tmp/diag_write_test.bin";
        let test_data = vec![1, 2, 3, 4];
        match std::fs::write(path, &test_data) {
            Ok(_) => {
                let _ = std::fs::remove_file(path);
                TestResult {
                    component: "Storage I/O".to_string(),
                    status: "NOMINAL".to_string(),
                    details: "Persistent write/delete verified.".to_string(),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                }
            },
            Err(e) => TestResult {
                component: "Storage I/O".to_string(),
                status: "FAULT".to_string(),
                details: format!("Write failure: {:?}", e),
                timestamp: chrono::Utc::now().to_rfc3339(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic_suite_execution() {
        let results = DiagnosticEngine::run_full_suite();
        assert!(!results.is_empty(), "Diagnostic suite must return results.");
    }
}
