//! Batch implementation for `rolling_skew`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `rolling_skew` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the rolling skew result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_skew(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = RollingSkew::new(timeperiod)?;
    Ok(input
        .iter()
        .map(|&value| state.append(value).unwrap_or(f64::NAN))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// Pre-optimization `rolling_moment_operator!` body (VecDeque window),
    /// kept verbatim as oracle for the fixed-ring rewrite.
    struct Reference {
        values: VecDeque<f64>,
        timeperiod: usize,
        nobs: usize,
        mean: f64,
        m2: f64,
        m3: f64,
        m4: f64,
    }

    impl Reference {
        fn new(timeperiod: usize) -> Self {
            Self {
                values: VecDeque::with_capacity(timeperiod),
                timeperiod,
                nobs: 0,
                mean: 0.0,
                m2: 0.0,
                m3: 0.0,
                m4: 0.0,
            }
        }

        fn append(&mut self, input: f64) -> Option<f64> {
            if self.values.len() == self.timeperiod {
                let old = self.values.pop_front().expect("full moment window");
                let n = (self.nobs - 1) as f64;
                let delta = old - self.mean;
                let delta_n = delta / n;
                let term1 = delta_n * delta * (n + 1.0);
                let old_m2 = self.m2;
                let old_m3 = self.m3;
                self.m4 += delta_n
                    * (4.0 * old_m3 + delta_n * (6.0 * old_m2 - term1 * (n * n + 3.0 * n + 3.0)));
                self.m3 = old_m3 - delta_n * (term1 * (n + 2.0) - 3.0 * old_m2);
                self.m2 = old_m2 - term1;
                self.mean -= delta_n;
                self.nobs -= 1;
            }
            self.values.push_back(input);
            let n_old = self.nobs as f64;
            let n = n_old + 1.0;
            let delta = input - self.mean;
            let delta_n = delta / n;
            let term1 = delta * delta_n * n_old;
            let old_m2 = self.m2;
            let old_m3 = self.m3;
            self.m4 += delta_n
                * (-4.0 * old_m3 + delta_n * (6.0 * old_m2 + term1 * (n * n - 3.0 * n + 3.0)));
            self.m3 += delta_n * (term1 * (n - 2.0) - 3.0 * old_m2);
            self.m2 = old_m2 + term1;
            self.mean += delta_n;
            self.nobs += 1;
            (self.nobs == self.timeperiod).then(|| {
                let n = self.nobs as f64;
                if self.m2 > 0.0 {
                    n.sqrt() * self.m3 / self.m2.powf(1.5)
                } else {
                    0.0
                }
            })
        }
    }

    fn lcg_series(len: usize, seed: u64) -> Vec<f64> {
        let mut state = seed;
        (0..len)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                ((state >> 11) as f64 / (1u64 << 53) as f64) * 4.0 - 2.0
            })
            .collect()
    }

    #[test]
    fn matches_reference_bitwise_and_survives_chunking() {
        let input = lcg_series(5_000, 0xA1_5EED_71);
        for period in [2usize, 3, 5, 30, 252] {
            let mut reference = Reference::new(period);
            let expected: Vec<f64> = input
                .iter()
                .map(|&v| reference.append(v).unwrap_or(f64::NAN))
                .collect();
            let mut state = RollingSkew::new(period).unwrap();
            for (i, want) in expected.iter().enumerate() {
                let got = state.append(input[i]).unwrap_or(f64::NAN);
                assert_eq!(want.to_bits(), got.to_bits(), "p={period} bar {i}");
            }
            state.reset();
            let mut fresh = Reference::new(period);
            for &v in input.iter().take(512) {
                let want = fresh.append(v).unwrap_or(f64::NAN);
                let got = state.append(v).unwrap_or(f64::NAN);
                assert_eq!(want.to_bits(), got.to_bits(), "p={period} post-reset");
            }
        }
    }

    #[test]
    fn batch_matches_streaming() {
        let input = lcg_series(1_000, 0xA2_5EED_72);
        let batch = rolling_skew(&input, 30).unwrap();
        let mut state = RollingSkew::new(30).unwrap();
        for (i, value) in batch.iter().enumerate() {
            let got = state.append(input[i]).unwrap_or(f64::NAN);
            assert_eq!(value.to_bits(), got.to_bits());
        }
    }
}
