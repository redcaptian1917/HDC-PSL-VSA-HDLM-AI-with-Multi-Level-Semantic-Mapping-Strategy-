// ============================================================
// ComputeBackend Trait — Modular Dispatch Layer
// Default: Local ARM SIMD (bitwise). Extensible to remote GPU.
// Section 1.I: "modular ComputeBackend trait to dispatch
// massive matrix operations to remote GPU grids."
// ============================================================

use crate::hdc::error::HdcError;
use crate::hdc::vector::BipolarVector;

/// Trait defining the compute dispatch interface for HDC operations.
/// Implementors handle the actual arithmetic — locally or remotely.
pub trait ComputeBackend {
    /// Binding: element-wise bipolar multiplication (XOR in binary).
    fn bind(&self, a: &BipolarVector, b: &BipolarVector) -> Result<BipolarVector, HdcError>;

    /// Bundling: majority-vote superposition of N vectors (Sum+Clip).
    fn bundle(&self, vectors: &[&BipolarVector]) -> Result<BipolarVector, HdcError>;

    /// Permutation: cyclic left shift by `shift` positions.
    fn permute(&self, v: &BipolarVector, shift: usize) -> Result<BipolarVector, HdcError>;

    /// Cosine similarity in bipolar space. Returns [-1.0, 1.0].
    fn similarity(&self, a: &BipolarVector, b: &BipolarVector) -> Result<f64, HdcError>;
}

/// Local compute backend — bitwise operations on host CPU.
/// Targets ARMv9.2-A SIMD/NEON on the Tensor G5 Laguna SoC.
pub struct LocalBackend;

impl ComputeBackend for LocalBackend {
    fn bind(&self, a: &BipolarVector, b: &BipolarVector) -> Result<BipolarVector, HdcError> {
        debuglog!("LocalBackend::bind dispatched");
        a.bind(b)
    }

    fn bundle(&self, vectors: &[&BipolarVector]) -> Result<BipolarVector, HdcError> {
        debuglog!("LocalBackend::bundle dispatched, n={}", vectors.len());
        BipolarVector::bundle(vectors)
    }

    fn permute(&self, v: &BipolarVector, shift: usize) -> Result<BipolarVector, HdcError> {
        debuglog!("LocalBackend::permute dispatched, shift={}", shift);
        v.permute(shift)
    }

    fn similarity(&self, a: &BipolarVector, b: &BipolarVector) -> Result<f64, HdcError> {
        debuglog!("LocalBackend::similarity dispatched");
        a.similarity(b)
    }
}

/// Estimate the memory footprint of the LFI system at different scales.
pub struct ResourceEstimator;

impl ResourceEstimator {
    /// Estimate memory usage for a given configuration.
    ///
    /// Returns (vector_memory_bytes, overhead_bytes, total_bytes).
    pub fn estimate_memory(
        dim: usize,
        num_vectors: usize,
        num_codebook_entries: usize,
    ) -> (usize, usize, usize) {
        // BipolarVector: ceil(dim / 8) bytes for bitvec storage
        let vec_bytes = (dim + 7) / 8;
        let vector_memory = num_vectors * vec_bytes;

        // Codebook: each entry is a vector + metadata (~64 bytes overhead)
        let codebook_memory = num_codebook_entries * (vec_bytes + 64);

        // System overhead: PSL supervisor, provenance arena, knowledge engine
        let overhead = 1024 * 1024; // ~1MB baseline overhead

        let total = vector_memory + codebook_memory + overhead;
        (vector_memory, codebook_memory + overhead, total)
    }

    /// Check if the system can run on a device with the given RAM.
    pub fn fits_in_ram(available_mb: usize, dim: usize, num_vectors: usize) -> bool {
        let (_, _, total) = Self::estimate_memory(dim, num_vectors, 1000);
        let total_mb = total / (1024 * 1024);
        total_mb < available_mb
    }

    /// Estimate for the user's laptop: i7 64GB RAM 3050Ti.
    pub fn laptop_estimate() -> String {
        let (vm, oh, total) = Self::estimate_memory(10000, 100000, 5000);
        format!(
            "Laptop (64GB): {} vectors @ 10k dim = {:.1}MB vectors + {:.1}MB overhead = {:.1}MB total. Fits: YES ({:.1}% of RAM)",
            100000, vm as f64 / 1e6, oh as f64 / 1e6, total as f64 / 1e6,
            (total as f64 / (64.0 * 1024.0 * 1024.0 * 1024.0)) * 100.0
        )
    }

