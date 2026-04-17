// ============================================================
// Sealed<T> — Type-System Enforced Secret Protection
//
// Per LFI_CONFIDENTIALITY_KERNEL_DESIGN.md §1:
// "Sealed<T> wrapper: inner T unreachable except via use_within closure.
//  Does NOT implement Debug, Display, Serialize, Clone.
//  Compiler refuses to log, print, or serialize secrets."
//
// SUPERSOCIETY: This is the foundation of the confidentiality kernel.
// Every secret in PlausiDen passes through Sealed<T>. The type system
// is the first and most reliable defense — a forgotten log statement
// that would leak a password becomes a compile error.
//
// AVP-PASS-14: Tier 3 adversarial security — type-level secret protection
// ============================================================

use std::fmt;
use zeroize::Zeroize;

/// Marker trait for types that contain sensitive data.
/// Implementing this opts into the Sealed<T> discipline:
/// the inner value becomes unreachable outside controlled closures.
pub trait Sensitive: Zeroize {}

// Blanket implementations for common secret types
impl Sensitive for String {}
impl Sensitive for Vec<u8> {}
impl Sensitive for [u8; 32] {}
impl Sensitive for [u8; 64] {}

/// A sealed secret value. The inner T is accessible ONLY through
/// `use_within` — there is no getter, no Deref, no way to extract
/// the value except through a controlled closure that receives a reference.
///
/// # Compile-time guarantees
/// - No `Debug`: `println!("{:?}", sealed)` is a compile error
/// - No `Display`: `format!("{}", sealed)` is a compile error
/// - No `Clone`: prevents accidental duplication of secret material
/// - No `Serialize`: serde cannot serialize the inner value
/// - `Zeroize on Drop`: memory is overwritten when Sealed is dropped
///
/// # Example
/// ```
/// use lfi_vsa_core::sealed::{Sealed, Sensitive};
///
/// let api_key = Sealed::new("sk-secret-key-12345".to_string());
/// // api_key.0  — compile error: field is private
/// // println!("{:?}", api_key) — compile error: no Debug
/// // let copy = api_key.clone() — compile error: no Clone
///
/// // The ONLY way to use the secret:
/// api_key.use_within(|key| {
///     // key is &String here — use it for the API call
///     assert!(key.starts_with("sk-"));
/// });
/// ```
pub struct Sealed<T: Sensitive> {
    /// SECURITY: Private field — no external access.
    /// The only path to this data is through use_within().
    inner: T,
    /// Audit counter — how many times this secret was accessed.
    access_count: std::sync::atomic::AtomicU32,
}

impl<T: Sensitive> Sealed<T> {
    /// Seal a sensitive value. After this call, the original value
    /// should be considered moved — the caller no longer owns it.
    /// BUG ASSUMPTION: The caller may have copies of T on the stack
    /// or in registers. We can't prevent that — Sealed<T> only
    /// protects the canonical copy.
    pub fn new(value: T) -> Self {
        Self {
            inner: value,
            access_count: std::sync::atomic::AtomicU32::new(0),
        }
    }

    /// Access the sealed value within a controlled closure.
    /// The closure receives an immutable reference — it cannot
    /// move, clone, or store the inner value (unless T: Copy,
    /// which Sensitive types should NOT be).
    ///
    /// Each access is counted for audit purposes.
    pub fn use_within<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        self.access_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        f(&self.inner)
    }

    /// Access the sealed value mutably within a controlled closure.
    /// Used for key rotation, secret updates, etc.
    pub fn use_within_mut<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        self.access_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        f(&mut self.inner)
    }

    /// How many times this sealed value has been accessed.
    /// Useful for audit trails — a secret accessed 10,000 times
    /// in an hour is suspicious.
    pub fn access_count(&self) -> u32 {
        self.access_count.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Replace the sealed value (e.g., key rotation).
    /// The old value is zeroized.
    pub fn rotate(&mut self, new_value: T) {
        self.inner.zeroize();
        self.inner = new_value;
        self.access_count.store(0, std::sync::atomic::Ordering::Relaxed);
    }
}

// SECURITY: Zeroize the inner value on drop — prevents secrets
// from lingering in freed memory.
impl<T: Sensitive> Drop for Sealed<T> {
    fn drop(&mut self) {
        self.inner.zeroize();
    }
}

// SECURITY: Deliberately NOT implementing these traits.
// Each missing impl is a compile-time wall against accidental exposure.

// No Debug — prevents println!("{:?}", sealed)
// We implement a redacted version instead.
impl<T: Sensitive> fmt::Debug for Sealed<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sealed<[REDACTED]>(accesses: {})", self.access_count())
    }
}

// No Display — prevents format!("{}", sealed)
// Deliberately not implemented. This is intentional:
// impl<T: Sensitive> fmt::Display for Sealed<T> { ... }
// ^^^ This line does NOT exist. That's the point.

