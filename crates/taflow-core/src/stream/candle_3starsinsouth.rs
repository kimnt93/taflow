//! Incremental Three Stars In The South recognition (CDL3STARSINSOUTH).
use super::pattern::*;
use crate::error::TaResult;
use std::collections::VecDeque;
#[derive(Clone, Copy)]
struct Candle {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}
impl Candle {
    fn body(self) -> f64 {
        (self.close - self.open).abs()
    }
    fn range(self) -> f64 {
        self.high - self.low
    }
    fn lower(self) -> f64 {
        self.open.min(self.close) - self.low
    }
    fn upper(self) -> f64 {
        self.high - self.open.max(self.close)
    }
    fn black(self) -> bool {
        self.close < self.open
    }
}
/// Incremental CDL3STARSINSOUTH state.
/// Persistent Rust state or aligned output type for `CandleThreeStarsInSouth`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleThreeStarsInSouth {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleThreeStarsInSouth {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleThreeStarsInSouth {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(12),
            value: None,
        }
    }
    fn avg_body(&self, start: usize) -> f64 {
        self.candles
            .iter()
            .skip(start)
            .take(10)
            .map(|c| c.body())
            .sum::<f64>()
            / 10.0
    }
    fn avg_range(&self, start: usize) -> f64 {
        self.candles
            .iter()
            .skip(start)
            .take(10)
            .map(|c| c.range())
            .sum::<f64>()
            * 0.01
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let current = Candle {
            open,
            high,
            low,
            close,
        };
        let output = if self.candles.len() == 12 {
            let first = self.candles[10];
            let second = self.candles[11];
            let pattern = first.black()
                && second.black()
                && current.black()
                && first.body() > self.avg_body(0)
                && first.lower() > first.body()
                && second.open.min(second.close) > first.open.min(first.close)
                && second.open.max(second.close) < first.open.max(first.close)
                && second.low < first.low
                && current.body() < self.avg_body(2)
                && current.upper() < self.avg_range(2)
                && current.lower() < self.avg_range(2)
                && current.low > second.low
                && current.high < second.high;
            Some((pattern as i32) * 100)
        } else {
            None
        };
        if self.candles.len() == 12 {
            self.candles.pop_front();
        }
        self.candles.push_back(current);
        self.value = output;
        output
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
        *self = Self::new()
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
/// Compute the candle three stars in south result for the supplied aligned series.
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
pub fn candle_three_stars_in_south(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = *[
        SHADOW_VERY_SHORT.avg_period,
        SHADOW_LONG.avg_period,
        BODY_LONG.avg_period,
        BODY_SHORT.avg_period,
    ]
    .iter()
    .max()
    .unwrap()
        + 2;
    if len <= lookback {
        return Ok(output);
    }

    let mut body_long_sum = 0.0;
    let mut shadow_long_sum = 0.0;
    let mut shadow_vs_sum = [0.0f64; 2]; // for 2nd and 3rd candles
    let mut body_short_sum = 0.0;
    let start = lookback;
    for i in (start - 2 - BODY_LONG.avg_period)..(start - 2) {
        body_long_sum += cr(BODY_LONG, open, high, low, close, i);
    }
    // SHADOW_LONG avg_period = 0, no init
    for i in (start - 1 - SHADOW_VERY_SHORT.avg_period)..(start - 1) {
        shadow_vs_sum[0] += cr(SHADOW_VERY_SHORT, open, high, low, close, i);
    }
    for i in (start - SHADOW_VERY_SHORT.avg_period)..start {
        shadow_vs_sum[1] += cr(SHADOW_VERY_SHORT, open, high, low, close, i);
    }
    for i in (start - BODY_SHORT.avg_period)..start {
        body_short_sum += cr(BODY_SHORT, open, high, low, close, i);
    }

    for i in start..len {
        output[i] = (candle_color(open[i-2], close[i-2]) == -1
            && candle_color(open[i-1], close[i-1]) == -1
            && candle_color(open[i], close[i]) == -1
            // 1st: long body, long lower shadow
            && real_body(open[i-2], close[i-2]) > ca(BODY_LONG, body_long_sum, open, high, low, close, i-2)
            && lower_shadow(open[i-2], low[i-2], close[i-2]) > ca(SHADOW_LONG, shadow_long_sum, open, high, low, close, i-2)
            // 2nd: body inside 1st, low < 1st low, short lower shadow
            && open[i-1].min(close[i-1]) > open[i-2].min(close[i-2])
            && open[i-1].max(close[i-1]) < open[i-2].max(close[i-2])
            && low[i-1] < low[i-2]
            // 3rd: short body, short shadows, within 2nd range
            && real_body(open[i], close[i]) < ca(BODY_SHORT, body_short_sum, open, high, low, close, i)
            && upper_shadow(open[i], high[i], close[i]) < ca(SHADOW_VERY_SHORT, shadow_vs_sum[1], open, high, low, close, i)
            && lower_shadow(open[i], low[i], close[i]) < ca(SHADOW_VERY_SHORT, shadow_vs_sum[1], open, high, low, close, i)
            && low[i] > low[i-1] && high[i] < high[i-1]) as i32
            * 100;
        body_long_sum += cr(BODY_LONG, open, high, low, close, i - 2)
            - cr(
                BODY_LONG,
                open,
                high,
                low,
                close,
                i - 2 - BODY_LONG.avg_period,
            );
        shadow_vs_sum[0] += cr(SHADOW_VERY_SHORT, open, high, low, close, i - 1)
            - cr(
                SHADOW_VERY_SHORT,
                open,
                high,
                low,
                close,
                i - 1 - SHADOW_VERY_SHORT.avg_period,
            );
        shadow_vs_sum[1] += cr(SHADOW_VERY_SHORT, open, high, low, close, i)
            - cr(
                SHADOW_VERY_SHORT,
                open,
                high,
                low,
                close,
                i - SHADOW_VERY_SHORT.avg_period,
            );
        body_short_sum += cr(BODY_SHORT, open, high, low, close, i)
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
        let open: Vec<f64> = (0..30).map(|i| 100. + i as f64 * 0.1).collect();
        let high: Vec<f64> = open.iter().map(|x| x + 2.).collect();
        let low: Vec<f64> = open.iter().map(|x| x - 2.).collect();
        let close: Vec<f64> = open.iter().map(|x| x + 1.).collect();
        let e = crate::stream::candle_three_stars_in_south(&open, &high, &low, &close).unwrap();
        let mut s = CandleThreeStarsInSouth::new();
        for (((&o, &h), &l), (&c, &e)) in open.iter().zip(&high).zip(&low).zip(close.iter().zip(&e))
        {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
