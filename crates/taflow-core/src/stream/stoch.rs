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

    /// Bulk kernel: vHGW sliding extrema for the fast %K window (via the
    /// `RollingMax`/`RollingMin` bulk paths, which also rebuild their deques),
    /// then the slow %K and slow %D sub-states are driven per emitted bar in
    /// exactly the order [`Self::append`] uses. The MA passes are deliberately
    /// not fused: their seeds are order dependent, and the extrema pass was the
    /// dominant cost. Outputs and post-run state are bit-identical to per-bar
    /// [`Self::append`]; warm-up bars are NaN.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        slowk_out: &mut Vec<f64>,
        slowd_out: &mut Vec<f64>,
    ) -> TaResult<()> {
        if high.len() != low.len() || high.len() != close.len() {
            return Err(TaError::LengthMismatch {
                expected: high.len(),
                got: low.len().min(close.len()),
            });
        }
        let n = high.len();
        slowk_out.reserve(n);
        slowd_out.reserve(n);
        let period = self.highest.period();
        let consumed = self.highest.count();
        let mut highest = Vec::with_capacity(n);
        let mut lowest = Vec::with_capacity(n);
        self.highest.extend_slice_into(high, &mut highest);
        self.lowest.extend_slice_into(low, &mut lowest);
        for index in 0..n {
            if consumed + index + 1 < period {
                slowk_out.push(f64::NAN);
                slowd_out.push(f64::NAN);
                continue;
            }
            let (highest, lowest) = (highest[index], lowest[index]);
            let divisor = (highest - lowest) / 100.0;
            let fastk = if divisor.abs() >= 1.0e-14 {
                (close[index] - lowest) / divisor
            } else {
                0.0
            };
            self.value = self.slowk.append(fastk).and_then(|slowk| {
                self.slowd
                    .append(slowk)
                    .map(|slowd| StochasticOscillatorValue { slowk, slowd })
            });
            match self.value {
                Some(value) => {
                    slowk_out.push(value.slowk);
                    slowd_out.push(value.slowd);
                }
                None => {
                    slowk_out.push(f64::NAN);
                    slowd_out.push(f64::NAN);
                }
            }
        }
        Ok(())
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

    /// Original O(n * fastk_period) double-loop batch, kept verbatim as the
    /// oracle for the vHGW fast %K path.
    fn reference_stochastic_oscillator(
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

        let slowk_arr = crate::ma_type::compute_ma(&fastk, slowk_period, slowk_matype)?;
        let slowk_valid = &slowk_arr[slowk_lookback..];
        let slowd_arr = crate::ma_type::compute_ma(slowk_valid, slowd_period, slowd_matype)?;

        let mut slowk_out = vec![f64::NAN; len];
        let mut slowd_out = vec![f64::NAN; len];
        for (offset, bar) in (lookback..len).enumerate() {
            let value_index = slowd_lookback + offset;
            slowk_out[bar] = slowk_valid[value_index];
            slowd_out[bar] = slowd_arr[value_index];
        }

        Ok((slowk_out, slowd_out))
    }

    #[test]
    fn batch_matches_double_loop_reference_bitwise() {
        use crate::stream::tests_extrema_support::{datasets, periods_and_lengths};
        for (period, len) in periods_and_lengths() {
            let len = len.min(4096);
            for data in datasets(len) {
                let spread_high: Vec<f64> = data.iter().map(|v| v + 1.5).collect();
                let spread_low: Vec<f64> = data.iter().map(|v| v - 1.5).collect();
                // The flat variant (high == low == close) exercises the
                // near-zero divisor branch.
                for (high, low, close) in [
                    (spread_high.clone(), spread_low.clone(), data.clone()),
                    (data.clone(), data.clone(), data.clone()),
                ] {
                    for (slowk, slowd) in [(1usize, 1usize), (3, 4)] {
                        let expected = reference_stochastic_oscillator(
                            &high,
                            &low,
                            &close,
                            period,
                            slowk,
                            MaType::SimpleMovingAverage,
                            slowd,
                            MaType::ExponentialMovingAverage,
                        );
                        let actual = stochastic_oscillator(
                            &high,
                            &low,
                            &close,
                            period,
                            slowk,
                            MaType::SimpleMovingAverage,
                            slowd,
                            MaType::ExponentialMovingAverage,
                        );
                        match (expected, actual) {
                            (Ok(expected), Ok(actual)) => {
                                for (e, a) in expected.0.iter().zip(&actual.0) {
                                    assert_eq!(
                                        e.to_bits(),
                                        a.to_bits(),
                                        "slowk p={period} len={len}"
                                    );
                                }
                                for (e, a) in expected.1.iter().zip(&actual.1) {
                                    assert_eq!(
                                        e.to_bits(),
                                        a.to_bits(),
                                        "slowd p={period} len={len}"
                                    );
                                }
                            }
                            (Err(_), Err(_)) => {}
                            _ => panic!("error parity mismatch p={period} len={len}"),
                        }
                    }
                }
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

    // vHGW extrema replace the O(n * fastk_period) double loop; the highest
    // buffer is turned into fast %K in place to avoid a third allocation.
    let fastk_len = len - (fastk_period - 1);
    let mut fastk = vec![0.0_f64; fastk_len];
    let mut lowest = vec![0.0_f64; fastk_len];
    super::vhgw::sliding_max_into(high, fastk_period, &mut fastk);
    super::vhgw::sliding_min_into(low, fastk_period, &mut lowest);
    for (offset, (slot, &ll)) in fastk.iter_mut().zip(&lowest).enumerate() {
        let hh = *slot;
        let divisor = (hh - ll) / 100.0;
        *slot = if divisor.abs() >= 1.0e-14 {
            (close[fastk_period - 1 + offset] - ll) / divisor
        } else {
            0.0
        };
    }
    drop(lowest);

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

#[cfg(test)]
mod stoch_bulk_tests {
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
    fn stoch_bulk_matches_append_bitwise() {
        let close = lcg_series(5_000, 0x2BAD_C0DE_1357_9BDF);
        let high: Vec<f64> = close.iter().map(|v| v + 1.0).collect();
        let low: Vec<f64> = close.iter().map(|v| v - 1.0).collect();
        for period in [2usize, 5, 14, 30, 200] {
            for (slowk, slowd, kt, dt) in [
                (
                    1usize,
                    1usize,
                    MaType::SimpleMovingAverage,
                    MaType::SimpleMovingAverage,
                ),
                (
                    3,
                    4,
                    MaType::SimpleMovingAverage,
                    MaType::ExponentialMovingAverage,
                ),
                (
                    5,
                    3,
                    MaType::WeightedMovingAverage,
                    MaType::TripleExponentialMovingAverage,
                ),
            ] {
                let mut reference =
                    StochasticOscillator::new(period, slowk, kt, slowd, dt).unwrap();
                let expected: Vec<(f64, f64)> = (0..close.len())
                    .map(|i| match reference.append(high[i], low[i], close[i]) {
                        Some(value) => (value.slowk, value.slowd),
                        None => (f64::NAN, f64::NAN),
                    })
                    .collect();
                for chunk in [1usize, 7, 97, close.len()] {
                    let mut state =
                        StochasticOscillator::new(period, slowk, kt, slowd, dt).unwrap();
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
                    assert_eq!(k_out.len(), close.len());
                    for (i, (ek, ed)) in expected.iter().enumerate() {
                        assert_eq!(
                            ek.to_bits(),
                            k_out[i].to_bits(),
                            "slowk p={period} c={chunk} i={i}"
                        );
                        assert_eq!(
                            ed.to_bits(),
                            d_out[i].to_bits(),
                            "slowd p={period} c={chunk} i={i}"
                        );
                    }
                    for i in 0..256 {
                        let e = reference.append(high[i], low[i], close[i]);
                        let a = state.append(high[i], low[i], close[i]);
                        match (e, a) {
                            (Some(e), Some(a)) => {
                                assert_eq!(
                                    e.slowk.to_bits(),
                                    a.slowk.to_bits(),
                                    "continue p={period} c={chunk}"
                                );
                                assert_eq!(e.slowd.to_bits(), a.slowd.to_bits());
                            }
                            (None, None) => {}
                            _ => panic!("continuation warm-up mismatch p={period} c={chunk}"),
                        }
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
    fn stoch_bulk_validates_lengths() {
        let mut state = StochasticOscillator::new(
            5,
            3,
            MaType::SimpleMovingAverage,
            3,
            MaType::SimpleMovingAverage,
        )
        .unwrap();
        let (mut k, mut d) = (Vec::new(), Vec::new());
        assert!(state
            .extend_slices_into(&[1.0, 2.0], &[1.0], &[1.0, 2.0], &mut k, &mut d)
            .is_err());
    }
}
