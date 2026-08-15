//! Incremental High-Wave candlestick recognition (CDLHIGHWAVE).

use std::collections::VecDeque;

use crate::error::TaResult;
use crate::stream::pattern::*;
/// Incremental CDLHIGHWAVE state using TA-Lib's rolling short-body average.
/// Persistent Rust state or aligned output type for `CandleHighWave`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleHighWave {
    bodies: VecDeque<f64>,
    sum: f64,
    value: Option<i32>,
}
impl Default for CandleHighWave {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleHighWave {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            bodies: VecDeque::with_capacity(10),
            sum: 0.0,
            value: None,
        }
    }
    /// Appends OHLC data and returns a signed high-wave signal after the ten-bar warmup.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let body = (close - open).abs();
        let output = if self.bodies.len() == 10 {
            let upper = high - open.max(close);
            let lower = open.min(close) - low;
            Some(
                (body < self.sum / 10.0 && upper > body * 2.0 && lower > body * 2.0) as i32
                    * if close >= open { 100 } else { -100 },
            )
        } else {
            None
        };
        if self.bodies.len() == 10 {
            // Slide exactly like the batch loop: sum += cr(new) - cr(evicted).
            let old = self.bodies.pop_front().expect("window is full");
            self.sum += body - old;
        } else {
            self.sum += body;
        }
        self.bodies.push_back(body);
        self.value = output;
        output
    }
    /// Bulk-append aligned OHLC slices, pushing one score per bar into `output`.
    ///
    /// From a pristine state this runs a direct rolling-sum slice kernel and
    /// reconstructs only its bounded tail state. A non-pristine state falls
    /// back to the per-bar loop. Either route is bit-identical to calling
    /// `append` once per bar (warm-up `None` becomes `0`).
    ///
    /// # Parameters
    ///
    /// * `open`, `high`, `low`, `close` - Equal-length chronological OHLC series.
    /// * `output` - Destination the aligned scores are appended to.
    ///
    /// # Returns
    ///
    /// `Ok(())`, or a validation error when the inputs are not aligned.
    pub fn extend_slices_into(
        &mut self,
        open: &[f64],
        high: &[f64],
        low: &[f64],
        close: &[f64],
        output: &mut Vec<i32>,
    ) -> TaResult<()> {
        let len = validate_ohlc(open, high, low, close)?;
        const LOOKBACK: usize = 10;
        if !self.bodies.is_empty() || len <= LOOKBACK {
            output.reserve(len);
            for i in 0..len {
                output.push(self.append(open[i], high[i], low[i], close[i]).unwrap_or(0));
            }
            return Ok(());
        }

        let start = output.len();
        output.resize(start + len, 0);
        let mut sum = open[..LOOKBACK]
            .iter()
            .zip(&close[..LOOKBACK])
            .fold(0.0, |sum, (&open, &close)| sum + (close - open).abs());
        for index in LOOKBACK..len {
            let body = (close[index] - open[index]).abs();
            let upper = high[index] - open[index].max(close[index]);
            let lower = open[index].min(close[index]) - low[index];
            output[start + index] = (body < sum / 10.0 && upper > body * 2.0 && lower > body * 2.0)
                as i32
                * if close[index] >= open[index] {
                    100
                } else {
                    -100
                };
            sum += body - (close[index - LOOKBACK] - open[index - LOOKBACK]).abs();
        }
        self.sum = sum;
        self.bodies.extend(
            open[len - LOOKBACK..]
                .iter()
                .zip(&close[len - LOOKBACK..])
                .map(|(&open, &close)| (close - open).abs()),
        );
        self.value = output.last().copied();
        Ok(())
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<i32> {
        self.value
    }
    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.bodies.clear();
        self.sum = 0.0;
        self.value = None;
    }
}
