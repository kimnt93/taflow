//! Incremental Commodity Channel Index (CCI).
//!
//! CCI's moving mean is O(1), while its exact mean absolute deviation needs a
//! bounded window scan. The state retains no full price history and continues
//! from each appended HLC bar without replaying earlier input.

use crate::error::{TaError, TaResult};

use super::{invalid_period, Window};

/// Compute the commodity channel index result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `close` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn commodity_channel_index(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    timeperiod: usize,
) -> TaResult<Vec<f64>> {
    if high.len() != low.len() || high.len() != close.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().min(close.len()),
        });
    }
    let mut state = CommodityChannelIndex::new(timeperiod)?;
    Ok(high
        .iter()
        .zip(low)
        .zip(close)
        .map(|((high, low), close)| state.append(*high, *low, *close).unwrap_or(f64::NAN))
        .collect())
}

/// Reseed cadence for the sliding typical-price sum, in absolute appends.
///
/// TA-Lib's `TA_CCI` rescans its circular buffer for the window average on
/// every bar, so its average carries no accumulated error; ours slides in
/// O(1) and therefore drifts. Every `CCI_RESEED_INTERVAL`-th append (counted
/// from construction/reset, so the reseed bars land at the same absolute
/// positions no matter how the input is chunked) the sum is recomputed from
/// the retained window in serial oldest-to-newest order, bounding the drift to
/// at most `K - 1` slide steps instead of letting it grow with the series
/// length.
///
/// Measured against `talib.CCI` on the benchmark's AR(1) price series
/// (p = 14, `rtol=1e-8 / atol=1e-10` gate):
///
/// | K    | max abs err @100k | @300k    | @1M      |
/// |------|-------------------|----------|----------|
/// | none | 7.12e-10 (fails)  | 7.12e-10 | 2.40e-09 |
/// | 64   | 1.32e-11          | 2.11e-11 | 2.50e-11 |
/// | 128  | 3.11e-11          | 3.11e-11 | 3.63e-11 |
/// | 256  | 2.42e-11          | 3.75e-11 | 5.71e-11 |
///
/// 64 is chosen for the widest margin (4× under `atol` even at 1M bars) at a
/// negligible amortized cost of `period / 64` extra additions per bar — the
/// mean-deviation scan already walks the whole window every bar. It also keeps
/// the constant aligned with `PAIR_MOMENTS_RESEED_INTERVAL`.
pub(super) const CCI_RESEED_INTERVAL: u64 = 64;

/// Persistent Commodity Channel Index with TA-Lib-compatible warm-up.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `CommodityChannelIndex`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CommodityChannelIndex {
    period: usize,
    window: Window,
    sum: f64,
    /// Total appends since construction/reset, driving the reseed cadence.
    count: u64,
    value: Option<f64>,
}

