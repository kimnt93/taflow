//! Incremental Spinning Top candlestick recognition (CDLSPINNINGTOP).

use std::collections::VecDeque;

use super::pattern::*;
use crate::error::TaResult;
/// Incremental CDLSPINNINGTOP state using TA-Lib's ten-bar short-body average.
/// Persistent Rust state or aligned output type for `CandleSpinningTop`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleSpinningTop {
    bodies: VecDeque<f64>,
    sum: f64,
    value: Option<i32>,
}
impl Default for CandleSpinningTop {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleSpinningTop {
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
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let body = (close - open).abs();
        let value = if self.bodies.len() == 10 {
            Some(
                (body < self.sum / 10.0
                    && high - open.max(close) > body
                    && open.min(close) - low > body) as i32
                    * if close >= open { 100 } else { -100 },
            )
        } else {
            None
        };
        if self.bodies.len() == 10 {
            self.sum -= self.bodies.pop_front().expect("window is full");
        }
        self.bodies.push_back(body);
        self.sum += body;
        self.value = value;
        value
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
        *self = Self::new();
    }
}

/// Compute the candle pattern signal for aligned OHLC bars.
///
/// # Parameters
///
/// * `open`, `high`, `low`, `close` - Equal-length chronological OHLC series.
///
/// # Returns
///
/// A same-length vector containing -100, 0, or 100 pattern signals; bars
/// Compute the candle spinningtop result for the supplied aligned series.
///
/// # Parameters
///
/// * `open` - Input series or configuration value.
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `close` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn candle_spinningtop(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_SHORT.avg_period;
    if len <= lookback {
        return Ok(output);
    }

    let mut body_sum = 0.0;
    let start = lookback;
    for i in (start - BODY_SHORT.avg_period)..start {
        body_sum += cr(BODY_SHORT, open, high, low, close, i);
    }

    for i in start..len {
        output[i] = (real_body(open[i], close[i])
            < ca(BODY_SHORT, body_sum, open, high, low, close, i)
            && upper_shadow(open[i], high[i], close[i]) > real_body(open[i], close[i])
            && lower_shadow(open[i], low[i], close[i]) > real_body(open[i], close[i]))
            as i32
            * candle_color(open[i], close[i])
            * 100;
        body_sum += cr(BODY_SHORT, open, high, low, close, i)
            - cr(
                BODY_SHORT,
                open,
                high,
                low,
                close,
                i - BODY_SHORT.avg_period,
            );
    }
    Ok(output)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_batch() {
        let o: Vec<f64> = (0..40).map(|i| 100.0 + i as f64 * 0.2).collect();
        let h: Vec<f64> = o.iter().map(|x| x + 2.0).collect();
        let l: Vec<f64> = o.iter().map(|x| x - 2.0).collect();
        let c: Vec<f64> = o
            .iter()
            .enumerate()
            .map(|(i, x)| x + if i % 2 == 0 { 0.1 } else { -0.1 })
            .collect();
        let expected = crate::stream::candle_spinningtop(&o, &h, &l, &c).unwrap();
        let mut state = CandleSpinningTop::new();
        for (((&o, &h), &l), (&c, &e)) in o.iter().zip(&h).zip(&l).zip(c.iter().zip(&expected)) {
            assert_eq!(state.append(o, h, l, c).unwrap_or(0), e);
        }
    }
}
