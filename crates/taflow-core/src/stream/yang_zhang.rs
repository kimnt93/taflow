//! Batch implementation for `yang_zhang`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `yang_zhang` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn yang_zhang(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
    timeperiod: usize,
) -> TaResult<Vec<f64>> {
    if open.len() != high.len() || high.len() != low.len() || low.len() != close.len() {
        return Err(TaError::LengthMismatch {
            expected: open.len(),
            got: high.len().max(low.len()).max(close.len()),
        });
    }
    let mut state = YangZhang::new(timeperiod)?;
    Ok(open
        .iter()
        .zip(high)
        .zip(low)
        .zip(close)
        .map(|(((&open, &high), &low), &close)| {
            state.append(open, high, low, close).unwrap_or(f64::NAN)
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// Pre-optimization rolling mean (`RollingMean`) used by the oracle.
    struct Mean {
        values: VecDeque<f64>,
        timeperiod: usize,
        sum: f64,
        value: Option<f64>,
    }

    impl Mean {
        fn new(timeperiod: usize) -> Self {
            Self {
                values: VecDeque::with_capacity(timeperiod),
                timeperiod,
                sum: 0.0,
                value: None,
            }
        }

        fn append(&mut self, input: f64) {
            if self.values.len() == self.timeperiod {
                self.sum -= self.values.pop_front().expect("ring is full");
            }
            self.values.push_back(input);
            self.sum += input;
            self.value = if self.values.len() == self.timeperiod {
                Some(self.sum / self.timeperiod as f64)
            } else {
                None
            };
        }
    }

    /// Pre-optimization `YangZhang::append` oracle (three separate means).
    struct Reference {
        overnight: Mean,
        open_close: Mean,
        rs: Mean,
        timeperiod: usize,
        previous_close: Option<f64>,
    }

    impl Reference {
        fn new(timeperiod: usize) -> Self {
            Self {
                overnight: Mean::new(timeperiod),
                open_close: Mean::new(timeperiod),
                rs: Mean::new(timeperiod),
                timeperiod,
                previous_close: None,
            }
        }

        fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<f64> {
            let previous_close = self.previous_close.replace(close);
            if open > 0.0 && high > 0.0 && low > 0.0 && close > 0.0 {
                if let Some(previous_close) = previous_close {
                    if previous_close > 0.0 {
                        let overnight = (open / previous_close).ln().powi(2);
                        let open_close = (close / open).ln().powi(2);
                        let rs = (high / close).ln() * (high / open).ln()
                            + (low / close).ln() * (low / open).ln();
                        self.overnight.append(overnight);
                        self.open_close.append(open_close);
                        self.rs.append(rs);
                    }
                }
            }
            match (self.overnight.value, self.open_close.value, self.rs.value) {
                (Some(on), Some(oc), Some(rs)) => {
                    let n = self.timeperiod as f64;
                    let k = 0.34 / (1.34 + (n + 1.0) / (n - 1.0));
                    Some((on + k * oc + (1.0 - k) * rs).max(0.0).sqrt())
                }
                _ => None,
            }
        }
    }

    fn lcg_series(len: usize, seed: u64) -> Vec<f64> {
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
        let close = lcg_series(5_000, 0xC1_5EED_91);
        let open: Vec<f64> = close.iter().map(|v| v * 0.999).collect();
        let high: Vec<f64> = close.iter().map(|v| v + 0.8).collect();
        let low: Vec<f64> = close.iter().map(|v| v - 0.8).collect();
        // Non-positive bars must still be skipped exactly as before.
        let mut open = open;
        open[500] = -1.0;
        open[501] = 0.0;
        for period in [2usize, 3, 5, 20, 252] {
            let mut reference = Reference::new(period);
            let expected: Vec<f64> = (0..close.len())
                .map(|i| {
                    reference
                        .append(open[i], high[i], low[i], close[i])
                        .unwrap_or(f64::NAN)
                })
                .collect();
            let mut state = YangZhang::new(period).unwrap();
            for (i, want) in expected.iter().enumerate() {
                let got = state
                    .append(open[i], high[i], low[i], close[i])
                    .unwrap_or(f64::NAN);
                assert_eq!(want.to_bits(), got.to_bits(), "p={period} bar {i}");
            }
            state.reset();
            let mut fresh = Reference::new(period);
            for i in 0..512 {
                let want = fresh
                    .append(open[i], high[i], low[i], close[i])
                    .unwrap_or(f64::NAN);
                let got = state
                    .append(open[i], high[i], low[i], close[i])
                    .unwrap_or(f64::NAN);
                assert_eq!(want.to_bits(), got.to_bits(), "p={period} post-reset {i}");
            }
        }
    }

    #[test]
    fn batch_matches_streaming() {
        let close = lcg_series(1_000, 0xC2_5EED_92);
        let open: Vec<f64> = close.iter().map(|v| v * 1.001).collect();
        let high: Vec<f64> = close.iter().map(|v| v + 0.5).collect();
        let low: Vec<f64> = close.iter().map(|v| v - 0.5).collect();
        let batch = yang_zhang(&open, &high, &low, &close, 20).unwrap();
        let mut state = YangZhang::new(20).unwrap();
        for (i, value) in batch.iter().enumerate() {
            let got = state
                .append(open[i], high[i], low[i], close[i])
                .unwrap_or(f64::NAN);
            assert_eq!(value.to_bits(), got.to_bits());
        }
    }
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// Yang-Zhang volatility: `σ² = σ²_on + k·σ²_oc + (1−k)·σ²_RS` with
/// `k = 0.34/(1.34 + (n+1)/(n−1))`. Highest-efficiency estimator.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `YangZhang`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct YangZhang {
    /// One shared ring of `[overnight, open_close, rs]` triples: the three
    /// variance components are pushed and evicted together (M4), so they need
    /// one window, one length check and one eviction index instead of three.
    window: Box<[[f64; 3]]>,
    head: usize,
    len: usize,
    sums: [f64; 3],
    means: Option<[f64; 3]>,
    timeperiod: usize,
    /// `0.34 / (1.34 + (n + 1) / (n - 1))`, constant for the state's lifetime.
    k: f64,
    previous_close: Option<f64>,
    value: Option<f64>,
}

impl YangZhang {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        if timeperiod < 2 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: timeperiod.to_string(),
                reason: "must be >= 2 for Yang-Zhang",
            });
        }
        let n = timeperiod as f64;
        Ok(Self {
            window: vec![[0.0; 3]; timeperiod].into_boxed_slice(),
            head: 0,
            len: 0,
            sums: [0.0; 3],
            means: None,
            timeperiod,
            k: 0.34 / (1.34 + (n + 1.0) / (n - 1.0)),
            previous_close: None,
            value: None,
        })
    }

    /// Pushes one triple through the shared ring, keeping the per-component
    /// arithmetic order of the three former `RollingMean` states
    /// (`sum -= evicted`, then `sum += input`, then `sum / period`).
    #[inline]
    fn push(&mut self, sample: [f64; 3]) {
        let capacity = self.window.len();
        if self.len == capacity {
            let evicted = self.window[self.head];
            for component in 0..3 {
                self.sums[component] -= evicted[component];
            }
            self.window[self.head] = sample;
            self.head += 1;
            if self.head == capacity {
                self.head = 0;
            }
        } else {
            let mut tail = self.head + self.len;
            if tail >= capacity {
                tail -= capacity;
            }
            self.window[tail] = sample;
            self.len += 1;
        }
        for component in 0..3 {
            self.sums[component] += sample[component];
        }
        self.means = (self.len == capacity).then(|| {
            [
                self.sums[0] / self.timeperiod as f64,
                self.sums[1] / self.timeperiod as f64,
                self.sums[2] / self.timeperiod as f64,
            ]
        });
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<f64> {
        let previous_close = self.previous_close.replace(close);
        if open > 0.0 && high > 0.0 && low > 0.0 && close > 0.0 {
            if let Some(previous_close) = previous_close {
                if previous_close > 0.0 {
                    let overnight = (open / previous_close).ln().powi(2);
                    let open_close = (close / open).ln().powi(2);
                    let rs = (high / close).ln() * (high / open).ln()
                        + (low / close).ln() * (low / open).ln();
                    self.push([overnight, open_close, rs]);
                }
            }
        }
        let k = self.k;
        self.value = self
            .means
            .map(|[on, oc, rs]| (on + k * oc + (1.0 - k) * rs).max(0.0).sqrt());
        self.value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.head = 0;
        self.len = 0;
        self.sums = [0.0; 3];
        self.means = None;
        self.previous_close = None;
        self.value = None;
    }
}
