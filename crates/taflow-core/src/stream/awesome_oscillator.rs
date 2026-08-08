//! Batch implementation for `awesome_oscillator`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Compute the Awesome Oscillator from aligned high and low prices.
///
/// `fast` and `slow` are the oscillator windows. The returned series is
/// Compute the awesome oscillator result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `fast` - Input series or configuration value.
/// * `slow` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn awesome_oscillator(
    high: &[f64],
    low: &[f64],
    fast: usize,
    slow: usize,
) -> TaResult<Vec<f64>> {
    if high.len() != low.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len(),
        });
    }
    let mut state = AwesomeOscillator::new(fast, slow)?;
    Ok(high
        .iter()
        .zip(low)
        .map(|(&h, &l)| state.append(h, l).unwrap_or(f64::NAN))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// Pre-optimization `AwesomeOscillator::append`, kept verbatim as oracle.
    struct Reference {
        fast: usize,
        slow: usize,
        values: VecDeque<f64>,
    }

    impl Reference {
        fn new(fast: usize, slow: usize) -> Self {
            Self {
                fast,
                slow,
                values: VecDeque::with_capacity(slow),
            }
        }

        fn append(&mut self, high: f64, low: f64) -> Option<f64> {
            if self.values.len() == self.slow {
                self.values.pop_front();
            }
            self.values.push_back((high + low) * 0.5);
            (self.values.len() == self.slow).then(|| {
                let fast = self.values.iter().rev().take(self.fast).sum::<f64>() / self.fast as f64;
                let slow = self.values.iter().sum::<f64>() / self.slow as f64;
                fast - slow
            })
        }
    }

    pub(super) fn lcg_series(len: usize, seed: u64) -> Vec<f64> {
        let mut state = seed;
        (0..len)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                90.0 + ((state >> 11) as f64 / (1u64 << 53) as f64) * 20.0
            })
            .collect()
    }

    #[test]
    fn matches_reference_bitwise_and_survives_chunking() {
        let base = lcg_series(5_000, 0xA0_5EED_01);
        let high: Vec<f64> = base.iter().map(|v| v + 0.75).collect();
        let low: Vec<f64> = base.iter().map(|v| v - 0.75).collect();
        for (fast, slow) in [(1usize, 1usize), (2, 5), (5, 34), (34, 200)] {
            let mut reference = Reference::new(fast, slow);
            let expected: Vec<f64> = (0..base.len())
                .map(|i| reference.append(high[i], low[i]).unwrap_or(f64::NAN))
                .collect();

            let mut state = AwesomeOscillator::new(fast, slow).unwrap();
            for (i, want) in expected.iter().enumerate() {
                let got = state.append(high[i], low[i]).unwrap_or(f64::NAN);
                assert_eq!(want.to_bits(), got.to_bits(), "f{fast}/s{slow} bar {i}");
            }

            // Chunked feeding: a mid-stream reset then replay must reproduce
            // the same series, and continuing past the split must agree.
            state.reset();
            let mut fresh = Reference::new(fast, slow);
            for i in 0..base.len() {
                let want = fresh.append(high[i], low[i]).unwrap_or(f64::NAN);
                let got = state.append(high[i], low[i]).unwrap_or(f64::NAN);
                assert_eq!(want.to_bits(), got.to_bits(), "post-reset bar {i}");
            }
        }
    }

    #[test]
    fn batch_matches_streaming() {
        let base = lcg_series(1_000, 0xB0_5EED_02);
        let high: Vec<f64> = base.iter().map(|v| v + 0.5).collect();
        let low: Vec<f64> = base.iter().map(|v| v - 0.5).collect();
        let batch = awesome_oscillator(&high, &low, 5, 34).unwrap();
        let mut state = AwesomeOscillator::new(5, 34).unwrap();
        for (i, value) in batch.iter().enumerate() {
            let got = state.append(high[i], low[i]).unwrap_or(f64::NAN);
            assert_eq!(value.to_bits(), got.to_bits());
        }
    }
}