impl CommodityChannelIndex {
    /// Creates an empty CCI state. TA-Lib requires a period of at least two.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            period,
            window: Window::new(period)?,
            sum: 0.0,
            count: 0,
            value: None,
        })
    }

    /// Appends one high/low/close bar and returns CCI after warm-up.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let typical_price = (high + low + close) / 3.0;
        if let Some(old) = self.window.push(typical_price) {
            self.sum -= old;
        }
        self.sum += typical_price;
        self.count += 1;
        if self.window.is_full() && self.count % CCI_RESEED_INTERVAL == 0 {
            self.reseed_serial();
        }

        self.value = self.window.is_full().then(|| {
            let period = self.period as f64;
            let average = self.sum / period;
            let mean_deviation = self
                .window
                .iter()
                .map(|value| (*value - average).abs())
                .sum::<f64>()
                / period;
            if mean_deviation > 0.0 {
                (typical_price - average) / (0.015 * mean_deviation)
            } else {
                0.0
            }
        });
        self.value
    }

    /// Recomputes the typical-price sum from the retained window, oldest to
    /// newest, with the same per-element accumulation the warm-up path uses.
    fn reseed_serial(&mut self) {
        let mut sum = 0.0;
        for value in self.window.iter() {
            sum += *value;
        }
        self.sum = sum;
    }

    /// Extends state with aligned HLC slices after validating all lengths.
    pub fn extend_slice(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
    ) -> TaResult<Vec<Option<f64>>> {
        if high.len() != low.len() || high.len() != close.len() {
            return Err(TaError::LengthMismatch {
                expected: high.len(),
                got: low.len().min(close.len()),
            });
        }
        Ok(high
            .iter()
            .zip(low)
            .zip(close)
            .map(|((&high, &low), &close)| self.append(high, low, close))
            .collect())
    }

    /// Returns the newest warm value without materializing history.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clears all accumulated state while retaining the allocated window.
    pub fn reset(&mut self) {
        self.window.clear();
        self.sum = 0.0;
        self.count = 0;
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lcg_series(n: usize, mut state: u64) -> Vec<f64> {
        (0..n)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                90.0 + (state >> 11) as f64 / (1u64 << 53) as f64 * 20.0
            })
            .collect()
    }

    fn hlc(n: usize, seed: u64) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let close = lcg_series(n, seed);
        let high = close.iter().map(|value| value + 0.75).collect();
        let low = close.iter().map(|value| value - 0.5).collect();
        (high, low, close)
    }

    /// CCI recomputed from scratch over the trailing window, serially, with no
    /// state carried across bars.
    fn exact_cci(high: &[f64], low: &[f64], close: &[f64], end: usize, period: usize) -> f64 {
        let typical = |i: usize| (high[i] + low[i] + close[i]) / 3.0;
        let period_f = period as f64;
        let mut sum = 0.0;
        for i in end + 1 - period..=end {
            sum += typical(i);
        }
        let average = sum / period_f;
        let mut deviation = 0.0;
        for i in end + 1 - period..=end {
            deviation += (typical(i) - average).abs();
        }
        deviation /= period_f;
        if deviation > 0.0 {
            (typical(end) - average) / (0.015 * deviation)
        } else {
            0.0
        }
    }

    /// Chunking must not move the reseed bars: the cadence counts absolute
    /// appends, so every chunking (and an `append` tail after a bulk call)
    /// has to stay bit-identical to a pure per-bar replay.
    #[test]
    fn extend_is_bitwise_identical_to_per_bar_append_across_reseeds() {
        let (high, low, close) = hlc(5_000, 0x0CC1_5EED);
        let (tail_high, tail_low, tail_close) = hlc(256, 0x0CC1_7A11);
        // 5,000 bars cross the 64-append reseed cadence ~78 times.
        for period in [2usize, 5, 14, 30, 200] {
            let mut per_bar = CommodityChannelIndex::new(period).unwrap();
            let reference: Vec<f64> = (0..close.len())
                .map(|i| {
                    per_bar
                        .append(high[i], low[i], close[i])
                        .unwrap_or(f64::NAN)
                })
                .collect();
            let tail_reference: Vec<f64> = (0..tail_close.len())
                .map(|i| {
                    per_bar
                        .append(tail_high[i], tail_low[i], tail_close[i])
                        .unwrap_or(f64::NAN)
                })
                .collect();

            for chunk in [usize::MAX, 1, 7, 10, 97, 1000] {
                let mut state = CommodityChannelIndex::new(period).unwrap();
                let mut actual = Vec::new();
                let mut start = 0;
                while start < close.len() {
                    let end = start.saturating_add(chunk).min(close.len());
                    actual.extend(
                        state
                            .extend_slice(&high[start..end], &low[start..end], &close[start..end])
                            .unwrap()
                            .into_iter()
                            .map(|value| value.unwrap_or(f64::NAN)),
                    );
                    start = end;
                }
                for (i, (actual, expected)) in actual.iter().zip(&reference).enumerate() {
                    assert_eq!(
                        actual.to_bits(),
                        expected.to_bits(),
                        "CCI p{period} chunk {chunk} bar {i}"
                    );
                }
                for i in 0..tail_close.len() {
                    let actual = state
                        .append(tail_high[i], tail_low[i], tail_close[i])
                        .unwrap_or(f64::NAN);
                    assert_eq!(
                        actual.to_bits(),
                        tail_reference[i].to_bits(),
                        "CCI p{period} chunk {chunk} tail bar {i}"
                    );
                }
            }
        }
    }

    /// A period that is a multiple of the reseed interval makes the very first
    /// warm bar (`count == period`) also a reseed bar; the reseed must see a
    /// full window and leave the first emitted value correct.
    #[test]
    fn reseed_landing_on_the_first_warm_bar_is_correct() {
        let (high, low, close) = hlc(1_000, 0x0CC1_F125);
        for period in [
            CCI_RESEED_INTERVAL as usize,
            2 * CCI_RESEED_INTERVAL as usize,
        ] {
            let mut state = CommodityChannelIndex::new(period).unwrap();
            for i in 0..close.len() {
                let value = state.append(high[i], low[i], close[i]);
                if i + 1 < period {
                    assert!(value.is_none(), "p{period} bar {i} should be warm-up");
                    continue;
                }
                let value = value.expect("warm value");
                let exact = exact_cci(&high, &low, &close, i, period);
                let drift = (value - exact).abs() / exact.abs().max(1.0);
                if (i as u64 + 1) % CCI_RESEED_INTERVAL == 0 {
                    // A reseed bar — including the first warm bar, which both
                    // fills the window and hits the cadence — must land on the
                    // freshly recomputed sum, so only the mean-deviation scan's
                    // own rounding separates it from the reference.
                    assert!(
                        drift < 1e-14,
                        "p{period} reseed bar {i}: drift {drift:e} (reseed did not take)"
                    );
                } else {
                    assert!(drift < 1e-11, "p{period} bar {i}: drift {drift:e}");
                }
            }
        }
    }

    /// The reseed bounds the sliding typical-price sum's error to at most
    /// `CCI_RESEED_INTERVAL - 1` slide steps, so the deviation from a fresh
    /// per-window recomputation stays flat instead of growing with the series.
    #[test]
    fn streaming_drift_stays_bounded_over_1m_bars() {
        let (high, low, close) = hlc(1_000_000, 0x0CC1_D21F);
        for period in [14usize, 30] {
            let mut state = CommodityChannelIndex::new(period).unwrap();
            for i in 0..close.len() {
                let Some(value) = state.append(high[i], low[i], close[i]) else {
                    continue;
                };
                if (i + 1) % 50_000 != 0 && i + 1 != close.len() {
                    continue;
                }
                let exact = exact_cci(&high, &low, &close, i, period);
                let drift = (value - exact).abs() / exact.abs().max(1.0);
                assert!(
                    drift < 1e-12,
                    "CCI p{period} bar {i}: drift {drift:e} vs a fresh window"
                );
            }
        }
    }

    #[test]
    fn matches_batch_for_extend_chunk_and_reset_replay() {
        let close: Vec<f64> = (0..96)
            .map(|index| 100.0 + index as f64 * 0.17 + (index as f64 * 0.23).sin() * 2.0)
            .collect();
        let high: Vec<f64> = close.iter().map(|value| value + 1.2).collect();
        let low: Vec<f64> = close.iter().map(|value| value - 0.9).collect();
        let expected = crate::stream::commodity_channel_index(&high, &low, &close, 14).unwrap();

        let mut state = CommodityChannelIndex::new(14).unwrap();
        let mut actual = state
            .extend_slice(&high[..37], &low[..37], &close[..37])
            .unwrap();
        actual.extend(
            state
                .extend_slice(&high[37..], &low[37..], &close[37..])
                .unwrap(),
        );
        for (actual, expected) in actual.iter().zip(expected.iter()) {
            match actual {
                Some(actual) => assert!((actual - expected).abs() < 1e-10),
                None => assert!(expected.is_nan()),
            }
        }

        state.reset();
        for ((&high, &low), (&close, expected)) in
            high.iter().zip(&low).zip(close.iter().zip(&expected))
        {
            match state.append(high, low, close) {
                Some(actual) => assert!((actual - expected).abs() < 1e-10),
                None => assert!(expected.is_nan()),
            }
        }
    }
}
