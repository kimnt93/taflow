//! Incremental Commodity Channel Index (CCI).
//!
//! CCI's moving mean is O(1), while its exact mean absolute deviation needs a
//! bounded window scan. The state retains no full price history and continues
//! from each appended HLC bar without replaying earlier input.

use crate::error::{TaError, TaResult};

use crate::stream::{invalid_period, Window};

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
