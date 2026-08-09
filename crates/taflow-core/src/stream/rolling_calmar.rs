//! Batch implementation for `rolling_calmar`.

use super::operator_states::*;
use super::*;
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
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// Rolling Calmar ratio: window mean over the window's maximum drawdown.
///
/// Split out of `rolling_risk_operator!` because the maximum drawdown is
/// driven by a *prefix* maximum inside the window — no rolling-extrema
/// structure and no sliding sum can reproduce the series when the window
/// slides. The O(period) rescan is therefore inherent; what this version
/// removes is the deque (one contiguous ring slice instead) and the second
/// pass: the window sum is accumulated in the same oldest-to-newest order,
/// inside the drawdown loop, so the emitted ratio is bit-identical.
#[derive(Debug, Clone)]
pub struct RollingCalmar {
    values: ContiguousWindow,
    timeperiod: usize,
    value: Option<f64>,
}

impl RollingCalmar {
    /// Creates the state for a positive rolling window.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self {
            values: ContiguousWindow::new(timeperiod),
            timeperiod,
            value: None,
        })
    }

    /// Append one causal observation and return the latest result.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.values.push(input);
        self.value = self.values.is_full().then(|| {
            let window = self.values.window();
            let mut sum = 0.0;
            let mut peak = window[0];
            let mut drawdown: f64 = 0.0;
            for &value in window {
                sum += value;
                peak = peak.max(value);
                drawdown = drawdown.min(if peak != 0.0 { value / peak - 1.0 } else { 0.0 });
            }
            let average = sum / self.timeperiod as f64;
            if drawdown < 0.0 {
                average / -drawdown
            } else {
                0.0
            }
        });
        self.value
    }

    /// Return the latest computed result, if warm-up is complete.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.values.clear();
        self.value = None;
    }
}
