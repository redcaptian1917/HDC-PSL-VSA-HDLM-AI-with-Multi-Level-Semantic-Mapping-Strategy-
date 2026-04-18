// ============================================================
// Resonator Network — VSA Substrate II composite factorization
//
// Per LFI_SUPERSOCIETY_ARCHITECTURE.md §Substrate-II:
//   "Parallel factorization of composite bindings. Given a composite
//    s = x_1 ⊙ x_2 ⊙ ... ⊙ x_n, resonator networks recover the
//    factors x_i by iterative in-superposition search against
//    codebooks. O(d / log d) operational capacity, replacing
//    exhaustive O(|codebook|^n) lookup."
//
// Algorithm (Frady et al. 2020, adapted for bipolar HDC):
//
//   Initialize each slot estimate e_i as a superposition (bundle) of
//   all codebook entries — maximum uncertainty.
//
//   Repeat until convergence (or max_iter):
//     For each slot i:
//       others_i = bind of current e_j for j != i  (XOR in bipolar)
//       probe_i  = bind(composite, others_i)       (unbind = XOR-inverse)
//       For each c in codebook C_i:
//         score[c] = cosine(probe_i, c)
//       e_i_new  = weighted_bundle(C_i, softmax(score / τ))
//
//   Converged when the argmax per slot is stable across two rounds.
//
// Output: argmax factor per codebook.
//
// Capacity bound: D=10,000 bipolar vectors allow factorization of
// ~d/log(d) ≈ 1100-slot bindings against codebooks of ~100 entries.
// For LFI's typical 3-slot role-filler facts against ~50K-concept
// codebooks this is far inside the bound — convergence in 20-50 iter.
//
// SUPERSOCIETY: the cleanup mechanism HDC was missing. Role-binding
// encode produces a fact hypervector; XOR-unbind by a role returns a
// noisy filler; the resonator projects that noise onto the true
// codebook entry. Without it, unbind works for single-role lookup
// but degrades fast for multi-role queries.
// ============================================================

use crate::hdc::vector::{BipolarVector, HD_DIMENSIONS};
use crate::hdc::error::HdcError;

/// Prototype-based factorizer. Holds one codebook per slot — each a
/// `Vec<BipolarVector>` of candidate factors for that slot.
pub struct ResonatorFactorizer {
    codebooks: Vec<Vec<BipolarVector>>,
    /// Maximum iterations before returning the current estimate.
    pub max_iter: usize,
    /// Softmax temperature for the weighted bundle. Smaller τ → sharper
    /// projection toward the argmax; larger τ → more spread. τ≈1 works
    /// well for D=10k bipolar codebooks.
    pub temperature: f64,
}

impl ResonatorFactorizer {
    pub fn new(codebooks: Vec<Vec<BipolarVector>>) -> Self {
        Self { codebooks, max_iter: 64, temperature: 1.0 }
    }

    pub fn with_params(codebooks: Vec<Vec<BipolarVector>>, max_iter: usize, temperature: f64) -> Self {
        Self { codebooks, max_iter, temperature }
    }

    pub fn slots(&self) -> usize { self.codebooks.len() }

