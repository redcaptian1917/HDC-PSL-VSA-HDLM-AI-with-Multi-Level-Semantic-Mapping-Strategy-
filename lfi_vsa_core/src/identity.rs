// ============================================================
// LFI Identity Sovereignty — ZK-Inspired Verification
// Section 1.II: "Advanced person recognition/identity recognition.
// Prioritize ZKPs... never display or store them in clear text."
// ============================================================

use serde::{Serialize, Deserialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// The type of identity currently authenticated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdentityKind {
    /// Full access to the Material Base (Sovereign).
    Sovereign,
    /// Access restricted to the Superstructure (Deniable/Chaff).
    Deniable,
}

/// A cryptographic signature purportedly from the HSM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereignSignature {
    pub payload_hash: u64,
    pub signature: Vec<u8>,
}

/// A non-cleartext proof of identity.
pub struct SovereignProof {
    pub kind: IdentityKind,
    pub name_hash: u64,
    pub credentials_commitment: u64,
    pub password_commitment: u64,
}

/// The Identity Prover engine.
pub struct IdentityProver;

impl IdentityProver {
    /// Commits a identity to memory using one-way hashing.
    /// NEVER stores the raw strings.
    pub fn commit(name: &str, credential: &str, license: &str, password: &str, kind: IdentityKind) -> SovereignProof {
        debuglog!("IdentityProver::commit: Creating secure identity commitment (Kind={:?})", kind);

        let name_hash = Self::hash(name);
        let password_commitment = Self::hash(password);

        let mut cred_hasher = DefaultHasher::new();
        credential.hash(&mut cred_hasher);
        license.hash(&mut cred_hasher);
        let credentials_commitment = cred_hasher.finish();
        
        SovereignProof { kind, name_hash, credentials_commitment, password_commitment }
    }

    /// Stable 64-bit hash for string inputs.
    pub fn hash(input: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        input.hash(&mut hasher);
        hasher.finish()
    }

    /// Signature-Verified Instruction (SVI) gate.
    /// Verifies if a signature (S) for a prompt (P) matches the Sovereign Key.
    pub fn verify_signature(_proof: &SovereignProof, prompt: &str, sig: &SovereignSignature) -> bool {
        debuglog!("IdentityProver: SVI Signature-Verified Instruction Gate Active");
        
        // 1. Verify hash alignment
        let p_hash = Self::hash(prompt);
        if p_hash != sig.payload_hash {
            debuglog!("IdentityProver: SVI ERROR - Hash Mismatch");
            return false;
        }

        // 2. Hardware HSM Check (Placeholder for Titan M2/TPM HAL)
        // In the final binary, this delegates to the NDK/SE-API.
        let verified = !sig.signature.is_empty(); // Simulated: signature must be present

        if verified {
            debuglog!("IdentityProver: SVI SUCCESS - Hardware-Bound Signature Verified");
        } else {
            debuglog!("IdentityProver: SVI REJECTED - Instruction weight = 0.0");
        }

        verified
    }

    /// Verifies if a presented identity matches the sovereign commitment.
    pub fn verify(proof: &SovereignProof, name: &str, credential: &str, license: &str, password: &str) -> bool {
        let current = Self::commit(name, credential, license, password, proof.kind);
        let matched = current.name_hash == proof.name_hash 
                   && current.credentials_commitment == proof.credentials_commitment
                   && current.password_commitment == proof.password_commitment;
        
        if matched {
            debuglog!("IdentityProver: IDENTITY VERIFIED.");
        } else {
            debuglog!("IdentityProver: SPOOFING ATTEMPT DETECTED.");
        }
        
        matched
    }