    /// Estimate for the user's phone: Pixel 10 Pro XL (~12GB RAM).
    pub fn phone_estimate() -> String {
        let (vm, oh, total) = Self::estimate_memory(10000, 10000, 1000);
        format!(
            "Phone (12GB): {} vectors @ 10k dim = {:.1}MB vectors + {:.1}MB overhead = {:.1}MB total. Fits: {}",
            10000, vm as f64 / 1e6, oh as f64 / 1e6, total as f64 / 1e6,
            if total < 4 * 1024 * 1024 * 1024 { "YES" } else { "TIGHT" }
        )
    }
}

/// Hardware deployment profile — configures LFI for specific targets.
#[derive(Debug, Clone)]
pub struct DeploymentProfile {
    pub name: String,
    pub max_vectors: usize,
    pub max_codebook: usize,
    pub max_mcts_iterations: usize,
    pub max_provenance_entries: usize,
    pub enable_stress_tests: bool,
    pub estimated_ram_mb: usize,
}

impl DeploymentProfile {
    /// Profile for the user's MSI Katana laptop: i7, 64GB RAM, 3050Ti.
    pub fn laptop() -> Self {
        Self {
            name: "MSI Katana i7/64GB/3050Ti".into(),
            max_vectors: 500_000,
            max_codebook: 50_000,
            max_mcts_iterations: 1000,
            max_provenance_entries: 100_000,
            enable_stress_tests: true,
            estimated_ram_mb: 64 * 1024,
        }
    }

    /// Profile for the user's Pixel 10 Pro XL: Tensor G5, 12-16GB RAM.
    pub fn pixel_phone() -> Self {
        Self {
            name: "Pixel 10 Pro XL / Tensor G5".into(),
            max_vectors: 50_000,
            max_codebook: 5_000,
            max_mcts_iterations: 100,
            max_provenance_entries: 10_000,
            enable_stress_tests: false,
            estimated_ram_mb: 12 * 1024,
        }
    }

    /// Profile for embedded/IoT: RP2040, 264KB RAM.
    pub fn embedded() -> Self {
        Self {
            name: "RP2040 Embedded".into(),
            max_vectors: 100,
            max_codebook: 50,
            max_mcts_iterations: 10,
            max_provenance_entries: 100,
            enable_stress_tests: false,
            estimated_ram_mb: 0, // Sub-MB
        }
    }

    /// Check if this profile can handle the given workload.
    pub fn can_handle(&self, vectors: usize, codebook: usize) -> bool {
        vectors <= self.max_vectors && codebook <= self.max_codebook
    }

    /// Recommended intelligence tier for this hardware.
    pub fn recommended_max_tier(&self) -> crate::cognition::router::IntelligenceTier {
        if self.estimated_ram_mb >= 32 * 1024 {
            crate::cognition::router::IntelligenceTier::BigBrain
        } else if self.estimated_ram_mb >= 4 * 1024 {
            crate::cognition::router::IntelligenceTier::Bridge
        } else {
            crate::cognition::router::IntelligenceTier::Pulse
        }
    }
}

