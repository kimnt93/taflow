//! Stateful Fast Stochastic Oscillator.
//!
//! STOCHF maintains rolling high/low extrema for fast %K and feeds that value
//! into the selected incremental moving average for fast %D.

use crate::error::{TaError, TaResult};
use crate::ma_type::MaType;

use super::{
    moving_average_dispatcher::MovingAverageDispatcher, RollingMax, RollingMin, StreamingIndicator,
};

/// One aligned fast %K and fast %D observation.
#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `FastStochasticOscillatorValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct FastStochasticOscillatorValue {
    pub fastk: f64,
    pub fastd: f64,
}

/// Incremental STOCHF with amortized constant work per bar.
/// Persistent Rust state or aligned output type for `FastStochasticOscillator`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
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
    pub fn append(
        &mut self,
        high: f64,
        low: f64,
        close: f64,
    ) -> Option<FastStochasticOscillatorValue> {
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

    /// Bulk kernel: vHGW sliding extrema for the fast %K window (via the
    /// `RollingMax`/`RollingMin` bulk paths, which also rebuild their deques),
    /// then the fast %D sub-state is driven per emitted bar exactly as
    /// [`Self::append`] does. Outputs and post-run state are bit-identical to
    /// per-bar [`Self::append`]; warm-up bars are NaN.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        fastk_out: &mut Vec<f64>,
        fastd_out: &mut Vec<f64>,
    ) -> TaResult<()> {
        if high.len() != low.len() || high.len() != close.len() {
            return Err(TaError::LengthMismatch {
                expected: high.len(),
                got: low.len().min(close.len()),
            });
        }
        let n = high.len();
        fastk_out.reserve(n);
        fastd_out.reserve(n);
        let period = self.highest.period();
        let consumed = self.highest.count();
        let mut highest = Vec::with_capacity(n);
        let mut lowest = Vec::with_capacity(n);
        self.highest.extend_slice_into(high, &mut highest);
        self.lowest.extend_slice_into(low, &mut lowest);
        for index in 0..n {
            if consumed + index + 1 < period {
                fastk_out.push(f64::NAN);
                fastd_out.push(f64::NAN);
                continue;
            }
            let (highest, lowest) = (highest[index], lowest[index]);
            let divisor = (highest - lowest) / 100.0;
            let fastk = if divisor.abs() >= 1.0e-14 {
                (close[index] - lowest) / divisor
            } else {
                0.0
            };
            self.value = self
                .fastd
                .append(fastk)
                .map(|fastd| FastStochasticOscillatorValue { fastk, fastd });
            match self.value {
                Some(value) => {
                    fastk_out.push(value.fastk);
                    fastd_out.push(value.fastd);
                }
                None => {
                    fastk_out.push(f64::NAN);
                    fastd_out.push(f64::NAN);
                }
            }
        }
        Ok(())
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
            let expected =
                crate::stream::fast_stochastic_oscillator(&high, &low, &close, 5, 13, ma_type)
                    .unwrap();
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

#[cfg(test)]
mod stochf_bulk_tests {
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
    fn stochf_bulk_matches_append_bitwise() {
        let close = lcg_series(5_000, 0x9E37_79B9_7F4A_7C15);
        let high: Vec<f64> = close.iter().map(|v| v + 1.0).collect();
        let low: Vec<f64> = close.iter().map(|v| v - 1.0).collect();
        for period in [2usize, 5, 14, 30, 200] {
            for (fastd, ma) in [
                (1usize, MaType::SimpleMovingAverage),
                (4, MaType::ExponentialMovingAverage),
                (3, MaType::WeightedMovingAverage),
            ] {
                let mut reference = FastStochasticOscillator::new(period, fastd, ma).unwrap();
                let expected: Vec<(f64, f64)> = (0..close.len())
                    .map(|i| match reference.append(high[i], low[i], close[i]) {
                        Some(value) => (value.fastk, value.fastd),
                        None => (f64::NAN, f64::NAN),
                    })
                    .collect();
                for chunk in [1usize, 7, 97, close.len()] {
                    let mut state = FastStochasticOscillator::new(period, fastd, ma).unwrap();
                    let (mut k_out, mut d_out) = (Vec::new(), Vec::new());
                    let mut offset = 0;
                    while offset < close.len() {
                        let end = (offset + chunk).min(close.len());
                        state
                            .extend_slices_into(
                                &high[offset..end],
                                &low[offset..end],
                                &close[offset..end],
                                &mut k_out,
                                &mut d_out,
                            )
                            .unwrap();
                        offset = end;
                    }
                    for (i, (ek, ed)) in expected.iter().enumerate() {
                        assert_eq!(
                            ek.to_bits(),
                            k_out[i].to_bits(),
                            "fastk p={period} c={chunk} i={i}"
                        );
                        assert_eq!(
                            ed.to_bits(),
                            d_out[i].to_bits(),
                            "fastd p={period} c={chunk} i={i}"
                        );
                    }
                    for i in 0..256 {
                        assert_eq!(
                            reference.append(high[i], low[i], close[i]),
                            state.append(high[i], low[i], close[i]),
                            "continue p={period} c={chunk}"
                        );
                    }
                    reference.reset();
                    for i in 0..close.len() {
                        reference.append(high[i], low[i], close[i]);
                    }
                }
            }
        }
    }

    #[test]
    fn stochf_bulk_validates_lengths() {
        let mut state = FastStochasticOscillator::new(5, 3, MaType::SimpleMovingAverage).unwrap();
        let (mut k, mut d) = (Vec::new(), Vec::new());
        assert!(state
            .extend_slices_into(&[1.0, 2.0], &[1.0], &[1.0, 2.0], &mut k, &mut d)
            .is_err());
    }
}
