//! Batch implementation for `accumulation_distribution_oscillator`.

use super::volume_states::*;
use crate::error::{TaError, TaResult};

/// Compute the accumulation distribution oscillator result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `close` - Input series or configuration value.
/// * `volume` - Input series or configuration value.
/// * `fastperiod` - Input series or configuration value.
/// * `slowperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn accumulation_distribution_oscillator(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    volume: &[f64],
    fastperiod: usize,
    slowperiod: usize,
) -> TaResult<Vec<f64>> {
    if high.len() != low.len() || high.len() != close.len() || high.len() != volume.len() {
        return Err(crate::TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().min(close.len()).min(volume.len()),
        });
    }
    let mut state = AccumulationDistributionOscillator::new(fastperiod, slowperiod)?;
    Ok(high
        .iter()
        .zip(low)
        .zip(close)
        .zip(volume)
        .map(|(((&high, &low), &close), &volume)| {
            state.append(high, low, close, volume).unwrap_or(f64::NAN)
        })
        .collect())
}
use super::*;

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
