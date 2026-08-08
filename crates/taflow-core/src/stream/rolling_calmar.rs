//! Batch implementation for `rolling_calmar`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `rolling_calmar` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the rolling calmar result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_calmar(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = RollingCalmar::new(timeperiod)?;
    Ok(input
        .iter()
        .map(|&value| state.append(value).unwrap_or(f64::NAN))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// Pre-optimization `rolling_risk_operator!(RollingCalmar, ...)` oracle.
    struct Reference {
        values: VecDeque<f64>,
        timeperiod: usize,
    }

    impl Reference {
        fn new(timeperiod: usize) -> Self {
            Self {
                values: VecDeque::with_capacity(timeperiod),
                timeperiod,
            }
        }

        fn append(&mut self, input: f64) -> Option<f64> {
            if self.values.len() == self.timeperiod {
                self.values.pop_front();
            }
            self.values.push_back(input);
            (self.values.len() == self.timeperiod).then(|| {
                let values = &self.values;
                let average = values.iter().sum::<f64>() / values.len() as f64;
                let mut peak = values[0];
                let mut drawdown: f64 = 0.0;
                for &value in values {
                    peak = peak.max(value);
                    drawdown = drawdown.min(if peak != 0.0 { value / peak - 1.0 } else { 0.0 });
                }
                if drawdown < 0.0 {
                    average / -drawdown
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
                ((state >> 11) as f64 / (1u64 << 53) as f64) * 0.04 - 0.02
            })
            .collect()
    }

    #[test]
    fn matches_reference_bitwise_and_survives_chunking() {
        let input = lcg_series(5_000, 0x80_5EED_51);
        for period in [1usize, 2, 5, 30, 252] {
            let mut reference = Reference::new(period);
            let expected: Vec<f64> = input
                .iter()
                .map(|&v| reference.append(v).unwrap_or(f64::NAN))
                .collect();
            let mut state = RollingCalmar::new(period).unwrap();
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
        let input = lcg_series(1_000, 0x81_5EED_52);
        let batch = rolling_calmar(&input, 30).unwrap();
        let mut state = RollingCalmar::new(30).unwrap();
        for (i, value) in batch.iter().enumerate() {
            let got = state.append(input[i]).unwrap_or(f64::NAN);
            assert_eq!(value.to_bits(), got.to_bits());
        }
    }
}