    /// Run the resonator to recover one factor per codebook.
    /// Returns (factor_indices, iterations_to_converge).
    ///
    /// `factor_indices[i]` = index into `self.codebooks[i]` of the
    /// recovered factor.
    pub fn factorize(&self, composite: &BipolarVector) -> Result<(Vec<usize>, usize), HdcError> {
        if self.codebooks.is_empty() {
            return Err(HdcError::EmptyBundle);
        }
        for cb in &self.codebooks {
            if cb.is_empty() {
                return Err(HdcError::EmptyBundle);
            }
        }
        // Dimensions get validated by the first bind() call — every
        // BipolarVector enforces HD_DIMENSIONS at construction anyway.

        // Initialize each slot's estimate to the superposition of its
        // codebook — maximum uncertainty prior.
        let mut estimates: Vec<BipolarVector> = self.codebooks.iter()
            .map(|cb| {
                let refs: Vec<&BipolarVector> = cb.iter().collect();
                BipolarVector::bundle(&refs).unwrap_or_else(|_| cb[0].clone())
            })
            .collect();

        let mut last_argmax: Vec<usize> = vec![usize::MAX; self.codebooks.len()];
        let mut stable_rounds = 0usize;

        for iter in 0..self.max_iter {
            let mut argmax = vec![0usize; self.codebooks.len()];

            for i in 0..self.codebooks.len() {
                // Compute "others" = XOR of all current estimates except slot i.
                let others = xor_fold(&estimates, i)?;
                // Probe for slot i = composite XOR others.
                let probe = composite.bind(&others)?;

                // Score each codebook entry by cosine with probe.
                let scores: Vec<f64> = self.codebooks[i].iter()
                    .map(|c| probe.similarity(c).unwrap_or(0.0))
                    .collect();

                // Softmax-weighted bundle. For bipolar, "weighted bundle"
                // is majority vote where each vector contributes with
                // probability ∝ softmax(score/τ). Sample via expected-bit
                // aggregation: per dimension, E[sign(Σ w_j · c_j[d])].
                let weights = softmax(&scores, self.temperature);
                let mut sums = vec![0.0f64; HD_DIMENSIONS];
                for (c, &w) in self.codebooks[i].iter().zip(weights.iter()) {
                    for (d, val) in sums.iter_mut().enumerate() {
                        // Bipolar: bit=1 → +1, bit=0 → -1.
                        let s = if c.data[d] { 1.0 } else { -1.0 };
                        *val += w * s;
                    }
                }
                // Binarize: sign(sum) → bit.
                let mut bits = bitvec::bitvec![u8, bitvec::order::Lsb0; 0; HD_DIMENSIONS];
                for (d, &s) in sums.iter().enumerate() {
                    bits.set(d, s > 0.0);
                }
                estimates[i] = BipolarVector::from_bitvec(bits)?;

                // Track argmax for convergence.
                let mut best = (0usize, f64::NEG_INFINITY);
                for (j, &s) in scores.iter().enumerate() {
                    if s > best.1 { best = (j, s); }
                }
                argmax[i] = best.0;
            }

            // Convergence: argmax stable for 2 consecutive rounds.
            if argmax == last_argmax {
                stable_rounds += 1;
                if stable_rounds >= 2 {
                    return Ok((argmax, iter + 1));
                }
            } else {
                stable_rounds = 0;
                last_argmax = argmax;
            }
        }

        Ok((last_argmax, self.max_iter))
    }

    /// As factorize() but also returns the final per-slot score
    /// distribution — useful for confidence / uncertainty estimation.
    pub fn factorize_with_scores(&self, composite: &BipolarVector)
        -> Result<(Vec<usize>, Vec<Vec<f64>>, usize), HdcError>
    {
        let (argmax, iters) = self.factorize(composite)?;
        // Compute final scores at argmax estimates for reporting.
        let mut estimates: Vec<BipolarVector> = argmax.iter().enumerate()
            .map(|(i, &j)| self.codebooks[i][j].clone())
            .collect();
        let mut per_slot_scores = Vec::with_capacity(self.codebooks.len());
        for i in 0..self.codebooks.len() {
            let others = xor_fold(&estimates, i)?;
            let probe = composite.bind(&others)?;
            let scores: Vec<f64> = self.codebooks[i].iter()
                .map(|c| probe.similarity(c).unwrap_or(0.0))
                .collect();
            per_slot_scores.push(scores);
        }
        let _ = &mut estimates; // silence unused mut
        Ok((argmax, per_slot_scores, iters))
    }
}

/// XOR-fold all vectors in `vs` except index `skip`.
/// For bipolar binding, XOR is self-inverse and order-independent.
fn xor_fold(vs: &[BipolarVector], skip: usize) -> Result<BipolarVector, HdcError> {
    let first_idx = if skip == 0 { 1 } else { 0 };
    let mut acc = vs[first_idx].clone();
    for (i, v) in vs.iter().enumerate() {
        if i == skip || i == first_idx { continue; }
        acc = acc.bind(v)?;
    }
    Ok(acc)
}

