//! Stateful volume and volume-derived streaming indicators.

use crate::error::{TaError, TaResult};

use super::{invalid_period, vhgw, MonotonicMax, MonotonicMin};

pub(crate) fn ad_increment(high: f64, low: f64, close: f64, volume: f64) -> f64 {
    let range = high - low;
    if range > 0.0 {
        ((close - low) - (high - close)) / range * volume
    } else {
        0.0
    }
}

/// Stateful Chaikin accumulation/distribution line.
#[derive(Debug, Clone, Default)]
/// Persistent Rust state or aligned output type for `AccumulationDistribution`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct AccumulationDistribution {
    total: f64,
    value: Option<f64>,
}

impl AccumulationDistribution {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self::default()
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> f64 {
        self.total += ad_increment(high, low, close, volume);
        self.value = Some(self.total);
        self.total
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.total = 0.0;
        self.value = None;
    }
}

/// Stateful Chaikin A/D oscillator with first-value EMA seeds.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `AccumulationDistributionOscillator`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct AccumulationDistributionOscillator {
    lookback: usize,
    index: usize,
    fast_k: f64,
    slow_k: f64,
    ad: f64,
    fast_ema: Option<f64>,
    slow_ema: Option<f64>,
    value: Option<f64>,
}

impl AccumulationDistributionOscillator {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(fast_period: usize, slow_period: usize) -> TaResult<Self> {
        if fast_period < 2 || slow_period < 2 {
            return Err(TaError::InvalidParameter {
                name: "fastperiod/slowperiod",
                value: format!("{fast_period}/{slow_period}"),
                reason: "both periods must be >= 2",
            });
        }
        Ok(Self {
            lookback: fast_period.max(slow_period) - 1,
            index: 0,
            fast_k: 2.0 / (fast_period as f64 + 1.0),
            slow_k: 2.0 / (slow_period as f64 + 1.0),
            ad: 0.0,
            fast_ema: None,
            slow_ema: None,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> Option<f64> {
        self.ad += ad_increment(high, low, close, volume);
        match (self.fast_ema, self.slow_ema) {
            (Some(fast), Some(slow)) => {
                self.fast_ema = Some(self.fast_k.mul_add(self.ad - fast, fast));
                self.slow_ema = Some(self.slow_k.mul_add(self.ad - slow, slow));
            }
            _ => {
                self.fast_ema = Some(self.ad);
                self.slow_ema = Some(self.ad);
            }
        }
        if self.index >= self.lookback {
            self.value = Some(
                self.fast_ema.expect("fast EMA is initialized")
                    - self.slow_ema.expect("slow EMA is initialized"),
            );
        }
        self.index += 1;
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

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.index = 0;
        self.ad = 0.0;
        self.fast_ema = None;
        self.slow_ema = None;
        self.value = None;
    }
}

/// Stateful on-balance volume.
#[derive(Debug, Clone, Default)]
/// Persistent Rust state or aligned output type for `OnBalanceVolume`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct OnBalanceVolume {
    previous_close: Option<f64>,
    total: f64,
    value: Option<f64>,
}

impl OnBalanceVolume {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self::default()
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, close: f64, volume: f64) -> f64 {
        match self.previous_close.replace(close) {
            None => self.total = volume,
            Some(previous) if close > previous => self.total += volume,
            Some(previous) if close < previous => self.total -= volume,
            Some(_) => {}
        }
        self.value = Some(self.total);
        self.total
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.total = 0.0;
        self.value = None;
    }
}

/// Stateful balance of power.
#[derive(Debug, Clone, Default)]
/// Persistent Rust state or aligned output type for `BalanceOfPower`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct BalanceOfPower {
    value: Option<f64>,
}

impl BalanceOfPower {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self::default()
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> f64 {
        let range = high - low;
        let value = if range > 0.0 {
            (close - open) / range
        } else {
            0.0
        };
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.value = None;
    }
}

/// Stateful Williams %R.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `WilliamsPercentR`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct WilliamsPercentR {
    highs: MonotonicMax,
    lows: MonotonicMin,
    value: Option<f64>,
}

impl WilliamsPercentR {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            highs: MonotonicMax::new(period)?,
            lows: MonotonicMin::new(period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let maximum = self.highs.append(high);
        let minimum = self.lows.append(low);
        self.value = maximum.zip(minimum).map(|(maximum, minimum)| {
            let range = maximum - minimum;
            if range > 0.0 {
                -100.0 * (maximum - close) / range
            } else {
                0.0
            }
        });
        self.value
    }

