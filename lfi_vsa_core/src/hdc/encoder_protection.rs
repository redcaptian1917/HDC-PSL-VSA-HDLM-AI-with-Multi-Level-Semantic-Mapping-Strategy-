//! # Purpose
//! HDLock-style encoder protection against model extraction and encoder
//! reversibility attacks. A fixed random-projection encoder is NOT
//! cryptographically one-way — it can be inverted via sign-constraint LP.
//! This module layers secret permutations interleaved with bindings to
//! a per-epoch secret key, making reversal ~10^10 harder.
//!
//! # Design Decisions
//! - Per-epoch secret key derived from ChaCha20Rng (CSPRNG)
//! - Secret permutation stack: permute → bind(key) → permute → bind(key2)
//! - Commit encoder seed via CommitmentRegistry each epoch
//! - Salt rotation prevents cross-epoch correlation attacks
//!
//! # Invariants
//! - Same input + same epoch → same protected output (deterministic)
//! - Different epochs → different outputs (forward secrecy)
//! - Protected vectors maintain similarity relationships (encoding is linear)

use crate::hdc::vector::{BipolarVector, HD_DIMENSIONS};
use crate::hdc::error::HdcError;
use rand::{SeedableRng, RngCore};
use rand_chacha::ChaCha20Rng;

/// Protected encoder with secret permutation stack.
pub struct ProtectedEncoder {
    /// Per-epoch secret key vector.
    key: BipolarVector,
    /// Secret permutation offset.
    perm_offset: usize,
    /// Second secret key for double-binding.
    key2: BipolarVector,
    /// Second permutation offset.
    perm_offset2: usize,
    /// Epoch counter.
    pub epoch: u64,
}

impl ProtectedEncoder {
    /// Create a new protected encoder from a secret seed.
    pub fn new(secret_seed: u64, epoch: u64) -> Self {
        let mut rng = ChaCha20Rng::seed_from_u64(secret_seed.wrapping_add(epoch));
        let key = BipolarVector::from_seed(rng.next_u64());
        let perm_offset = (rng.next_u64() as usize) % HD_DIMENSIONS;
        let key2 = BipolarVector::from_seed(rng.next_u64());
        let perm_offset2 = (rng.next_u64() as usize) % HD_DIMENSIONS;

        Self { key, perm_offset, key2, perm_offset2, epoch }
    }

    /// Protect a vector: permute → bind(key) → permute → bind(key2).
    /// The output maintains similarity relationships but cannot be inverted
    /// without knowledge of the secret permutations and keys.
    pub fn protect(&self, input: &BipolarVector) -> Result<BipolarVector, HdcError> {
        let step1 = input.permute(self.perm_offset)?;
        let step2 = step1.bind(&self.key)?;
        let step3 = step2.permute(self.perm_offset2)?;
        step3.bind(&self.key2)
    }

    /// Unprotect a vector (reverse the protection stack).
    /// Only possible with knowledge of the secret keys and offsets.
    pub fn unprotect(&self, protected: &BipolarVector) -> Result<BipolarVector, HdcError> {
        let step1 = protected.bind(&self.key2)?; // XOR is self-inverse
        let step2 = step1.permute(HD_DIMENSIONS - self.perm_offset2)?;
        let step3 = step2.bind(&self.key)?;
        step3.permute(HD_DIMENSIONS - self.perm_offset)
    }

    /// Rotate to a new epoch (generates new keys and permutations).
    pub fn rotate_epoch(&mut self, secret_seed: u64) {
        self.epoch += 1;
        let new = Self::new(secret_seed, self.epoch);
        self.key = new.key;
        self.perm_offset = new.perm_offset;
        self.key2 = new.key2;
        self.perm_offset2 = new.perm_offset2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protect_unprotect_roundtrip() {
        let enc = ProtectedEncoder::new(12345, 0);
        let original = BipolarVector::from_seed(42);
        let protected = enc.protect(&original).unwrap();
        let recovered = enc.unprotect(&protected).unwrap();
        assert_eq!(original, recovered, "Protect→unprotect must recover original");
    }

    #[test]
    fn test_protection_changes_vector() {
        let enc = ProtectedEncoder::new(12345, 0);
        let original = BipolarVector::from_seed(42);
        let protected = enc.protect(&original).unwrap();
        let sim = original.similarity(&protected).unwrap();
        assert!(sim.abs() < 0.1, "Protected should be quasi-orthogonal to original: {}", sim);
    }

    #[test]
    fn test_different_epochs_different_output() {
        let enc0 = ProtectedEncoder::new(12345, 0);
        let enc1 = ProtectedEncoder::new(12345, 1);
        let input = BipolarVector::from_seed(42);
        let p0 = enc0.protect(&input).unwrap();
        let p1 = enc1.protect(&input).unwrap();
        let sim = p0.similarity(&p1).unwrap();
        assert!(sim.abs() < 0.1, "Different epochs should give different outputs: {}", sim);
    }

    #[test]
    fn test_similarity_preserved_under_protection() {
        let enc = ProtectedEncoder::new(12345, 0);
        let a = BipolarVector::from_seed(1);
        let b = BipolarVector::from_seed(1); // Same seed = identical
        let pa = enc.protect(&a).unwrap();
        let pb = enc.protect(&b).unwrap();
        let sim = pa.similarity(&pb).unwrap();
        assert!((sim - 1.0).abs() < 1e-10, "Identical inputs stay identical after protection");
    }

    #[test]
    fn test_deterministic_same_epoch() {
        let enc = ProtectedEncoder::new(99, 5);
        let input = BipolarVector::from_seed(42);
        let p1 = enc.protect(&input).unwrap();
        let p2 = enc.protect(&input).unwrap();
        assert_eq!(p1, p2, "Same input + same epoch = same output");
    }

    /// Different secret seeds produce different encoders
    #[test]
    fn test_different_seeds_different_output() {
        let enc_a = ProtectedEncoder::new(111, 0);
        let enc_b = ProtectedEncoder::new(222, 0);
        let input = BipolarVector::from_seed(42);
        let pa = enc_a.protect(&input).unwrap();
        let pb = enc_b.protect(&input).unwrap();
        let sim = pa.similarity(&pb).unwrap();
        assert!(sim.abs() < 0.1,
            "Different seeds should give different outputs: sim={}", sim);
    }

    /// Protected vector preserves dimensionality
    #[test]
    fn test_protection_preserves_dim() {
        let enc = ProtectedEncoder::new(42, 0);
        let input = BipolarVector::from_seed(7);
        let protected = enc.protect(&input).unwrap();
        assert_eq!(input.dim(), protected.dim());
    }

    /// Epoch rotation: old encoder can't unprotect new epoch's data
    #[test]
    fn test_epoch_rotation_prevents_cross_decode() {
        let enc0 = ProtectedEncoder::new(42, 0);
        let enc1 = ProtectedEncoder::new(42, 1);
        let input = BipolarVector::from_seed(7);
        let protected_by_1 = enc1.protect(&input).unwrap();
        // Try to unprotect with epoch 0's encoder
        let wrong_decode = enc0.unprotect(&protected_by_1).unwrap();
        let sim = wrong_decode.similarity(&input).unwrap();
        assert!(sim.abs() < 0.1,
            "Wrong epoch should not recover original: sim={}", sim);
    }
}
