// ============================================================
// #347 Precision-tier hypervector wrapper
//
// BipolarVector (10000-dim {-1,+1}) is fast but lossy: every sign()
// after bundling throws away the magnitude that disambiguates near-
// orthogonal sums. For most of the HDC pipeline that's fine — noise
// is expected, cosine similarity absorbs it.
//
// For cryptographic commitments, formal proofs, and consensus
// aggregation (#305 audit chain, #346 CRDT readouts, #182 Lean4
// proof-carrying inference), a single flipped bit is a correctness
// failure. For those paths we promote to TensorTrain, which is
// lossless under bind and bundle until an explicit truncation.
//
// The wrapper below is the thin dispatch layer:
//   Precision::Fast(bv)     — plain bipolar, used by RAG/chat
//   Precision::Precise(tt)  — tensor-train, used by critical paths
//
// promote() / demote() round-trip between them. bind() dispatches
// inside the same variant so a Fast∘Fast stays fast and a
// Precise∘Precise stays precise. Mixed calls are rejected rather than
// silently downgraded — the caller must explicitly promote or demote.
// ============================================================

use crate::hdc::vector::BipolarVector;
use crate::hdc::tensor_train::TensorTrain;
use crate::hdc::error::HdcError;

/// Tiered hypervector. Every public operation dispatches on the tier
/// and refuses to silently cross the boundary.
#[derive(Debug, Clone)]
pub enum Precision {
    /// 10000-bit bipolar, ~1.25 kB, fast bind/bundle/similarity.
    /// Lossy after aggregation.
    Fast(BipolarVector),
    /// TensorTrain over mode dims 10×10×10×10, lossless under bind/
    /// bundle until explicit truncation. Storage grows with rank.
    Precise(TensorTrain),
}

impl Precision {
    /// Promote a Fast vector to Precise. No-op if already Precise.
    pub fn promote(self) -> Result<Self, HdcError> {
        match self {
            Precision::Fast(bv) => {
                TensorTrain::from_bipolar(&bv)
                    .map(Precision::Precise)
                    .ok_or(HdcError::InitializationFailed {
                        reason: "TT from_bipolar failed".into(),
                    })
            }
            p @ Precision::Precise(_) => Ok(p),
        }
    }

    /// Demote a Precise vector to Fast. No-op if already Fast.
    /// This step IS lossy: the TT is sign-clipped back to bipolar.
    pub fn demote(self) -> Self {
        match self {
            p @ Precision::Fast(_) => p,
            Precision::Precise(tt) => Precision::Fast(tt.to_bipolar()),
        }
    }

    /// True if this vector is in the Precise tier.
    pub fn is_precise(&self) -> bool {
        matches!(self, Precision::Precise(_))
    }

    /// Bind two vectors. Both must be in the same tier; mixed calls
    /// return `Err(LogicFault)` so the caller can decide which tier to
    /// unify in rather than inherit a silent downgrade.
    pub fn bind(a: &Self, b: &Self) -> Result<Self, HdcError> {
        match (a, b) {
            (Precision::Fast(x), Precision::Fast(y)) => {
                x.bind(y).map(Precision::Fast)
            }
            (Precision::Precise(x), Precision::Precise(y)) => {
                TensorTrain::bind(x, y)
                    .map(Precision::Precise)
                    .ok_or(HdcError::LogicFault {
                        reason: "TT bind failed (likely mode-dim mismatch)".into(),
                    })
            }
            _ => Err(HdcError::LogicFault {
                reason: "Precision tiers must match; call promote/demote first".into(),
            }),
        }
    }

    /// Cosine similarity. Mixed tiers demote to Fast for the measurement
    /// — similarity is intrinsically a floating-point quantity and the
    /// lossy sign-clip is safe here (callers inspect the number, they
    /// don't aggregate from it).
    pub fn similarity(a: &Self, b: &Self) -> Result<f64, HdcError> {
        match (a, b) {
            (Precision::Fast(x), Precision::Fast(y)) => x.similarity(y),
            (Precision::Precise(x), Precision::Precise(y)) => {
                Ok(TensorTrain::cosine_similarity(x, y))
            }
            // Mixed — demote the Precise side to a temporary Fast for
            // the comparison. Doesn't mutate the originals.
            (Precision::Fast(x), Precision::Precise(y)) => {
                x.similarity(&y.to_bipolar())
            }
            (Precision::Precise(x), Precision::Fast(y)) => {
                x.to_bipolar().similarity(y)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn promote_then_demote_roundtrips_a_bipolar() {
        let bv = BipolarVector::from_seed(4242);
        let precise = Precision::Fast(bv.clone()).promote().unwrap();
        assert!(precise.is_precise());
        let back = precise.demote();
        match back {
            Precision::Fast(recovered) => {
                // Sign-preserving roundtrip must recover the exact bits.
                assert_eq!(recovered, bv);
            }
            _ => panic!("demote must produce Fast"),
        }
    }

    #[test]
    fn bind_fast_stays_fast() {
        let a = Precision::Fast(BipolarVector::from_seed(1));
        let b = Precision::Fast(BipolarVector::from_seed(2));
        let bound = Precision::bind(&a, &b).unwrap();
        assert!(!bound.is_precise());
    }

    #[test]
    fn bind_precise_stays_precise() {
        let a = Precision::Fast(BipolarVector::from_seed(10)).promote().unwrap();
        let b = Precision::Fast(BipolarVector::from_seed(20)).promote().unwrap();
        let bound = Precision::bind(&a, &b).unwrap();
        assert!(bound.is_precise());
    }

    #[test]
    fn bind_mixed_tiers_errors() {
        let a = Precision::Fast(BipolarVector::from_seed(100));
        let b = Precision::Fast(BipolarVector::from_seed(200)).promote().unwrap();
        assert!(Precision::bind(&a, &b).is_err());
    }

    #[test]
    fn similarity_self_near_one() {
        let a = Precision::Fast(BipolarVector::from_seed(9));
        let s = Precision::similarity(&a, &a).unwrap();
        assert!(s > 0.99, "cosine(v,v) should be ~1, got {}", s);
    }

    #[test]
    fn similarity_mixed_tiers_still_works() {
        // Fast vs Precise of the same underlying bits should match
        // near-perfectly because promote/demote is sign-preserving.
        let bv = BipolarVector::from_seed(55);
        let fast = Precision::Fast(bv.clone());
        let precise = fast.clone().promote().unwrap();
        let s = Precision::similarity(&fast, &precise).unwrap();
        assert!(s > 0.99, "mixed-tier self-cosine should be ~1, got {}", s);
    }
}
