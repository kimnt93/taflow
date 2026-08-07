//! Incremental Belt Hold candlestick recognition (CDLBELTHOLD).
use super::pattern::*;
use crate::error::TaResult;
use std::collections::VecDeque;
/// Stateful CandleBeltHold candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleBeltHold {
    b: VecDeque<f64>,
    r: VecDeque<f64>,
    bs: f64,
    rs: f64,
    value: Option<i32>,
}
impl Default for CandleBeltHold {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleBeltHold {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            b: VecDeque::with_capacity(10),
            r: VecDeque::with_capacity(10),
            bs: 0.,
            rs: 0.,
            value: None,
        }
    }
    fn push(q: &mut VecDeque<f64>, s: &mut f64, v: f64) {
        if q.len() == 10 {
            *s -= q.pop_front().unwrap();
        }
        q.push_back(v);
        *s += v;
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, o: f64, h: f64, l: f64, c: f64) -> Option<i32> {
        let body = (c - o).abs();
        let range = h - l;
        let upper = h - o.max(c);
        let lower = o.min(c) - l;
        let v = if self.b.len() == 10 {
            let long = body > self.bs / 10.0;
            let lim = self.rs * 0.01;
            Some(if long && c >= o && lower < lim {
                100
            } else if long && c < o && upper < lim {
                -100
            } else {
                0
            })
        } else {
            None
        };
        Self::push(&mut self.b, &mut self.bs, body);
        Self::push(&mut self.r, &mut self.rs, range);
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

/// Compute the candle pattern signal for aligned OHLC bars.
///
/// # Parameters
///
/// * `open`, `high`, `low`, `close` - Equal-length chronological OHLC series.
///
/// # Returns
///
/// A same-length vector containing -100, 0, or 100 pattern signals; bars
/// Compute the candle belt hold result for the supplied aligned series.
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
pub fn candle_belt_hold(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_LONG.avg_period.max(SHADOW_VERY_SHORT.avg_period);
    if len <= lookback {
        return Ok(output);
    }

    let mut body_sum = 0.0;
    let mut shadow_sum = 0.0;
    let start = lookback;
    for i in (start - BODY_LONG.avg_period)..start {
        body_sum += cr(BODY_LONG, open, high, low, close, i);
    }
    for i in (start - SHADOW_VERY_SHORT.avg_period)..start {
        shadow_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i);
    }

    for i in start..len {
        let long_body =
            real_body(open[i], close[i]) > ca(BODY_LONG, body_sum, open, high, low, close, i);
        let bull = long_body
            && candle_color(open[i], close[i]) == 1
            && lower_shadow(open[i], low[i], close[i])
                < ca(SHADOW_VERY_SHORT, shadow_sum, open, high, low, close, i);
        let bear = long_body
            && candle_color(open[i], close[i]) == -1
            && upper_shadow(open[i], high[i], close[i])
                < ca(SHADOW_VERY_SHORT, shadow_sum, open, high, low, close, i);
        output[i] = (bull as i32) * 100 - (bear as i32) * 100;
        body_sum += cr(BODY_LONG, open, high, low, close, i)
            - cr(BODY_LONG, open, high, low, close, i - BODY_LONG.avg_period);
        shadow_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i)
            - cr(
                SHADOW_VERY_SHORT,
                open,
                high,
                low,
                close,
                i - SHADOW_VERY_SHORT.avg_period,
            );
    }
    Ok(output)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_batch() {
        let o: Vec<f64> = (0..45).map(|i| 100.0 + i as f64 * 0.2).collect();
        let h: Vec<f64> = o.iter().map(|x| x + 2.0).collect();
        let l: Vec<f64> = o.iter().map(|x| x - 2.0).collect();
        let c: Vec<f64> = o
            .iter()
            .enumerate()
            .map(|(i, x)| x + if i % 3 == 0 { -1.0 } else { 1.0 })
            .collect();
        let e = crate::stream::candle_belt_hold(&o, &h, &l, &c).unwrap();
        let mut s = CandleBeltHold::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            assert_eq!(s.append(o, h, l, c).unwrap_or(0), e)
        }
    }
}
