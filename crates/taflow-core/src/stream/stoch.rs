//! Stateful Stochastic Oscillator.
//!
//! STOCH maintains rolling high/low extrema for fast %K, then feeds each
//! warmed value through independently selectable slow-%K and slow-%D moving
//! averages.

use crate::error::{TaError, TaResult};
use crate::ma_type::MaType;

use super::{moving_average::MovingAverageDispatcher, RollingMax, RollingMin, StreamingIndicator};

/// One aligned slow %K and slow %D observation.
#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `StochasticOscillatorValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct StochasticOscillatorValue {
    pub slowk: f64,
    pub slowd: f64,
}

/// Incremental STOCH with amortized constant work per bar.
/// Persistent Rust state or aligned output type for `StochasticOscillator`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct StochasticOscillator {
    highest: RollingMax,
    lowest: RollingMin,
    slowk: MovingAverageDispatcher,
    slowd: MovingAverageDispatcher,
    value: Option<StochasticOscillatorValue>,
}

impl StochasticOscillator {
    /// Creates a STOCH state for the selected smoothing types.
    pub fn new(
        fastk_period: usize,
        slowk_period: usize,
        slowk_matype: MaType,
        slowd_period: usize,
        slowd_matype: MaType,
    ) -> TaResult<Self> {
        Ok(Self {
            highest: RollingMax::new(fastk_period)?,
            lowest: RollingMin::new(fastk_period)?,
            slowk: MovingAverageDispatcher::new(slowk_period, slowk_matype)?,
            slowd: MovingAverageDispatcher::new(slowd_period, slowd_matype)?,
            value: None,
        })
    }

    /// Appends one high, low, and close bar.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<StochasticOscillatorValue> {
        let fastk =
            self.highest
                .append(high)
                .zip(self.lowest.append(low))
                .map(|(highest, lowest)| {
                    let divisor = (highest - lowest) / 100.0;
                    if divisor.abs() >= 1.0e-14 {
                        (close - lowest) / divisor
                    } else {
                        0.0
                    }
                });
        self.value = fastk
            .and_then(|fastk| self.slowk.append(fastk))
            .and_then(|slowk| {
                self.slowd
                    .append(slowk)
                    .map(|slowd| StochasticOscillatorValue { slowk, slowd })
            });
        self.value
    }

    /// Returns the latest warmed output.
    pub fn value(&self) -> Option<StochasticOscillatorValue> {
        self.value
    }

    /// Restores the post-construction state while retaining allocated buffers.
    pub fn reset(&mut self) {
        self.highest.reset();
        self.lowest.reset();
        self.slowk.reset();
        self.slowd.reset();
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_batch_for_all_moving_average_pairs() {
        let close: Vec<f64> = (0..500)
            .map(|index| 100.0 + (index as f64 * 0.17).sin() * 8.0 + index as f64 * 0.01)
            .collect();
        let high: Vec<f64> = close
            .iter()
            .enumerate()
            .map(|(index, close)| close + 1.0 + (index as f64 * 0.11).sin().abs())
            .collect();
        let low: Vec<f64> = close
            .iter()
            .enumerate()
            .map(|(index, close)| close - 1.0 - (index as f64 * 0.13).cos().abs())
            .collect();
        for slowk_code in 0..=8 {
            for slowd_code in 0..=8 {
                let slowk_type = MaType::try_from(slowk_code).unwrap();
                let slowd_type = MaType::try_from(slowd_code).unwrap();
                let expected = crate::stream::stochastic_oscillator(
                    &high, &low, &close, 5, 13, slowk_type, 11, slowd_type,
                )
                .unwrap();
                let mut state =
                    StochasticOscillator::new(5, 13, slowk_type, 11, slowd_type).unwrap();
                for index in 0..close.len() {
                    match state.append(high[index], low[index], close[index]) {
                        Some(actual) => {
                            assert!((actual.slowk - expected.0[index]).abs() < 1e-8);
                            assert!((actual.slowd - expected.1[index]).abs() < 1e-8);
                        }
                        None => assert!(expected.0[index].is_nan()),
                    }
                }
                let final_value = state.value();
                state.reset();
                for index in 0..close.len() {
                    state.append(high[index], low[index], close[index]);
                }
                assert_eq!(state.value(), final_value);
            }
        }
    }
}
// Batch Stochastic Oscillator.
//
// STOCH calculates fast %K from a rolling high/low range, then applies two
// independently selectable TA-Lib moving averages to produce slow %K and
// slow %D with their shared output alignment.

use crate::ma_type::compute_ma;

/// Computes aligned slow %K and slow %D output arrays.
///
/// # Parameters
///
/// * `high`, `low`, `close` - Equal-length chronological OHLC series.
/// * Period and moving-average parameters configure the stochastic windows.
///
/// # Returns
///
/// A pair of same-length slow %K and slow %D arrays with warm-up NaNs.
pub fn stochastic_oscillator(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    fastk_period: usize,
    slowk_period: usize,
    slowk_matype: MaType,
    slowd_period: usize,
    slowd_matype: MaType,
) -> TaResult<(Vec<f64>, Vec<f64>)> {
    let len = high.len();
    if len != low.len() || len != close.len() {
        return Err(TaError::LengthMismatch {
            expected: len,
            got: low.len().min(close.len()),
        });
    }
    if fastk_period < 1 || slowk_period < 1 || slowd_period < 1 {
        return Err(TaError::InvalidParameter {
            name: "periods",
            value: format!("{}/{}/{}", fastk_period, slowk_period, slowd_period),
            reason: "all periods must be >= 1",
        });
    }

    let slowk_lookback = slowk_matype.lookback(slowk_period);
    let slowd_lookback = slowd_matype.lookback(slowd_period);
    let lookback = fastk_period - 1 + slowk_lookback + slowd_lookback;
    if len <= lookback {
        return Err(TaError::InsufficientData {
            need: lookback + 1,
            got: len,
        });
    }

    let fastk_len = len - (fastk_period - 1);
    let mut fastk = Vec::with_capacity(fastk_len);
    for i in (fastk_period - 1)..len {
        let start = i + 1 - fastk_period;
        let mut hh = f64::NEG_INFINITY;
        let mut ll = f64::INFINITY;
        for j in start..=i {
            let h = high[j];
            let l = low[j];
            if h > hh {
                hh = h;
            }
            if l < ll {
                ll = l;
            }
        }
        let divisor = (hh - ll) / 100.0;
        if divisor.abs() >= 1.0e-14 {
            fastk.push((close[i] - ll) / divisor);
        } else {
            fastk.push(0.0);
        }
    }

    let slowk_arr = compute_ma(&fastk, slowk_period, slowk_matype)?;
    let slowk_valid = &slowk_arr[slowk_lookback..];
    let slowd_arr = compute_ma(slowk_valid, slowd_period, slowd_matype)?;

    let mut slowk_out = vec![f64::NAN; len];
    let mut slowd_out = vec![f64::NAN; len];
    for (offset, bar) in (lookback..len).enumerate() {
        let value_index = slowd_lookback + offset;
        slowk_out[bar] = slowk_valid[value_index];
        slowd_out[bar] = slowd_arr[value_index];
    }

    Ok((slowk_out, slowd_out))
}