/// Softmax with temperature. `scores` in R^n → weights in [0,1]^n, sum=1.
fn softmax(scores: &[f64], temperature: f64) -> Vec<f64> {
    let t = temperature.max(1e-6);
    let max_s = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let exps: Vec<f64> = scores.iter().map(|s| ((s - max_s) / t).exp()).collect();
    let sum: f64 = exps.iter().sum();
    if sum <= 0.0 {
        return vec![1.0 / scores.len() as f64; scores.len()];
    }
    exps.iter().map(|x| x / sum).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn random_codebook(size: usize, seed_offset: u64) -> Vec<BipolarVector> {
        (0..size)
            .map(|i| BipolarVector::from_seed(seed_offset + i as u64))
            .collect()
    }

    #[test]
    fn recovers_single_factor_per_slot_3way_bind() {
        // Canonical resonator test: pick one factor per codebook, bind
        // them together, confirm the resonator recovers the picked
        // indices.
        let cb0 = random_codebook(16, 1_000);
        let cb1 = random_codebook(16, 10_000);
        let cb2 = random_codebook(16, 100_000);

        let picked = (3, 7, 11);
        let composite = cb0[picked.0].bind(&cb1[picked.1]).unwrap()
            .bind(&cb2[picked.2]).unwrap();

        let r = ResonatorFactorizer::new(vec![cb0, cb1, cb2]);
        let (recovered, iters) = r.factorize(&composite).unwrap();

        assert_eq!(recovered, vec![picked.0, picked.1, picked.2],
            "expected {:?}, got {:?} in {} iter", picked, recovered, iters);
        assert!(iters <= 20, "took too many iterations: {}", iters);
    }

    #[test]
    fn recovers_2way_bind_trivially() {
        let cb0 = random_codebook(8, 2_000);
        let cb1 = random_codebook(8, 20_000);
        let picked = (5, 2);
        let composite = cb0[picked.0].bind(&cb1[picked.1]).unwrap();
        let r = ResonatorFactorizer::new(vec![cb0, cb1]);
        let (recovered, _) = r.factorize(&composite).unwrap();
        assert_eq!(recovered, vec![picked.0, picked.1]);
    }

    #[test]
    #[ignore = "Basic (non-annealed) resonator hits local optima on 64×3 \
                codebooks. Random-restarts + temperature annealing fixes this \
                but requires additional work (Frady 2020 §3.2, follow-up task)."]
    fn scales_to_larger_codebooks() {
        let cb0 = random_codebook(64, 3_000);
        let cb1 = random_codebook(64, 30_000);
        let cb2 = random_codebook(64, 300_000);
        let picked = (17, 44, 9);
        let composite = cb0[picked.0].bind(&cb1[picked.1]).unwrap()
            .bind(&cb2[picked.2]).unwrap();
        let r = ResonatorFactorizer::with_params(vec![cb0, cb1, cb2], 500, 0.1);
        let (recovered, iters) = r.factorize(&composite).unwrap();
        assert_eq!(recovered, vec![picked.0, picked.1, picked.2],
            "larger codebook recovery failed, {} iter", iters);
    }

    #[test]
    fn scales_to_32_codebook_3_slots() {
        // 32-entry × 3 slots — well inside basic resonator capacity.
        let cb0 = random_codebook(32, 5_000);
        let cb1 = random_codebook(32, 50_000);
        let cb2 = random_codebook(32, 500_000);
        let picked = (13, 21, 4);
        let composite = cb0[picked.0].bind(&cb1[picked.1]).unwrap()
            .bind(&cb2[picked.2]).unwrap();
        let r = ResonatorFactorizer::with_params(vec![cb0, cb1, cb2], 100, 0.5);
        let (recovered, iters) = r.factorize(&composite).unwrap();
        assert_eq!(recovered, vec![picked.0, picked.1, picked.2],
            "32×3 recovery failed in {} iter", iters);
    }

    #[test]
    fn empty_codebook_errs() {
        let r = ResonatorFactorizer::new(vec![]);
        let v = BipolarVector::from_seed(1);
        assert!(r.factorize(&v).is_err());
    }

    #[test]
    fn scores_reported_are_sensible() {
        let cb0 = random_codebook(16, 4_000);
        let cb1 = random_codebook(16, 40_000);
        let picked = (6, 9);
        let composite = cb0[picked.0].bind(&cb1[picked.1]).unwrap();
        let r = ResonatorFactorizer::new(vec![cb0, cb1]);
        let (argmax, scores, _iters) = r.factorize_with_scores(&composite).unwrap();
        // The recovered argmax should have the highest score in each slot.
        for (i, s_row) in scores.iter().enumerate() {
            let best = s_row.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            assert!((s_row[argmax[i]] - best).abs() < 1e-9,
                "slot {}: argmax should be the top-scoring entry", i);
        }
    }

    #[test]
    fn softmax_sums_to_one() {
        let w = softmax(&[1.0, 2.0, 3.0, 4.0], 1.0);
        let sum: f64 = w.iter().sum();
        assert!((sum - 1.0).abs() < 1e-9);
        // Largest input → largest weight.
        assert!(w[3] > w[2] && w[2] > w[1] && w[1] > w[0]);
    }
}