    /// Bulk kernel: one vHGW pass over `high` and one over `low`, then the
    /// per-bar %R arithmetic in a flat loop. The trailing `period` inputs are
    /// replayed to rebuild the monotonic deques, so outputs and post-run state
    /// are bit-identical to per-bar [`Self::append`]; warm-up bars are NaN.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        if high.len() != low.len() || high.len() != close.len() {
            return Err(TaError::LengthMismatch {
                expected: high.len(),
                got: low.len().min(close.len()),
            });
        }
        let n = high.len();
        let period = self.highs.period();
        if self.highs.count() != 0 || n < period {
            output.reserve(n);
            for index in 0..n {
                output.push(
                    self.append(high[index], low[index], close[index])
                        .unwrap_or(f64::NAN),
                );
            }
            return Ok(());
        }
        let start = output.len();
        output.resize(start + n, f64::NAN);
        let warm = start + period - 1;
        let mut lowest = vec![0.0_f64; n - (period - 1)];
        vhgw::sliding_max_into(high, period, &mut output[warm..]);
        vhgw::sliding_min_into(low, period, &mut lowest);
        for (offset, (slot, &minimum)) in output[warm..].iter_mut().zip(&lowest).enumerate() {
            let maximum = *slot;
            let range = maximum - minimum;
            *slot = if range > 0.0 {
                -100.0 * (maximum - close[period - 1 + offset]) / range
            } else {
                0.0
            };
        }
        self.highs.rebuild_from_full_run(high);
        self.lows.rebuild_from_full_run(low);
        self.value = output.last().copied();
        Ok(())
    }

    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.highs.reset();
        self.lows.reset();
        self.value = None;
    }
}

#[cfg(test)]
mod williams_bulk_tests {
    use super::*;

    fn lcg_series(len: usize, seed: u64) -> Vec<f64> {
        let mut state = seed;
        (0..len)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                ((state >> 33) % 100_003) as f64 / 101.0
            })
            .collect()
    }

    #[test]
    fn williams_percent_r_bulk_matches_append_bitwise() {
        let close = lcg_series(5_000, 0xA5A5_5A5A_1234_9876);
        let high: Vec<f64> = close.iter().map(|v| v + 1.25).collect();
        let low: Vec<f64> = close.iter().map(|v| v - 1.25).collect();
        // Degenerate variant with a zero range exercises the `range > 0` branch.
        let flat = vec![7.5_f64; 5_000];
        for (high, low, close) in [
            (high, low, close.clone()),
            (flat.clone(), flat.clone(), flat.clone()),
        ] {
            for period in [2usize, 5, 14, 30, 200] {
                let mut reference = WilliamsPercentR::new(period).unwrap();
                let expected: Vec<f64> = (0..close.len())
                    .map(|i| {
                        reference
                            .append(high[i], low[i], close[i])
                            .unwrap_or(f64::NAN)
                    })
                    .collect();
                for chunk in [1usize, 7, 97, close.len()] {
                    let mut state = WilliamsPercentR::new(period).unwrap();
                    let mut out = Vec::new();
                    let mut offset = 0;
                    while offset < close.len() {
                        let end = (offset + chunk).min(close.len());
                        state
                            .extend_slices_into(
                                &high[offset..end],
                                &low[offset..end],
                                &close[offset..end],
                                &mut out,
                            )
                            .unwrap();
                        offset = end;
                    }
                    assert_eq!(out.len(), close.len());
                    for (i, e) in expected.iter().enumerate() {
                        assert_eq!(
                            e.to_bits(),
                            out[i].to_bits(),
                            "p={period} chunk={chunk} i={i}"
                        );
                    }
                    let mut follow = reference.clone();
                    for i in 0..256 {
                        assert_eq!(
                            follow.append(high[i], low[i], close[i]),
                            state.append(high[i], low[i], close[i]),
                            "continue p={period} chunk={chunk}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn williams_percent_r_bulk_validates_lengths() {
        let mut state = WilliamsPercentR::new(5).unwrap();
        let mut out = Vec::new();
        assert!(state
            .extend_slices_into(&[1.0, 2.0], &[1.0], &[1.0, 2.0], &mut out)
            .is_err());
    }
}