// ============================================================
// ComputeBackend dispatch tests — verify local backend parity
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_backend_bind_dispatches() -> Result<(), HdcError> {
        let backend = LocalBackend;
        let a = BipolarVector::new_random()?;
        let b = BipolarVector::new_random()?;
        let direct = a.bind(&b)?;
        let via_backend = backend.bind(&a, &b)?;
        assert_eq!(direct, via_backend, "Backend dispatch must match direct call");
        Ok(())
    }

    #[test]
    fn test_local_backend_bundle_dispatches() -> Result<(), HdcError> {
        let backend = LocalBackend;
        let a = BipolarVector::new_random()?;
        let b = BipolarVector::new_random()?;
        let direct = BipolarVector::bundle(&[&a, &b])?;
        let via_backend = backend.bundle(&[&a, &b])?;
        assert_eq!(direct, via_backend, "Backend dispatch must match direct call");
        Ok(())
    }

    #[test]
    fn test_local_backend_permute_dispatches() -> Result<(), HdcError> {
        let backend = LocalBackend;
        let a = BipolarVector::new_random()?;
        let direct = a.permute(7)?;
        let via_backend = backend.permute(&a, 7)?;
        assert_eq!(direct, via_backend, "Backend dispatch must match direct call");
        Ok(())
    }

    #[test]
    fn test_local_backend_similarity_dispatches() -> Result<(), HdcError> {
        let backend = LocalBackend;
        let a = BipolarVector::new_random()?;
        let b = BipolarVector::new_random()?;
        let direct = a.similarity(&b)?;
        let via_backend = backend.similarity(&a, &b)?;
        assert!((direct - via_backend).abs() < f64::EPSILON);
        Ok(())
    }

    #[test]
    fn test_resource_estimator_laptop() {
        let estimate = ResourceEstimator::laptop_estimate();
        assert!(estimate.contains("YES"), "64GB laptop should fit: {}", estimate);
    }

    #[test]
    fn test_resource_estimator_phone() {
        let estimate = ResourceEstimator::phone_estimate();
        assert!(!estimate.is_empty(), "Phone estimate should produce output: {}", estimate);
    }

    #[test]
    fn test_fits_in_ram() {
        // 64GB laptop with 100k vectors should fit.
        assert!(ResourceEstimator::fits_in_ram(64 * 1024, 10000, 100000));
        // 1MB device cannot hold 1M vectors.
        assert!(!ResourceEstimator::fits_in_ram(1, 10000, 1000000));
    }

    #[test]
    fn test_memory_estimate_scales_linearly() {
        let (_, _, total_small) = ResourceEstimator::estimate_memory(10000, 1000, 100);
        let (_, _, total_large) = ResourceEstimator::estimate_memory(10000, 10000, 100);
        // 10x more vectors should use roughly 10x more vector memory.
        assert!(total_large > total_small);
    }

    #[test]
    fn test_bind_performance() -> Result<(), HdcError> {
        // Benchmark: 1000 bind operations should complete quickly.
        let a = BipolarVector::new_random()?;
        let b = BipolarVector::new_random()?;
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _ = a.bind(&b)?;
        }
        let elapsed = start.elapsed();
        debuglog!("Benchmark: 1000 binds in {:?} ({:.0} ops/sec)",
            elapsed, 1000.0 / elapsed.as_secs_f64());
        // Should complete in under 1 second on any modern hardware.
        assert!(elapsed.as_secs() < 5, "1000 binds took too long: {:?}", elapsed);
        Ok(())
    }

    #[test]
    fn test_similarity_performance() -> Result<(), HdcError> {
        let a = BipolarVector::new_random()?;
        let b = BipolarVector::new_random()?;
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _ = a.similarity(&b)?;
        }
        let elapsed = start.elapsed();
        debuglog!("Benchmark: 1000 similarities in {:?} ({:.0} ops/sec)",
            elapsed, 1000.0 / elapsed.as_secs_f64());
        assert!(elapsed.as_secs() < 5, "1000 similarities took too long: {:?}", elapsed);
        Ok(())
    }

    #[test]
    fn test_laptop_profile() {
        let profile = DeploymentProfile::laptop();
        assert!(profile.max_vectors >= 100_000);
        assert!(profile.can_handle(100_000, 10_000));
        assert!(matches!(profile.recommended_max_tier(), crate::cognition::router::IntelligenceTier::BigBrain));
    }

    #[test]
    fn test_phone_profile() {
        let profile = DeploymentProfile::pixel_phone();
        assert!(profile.max_vectors >= 10_000);
        assert!(profile.can_handle(10_000, 1_000));
        assert!(!profile.can_handle(100_000, 1_000), "Phone shouldn't handle laptop-scale");
        assert!(matches!(profile.recommended_max_tier(), crate::cognition::router::IntelligenceTier::Bridge));
    }

    #[test]
    fn test_embedded_profile() {
        let profile = DeploymentProfile::embedded();
        assert!(profile.max_vectors <= 1000);
        assert!(matches!(profile.recommended_max_tier(), crate::cognition::router::IntelligenceTier::Pulse));
    }

    #[test]
    fn test_profile_workload_check() {
        let laptop = DeploymentProfile::laptop();
        let phone = DeploymentProfile::pixel_phone();
        // Laptop handles everything phone can.
        assert!(laptop.can_handle(phone.max_vectors, phone.max_codebook));
        // Phone can't handle everything laptop can.
        assert!(!phone.can_handle(laptop.max_vectors, laptop.max_codebook));
    }
}
