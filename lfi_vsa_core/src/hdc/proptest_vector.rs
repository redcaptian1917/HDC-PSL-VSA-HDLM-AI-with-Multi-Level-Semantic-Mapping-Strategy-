// ============================================================
// Property-Based Tests for HDC Vector Operations
// AVP-2 Tier 6: Algebraic property verification
//
// PURPOSE: Verify that bind, bundle, and permute satisfy
// their mathematical invariants across random inputs.
// ============================================================

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use crate::hdc::vector::BipolarVector;

    /// Helper: create a deterministic random vector from a seed
    fn vec_from_seed(seed: u64) -> BipolarVector {
        BipolarVector::from_seed(seed)
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(500))]

        /// Bind is self-inverse: A XOR A = identity (all zeros in XOR = same vector)
        /// More precisely: bind(A, bind(A, B)) ≈ B
        #[test]
        fn bind_is_self_inverse(seed_a in 0u64..1000000, seed_b in 0u64..1000000) {
            let a = vec_from_seed(seed_a);
            let b = vec_from_seed(seed_b);

            let ab = a.bind(&b).unwrap();
            let result = a.bind(&ab).unwrap();

            // result should equal b (XOR is self-inverse)
            let distance = result.hamming_distance(&b).unwrap();
            prop_assert_eq!(distance, 0,
                "bind(A, bind(A, B)) should equal B, hamming distance = {}", distance);
        }

        /// Bind preserves dimensionality
        #[test]
        fn bind_preserves_dim(seed_a in 0u64..1000000, seed_b in 0u64..1000000) {
            let a = vec_from_seed(seed_a);
            let b = vec_from_seed(seed_b);
            let result = a.bind(&b).unwrap();
            prop_assert_eq!(result.dim(), a.dim());
        }

        /// Bind is commutative: A XOR B = B XOR A
        #[test]
        fn bind_is_commutative(seed_a in 0u64..1000000, seed_b in 0u64..1000000) {
            let a = vec_from_seed(seed_a);
            let b = vec_from_seed(seed_b);
            let ab = a.bind(&b).unwrap();
            let ba = b.bind(&a).unwrap();
            let distance = ab.hamming_distance(&ba).unwrap();
            prop_assert_eq!(distance, 0, "bind should be commutative");
        }

        /// Permute preserves dimensionality
        #[test]
        fn permute_preserves_dim(seed in 0u64..1000000, shift in 0usize..10000) {
            let v = vec_from_seed(seed);
            let result = v.permute(shift).unwrap();
            prop_assert_eq!(result.dim(), v.dim());
        }

        /// Permute by 0 is identity
        #[test]
        fn permute_zero_is_identity(seed in 0u64..1000000) {
            let v = vec_from_seed(seed);
            let result = v.permute(0).unwrap();
            let distance = v.hamming_distance(&result).unwrap();
            prop_assert_eq!(distance, 0, "permute(0) should be identity");
        }

        /// Permute by full dimension is identity (cyclic)
        #[test]
        fn permute_full_cycle_is_identity(seed in 0u64..1000000) {
            let v = vec_from_seed(seed);
            let dim = v.dim();
            let result = v.permute(dim).unwrap();
            let distance = v.hamming_distance(&result).unwrap();
            prop_assert_eq!(distance, 0,
                "permute(dim) should be identity (full cycle), distance = {}", distance);
        }

        /// Hamming distance is symmetric: d(A,B) = d(B,A)
        #[test]
        fn hamming_is_symmetric(seed_a in 0u64..1000000, seed_b in 0u64..1000000) {
            let a = vec_from_seed(seed_a);
            let b = vec_from_seed(seed_b);
            let d_ab = a.hamming_distance(&b).unwrap();
            let d_ba = b.hamming_distance(&a).unwrap();
            prop_assert_eq!(d_ab, d_ba, "hamming distance should be symmetric");
        }

        /// Hamming distance to self is always 0
        #[test]
        fn hamming_self_is_zero(seed in 0u64..1000000) {
            let v = vec_from_seed(seed);
            let d = v.hamming_distance(&v).unwrap();
            prop_assert_eq!(d, 0, "distance to self should be 0");
        }

        /// Bundle of a single vector returns that vector
        #[test]
        fn bundle_single_is_identity(seed in 0u64..1000000) {
            let v = vec_from_seed(seed);
            let result = BipolarVector::bundle(&[&v]).unwrap();
            let distance = v.hamming_distance(&result).unwrap();
            prop_assert_eq!(distance, 0, "bundle of single vector should be identity");
        }

        /// Bundle result has correct dimensionality
        #[test]
        fn bundle_preserves_dim(seed_a in 0u64..1000000, seed_b in 0u64..1000000, seed_c in 0u64..1000000) {
            let a = vec_from_seed(seed_a);
            let b = vec_from_seed(seed_b);
            let c = vec_from_seed(seed_c);
            let result = BipolarVector::bundle(&[&a, &b, &c]).unwrap();
            prop_assert_eq!(result.dim(), a.dim());
        }

        /// Random vectors are approximately orthogonal (hamming ≈ dim/2)
        #[test]
        fn random_vectors_near_orthogonal(seed_a in 0u64..500000, seed_b in 500000u64..1000000) {
            let a = vec_from_seed(seed_a);
            let b = vec_from_seed(seed_b);
            let d = a.hamming_distance(&b).unwrap();
            let dim = a.dim();
            // In high dimensions, random vectors should have hamming ≈ dim/2
            // Allow ±10% tolerance
            let expected = dim / 2;
            let tolerance = dim / 10;
            prop_assert!(
                d > expected - tolerance && d < expected + tolerance,
                "Random vectors should be near-orthogonal: distance {} vs expected {} (dim={})",
                d, expected, dim
            );
        }
    }
}