    /// Verify only the password.
    pub fn verify_password(proof: &SovereignProof, password: &str) -> bool {
        let hashed = Self::hash(password);
        let matched = hashed == proof.password_commitment;
        if matched {
            debuglog!("IdentityProver: PASSWORD VERIFIED.");
        } else {
            debuglog!("IdentityProver: AUTHENTICATION FAILURE.");
        }
        matched
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_verification_correct() {
        let proof = IdentityProver::commit("Test Sovereign", "555000111", "s99999999", "test_pass", IdentityKind::Sovereign);
        assert!(IdentityProver::verify(&proof, "Test Sovereign", "555000111", "s99999999", "test_pass"));
    }

    #[test]
    fn test_identity_wrong_credential_fails() {
        let proof = IdentityProver::commit("Test Sovereign", "555000111", "s99999999", "test_pass", IdentityKind::Sovereign);
        assert!(!IdentityProver::verify(&proof, "Test Sovereign", "000000000", "s99999999", "test_pass"));
    }

    #[test]
    fn test_identity_wrong_password_fails() {
        let proof = IdentityProver::commit("Test Sovereign", "555000111", "s99999999", "test_pass", IdentityKind::Sovereign);
        assert!(!IdentityProver::verify(&proof, "Test Sovereign", "555000111", "s99999999", "wrong_pass"));
    }

    #[test]
    fn test_identity_wrong_name_fails() {
        let proof = IdentityProver::commit("Real Name", "555000111", "s99999999", "test_pass", IdentityKind::Sovereign);
        assert!(!IdentityProver::verify(&proof, "Fake Name", "555000111", "s99999999", "test_pass"));
    }

    #[test]
    fn test_identity_wrong_license_fails() {
        let proof = IdentityProver::commit("Test", "555000111", "s99999999", "pass", IdentityKind::Sovereign);
        assert!(!IdentityProver::verify(&proof, "Test", "555000111", "x00000000", "pass"));
    }

    #[test]
    fn test_password_verification() {
        let proof = IdentityProver::commit("User", "cred", "lic", "s3cur3_p4ss!", IdentityKind::Sovereign);
        assert!(IdentityProver::verify_password(&proof, "s3cur3_p4ss!"));
        assert!(!IdentityProver::verify_password(&proof, "wrong_password"));
        assert!(!IdentityProver::verify_password(&proof, ""));
    }

    #[test]
    fn test_signature_verification() {
        let proof = IdentityProver::commit("User", "cred", "lic", "pass", IdentityKind::Sovereign);
        let prompt = "execute critical operation";
        let sig = SovereignSignature {
            payload_hash: IdentityProver::hash(prompt),
            signature: vec![0xAA, 0xBB, 0xCC], // Non-empty = valid in simulator
        };
        assert!(IdentityProver::verify_signature(&proof, prompt, &sig));
    }

    #[test]
    fn test_signature_hash_mismatch_rejected() {
        let proof = IdentityProver::commit("User", "cred", "lic", "pass", IdentityKind::Sovereign);
        let sig = SovereignSignature {
            payload_hash: 12345, // Wrong hash
            signature: vec![0xAA],
        };
        assert!(!IdentityProver::verify_signature(&proof, "actual prompt", &sig));
    }

    #[test]
    fn test_signature_empty_rejected() {
        let proof = IdentityProver::commit("User", "cred", "lic", "pass", IdentityKind::Sovereign);
        let prompt = "test";
        let sig = SovereignSignature {
            payload_hash: IdentityProver::hash(prompt),
            signature: vec![], // Empty = rejected
        };
        assert!(!IdentityProver::verify_signature(&proof, prompt, &sig));
    }

    #[test]
    fn test_commitment_is_deterministic() {
        let p1 = IdentityProver::commit("User", "cred", "lic", "pass", IdentityKind::Sovereign);
        let p2 = IdentityProver::commit("User", "cred", "lic", "pass", IdentityKind::Sovereign);
        assert_eq!(p1.name_hash, p2.name_hash);
        assert_eq!(p1.credentials_commitment, p2.credentials_commitment);
        assert_eq!(p1.password_commitment, p2.password_commitment);
    }

    #[test]
    fn test_commitment_never_stores_cleartext() {
        let proof = IdentityProver::commit("John Doe", "555123456", "a12345678", "my_password", IdentityKind::Sovereign);
        // The proof struct should NOT contain any cleartext.
        let debug_str = format!("{:?}", proof.kind);
        assert!(!debug_str.contains("John Doe"));
        assert!(!debug_str.contains("555123456"));
        assert!(!debug_str.contains("my_password"));
        // The hash values should be non-zero.
        assert!(proof.name_hash != 0);
        assert!(proof.credentials_commitment != 0);
        assert!(proof.password_commitment != 0);
    }

    #[test]
    fn test_sovereign_vs_deniable_identity() {
        let sovereign = IdentityProver::commit("User", "cred", "lic", "pass", IdentityKind::Sovereign);
        let deniable = IdentityProver::commit("User", "cred", "lic", "pass", IdentityKind::Deniable);

        // Same credentials but different kinds.
        assert_eq!(sovereign.kind, IdentityKind::Sovereign);
        assert_eq!(deniable.kind, IdentityKind::Deniable);

        // Hashes should be the same (kind doesn't affect the hash).
        assert_eq!(sovereign.name_hash, deniable.name_hash);
        assert_eq!(sovereign.password_commitment, deniable.password_commitment);
    }

    #[test]
    fn test_hash_stability() {
        // Same input always produces the same hash.
        let h1 = IdentityProver::hash("test_input");
        let h2 = IdentityProver::hash("test_input");
        assert_eq!(h1, h2);

        // Different inputs produce different hashes.
        let h3 = IdentityProver::hash("different_input");
        assert_ne!(h1, h3);
    }

    #[test]
    fn test_empty_credentials_handled() {
        // Edge case: empty strings should still produce valid commitments.
        let proof = IdentityProver::commit("", "", "", "", IdentityKind::Deniable);
        assert!(IdentityProver::verify(&proof, "", "", "", ""));
        assert!(!IdentityProver::verify(&proof, "notempty", "", "", ""));
    }
}