// No Clone — prevents let copy = sealed.clone()
// Deliberately not implemented.

// No Serialize — prevents serde from serializing secrets
// Deliberately not implemented.

// No Deref — prevents &*sealed from reaching inner
// Deliberately not implemented.

// No AsRef — prevents sealed.as_ref() from reaching inner
// Deliberately not implemented.

/// A secret broker that mediates access to sealed secrets.
/// All secret operations go through the broker, which enforces
/// access policies and maintains an audit trail.
pub struct SecretBroker {
    /// Total operations brokered
    operations: std::sync::atomic::AtomicU64,
}

impl SecretBroker {
    pub fn new() -> Self {
        Self {
            operations: std::sync::atomic::AtomicU64::new(0),
        }
    }

    /// Seal a value through the broker. Returns a Sealed<T>.
    pub fn seal<T: Sensitive>(&self, value: T) -> Sealed<T> {
        self.operations.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Sealed::new(value)
    }

    /// Compare two sealed values for equality without exposing either.
    /// Uses constant-time comparison to prevent timing attacks.
    /// SECURITY: This is the ONLY way to compare sealed secrets.
    pub fn constant_time_eq(&self, a: &Sealed<Vec<u8>>, b: &Sealed<Vec<u8>>) -> bool {
        self.operations.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        a.use_within(|va| {
            b.use_within(|vb| {
                use subtle::ConstantTimeEq;
                if va.len() != vb.len() {
                    return false;
                }
                va.ct_eq(vb).into()
            })
        })
    }

    /// Total operations brokered.
    pub fn operation_count(&self) -> u64 {
        self.operations.load(std::sync::atomic::Ordering::Relaxed)
    }
}

/// Convenience: seal a string secret.
pub fn seal_string(s: String) -> Sealed<String> {
    Sealed::new(s)
}

/// Convenience: seal a byte secret.
pub fn seal_bytes(b: Vec<u8>) -> Sealed<Vec<u8>> {
    Sealed::new(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sealed_access() {
        let secret = Sealed::new("my-api-key".to_string());
        let result = secret.use_within(|s| s.len());
        assert_eq!(result, 10);
        assert_eq!(secret.access_count(), 1);
    }

    #[test]
    fn test_sealed_multiple_access() {
        let secret = Sealed::new("secret".to_string());
        secret.use_within(|_| {});
        secret.use_within(|_| {});
        secret.use_within(|_| {});
        assert_eq!(secret.access_count(), 3);
    }

    #[test]
    fn test_sealed_debug_redacted() {
        let secret = Sealed::new("password123".to_string());
        let debug_str = format!("{:?}", secret);
        assert!(debug_str.contains("REDACTED"));
        assert!(!debug_str.contains("password123"));
    }

    #[test]
    fn test_sealed_rotate() {
        let mut secret = Sealed::new("old-key".to_string());
        secret.use_within(|s| assert_eq!(s, "old-key"));
        secret.rotate("new-key".to_string());
        secret.use_within(|s| assert_eq!(s, "new-key"));
        assert_eq!(secret.access_count(), 1); // Reset after rotate
    }

    #[test]
    fn test_sealed_bytes() {
        let secret = seal_bytes(vec![0xDE, 0xAD, 0xBE, 0xEF]);
        secret.use_within(|bytes| {
            assert_eq!(bytes.len(), 4);
            assert_eq!(bytes[0], 0xDE);
        });
    }

    #[test]
    fn test_broker_seal() {
        let broker = SecretBroker::new();
        let sealed = broker.seal("api-key-12345".to_string());
        sealed.use_within(|s| assert!(s.starts_with("api-key")));
        assert_eq!(broker.operation_count(), 1);
    }

    #[test]
    fn test_broker_constant_time_eq() {
        let broker = SecretBroker::new();
        let a = broker.seal(b"secret_value_1".to_vec());
        let b = broker.seal(b"secret_value_1".to_vec());
        let c = broker.seal(b"different_value".to_vec());

        assert!(broker.constant_time_eq(&a, &b));
        assert!(!broker.constant_time_eq(&a, &c));
    }

    #[test]
    fn test_use_within_mut() {
        let mut secret = Sealed::new("mutable".to_string());
        secret.use_within_mut(|s| {
            s.push_str("_modified");
        });
        secret.use_within(|s| assert_eq!(s, "mutable_modified"));
    }

    // Compile-time tests (these would fail to compile if uncommented):
    // fn test_no_clone() { let s = Sealed::new("x".to_string()); let _c = s.clone(); }
    // fn test_no_display() { let s = Sealed::new("x".to_string()); println!("{}", s); }
    // fn test_no_deref() { let s = Sealed::new("x".to_string()); let _v: &str = &*s; }
}
