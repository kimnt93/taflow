//! Stateful Fast Stochastic Oscillator.
//!
//! STOCHF maintains rolling high/low extrema for fast %K and feeds that value
//! into the selected incremental moving average for fast %D.

use crate::error::{TaError, TaResult};
use crate::ma_type::MaType;

use super::{moving_average::MovingAverageDispatcher, RollingMax, RollingMin, StreamingIndicator};

/// One aligned fast %K and fast %D observation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FastStochasticOscillatorValue {
    pub fastk: f64,
    pub fastd: f64,
}

/// Incremental STOCHF with amortized constant work per bar.
pub struct FastStochasticOscillator {
    highest: RollingMax,
    lowest: RollingMin,
    fastd: MovingAverageDispatcher,
    value: Option<FastStochasticOscillatorValue>,
}

impl FastStochasticOscillator {
    /// Creates a STOCHF state for the selected fast %D moving-average type.
    pub fn new(fastk_period: usize, fastd_period: usize, fastd_matype: MaType) -> TaResult<Self> {
        Ok(Self {
            highest: RollingMax::new(fastk_period)?,
            lowest: RollingMin::new(fastk_period)?,
            fastd: MovingAverageDispatcher::new(fastd_period, fastd_matype)?,
            value: None,
        })
    }

    /// Appends one high, low, and close bar.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<FastStochasticOscillatorValue> {
        let highest = self.highest.append(high);
        let lowest = self.lowest.append(low);
        let fastk = highest.zip(lowest).map(|(highest, lowest)| {
            let divisor = (highest - lowest) / 100.0;
            if divisor.abs() >= 1.0e-14 {
                (close - lowest) / divisor
            } else {
                0.0
            }
        });
        self.value = fastk.and_then(|fastk| {
            self.fastd
                .append(fastk)
                .map(|fastd| FastStochasticOscillatorValue { fastk, fastd })
        });
        self.value
    }

    /// Returns the latest warmed output.
    pub fn value(&self) -> Option<FastStochasticOscillatorValue> {
        self.value
    }

    /// Restores the post-construction state while retaining allocated buffers.
    pub fn reset(&mut self) {
        self.highest.reset();
        self.lowest.reset();
        self.fastd.reset();
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn matches_batch_for_all_moving_average_types() {
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
        for code in 0..=8 {
            let ma_type = MaType::try_from(code).unwrap();
            let expected = crate::stream::fast_stochastic_oscillator(&high, &low, &close, 5, 13, ma_type).unwrap();
            let mut state = FastStochasticOscillator::new(5, 13, ma_type).unwrap();
            for index in 0..close.len() {
                match state.append(high[index], low[index], close[index]) {
                    Some(actual) => {
                        assert!(
                            (actual.fastk - expected.0[index]).abs() < 1e-8,
                            "type {code}"
                        );
                        assert!(
                            (actual.fastd - expected.1[index]).abs() < 1e-8,
                            "type {code}"
                        );
                    }
                    None => assert!(expected.0[index].is_nan(), "type {code}"),
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
// Batch Fast Stochastic Oscillator.
//
// STOCHF calculates a rolling fast %K from high, low, and close, then applies
// the selected TA-Lib moving-average type to produce fast %D.

use crate::ma_type::compute_ma;

/// Computes aligned fast %K and fast %D output arrays.
///
/// # Parameters
///
/// * `high`, `low`, `close` - Equal-length chronological OHLC series.
/// * Period and moving-average parameters configure the stochastic windows.
///
/// # Returns
///
/// A pair of same-length fast %K and fast %D arrays with warm-up NaNs.
pub fn fast_stochastic_oscillator(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    fastk_period: usize,
    fastd_period: usize,
    fastd_matype: MaType,
) -> TaResult<(Vec<f64>, Vec<f64>)> {
    let len = high.len();
    if len != low.len() || len != close.len() {
        return Err(TaError::LengthMismatch {
            expected: len,
            got: low.len().min(close.len()),
        });
    }
    if fastk_period == 0 || fastd_period == 0 {
        return Err(TaError::InvalidParameter {
            name: "fastk_period/fastd_period",
            value: format!("{fastk_period}/{fastd_period}"),
            reason: "periods must be >= 1",
        });
    }

    let fastd_lookback = fastd_matype.lookback(fastd_period);
    let lookback = fastk_period - 1 + fastd_lookback;
    if len <= lookback {
        return Err(TaError::InsufficientData {
            need: lookback + 1,
            got: len,
        });
    }

    let mut fastk_values = Vec::with_capacity(len - (fastk_period - 1));
    for today in (fastk_period - 1)..len {
        let start = today + 1 - fastk_period;
        let highest = high[start..=today]
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
        let lowest = low[start..=today]
            .iter()
            .copied()
            .fold(f64::INFINITY, f64::min);
        let divisor = (highest - lowest) / 100.0;
        fastk_values.push(if divisor.abs() >= 1.0e-14 {
            (close[today] - lowest) / divisor
        } else {
            0.0
        });
    }

    let fastd_values = compute_ma(&fastk_values, fastd_period, fastd_matype)?;
    let mut fastk = vec![f64::NAN; len];
    let mut fastd = vec![f64::NAN; len];
    for (offset, bar) in (lookback..len).enumerate() {
        let value_index = fastd_lookback + offset;
        fastk[bar] = fastk_values[value_index];
        fastd[bar] = fastd_values[value_index];
    }
    Ok((fastk, fastd))
}
