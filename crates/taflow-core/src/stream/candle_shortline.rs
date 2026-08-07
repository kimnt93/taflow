use super::pattern::*;
use crate::error::TaResult;
use std::collections::VecDeque;
/// Stateful CandleShortLine candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleShortLine {
    b: VecDeque<f64>,
    s: VecDeque<f64>,
    bs: f64,
    ss: f64,
    value: Option<i32>,
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
/// Compute the candle short line result for the supplied aligned series.
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
pub fn candle_short_line(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_SHORT.avg_period.max(SHADOW_SHORT.avg_period);
    if len <= lookback {
        return Ok(output);
    }

    let mut body_sum = 0.0;
    let mut shadow_sum = 0.0;
    let start = lookback;
    for i in (start - BODY_SHORT.avg_period)..start {
        body_sum += cr(BODY_SHORT, open, high, low, close, i);
    }
    for i in (start - SHADOW_SHORT.avg_period)..start {
        shadow_sum += cr(SHADOW_SHORT, open, high, low, close, i);
    }

    for i in start..len {
        output[i] =
            (real_body(open[i], close[i]) < ca(BODY_SHORT, body_sum, open, high, low, close, i)
                && upper_shadow(open[i], high[i], close[i])
                    < ca(SHADOW_SHORT, shadow_sum, open, high, low, close, i)
                && lower_shadow(open[i], low[i], close[i])
                    < ca(SHADOW_SHORT, shadow_sum, open, high, low, close, i)) as i32
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
        shadow_sum += cr(SHADOW_SHORT, open, high, low, close, i)
            - cr(
                SHADOW_SHORT,
                open,
                high,
                low,
                close,
                i - SHADOW_SHORT.avg_period,
            );
    }
    Ok(output)
}
impl CandleShortLine {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            b: VecDeque::with_capacity(10),
            s: VecDeque::with_capacity(10),
            bs: 0.,
            ss: 0.,
            value: None,
        }
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, o: f64, h: f64, l: f64, c: f64) -> Option<i32> {
        let body = (c - o).abs();
        let sh = (h - o.max(c)) + (o.min(c) - l);
        let v = if self.b.len() == 10 {
            Some(
                (body < self.bs / 10.
                    && h - o.max(c) < self.ss / 20.
                    && o.min(c) - l < self.ss / 20.) as i32
                    * if c >= o { 100 } else { -100 },
            )
        } else {
            None
        };
        if self.b.len() == 10 {
            self.bs -= self.b.pop_front().unwrap();
            self.ss -= self.s.pop_front().unwrap();
        }
        self.b.push_back(body);
        self.s.push_back(sh);
        self.bs += body;
        self.ss += sh;
        self.value = v;
        v
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
