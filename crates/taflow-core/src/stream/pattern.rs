// Candlestick pattern recognition — 61 patterns
// Exact reimplementation of C TA-Lib's CandleAverage system
// Output: Vec<i32>, -100 = bearish, 0 = no signal, +100 = bullish

use crate::error::{TaError, TaResult};

// ========== C TA-Lib Candle Setting System ==========

/// Range types used by candle settings
#[derive(Clone, Copy)]
enum RangeType {
    RealBody,
    HighLow,
    Shadows,
}

/// A candle setting definition matching C TA-Lib defaults
#[derive(Clone, Copy)]
struct CandleSetting {
    range_type: RangeType,
    avg_period: usize,
    factor: f64,
}

// Default candle settings exactly matching C TA-Lib ta_common.c
const BODY_LONG: CandleSetting = CandleSetting { range_type: RangeType::RealBody, avg_period: 10, factor: 1.0 };
const BODY_VERY_LONG: CandleSetting = CandleSetting { range_type: RangeType::RealBody, avg_period: 10, factor: 3.0 };
const BODY_SHORT: CandleSetting = CandleSetting { range_type: RangeType::RealBody, avg_period: 10, factor: 1.0 };
const BODY_DOJI: CandleSetting = CandleSetting { range_type: RangeType::HighLow, avg_period: 10, factor: 0.1 };
const SHADOW_LONG: CandleSetting = CandleSetting { range_type: RangeType::RealBody, avg_period: 0, factor: 1.0 };
const SHADOW_VERY_LONG: CandleSetting = CandleSetting { range_type: RangeType::RealBody, avg_period: 0, factor: 2.0 };
const SHADOW_SHORT: CandleSetting = CandleSetting { range_type: RangeType::Shadows, avg_period: 10, factor: 1.0 };
const SHADOW_VERY_SHORT: CandleSetting = CandleSetting { range_type: RangeType::HighLow, avg_period: 10, factor: 0.1 };
const NEAR: CandleSetting = CandleSetting { range_type: RangeType::HighLow, avg_period: 5, factor: 0.2 };
const FAR: CandleSetting = CandleSetting { range_type: RangeType::HighLow, avg_period: 5, factor: 0.6 };
const EQUAL: CandleSetting = CandleSetting { range_type: RangeType::HighLow, avg_period: 5, factor: 0.05 };

// ========== Helper Functions ==========

#[inline(always)]
fn real_body(open: f64, close: f64) -> f64 {
    (close - open).abs()
}

#[inline(always)]
fn upper_shadow(open: f64, high: f64, close: f64) -> f64 {
    high - open.max(close)
}

#[inline(always)]
fn lower_shadow(open: f64, low: f64, close: f64) -> f64 {
    open.min(close) - low
}

#[inline(always)]
fn candle_color(open: f64, close: f64) -> i32 {
    if close >= open { 1 } else { -1 }
}

// ========== Monomorphized range/average functions ==========
// Each RangeType gets its own function to eliminate runtime match dispatch.
// These inline to 2-3 instructions — equivalent to C TA-Lib macros.

#[inline(always)]
fn range_realbody(_o: f64, _h: f64, _l: f64, c: f64, o: f64) -> f64 { (c - o).abs() }
#[inline(always)]
fn range_highlow(_o: f64, h: f64, l: f64, _c: f64, _o2: f64) -> f64 { h - l }
#[inline(always)]
fn range_shadows(_o: f64, h: f64, l: f64, c: f64, o: f64) -> f64 { (h - l) - (c - o).abs() }

/// Compute the range value for a single bar based on the setting's range_type
#[inline(always)]
fn candle_range(setting: CandleSetting, open: f64, high: f64, low: f64, close: f64) -> f64 {
    match setting.range_type {
        RangeType::RealBody => (close - open).abs(),
        RangeType::HighLow => high - low,
        RangeType::Shadows => (high - low) - (close - open).abs(),
    }
}

/// Compute candle average = factor * (sum / avg_period) / divisor
/// When avg_period == 0, use the current bar's range value directly (no averaging)
/// NOTE: C TA-Lib divides by 2.0 when range_type is Shadows
#[inline(always)]
fn candle_average(setting: CandleSetting, sum: f64, open: f64, high: f64, low: f64, close: f64) -> f64 {
    let divisor = match setting.range_type {
        RangeType::Shadows => 2.0,
        _ => 1.0,
    };
    if setting.avg_period > 0 {
        setting.factor * (sum / setting.avg_period as f64) / divisor
    } else {
        setting.factor * candle_range(setting, open, high, low, close) / divisor
    }
}

// ---- Monomorphized cr/ca per CandleSetting constant ----
// Eliminates match dispatch in hot loops by hardcoding the range_type.

// RealBody types: BODY_LONG, BODY_VERY_LONG, BODY_SHORT, SHADOW_LONG, SHADOW_VERY_LONG
#[inline(always)]
fn cr_realbody(o: &[f64], _h: &[f64], _l: &[f64], c: &[f64], i: usize) -> f64 {
    (c[i] - o[i]).abs()
}
#[inline(always)]
fn ca_realbody(setting: CandleSetting, sum: f64, o: &[f64], _h: &[f64], _l: &[f64], c: &[f64], i: usize) -> f64 {
    if setting.avg_period > 0 {
        setting.factor * (sum / setting.avg_period as f64)
    } else {
        setting.factor * (c[i] - o[i]).abs()
    }
}

// HighLow types: BODY_DOJI, SHADOW_VERY_SHORT, NEAR, FAR, EQUAL
#[inline(always)]
fn cr_highlow(_o: &[f64], h: &[f64], l: &[f64], _c: &[f64], i: usize) -> f64 {
    h[i] - l[i]
}
#[inline(always)]
fn ca_highlow(setting: CandleSetting, sum: f64, _o: &[f64], h: &[f64], l: &[f64], _c: &[f64], i: usize) -> f64 {
    if setting.avg_period > 0 {
        setting.factor * (sum / setting.avg_period as f64)
    } else {
        setting.factor * (h[i] - l[i])
    }
}

// Shadows type: SHADOW_SHORT
#[inline(always)]
fn cr_shadows(o: &[f64], h: &[f64], l: &[f64], c: &[f64], i: usize) -> f64 {
    (h[i] - l[i]) - (c[i] - o[i]).abs()
}
#[inline(always)]
fn ca_shadows(setting: CandleSetting, sum: f64, o: &[f64], h: &[f64], l: &[f64], c: &[f64], i: usize) -> f64 {
    if setting.avg_period > 0 {
        setting.factor * (sum / setting.avg_period as f64) / 2.0
    } else {
        setting.factor * ((h[i] - l[i]) - (c[i] - o[i]).abs()) / 2.0
    }
}

/// Generic cr/ca — still available for rare/complex patterns
#[inline(always)]
fn cr(setting: CandleSetting, o: &[f64], h: &[f64], l: &[f64], c: &[f64], i: usize) -> f64 {
    candle_range(setting, o[i], h[i], l[i], c[i])
}

#[inline(always)]
fn ca(setting: CandleSetting, sum: f64, o: &[f64], h: &[f64], l: &[f64], c: &[f64], i: usize) -> f64 {
    candle_average(setting, sum, o[i], h[i], l[i], c[i])
}

/// Helper: real body gap up (min(o,c) of bar2 > max(o,c) of bar1)
#[inline]
fn real_body_gap_up(o: &[f64], c: &[f64], bar2: usize, bar1: usize) -> bool {
    o[bar2].min(c[bar2]) > o[bar1].max(c[bar1])
}

/// Helper: real body gap down (max(o,c) of bar2 < min(o,c) of bar1)
#[inline]
fn real_body_gap_down(o: &[f64], c: &[f64], bar2: usize, bar1: usize) -> bool {
    o[bar2].max(c[bar2]) < o[bar1].min(c[bar1])
}

/// Validate OHLC arrays have same length
fn validate_ohlc(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<usize> {
    let len = open.len();
    if len != high.len() || len != low.len() || len != close.len() {
        return Err(TaError::LengthMismatch {
            expected: len,
            got: high.len().min(low.len()).min(close.len()),
        });
    }
    Ok(len)
}

// ========== Pattern Functions ==========

/// CDL_DOJI — copysign-based branchless output
///
/// Uses `100.0_f64.copysign(thresh - body).max(0.0) as i32` to produce 0 or 100
/// without any conditional branch. This stays entirely in float registers (NEON fmaxnm),
/// avoiding the conditional-store penalty that LLVM generates for bool→i32 patterns.
/// Compute the candle pattern signal for aligned OHLC bars.
///
/// # Parameters
///
/// * `open`, `high`, `low`, `close` - Equal-length chronological OHLC series.
///
/// # Returns
///
/// A same-length vector containing -100, 0, or 100 pattern signals; bars
/// Compute the candle doji result for the supplied aligned series.
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
pub fn candle_doji(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_DOJI.avg_period; // 10
    if len <= lookback { return Ok(output); }

    let factor_div = BODY_DOJI.factor / lookback as f64; // 0.01

    let mut sum = 0.0_f64;
    for i in 0..lookback {
        sum += high[i] - low[i];
    }

    for i in lookback..len {
        let body = (close[i] - open[i]).abs();
        let thresh = sum * factor_div;
        // copysign(100, thresh-body): +100 if doji (body<=thresh), -100 if not
        // max(0): clamp -100 to 0 → result is 0 or 100, zero branches
        output[i] = 100.0_f64.copysign(thresh - body).max(0.0) as i32;
        sum += (high[i] - low[i]) - (high[i - lookback] - low[i - lookback]);
    }

    Ok(output)
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
/// Compute the candle hammer result for the supplied aligned series.
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
pub fn candle_hammer(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = *[BODY_SHORT.avg_period, SHADOW_LONG.avg_period, SHADOW_VERY_SHORT.avg_period, NEAR.avg_period].iter().max().unwrap() + 1;
    if len <= lookback { return Ok(output); }

    let mut body_sum = 0.0;
    let mut shadow_long_sum = 0.0;
    let mut shadow_vs_sum = 0.0;
    let mut near_sum = 0.0;

    let start = lookback;
    // BODY_SHORT: RealBody, SHADOW_LONG: RealBody(avg=0), SHADOW_VERY_SHORT: HighLow, NEAR: HighLow
    for i in (start - BODY_SHORT.avg_period)..start { body_sum += cr_realbody(open, high, low, close, i); }
    for i in (start - SHADOW_VERY_SHORT.avg_period)..start { shadow_vs_sum += cr_highlow(open, high, low, close, i); }
    for i in (start - 1 - NEAR.avg_period)..(start - 1) { near_sum += cr_highlow(open, high, low, close, i); }

    for i in start..len {
        output[i] = (real_body(open[i], close[i]) < ca_realbody(BODY_SHORT, body_sum, open, high, low, close, i)
            && lower_shadow(open[i], low[i], close[i]) > ca_realbody(SHADOW_LONG, shadow_long_sum, open, high, low, close, i)
            && upper_shadow(open[i], high[i], close[i]) < ca_highlow(SHADOW_VERY_SHORT, shadow_vs_sum, open, high, low, close, i)
            && open[i].min(close[i]) <= low[i-1] + ca_highlow(NEAR, near_sum, open, high, low, close, i-1)) as i32 * 100;
        // Update sums — monomorphized: no match dispatch
        if BODY_SHORT.avg_period > 0 { body_sum += cr_realbody(open, high, low, close, i) - cr_realbody(open, high, low, close, i - BODY_SHORT.avg_period); }
        if SHADOW_VERY_SHORT.avg_period > 0 { shadow_vs_sum += cr_highlow(open, high, low, close, i) - cr_highlow(open, high, low, close, i - SHADOW_VERY_SHORT.avg_period); }
        if NEAR.avg_period > 0 { near_sum += cr_highlow(open, high, low, close, i-1) - cr_highlow(open, high, low, close, i - 1 - NEAR.avg_period); }
    }
    Ok(output)
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
/// Compute the candle engulfing result for the supplied aligned series.
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
pub fn candle_engulfing(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    // Engulfing has no candle settings, lookback = 2 (need i-1)
    if len < 2 { return Ok(output); }

    for i in 1..len {
        // Bullish: prev black, curr white, curr close >= prev open, curr open <= prev close
        let bull = candle_color(open[i-1], close[i-1]) == -1
            && candle_color(open[i], close[i]) == 1
            && close[i] >= open[i-1]
            && open[i] <= close[i-1];
        // Bearish: prev white, curr black, curr open >= prev close, curr close <= prev open
        let bear = candle_color(open[i-1], close[i-1]) == 1
            && candle_color(open[i], close[i]) == -1
            && open[i] >= close[i-1]
            && close[i] <= open[i-1];
        output[i] = (bull as i32) * 100 - (bear as i32) * 100;
    }
    Ok(output)
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
/// Compute the candle closing marubozu result for the supplied aligned series.
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
pub fn candle_closing_marubozu(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_LONG.avg_period.max(SHADOW_VERY_SHORT.avg_period);
    if len <= lookback { return Ok(output); }

    let mut body_sum = 0.0;
    let mut shadow_sum = 0.0;
    let start = lookback;
    for i in (start - BODY_LONG.avg_period)..start { body_sum += cr(BODY_LONG, open, high, low, close, i); }
    for i in (start - SHADOW_VERY_SHORT.avg_period)..start { shadow_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i); }

    for i in start..len {
        let long_body = real_body(open[i], close[i]) > ca(BODY_LONG, body_sum, open, high, low, close, i);
        let bull = long_body
            && candle_color(open[i], close[i]) == 1
            && upper_shadow(open[i], high[i], close[i]) < ca(SHADOW_VERY_SHORT, shadow_sum, open, high, low, close, i);
        let bear = long_body
            && candle_color(open[i], close[i]) == -1
            && lower_shadow(open[i], low[i], close[i]) < ca(SHADOW_VERY_SHORT, shadow_sum, open, high, low, close, i);
        output[i] = (bull as i32) * 100 - (bear as i32) * 100;
        body_sum += cr(BODY_LONG, open, high, low, close, i) - cr(BODY_LONG, open, high, low, close, i - BODY_LONG.avg_period);
        shadow_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i) - cr(SHADOW_VERY_SHORT, open, high, low, close, i - SHADOW_VERY_SHORT.avg_period);
    }
    Ok(output)
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
/// Compute the candle dragonfly doji result for the supplied aligned series.
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
pub fn candle_dragonfly_doji(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_DOJI.avg_period.max(SHADOW_VERY_SHORT.avg_period);
    if len <= lookback { return Ok(output); }

    let mut body_sum = 0.0;
    let mut shadow_sum = 0.0;
    let start = lookback;
    for i in (start - BODY_DOJI.avg_period)..start { body_sum += cr(BODY_DOJI, open, high, low, close, i); }
    for i in (start - SHADOW_VERY_SHORT.avg_period)..start { shadow_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i); }

    for i in start..len {
        output[i] = (real_body(open[i], close[i]) <= ca(BODY_DOJI, body_sum, open, high, low, close, i)
            && upper_shadow(open[i], high[i], close[i]) < ca(SHADOW_VERY_SHORT, shadow_sum, open, high, low, close, i)
            && lower_shadow(open[i], low[i], close[i]) > ca(SHADOW_VERY_SHORT, shadow_sum, open, high, low, close, i)) as i32 * 100;
        body_sum += cr(BODY_DOJI, open, high, low, close, i) - cr(BODY_DOJI, open, high, low, close, i - BODY_DOJI.avg_period);
        shadow_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i) - cr(SHADOW_VERY_SHORT, open, high, low, close, i - SHADOW_VERY_SHORT.avg_period);
    }
    Ok(output)
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
/// Compute the candle gravestone doji result for the supplied aligned series.
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
pub fn candle_gravestone_doji(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_DOJI.avg_period.max(SHADOW_VERY_SHORT.avg_period);
    if len <= lookback { return Ok(output); }

    let mut body_sum = 0.0;
    let mut shadow_sum = 0.0;
    let start = lookback;
    for i in (start - BODY_DOJI.avg_period)..start { body_sum += cr(BODY_DOJI, open, high, low, close, i); }
    for i in (start - SHADOW_VERY_SHORT.avg_period)..start { shadow_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i); }

    for i in start..len {
        output[i] = (real_body(open[i], close[i]) <= ca(BODY_DOJI, body_sum, open, high, low, close, i)
            && lower_shadow(open[i], low[i], close[i]) < ca(SHADOW_VERY_SHORT, shadow_sum, open, high, low, close, i)
            && upper_shadow(open[i], high[i], close[i]) > ca(SHADOW_VERY_SHORT, shadow_sum, open, high, low, close, i)) as i32 * 100;
        body_sum += cr(BODY_DOJI, open, high, low, close, i) - cr(BODY_DOJI, open, high, low, close, i - BODY_DOJI.avg_period);
        shadow_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i) - cr(SHADOW_VERY_SHORT, open, high, low, close, i - SHADOW_VERY_SHORT.avg_period);
    }
    Ok(output)
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
/// Compute the candle high wave result for the supplied aligned series.
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
pub fn candle_high_wave(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_SHORT.avg_period.max(SHADOW_VERY_LONG.avg_period);
    if len <= lookback { return Ok(output); }

    let mut body_sum = 0.0;
    let mut shadow_sum = 0.0;
    let start = lookback;
    for i in (start - BODY_SHORT.avg_period)..start { body_sum += cr(BODY_SHORT, open, high, low, close, i); }
    // SHADOW_VERY_LONG avg_period=0, no init needed

    for i in start..len {
        output[i] = (real_body(open[i], close[i]) < ca(BODY_SHORT, body_sum, open, high, low, close, i)
            && upper_shadow(open[i], high[i], close[i]) > ca(SHADOW_VERY_LONG, shadow_sum, open, high, low, close, i)
            && lower_shadow(open[i], low[i], close[i]) > ca(SHADOW_VERY_LONG, shadow_sum, open, high, low, close, i)) as i32 * candle_color(open[i], close[i]) * 100;
        if BODY_SHORT.avg_period > 0 { body_sum += cr(BODY_SHORT, open, high, low, close, i) - cr(BODY_SHORT, open, high, low, close, i - BODY_SHORT.avg_period); }
    }
    Ok(output)
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
/// Compute the candle long legged doji result for the supplied aligned series.
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
pub fn candle_long_legged_doji(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_DOJI.avg_period.max(SHADOW_LONG.avg_period);
    if len <= lookback { return Ok(output); }

    let mut body_sum = 0.0;
    let mut shadow_sum = 0.0;
    let start = lookback;
    for i in (start - BODY_DOJI.avg_period)..start { body_sum += cr(BODY_DOJI, open, high, low, close, i); }
    // SHADOW_LONG avg_period=0, no init

    for i in start..len {
        output[i] = (real_body(open[i], close[i]) <= ca(BODY_DOJI, body_sum, open, high, low, close, i)
            && (lower_shadow(open[i], low[i], close[i]) > ca(SHADOW_LONG, shadow_sum, open, high, low, close, i)
                || upper_shadow(open[i], high[i], close[i]) > ca(SHADOW_LONG, shadow_sum, open, high, low, close, i))) as i32 * 100;
        body_sum += cr(BODY_DOJI, open, high, low, close, i) - cr(BODY_DOJI, open, high, low, close, i - BODY_DOJI.avg_period);
    }
    Ok(output)
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
/// Compute the candle long line result for the supplied aligned series.
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
pub fn candle_long_line(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_LONG.avg_period.max(SHADOW_SHORT.avg_period);
    if len <= lookback { return Ok(output); }

    let mut body_sum = 0.0;
    let mut shadow_sum = 0.0;
    let start = lookback;
    for i in (start - BODY_LONG.avg_period)..start { body_sum += cr(BODY_LONG, open, high, low, close, i); }
    for i in (start - SHADOW_SHORT.avg_period)..start { shadow_sum += cr(SHADOW_SHORT, open, high, low, close, i); }

    for i in start..len {
        output[i] = (real_body(open[i], close[i]) > ca(BODY_LONG, body_sum, open, high, low, close, i)
            && upper_shadow(open[i], high[i], close[i]) < ca(SHADOW_SHORT, shadow_sum, open, high, low, close, i)
            && lower_shadow(open[i], low[i], close[i]) < ca(SHADOW_SHORT, shadow_sum, open, high, low, close, i)) as i32 * candle_color(open[i], close[i]) * 100;
        body_sum += cr(BODY_LONG, open, high, low, close, i) - cr(BODY_LONG, open, high, low, close, i - BODY_LONG.avg_period);
        shadow_sum += cr(SHADOW_SHORT, open, high, low, close, i) - cr(SHADOW_SHORT, open, high, low, close, i - SHADOW_SHORT.avg_period);
    }
    Ok(output)
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
/// Compute the candle marubozu result for the supplied aligned series.
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
pub fn candle_marubozu(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_LONG.avg_period.max(SHADOW_VERY_SHORT.avg_period);
    if len <= lookback { return Ok(output); }

    let mut body_sum = 0.0;
    let mut shadow_sum = 0.0;
    let start = lookback;
    for i in (start - BODY_LONG.avg_period)..start { body_sum += cr(BODY_LONG, open, high, low, close, i); }
    for i in (start - SHADOW_VERY_SHORT.avg_period)..start { shadow_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i); }

    for i in start..len {
        output[i] = (real_body(open[i], close[i]) > ca(BODY_LONG, body_sum, open, high, low, close, i)
            && upper_shadow(open[i], high[i], close[i]) < ca(SHADOW_VERY_SHORT, shadow_sum, open, high, low, close, i)
            && lower_shadow(open[i], low[i], close[i]) < ca(SHADOW_VERY_SHORT, shadow_sum, open, high, low, close, i)) as i32 * candle_color(open[i], close[i]) * 100;
        body_sum += cr(BODY_LONG, open, high, low, close, i) - cr(BODY_LONG, open, high, low, close, i - BODY_LONG.avg_period);
        shadow_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i) - cr(SHADOW_VERY_SHORT, open, high, low, close, i - SHADOW_VERY_SHORT.avg_period);
    }
    Ok(output)
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
/// Compute the candle rickshawman result for the supplied aligned series.
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
pub fn candle_rickshawman(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = *[BODY_DOJI.avg_period, SHADOW_LONG.avg_period, NEAR.avg_period].iter().max().unwrap();
    if len <= lookback { return Ok(output); }

    let mut body_sum = 0.0;
    let mut shadow_sum = 0.0;
    let mut near_sum = 0.0;
    let start = lookback;
    for i in (start - BODY_DOJI.avg_period)..start { body_sum += cr(BODY_DOJI, open, high, low, close, i); }
    // SHADOW_LONG avg_period=0
    for i in (start - NEAR.avg_period)..start { near_sum += cr(NEAR, open, high, low, close, i); }

    for i in start..len {
        let mid = low[i] + (high[i] - low[i]) / 2.0;
        let near_avg = ca(NEAR, near_sum, open, high, low, close, i);
        output[i] = (real_body(open[i], close[i]) <= ca(BODY_DOJI, body_sum, open, high, low, close, i)
            && lower_shadow(open[i], low[i], close[i]) > ca(SHADOW_LONG, shadow_sum, open, high, low, close, i)
            && upper_shadow(open[i], high[i], close[i]) > ca(SHADOW_LONG, shadow_sum, open, high, low, close, i)
            && open[i].min(close[i]) <= mid + near_avg
            && open[i].max(close[i]) >= mid - near_avg) as i32 * 100;
        body_sum += cr(BODY_DOJI, open, high, low, close, i) - cr(BODY_DOJI, open, high, low, close, i - BODY_DOJI.avg_period);
        if NEAR.avg_period > 0 { near_sum += cr(NEAR, open, high, low, close, i) - cr(NEAR, open, high, low, close, i - NEAR.avg_period); }
    }
    Ok(output)
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
pub fn candle_short_line(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_SHORT.avg_period.max(SHADOW_SHORT.avg_period);
    if len <= lookback { return Ok(output); }

    let mut body_sum = 0.0;
    let mut shadow_sum = 0.0;
    let start = lookback;
    for i in (start - BODY_SHORT.avg_period)..start { body_sum += cr(BODY_SHORT, open, high, low, close, i); }
    for i in (start - SHADOW_SHORT.avg_period)..start { shadow_sum += cr(SHADOW_SHORT, open, high, low, close, i); }

    for i in start..len {
        output[i] = (real_body(open[i], close[i]) < ca(BODY_SHORT, body_sum, open, high, low, close, i)
            && upper_shadow(open[i], high[i], close[i]) < ca(SHADOW_SHORT, shadow_sum, open, high, low, close, i)
            && lower_shadow(open[i], low[i], close[i]) < ca(SHADOW_SHORT, shadow_sum, open, high, low, close, i)) as i32 * candle_color(open[i], close[i]) * 100;
        body_sum += cr(BODY_SHORT, open, high, low, close, i) - cr(BODY_SHORT, open, high, low, close, i - BODY_SHORT.avg_period);
        shadow_sum += cr(SHADOW_SHORT, open, high, low, close, i) - cr(SHADOW_SHORT, open, high, low, close, i - SHADOW_SHORT.avg_period);
    }
    Ok(output)
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
pub fn candle_spinningtop(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_SHORT.avg_period;
    if len <= lookback { return Ok(output); }

    let mut body_sum = 0.0;
    let start = lookback;
    for i in (start - BODY_SHORT.avg_period)..start { body_sum += cr(BODY_SHORT, open, high, low, close, i); }

    for i in start..len {
        output[i] = (real_body(open[i], close[i]) < ca(BODY_SHORT, body_sum, open, high, low, close, i)
            && upper_shadow(open[i], high[i], close[i]) > real_body(open[i], close[i])
            && lower_shadow(open[i], low[i], close[i]) > real_body(open[i], close[i])) as i32 * candle_color(open[i], close[i]) * 100;
        body_sum += cr(BODY_SHORT, open, high, low, close, i) - cr(BODY_SHORT, open, high, low, close, i - BODY_SHORT.avg_period);
    }
    Ok(output)
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
/// Compute the candle takuri result for the supplied aligned series.
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
pub fn candle_takuri(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = *[BODY_DOJI.avg_period, SHADOW_VERY_SHORT.avg_period, SHADOW_VERY_LONG.avg_period].iter().max().unwrap();
    if len <= lookback { return Ok(output); }

    let mut body_sum = 0.0;
    let mut shadow_vs_sum = 0.0;
    let mut shadow_vl_sum = 0.0;
    let start = lookback;
    for i in (start - BODY_DOJI.avg_period)..start { body_sum += cr(BODY_DOJI, open, high, low, close, i); }
    for i in (start - SHADOW_VERY_SHORT.avg_period)..start { shadow_vs_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i); }
    // SHADOW_VERY_LONG avg_period=0

    for i in start..len {
        output[i] = (real_body(open[i], close[i]) <= ca(BODY_DOJI, body_sum, open, high, low, close, i)
            && upper_shadow(open[i], high[i], close[i]) < ca(SHADOW_VERY_SHORT, shadow_vs_sum, open, high, low, close, i)
            && lower_shadow(open[i], low[i], close[i]) > ca(SHADOW_VERY_LONG, shadow_vl_sum, open, high, low, close, i)) as i32 * 100;
        body_sum += cr(BODY_DOJI, open, high, low, close, i) - cr(BODY_DOJI, open, high, low, close, i - BODY_DOJI.avg_period);
        shadow_vs_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i) - cr(SHADOW_VERY_SHORT, open, high, low, close, i - SHADOW_VERY_SHORT.avg_period);
    }
    Ok(output)
}

// ========== Two-candle patterns ==========

/// Compute the candle pattern signal for aligned OHLC bars.
///
/// # Parameters
///
/// * `open`, `high`, `low`, `close` - Equal-length chronological OHLC series.
///
/// # Returns
///
/// A same-length vector containing -100, 0, or 100 pattern signals; bars
/// Compute the candle two crows result for the supplied aligned series.
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
pub fn candle_two_crows(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_LONG.avg_period + 2;
    if len <= lookback { return Ok(output); }

    let mut body_sum = 0.0;
    let start = lookback;
    for i in (start - 2 - BODY_LONG.avg_period)..(start - 2) { body_sum += cr(BODY_LONG, open, high, low, close, i); }

    for i in start..len {
        // 1st: long white
        output[i] = (candle_color(open[i-2], close[i-2]) == 1
            && real_body(open[i-2], close[i-2]) > ca(BODY_LONG, body_sum, open, high, low, close, i-2)
            // 2nd: black, gap up
            && candle_color(open[i-1], close[i-1]) == -1
            && real_body_gap_up(open, close, i-1, i-2)
            // 3rd: black, opens within 2nd body, closes within 1st body
            && candle_color(open[i], close[i]) == -1
            && open[i] < open[i-1] && open[i] > close[i-1]
            && close[i] > open[i-2] && close[i] < close[i-2]) as i32 * -100;
        body_sum += cr(BODY_LONG, open, high, low, close, i-2) - cr(BODY_LONG, open, high, low, close, i - 2 - BODY_LONG.avg_period);
    }
    Ok(output)
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
/// Compute the candle counterattack result for the supplied aligned series.
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
pub fn candle_counterattack(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = EQUAL.avg_period.max(BODY_LONG.avg_period) + 1;
    if len <= lookback { return Ok(output); }

    let mut equal_sum = 0.0;
    let mut body_sum = [0.0f64; 2]; // [0]=current, [1]=prev
    let start = lookback;
    for i in (start - 1 - EQUAL.avg_period)..(start - 1) { equal_sum += cr(EQUAL, open, high, low, close, i); }
    for i in (start - 1 - BODY_LONG.avg_period)..(start - 1) { body_sum[1] += cr(BODY_LONG, open, high, low, close, i); }
    for i in (start - BODY_LONG.avg_period)..start { body_sum[0] += cr(BODY_LONG, open, high, low, close, i); }

    for i in start..len {
        output[i] = (candle_color(open[i-1], close[i-1]) != candle_color(open[i], close[i])
            && real_body(open[i-1], close[i-1]) > ca(BODY_LONG, body_sum[1], open, high, low, close, i-1)
            && real_body(open[i], close[i]) > ca(BODY_LONG, body_sum[0], open, high, low, close, i)
            && (close[i] - close[i-1]).abs() <= ca(EQUAL, equal_sum, open, high, low, close, i-1)) as i32 * candle_color(open[i], close[i]) * 100;
        equal_sum += cr(EQUAL, open, high, low, close, i-1) - cr(EQUAL, open, high, low, close, i - 1 - EQUAL.avg_period);
        body_sum[1] += cr(BODY_LONG, open, high, low, close, i-1) - cr(BODY_LONG, open, high, low, close, i - 1 - BODY_LONG.avg_period);
        body_sum[0] += cr(BODY_LONG, open, high, low, close, i) - cr(BODY_LONG, open, high, low, close, i - BODY_LONG.avg_period);
    }
    Ok(output)
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
/// Compute the candle dark cloud cover result for the supplied aligned series.
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
pub fn candle_dark_cloud_cover(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let penetration = 0.5;
    let lookback = BODY_LONG.avg_period + 1;
    if len <= lookback { return Ok(output); }

    let mut body_sum = 0.0;
    let start = lookback;
    for i in (start - 1 - BODY_LONG.avg_period)..(start - 1) { body_sum += cr(BODY_LONG, open, high, low, close, i); }

    for i in start..len {
        output[i] = (candle_color(open[i-1], close[i-1]) == 1
            && real_body(open[i-1], close[i-1]) > ca(BODY_LONG, body_sum, open, high, low, close, i-1)
            && candle_color(open[i], close[i]) == -1
            && open[i] > high[i-1]
            && close[i] > open[i-1]
            && close[i] < close[i-1] - real_body(open[i-1], close[i-1]) * penetration) as i32 * -100;
        body_sum += cr(BODY_LONG, open, high, low, close, i-1) - cr(BODY_LONG, open, high, low, close, i - 1 - BODY_LONG.avg_period);
    }
    Ok(output)
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
/// Compute the candle doji star result for the supplied aligned series.
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
pub fn candle_doji_star(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_DOJI.avg_period.max(BODY_LONG.avg_period) + 1;
    if len <= lookback { return Ok(output); }

    let mut body_long_sum = 0.0;
    let mut body_doji_sum = 0.0;
    let start = lookback;
    for i in (start - 1 - BODY_LONG.avg_period)..(start - 1) { body_long_sum += cr(BODY_LONG, open, high, low, close, i); }
    for i in (start - BODY_DOJI.avg_period)..start { body_doji_sum += cr(BODY_DOJI, open, high, low, close, i); }

    for i in start..len {
        let base = real_body(open[i-1], close[i-1]) > ca(BODY_LONG, body_long_sum, open, high, low, close, i-1)
            && real_body(open[i], close[i]) <= ca(BODY_DOJI, body_doji_sum, open, high, low, close, i);
        let bear = base && candle_color(open[i-1], close[i-1]) == 1 && real_body_gap_up(open, close, i, i-1);
        let bull = base && candle_color(open[i-1], close[i-1]) == -1 && real_body_gap_down(open, close, i, i-1);
        output[i] = (bull as i32) * 100 - (bear as i32) * 100;
        body_long_sum += cr(BODY_LONG, open, high, low, close, i-1) - cr(BODY_LONG, open, high, low, close, i - 1 - BODY_LONG.avg_period);
        body_doji_sum += cr(BODY_DOJI, open, high, low, close, i) - cr(BODY_DOJI, open, high, low, close, i - BODY_DOJI.avg_period);
    }
    Ok(output)
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
/// Compute the candle hanging man result for the supplied aligned series.
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
pub fn candle_hanging_man(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = *[BODY_SHORT.avg_period, SHADOW_LONG.avg_period, SHADOW_VERY_SHORT.avg_period, NEAR.avg_period].iter().max().unwrap() + 1;
    if len <= lookback { return Ok(output); }

    let mut body_sum = 0.0;
    let mut shadow_long_sum = 0.0;
    let mut shadow_vs_sum = 0.0;
    let mut near_sum = 0.0;
    let start = lookback;
    for i in (start - BODY_SHORT.avg_period)..start { body_sum += cr(BODY_SHORT, open, high, low, close, i); }
    for i in (start - SHADOW_VERY_SHORT.avg_period)..start { shadow_vs_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i); }
    for i in (start - 1 - NEAR.avg_period)..(start - 1) { near_sum += cr(NEAR, open, high, low, close, i); }

    for i in start..len {
        output[i] = (real_body(open[i], close[i]) < ca(BODY_SHORT, body_sum, open, high, low, close, i)
            && lower_shadow(open[i], low[i], close[i]) > ca(SHADOW_LONG, shadow_long_sum, open, high, low, close, i)
            && upper_shadow(open[i], high[i], close[i]) < ca(SHADOW_VERY_SHORT, shadow_vs_sum, open, high, low, close, i)
            && open[i].min(close[i]) >= high[i-1] - ca(NEAR, near_sum, open, high, low, close, i-1)) as i32 * -100;
        if BODY_SHORT.avg_period > 0 { body_sum += cr(BODY_SHORT, open, high, low, close, i) - cr(BODY_SHORT, open, high, low, close, i - BODY_SHORT.avg_period); }
        if SHADOW_VERY_SHORT.avg_period > 0 { shadow_vs_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i) - cr(SHADOW_VERY_SHORT, open, high, low, close, i - SHADOW_VERY_SHORT.avg_period); }
        if NEAR.avg_period > 0 { near_sum += cr(NEAR, open, high, low, close, i-1) - cr(NEAR, open, high, low, close, i - 1 - NEAR.avg_period); }
    }
    Ok(output)
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
/// Compute the candle harami result for the supplied aligned series.
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
pub fn candle_harami(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_SHORT.avg_period.max(BODY_LONG.avg_period) + 1;
    if len <= lookback { return Ok(output); }

    let mut body_long_sum = 0.0;
    let mut body_short_sum = 0.0;
    let start = lookback;
    for i in (start - 1 - BODY_LONG.avg_period)..(start - 1) { body_long_sum += cr(BODY_LONG, open, high, low, close, i); }
    for i in (start - BODY_SHORT.avg_period)..start { body_short_sum += cr(BODY_SHORT, open, high, low, close, i); }

    for i in start..len {
        output[i] = (real_body(open[i-1], close[i-1]) > ca(BODY_LONG, body_long_sum, open, high, low, close, i-1)
            && real_body(open[i], close[i]) <= ca(BODY_SHORT, body_short_sum, open, high, low, close, i)
            && open[i].max(close[i]) < open[i-1].max(close[i-1])
            && open[i].min(close[i]) > open[i-1].min(close[i-1])) as i32 * -candle_color(open[i-1], close[i-1]) * 100;
        body_long_sum += cr(BODY_LONG, open, high, low, close, i-1) - cr(BODY_LONG, open, high, low, close, i - 1 - BODY_LONG.avg_period);
        body_short_sum += cr(BODY_SHORT, open, high, low, close, i) - cr(BODY_SHORT, open, high, low, close, i - BODY_SHORT.avg_period);
    }
    Ok(output)
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
/// Compute the candle harami cross result for the supplied aligned series.
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
pub fn candle_harami_cross(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_DOJI.avg_period.max(BODY_LONG.avg_period) + 1;
    if len <= lookback { return Ok(output); }

    let mut body_long_sum = 0.0;
    let mut body_doji_sum = 0.0;
    let start = lookback;
    for i in (start - 1 - BODY_LONG.avg_period)..(start - 1) { body_long_sum += cr(BODY_LONG, open, high, low, close, i); }
    for i in (start - BODY_DOJI.avg_period)..start { body_doji_sum += cr(BODY_DOJI, open, high, low, close, i); }

    for i in start..len {
        output[i] = (real_body(open[i-1], close[i-1]) > ca(BODY_LONG, body_long_sum, open, high, low, close, i-1)
            && real_body(open[i], close[i]) <= ca(BODY_DOJI, body_doji_sum, open, high, low, close, i)
            && open[i].max(close[i]) < open[i-1].max(close[i-1])
            && open[i].min(close[i]) > open[i-1].min(close[i-1])) as i32 * -candle_color(open[i-1], close[i-1]) * 100;
        body_long_sum += cr(BODY_LONG, open, high, low, close, i-1) - cr(BODY_LONG, open, high, low, close, i - 1 - BODY_LONG.avg_period);
        body_doji_sum += cr(BODY_DOJI, open, high, low, close, i) - cr(BODY_DOJI, open, high, low, close, i - BODY_DOJI.avg_period);
    }
    Ok(output)
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
/// Compute the candle hikkake result for the supplied aligned series.
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
pub fn candle_hikkake(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = 5;
    if len <= lookback { return Ok(output); }

    let mut pattern_idx: i32 = -1;
    let mut pattern_result: i32 = 0;

    // Pre-scan bars before start
    let start = lookback;
    for i in (start.saturating_sub(3))..start {
        if i >= 2 {
            // Inside bar: 2nd has lower high and higher low than 1st
            if high[i-1] < high[i-2] && low[i-1] > low[i-2] {
                // 3rd bar determines direction
                if high[i] < high[i-1] && low[i] < low[i-1] {
                    pattern_result = 100; // bullish
                    pattern_idx = i as i32;
                } else if high[i] > high[i-1] && low[i] > low[i-1] {
                    pattern_result = -100; // bearish
                    pattern_idx = i as i32;
                }
            }
        }
    }

    for i in start..len {
        if i >= 2 && high[i-1] < high[i-2] && low[i-1] > low[i-2] {
            // Inside bar found at i-1,i-2
            if high[i] < high[i-1] && low[i] < low[i-1] {
                pattern_result = 100;
                pattern_idx = i as i32;
                output[i] = pattern_result;
            } else if high[i] > high[i-1] && low[i] > low[i-1] {
                pattern_result = -100;
                pattern_idx = i as i32;
                output[i] = pattern_result;
            } else {
                // Check confirmation
                if pattern_idx >= 0 && (i as i32 - pattern_idx) <= 3 {
                    if pattern_result > 0 && close[i] > high[pattern_idx as usize - 1] {
                        output[i] = pattern_result + 100;
                        pattern_idx = -1;
                    } else if pattern_result < 0 && close[i] < low[pattern_idx as usize - 1] {
                        output[i] = pattern_result - 100;
                        pattern_idx = -1;
                    }
                }
            }
        } else {
            // Check confirmation
            if pattern_idx >= 0 && (i as i32 - pattern_idx) <= 3 {
                if pattern_result > 0 && close[i] > high[pattern_idx as usize - 1] {
                    output[i] = pattern_result + 100;
                    pattern_idx = -1;
                } else if pattern_result < 0 && close[i] < low[pattern_idx as usize - 1] {
                    output[i] = pattern_result - 100;
                    pattern_idx = -1;
                }
            }
        }
    }
    Ok(output)
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
/// Compute the candle hikkake modified result for the supplied aligned series.
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
pub fn candle_hikkake_modified(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    // C TA-Lib: lookback = max(1, TA_CandleAvgPeriod(Near)) + 5
    let lookback = 1_usize.max(NEAR.avg_period) + 5;
    if len <= lookback { return Ok(output); }

    // Initialize Near sum for bar (start - 3), i.e. the "2nd candle" at start
    let mut near_sum = 0.0;
    let near_bar = lookback - 3; // the 2nd bar of the pattern at first evaluation
    if NEAR.avg_period > 0 && near_bar >= NEAR.avg_period {
        for j in (near_bar - NEAR.avg_period)..near_bar {
            near_sum += cr(NEAR, open, high, low, close, j);
        }
    }

    let mut pattern_idx: i32 = -10; // no active pattern
    let mut pattern_result: i32 = 0;

    for i in lookback..len {
        // C TA-Lib indices: i is current bar
        // Pattern: bar[i-3] contains bar[i-2], bar[i-2] contains bar[i-1]
        // Then bar[i] breaks out
        if high[i-1] < high[i-2] && low[i-1] > low[i-2]   // bar[i-1] inside bar[i-2]
            && high[i-2] < high[i-3] && low[i-2] > low[i-3] // bar[i-2] inside bar[i-3]
        {
            let near_avg = ca(NEAR, near_sum, open, high, low, close, i - 2);
            // Bullish: bar[i] breaks down (lower high AND lower low)
            if high[i] < high[i-1] && low[i] < low[i-1]
                // 2nd bar close near the low
                && close[i-2] <= low[i-2] + near_avg
            {
                pattern_result = 100;
                pattern_idx = i as i32;
                output[i] = pattern_result;
            }
            // Bearish: bar[i] breaks up (higher high AND higher low)
            else if high[i] > high[i-1] && low[i] > low[i-1]
                // 2nd bar close near the high
                && close[i-2] >= high[i-2] - near_avg
            {
                pattern_result = -100;
                pattern_idx = i as i32;
                output[i] = pattern_result;
            }
        }

        // Confirmation: within 3 bars of pattern
        if pattern_idx >= 0 && (i as i32) <= pattern_idx + 3 {
            if pattern_result > 0 && close[i] > high[(pattern_idx - 1) as usize] {
                output[i] = pattern_result + 100;
                pattern_idx = -10;
            } else if pattern_result < 0 && close[i] < low[(pattern_idx - 1) as usize] {
                output[i] = pattern_result - 100;
                pattern_idx = -10;
            }
        }

        // Update Near sum (for the "2nd bar" position, which is i-2)
        if NEAR.avg_period > 0 && (i - 2) >= NEAR.avg_period {
            near_sum += cr(NEAR, open, high, low, close, i - 2)
                - cr(NEAR, open, high, low, close, i - 2 - NEAR.avg_period);
        }
    }
    Ok(output)
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
/// Compute the candle homing pigeon result for the supplied aligned series.
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
pub fn candle_homing_pigeon(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_SHORT.avg_period.max(BODY_LONG.avg_period) + 1;
    if len <= lookback { return Ok(output); }

    let mut body_long_sum = 0.0;
    let mut body_short_sum = 0.0;
    let start = lookback;
    for i in (start - 1 - BODY_LONG.avg_period)..(start - 1) { body_long_sum += cr(BODY_LONG, open, high, low, close, i); }
    for i in (start - BODY_SHORT.avg_period)..start { body_short_sum += cr(BODY_SHORT, open, high, low, close, i); }

    for i in start..len {
        output[i] = (candle_color(open[i-1], close[i-1]) == -1
            && candle_color(open[i], close[i]) == -1
            && real_body(open[i-1], close[i-1]) > ca(BODY_LONG, body_long_sum, open, high, low, close, i-1)
            && real_body(open[i], close[i]) <= ca(BODY_SHORT, body_short_sum, open, high, low, close, i)
            && open[i] < open[i-1]
            && close[i] > close[i-1]) as i32 * 100;
        body_long_sum += cr(BODY_LONG, open, high, low, close, i-1) - cr(BODY_LONG, open, high, low, close, i - 1 - BODY_LONG.avg_period);
        body_short_sum += cr(BODY_SHORT, open, high, low, close, i) - cr(BODY_SHORT, open, high, low, close, i - BODY_SHORT.avg_period);
    }
    Ok(output)
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
/// Compute the candle in neck result for the supplied aligned series.
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
pub fn candle_in_neck(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = EQUAL.avg_period.max(BODY_LONG.avg_period) + 1;
    if len <= lookback { return Ok(output); }

    let mut equal_sum = 0.0;
    let mut body_sum = 0.0;
    let start = lookback;
    for i in (start - 1 - EQUAL.avg_period)..(start - 1) { equal_sum += cr(EQUAL, open, high, low, close, i); }
    for i in (start - 1 - BODY_LONG.avg_period)..(start - 1) { body_sum += cr(BODY_LONG, open, high, low, close, i); }

    for i in start..len {
        // 1st: long black
        output[i] = (candle_color(open[i-1], close[i-1]) == -1
            && real_body(open[i-1], close[i-1]) > ca(BODY_LONG, body_sum, open, high, low, close, i-1)
            // 2nd: white, opens below prev low
            && candle_color(open[i], close[i]) == 1
            && open[i] < low[i-1]
            // close slightly into prev body: >= prev close and <= prev close + Equal avg
            && close[i] >= close[i-1]
            && close[i] <= close[i-1] + ca(EQUAL, equal_sum, open, high, low, close, i-1)) as i32 * -100;
        equal_sum += cr(EQUAL, open, high, low, close, i-1) - cr(EQUAL, open, high, low, close, i - 1 - EQUAL.avg_period);
        body_sum += cr(BODY_LONG, open, high, low, close, i-1) - cr(BODY_LONG, open, high, low, close, i - 1 - BODY_LONG.avg_period);
    }
    Ok(output)
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
/// Compute the candle inverted hammer result for the supplied aligned series.
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
pub fn candle_inverted_hammer(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = *[BODY_SHORT.avg_period, SHADOW_LONG.avg_period, SHADOW_VERY_SHORT.avg_period].iter().max().unwrap() + 1;
    if len <= lookback { return Ok(output); }

    let mut body_sum = 0.0;
    let mut shadow_long_sum = 0.0;
    let mut shadow_vs_sum = 0.0;
    let start = lookback;
    for i in (start - BODY_SHORT.avg_period)..start { body_sum += cr(BODY_SHORT, open, high, low, close, i); }
    for i in (start - SHADOW_VERY_SHORT.avg_period)..start { shadow_vs_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i); }

    for i in start..len {
        output[i] = (real_body(open[i], close[i]) < ca(BODY_SHORT, body_sum, open, high, low, close, i)
            && upper_shadow(open[i], high[i], close[i]) > ca(SHADOW_LONG, shadow_long_sum, open, high, low, close, i)
            && lower_shadow(open[i], low[i], close[i]) < ca(SHADOW_VERY_SHORT, shadow_vs_sum, open, high, low, close, i)
            && real_body_gap_down(open, close, i, i-1)) as i32 * 100;
        if BODY_SHORT.avg_period > 0 { body_sum += cr(BODY_SHORT, open, high, low, close, i) - cr(BODY_SHORT, open, high, low, close, i - BODY_SHORT.avg_period); }
        if SHADOW_VERY_SHORT.avg_period > 0 { shadow_vs_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i) - cr(SHADOW_VERY_SHORT, open, high, low, close, i - SHADOW_VERY_SHORT.avg_period); }
    }
    Ok(output)
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
/// Compute the candle kicking result for the supplied aligned series.
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
pub fn candle_kicking(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = SHADOW_VERY_SHORT.avg_period.max(BODY_LONG.avg_period) + 1;
    if len <= lookback { return Ok(output); }

    let mut shadow_sum = [0.0f64; 2];
    let mut body_sum = [0.0f64; 2];
    let start = lookback;
    for i in (start - 1 - SHADOW_VERY_SHORT.avg_period)..(start - 1) { shadow_sum[1] += cr(SHADOW_VERY_SHORT, open, high, low, close, i); }
    for i in (start - SHADOW_VERY_SHORT.avg_period)..start { shadow_sum[0] += cr(SHADOW_VERY_SHORT, open, high, low, close, i); }
    for i in (start - 1 - BODY_LONG.avg_period)..(start - 1) { body_sum[1] += cr(BODY_LONG, open, high, low, close, i); }
    for i in (start - BODY_LONG.avg_period)..start { body_sum[0] += cr(BODY_LONG, open, high, low, close, i); }

    for i in start..len {
        let color_prev = candle_color(open[i-1], close[i-1]);
        let color_curr = candle_color(open[i], close[i]);
        if color_prev != color_curr
            && real_body(open[i-1], close[i-1]) > ca(BODY_LONG, body_sum[1], open, high, low, close, i-1)
            && upper_shadow(open[i-1], high[i-1], close[i-1]) < ca(SHADOW_VERY_SHORT, shadow_sum[1], open, high, low, close, i-1)
            && lower_shadow(open[i-1], low[i-1], close[i-1]) < ca(SHADOW_VERY_SHORT, shadow_sum[1], open, high, low, close, i-1)
            && real_body(open[i], close[i]) > ca(BODY_LONG, body_sum[0], open, high, low, close, i)
            && upper_shadow(open[i], high[i], close[i]) < ca(SHADOW_VERY_SHORT, shadow_sum[0], open, high, low, close, i)
            && lower_shadow(open[i], low[i], close[i]) < ca(SHADOW_VERY_SHORT, shadow_sum[0], open, high, low, close, i)
        {
            // Gap: black then white = bullish, white then black = bearish
            let bull = color_prev == -1 && color_curr == 1 && open[i] > open[i-1];
            let bear = color_prev == 1 && color_curr == -1 && open[i] < open[i-1];
            output[i] = (bull as i32) * 100 - (bear as i32) * 100;
        }
        shadow_sum[1] += cr(SHADOW_VERY_SHORT, open, high, low, close, i-1) - cr(SHADOW_VERY_SHORT, open, high, low, close, i - 1 - SHADOW_VERY_SHORT.avg_period);
        shadow_sum[0] += cr(SHADOW_VERY_SHORT, open, high, low, close, i) - cr(SHADOW_VERY_SHORT, open, high, low, close, i - SHADOW_VERY_SHORT.avg_period);
        body_sum[1] += cr(BODY_LONG, open, high, low, close, i-1) - cr(BODY_LONG, open, high, low, close, i - 1 - BODY_LONG.avg_period);
        body_sum[0] += cr(BODY_LONG, open, high, low, close, i) - cr(BODY_LONG, open, high, low, close, i - BODY_LONG.avg_period);
    }
    Ok(output)
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
/// Compute the candle kicking by length result for the supplied aligned series.
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
pub fn candle_kicking_by_length(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = SHADOW_VERY_SHORT.avg_period.max(BODY_LONG.avg_period) + 1;
    if len <= lookback { return Ok(output); }

    let mut shadow_sum = [0.0f64; 2];
    let mut body_sum = [0.0f64; 2];
    let start = lookback;
    for i in (start - 1 - SHADOW_VERY_SHORT.avg_period)..(start - 1) { shadow_sum[1] += cr(SHADOW_VERY_SHORT, open, high, low, close, i); }
    for i in (start - SHADOW_VERY_SHORT.avg_period)..start { shadow_sum[0] += cr(SHADOW_VERY_SHORT, open, high, low, close, i); }
    for i in (start - 1 - BODY_LONG.avg_period)..(start - 1) { body_sum[1] += cr(BODY_LONG, open, high, low, close, i); }
    for i in (start - BODY_LONG.avg_period)..start { body_sum[0] += cr(BODY_LONG, open, high, low, close, i); }

    for i in start..len {
        let color_prev = candle_color(open[i-1], close[i-1]);
        let color_curr = candle_color(open[i], close[i]);
        if color_prev != color_curr
            && real_body(open[i-1], close[i-1]) > ca(BODY_LONG, body_sum[1], open, high, low, close, i-1)
            && upper_shadow(open[i-1], high[i-1], close[i-1]) < ca(SHADOW_VERY_SHORT, shadow_sum[1], open, high, low, close, i-1)
            && lower_shadow(open[i-1], low[i-1], close[i-1]) < ca(SHADOW_VERY_SHORT, shadow_sum[1], open, high, low, close, i-1)
            && real_body(open[i], close[i]) > ca(BODY_LONG, body_sum[0], open, high, low, close, i)
            && upper_shadow(open[i], high[i], close[i]) < ca(SHADOW_VERY_SHORT, shadow_sum[0], open, high, low, close, i)
            && lower_shadow(open[i], low[i], close[i]) < ca(SHADOW_VERY_SHORT, shadow_sum[0], open, high, low, close, i)
        {
            // Gap check
            let has_gap = (color_prev == -1 && color_curr == 1 && open[i] > open[i-1])
                || (color_prev == 1 && color_curr == -1 && open[i] < open[i-1]);
            let curr_longer = real_body(open[i], close[i]) >= real_body(open[i-1], close[i-1]);
            // Branchless: select color based on which marubozu is longer
            let color = if curr_longer { color_curr } else { color_prev };
            output[i] = has_gap as i32 * color * 100;
        }
        shadow_sum[1] += cr(SHADOW_VERY_SHORT, open, high, low, close, i-1) - cr(SHADOW_VERY_SHORT, open, high, low, close, i - 1 - SHADOW_VERY_SHORT.avg_period);
        shadow_sum[0] += cr(SHADOW_VERY_SHORT, open, high, low, close, i) - cr(SHADOW_VERY_SHORT, open, high, low, close, i - SHADOW_VERY_SHORT.avg_period);
        body_sum[1] += cr(BODY_LONG, open, high, low, close, i-1) - cr(BODY_LONG, open, high, low, close, i - 1 - BODY_LONG.avg_period);
        body_sum[0] += cr(BODY_LONG, open, high, low, close, i) - cr(BODY_LONG, open, high, low, close, i - BODY_LONG.avg_period);
    }
    Ok(output)
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
/// Compute the candle matching low result for the supplied aligned series.
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
pub fn candle_matching_low(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = EQUAL.avg_period + 1;
    if len <= lookback { return Ok(output); }

    let mut equal_sum = 0.0;
    let start = lookback;
    for i in (start - 1 - EQUAL.avg_period)..(start - 1) { equal_sum += cr(EQUAL, open, high, low, close, i); }

    for i in start..len {
        output[i] = (candle_color(open[i-1], close[i-1]) == -1
            && candle_color(open[i], close[i]) == -1
            && (close[i] - close[i-1]).abs() <= ca(EQUAL, equal_sum, open, high, low, close, i-1)) as i32 * 100;
        equal_sum += cr(EQUAL, open, high, low, close, i-1) - cr(EQUAL, open, high, low, close, i - 1 - EQUAL.avg_period);
    }
    Ok(output)
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
/// Compute the candle on neck result for the supplied aligned series.
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
pub fn candle_on_neck(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = EQUAL.avg_period.max(BODY_LONG.avg_period) + 1;
    if len <= lookback { return Ok(output); }

    let mut equal_sum = 0.0;
    let mut body_sum = 0.0;
    let start = lookback;
    for i in (start - 1 - EQUAL.avg_period)..(start - 1) { equal_sum += cr(EQUAL, open, high, low, close, i); }
    for i in (start - 1 - BODY_LONG.avg_period)..(start - 1) { body_sum += cr(BODY_LONG, open, high, low, close, i); }

    for i in start..len {
        output[i] = (candle_color(open[i-1], close[i-1]) == -1
            && real_body(open[i-1], close[i-1]) > ca(BODY_LONG, body_sum, open, high, low, close, i-1)
            && candle_color(open[i], close[i]) == 1
            && open[i] < low[i-1]
            && (close[i] - low[i-1]).abs() <= ca(EQUAL, equal_sum, open, high, low, close, i-1)) as i32 * -100;
        equal_sum += cr(EQUAL, open, high, low, close, i-1) - cr(EQUAL, open, high, low, close, i - 1 - EQUAL.avg_period);
        body_sum += cr(BODY_LONG, open, high, low, close, i-1) - cr(BODY_LONG, open, high, low, close, i - 1 - BODY_LONG.avg_period);
    }
    Ok(output)
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
/// Compute the candle piercing result for the supplied aligned series.
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
pub fn candle_piercing(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_LONG.avg_period + 1;
    if len <= lookback { return Ok(output); }

    let mut body_sum = [0.0f64; 2];
    let start = lookback;
    for i in (start - 1 - BODY_LONG.avg_period)..(start - 1) { body_sum[1] += cr(BODY_LONG, open, high, low, close, i); }
    for i in (start - BODY_LONG.avg_period)..start { body_sum[0] += cr(BODY_LONG, open, high, low, close, i); }

    for i in start..len {
        output[i] = (candle_color(open[i-1], close[i-1]) == -1
            && real_body(open[i-1], close[i-1]) > ca(BODY_LONG, body_sum[1], open, high, low, close, i-1)
            && candle_color(open[i], close[i]) == 1
            && real_body(open[i], close[i]) > ca(BODY_LONG, body_sum[0], open, high, low, close, i)
            && open[i] < low[i-1]
            && close[i] < open[i-1]
            && close[i] > close[i-1] + real_body(open[i-1], close[i-1]) * 0.5) as i32 * 100;
        body_sum[1] += cr(BODY_LONG, open, high, low, close, i-1) - cr(BODY_LONG, open, high, low, close, i - 1 - BODY_LONG.avg_period);
        body_sum[0] += cr(BODY_LONG, open, high, low, close, i) - cr(BODY_LONG, open, high, low, close, i - BODY_LONG.avg_period);
    }
    Ok(output)
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
/// Compute the candle separating lines result for the supplied aligned series.
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
pub fn candle_separating_lines(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = *[SHADOW_VERY_SHORT.avg_period, BODY_LONG.avg_period, EQUAL.avg_period].iter().max().unwrap() + 1;
    if len <= lookback { return Ok(output); }

    let mut shadow_sum = 0.0;
    let mut body_sum = 0.0;
    let mut equal_sum = 0.0;
    let start = lookback;
    for i in (start - SHADOW_VERY_SHORT.avg_period)..start { shadow_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i); }
    for i in (start - BODY_LONG.avg_period)..start { body_sum += cr(BODY_LONG, open, high, low, close, i); }
    for i in (start - 1 - EQUAL.avg_period)..(start - 1) { equal_sum += cr(EQUAL, open, high, low, close, i); }

    for i in start..len {
        let color_prev = candle_color(open[i-1], close[i-1]);
        let color_curr = candle_color(open[i], close[i]);
        let base = color_prev != color_curr
            && (open[i] - open[i-1]).abs() <= ca(EQUAL, equal_sum, open, high, low, close, i-1)
            && real_body(open[i], close[i]) > ca(BODY_LONG, body_sum, open, high, low, close, i);
        // Bullish: very short lower shadow, bearish: very short upper shadow
        let bull = base && color_curr == 1 && lower_shadow(open[i], low[i], close[i]) < ca(SHADOW_VERY_SHORT, shadow_sum, open, high, low, close, i);
        let bear = base && color_curr == -1 && upper_shadow(open[i], high[i], close[i]) < ca(SHADOW_VERY_SHORT, shadow_sum, open, high, low, close, i);
        output[i] = (bull as i32) * 100 - (bear as i32) * 100;
        shadow_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i) - cr(SHADOW_VERY_SHORT, open, high, low, close, i - SHADOW_VERY_SHORT.avg_period);
        body_sum += cr(BODY_LONG, open, high, low, close, i) - cr(BODY_LONG, open, high, low, close, i - BODY_LONG.avg_period);
        equal_sum += cr(EQUAL, open, high, low, close, i-1) - cr(EQUAL, open, high, low, close, i - 1 - EQUAL.avg_period);
    }
    Ok(output)
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
/// Compute the candle shooting star result for the supplied aligned series.
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
pub fn candle_shooting_star(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = *[BODY_SHORT.avg_period, SHADOW_LONG.avg_period, SHADOW_VERY_SHORT.avg_period].iter().max().unwrap() + 1;
    if len <= lookback { return Ok(output); }

    let mut body_sum = 0.0;
    let mut shadow_long_sum = 0.0;
    let mut shadow_vs_sum = 0.0;
    let start = lookback;
    for i in (start - BODY_SHORT.avg_period)..start { body_sum += cr(BODY_SHORT, open, high, low, close, i); }
    for i in (start - SHADOW_VERY_SHORT.avg_period)..start { shadow_vs_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i); }

    for i in start..len {
        output[i] = (real_body(open[i], close[i]) < ca(BODY_SHORT, body_sum, open, high, low, close, i)
            && upper_shadow(open[i], high[i], close[i]) > ca(SHADOW_LONG, shadow_long_sum, open, high, low, close, i)
            && lower_shadow(open[i], low[i], close[i]) < ca(SHADOW_VERY_SHORT, shadow_vs_sum, open, high, low, close, i)
            && real_body_gap_up(open, close, i, i-1)) as i32 * -100;
        if BODY_SHORT.avg_period > 0 { body_sum += cr(BODY_SHORT, open, high, low, close, i) - cr(BODY_SHORT, open, high, low, close, i - BODY_SHORT.avg_period); }
        if SHADOW_VERY_SHORT.avg_period > 0 { shadow_vs_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i) - cr(SHADOW_VERY_SHORT, open, high, low, close, i - SHADOW_VERY_SHORT.avg_period); }
    }
    Ok(output)
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
/// Compute the candle stick sandwich result for the supplied aligned series.
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
pub fn candle_stick_sandwich(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = EQUAL.avg_period + 2;
    if len <= lookback { return Ok(output); }

    let mut equal_sum = 0.0;
    let start = lookback;
    for i in (start - 2 - EQUAL.avg_period)..(start - 2) { equal_sum += cr(EQUAL, open, high, low, close, i); }

    for i in start..len {
        output[i] = (candle_color(open[i-2], close[i-2]) == -1
            && candle_color(open[i-1], close[i-1]) == 1
            && candle_color(open[i], close[i]) == -1
            && low[i-1] > close[i-2]
            && (close[i] - close[i-2]).abs() <= ca(EQUAL, equal_sum, open, high, low, close, i-2)) as i32 * 100;
        equal_sum += cr(EQUAL, open, high, low, close, i-2) - cr(EQUAL, open, high, low, close, i - 2 - EQUAL.avg_period);
    }
    Ok(output)
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
/// Compute the candle thrusting result for the supplied aligned series.
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
pub fn candle_thrusting(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = EQUAL.avg_period.max(BODY_LONG.avg_period) + 1;
    if len <= lookback { return Ok(output); }

    let mut equal_sum = 0.0;
    let mut body_sum = 0.0;
    let start = lookback;
    for i in (start - 1 - EQUAL.avg_period)..(start - 1) { equal_sum += cr(EQUAL, open, high, low, close, i); }
    for i in (start - 1 - BODY_LONG.avg_period)..(start - 1) { body_sum += cr(BODY_LONG, open, high, low, close, i); }

    for i in start..len {
        output[i] = (candle_color(open[i-1], close[i-1]) == -1
            && real_body(open[i-1], close[i-1]) > ca(BODY_LONG, body_sum, open, high, low, close, i-1)
            && candle_color(open[i], close[i]) == 1
            && open[i] < low[i-1]
            && close[i] > close[i-1] + ca(EQUAL, equal_sum, open, high, low, close, i-1)
            && close[i] <= close[i-1] + real_body(open[i-1], close[i-1]) * 0.5) as i32 * -100;
        equal_sum += cr(EQUAL, open, high, low, close, i-1) - cr(EQUAL, open, high, low, close, i - 1 - EQUAL.avg_period);
        body_sum += cr(BODY_LONG, open, high, low, close, i-1) - cr(BODY_LONG, open, high, low, close, i - 1 - BODY_LONG.avg_period);
    }
    Ok(output)
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
pub fn candle_belt_hold(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_LONG.avg_period.max(SHADOW_VERY_SHORT.avg_period);
    if len <= lookback { return Ok(output); }

    let mut body_sum = 0.0;
    let mut shadow_sum = 0.0;
    let start = lookback;
    for i in (start - BODY_LONG.avg_period)..start { body_sum += cr(BODY_LONG, open, high, low, close, i); }
    for i in (start - SHADOW_VERY_SHORT.avg_period)..start { shadow_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i); }

    for i in start..len {
        let long_body = real_body(open[i], close[i]) > ca(BODY_LONG, body_sum, open, high, low, close, i);
        let bull = long_body
            && candle_color(open[i], close[i]) == 1
            && lower_shadow(open[i], low[i], close[i]) < ca(SHADOW_VERY_SHORT, shadow_sum, open, high, low, close, i);
        let bear = long_body
            && candle_color(open[i], close[i]) == -1
            && upper_shadow(open[i], high[i], close[i]) < ca(SHADOW_VERY_SHORT, shadow_sum, open, high, low, close, i);
        output[i] = (bull as i32) * 100 - (bear as i32) * 100;
        body_sum += cr(BODY_LONG, open, high, low, close, i) - cr(BODY_LONG, open, high, low, close, i - BODY_LONG.avg_period);
        shadow_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i) - cr(SHADOW_VERY_SHORT, open, high, low, close, i - SHADOW_VERY_SHORT.avg_period);
    }
    Ok(output)
}

// ========== Three+ candle patterns ==========

/// Compute the candle pattern signal for aligned OHLC bars.
///
/// # Parameters
///
/// * `open`, `high`, `low`, `close` - Equal-length chronological OHLC series.
///
/// # Returns
///
/// A same-length vector containing -100, 0, or 100 pattern signals; bars
/// Compute the candle three black crows result for the supplied aligned series.
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
pub fn candle_three_black_crows(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = SHADOW_VERY_SHORT.avg_period + 3;
    if len <= lookback { return Ok(output); }

    let mut shadow_sum = [0.0f64; 3];
    let start = lookback;
    for k in 0..3 {
        let bar_offset = start - 3 + k;
        if bar_offset >= SHADOW_VERY_SHORT.avg_period {
            for j in (bar_offset - SHADOW_VERY_SHORT.avg_period)..bar_offset {
                shadow_sum[k] += cr(SHADOW_VERY_SHORT, open, high, low, close, j);
            }
        }
    }

    for i in start..len {
        output[i] = (candle_color(open[i-2], close[i-2]) == -1
            && candle_color(open[i-1], close[i-1]) == -1
            && candle_color(open[i], close[i]) == -1
            && close[i-1] < close[i-2] && close[i] < close[i-1]
            && open[i-2] <= open[i-3].max(close[i-3])
            && open[i-1] <= open[i-2] && open[i-1] >= close[i-2]
            && open[i] <= open[i-1] && open[i] >= close[i-1]
            && lower_shadow(open[i-2], low[i-2], close[i-2]) < ca(SHADOW_VERY_SHORT, shadow_sum[0], open, high, low, close, i-2)
            && lower_shadow(open[i-1], low[i-1], close[i-1]) < ca(SHADOW_VERY_SHORT, shadow_sum[1], open, high, low, close, i-1)
            && lower_shadow(open[i], low[i], close[i]) < ca(SHADOW_VERY_SHORT, shadow_sum[2], open, high, low, close, i)) as i32 * -100;
        for k in 0..3 {
            let bar = i - 2 + k;
            if bar >= SHADOW_VERY_SHORT.avg_period {
                shadow_sum[k] += cr(SHADOW_VERY_SHORT, open, high, low, close, bar)
                    - cr(SHADOW_VERY_SHORT, open, high, low, close, bar - SHADOW_VERY_SHORT.avg_period);
            }
        }
    }
    Ok(output)
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
/// Compute the candle three inside result for the supplied aligned series.
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
pub fn candle_three_inside(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_SHORT.avg_period.max(BODY_LONG.avg_period) + 2;
    if len <= lookback { return Ok(output); }

    let mut body_long_sum = 0.0;
    let mut body_short_sum = 0.0;
    let start = lookback;
    for i in (start - 2 - BODY_LONG.avg_period)..(start - 2) { body_long_sum += cr(BODY_LONG, open, high, low, close, i); }
    for i in (start - 1 - BODY_SHORT.avg_period)..(start - 1) { body_short_sum += cr(BODY_SHORT, open, high, low, close, i); }

    for i in start..len {
        output[i] = (real_body(open[i-2], close[i-2]) > ca(BODY_LONG, body_long_sum, open, high, low, close, i-2)
            && real_body(open[i-1], close[i-1]) <= ca(BODY_SHORT, body_short_sum, open, high, low, close, i-1)
            && open[i-1].max(close[i-1]) < open[i-2].max(close[i-2])
            && open[i-1].min(close[i-1]) > open[i-2].min(close[i-2])
            && ((candle_color(open[i-2], close[i-2]) == 1 && candle_color(open[i], close[i]) == -1 && close[i] < open[i-2])
                || (candle_color(open[i-2], close[i-2]) == -1 && candle_color(open[i], close[i]) == 1 && close[i] > open[i-2]))) as i32 * -candle_color(open[i-2], close[i-2]) * 100;
        body_long_sum += cr(BODY_LONG, open, high, low, close, i-2) - cr(BODY_LONG, open, high, low, close, i - 2 - BODY_LONG.avg_period);
        body_short_sum += cr(BODY_SHORT, open, high, low, close, i-1) - cr(BODY_SHORT, open, high, low, close, i - 1 - BODY_SHORT.avg_period);
    }
    Ok(output)
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
/// Compute the candle three line strike result for the supplied aligned series.
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
pub fn candle_three_line_strike(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = NEAR.avg_period + 3;
    if len <= lookback { return Ok(output); }

    let mut near_sum = [0.0f64; 4];
    let start = lookback;
    // Init near sums for bars i-3 and i-2
    for k in [2usize, 3] {
        let bar = start - k;
        if bar >= NEAR.avg_period {
            for j in (bar - NEAR.avg_period)..bar {
                near_sum[k] += cr(NEAR, open, high, low, close, j);
            }
        }
    }

    for i in start..len {
        let c3 = candle_color(open[i-3], close[i-3]);
        let c2 = candle_color(open[i-2], close[i-2]);
        let c1 = candle_color(open[i-1], close[i-1]);
        let c0 = candle_color(open[i], close[i]);

        if c3 == c2 && c2 == c1 && c0 != c1 {
            // Three same-color, 4th opposite
            let progressive = if c3 == 1 {
                close[i-2] > close[i-3] && close[i-1] > close[i-2]
            } else {
                close[i-2] < close[i-3] && close[i-1] < close[i-2]
            };
            let opens_near = if c3 == 1 {
                open[i-2] >= open[i-3].min(close[i-3])
                    && open[i-2] <= close[i-3] + ca(NEAR, near_sum[3], open, high, low, close, i-3)
                    && open[i-1] >= open[i-2].min(close[i-2])
                    && open[i-1] <= close[i-2] + ca(NEAR, near_sum[2], open, high, low, close, i-2)
            } else {
                open[i-2] <= open[i-3].max(close[i-3])
                    && open[i-2] >= close[i-3] - ca(NEAR, near_sum[3], open, high, low, close, i-3)
                    && open[i-1] <= open[i-2].max(close[i-2])
                    && open[i-1] >= close[i-2] - ca(NEAR, near_sum[2], open, high, low, close, i-2)
            };
            let strike = if c3 == 1 {
                open[i] >= close[i-1] && close[i] <= open[i-3]
            } else {
                open[i] <= close[i-1] && close[i] >= open[i-3]
            };
            output[i] = (progressive && opens_near && strike) as i32 * c3 * 100;
        }
        // Update near sums
        for k in [2usize, 3] {
            let bar = i - k;
            if bar >= NEAR.avg_period && NEAR.avg_period > 0 {
                near_sum[k] += cr(NEAR, open, high, low, close, bar) - cr(NEAR, open, high, low, close, bar - NEAR.avg_period);
            }
        }
    }
    Ok(output)
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
/// Compute the candle three outside result for the supplied aligned series.
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
pub fn candle_three_outside(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    // lookback = 3
    if len < 3 { return Ok(output); }

    for i in 2..len {
        // Bullish: 1st black, 2nd white engulfs, 3rd closes higher
        let bull = candle_color(open[i-2], close[i-2]) == -1
            && candle_color(open[i-1], close[i-1]) == 1
            && close[i-1] >= open[i-2]
            && open[i-1] <= close[i-2]
            && close[i] > close[i-1];
        // Bearish: 1st white, 2nd black engulfs, 3rd closes lower
        let bear = candle_color(open[i-2], close[i-2]) == 1
            && candle_color(open[i-1], close[i-1]) == -1
            && open[i-1] >= close[i-2]
            && close[i-1] <= open[i-2]
            && close[i] < close[i-1];
        output[i] = (bull as i32) * 100 - (bear as i32) * 100;
    }
    Ok(output)
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
pub fn candle_three_stars_in_south(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = *[SHADOW_VERY_SHORT.avg_period, SHADOW_LONG.avg_period, BODY_LONG.avg_period, BODY_SHORT.avg_period].iter().max().unwrap() + 2;
    if len <= lookback { return Ok(output); }

    let mut body_long_sum = 0.0;
    let mut shadow_long_sum = 0.0;
    let mut shadow_vs_sum = [0.0f64; 2]; // for 2nd and 3rd candles
    let mut body_short_sum = 0.0;
    let start = lookback;
    for i in (start - 2 - BODY_LONG.avg_period)..(start - 2) { body_long_sum += cr(BODY_LONG, open, high, low, close, i); }
    // SHADOW_LONG avg_period = 0, no init
    for i in (start - 1 - SHADOW_VERY_SHORT.avg_period)..(start - 1) { shadow_vs_sum[0] += cr(SHADOW_VERY_SHORT, open, high, low, close, i); }
    for i in (start - SHADOW_VERY_SHORT.avg_period)..start { shadow_vs_sum[1] += cr(SHADOW_VERY_SHORT, open, high, low, close, i); }
    for i in (start - BODY_SHORT.avg_period)..start { body_short_sum += cr(BODY_SHORT, open, high, low, close, i); }

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
            && low[i] > low[i-1] && high[i] < high[i-1]) as i32 * 100;
        body_long_sum += cr(BODY_LONG, open, high, low, close, i-2) - cr(BODY_LONG, open, high, low, close, i - 2 - BODY_LONG.avg_period);
        shadow_vs_sum[0] += cr(SHADOW_VERY_SHORT, open, high, low, close, i-1) - cr(SHADOW_VERY_SHORT, open, high, low, close, i - 1 - SHADOW_VERY_SHORT.avg_period);
        shadow_vs_sum[1] += cr(SHADOW_VERY_SHORT, open, high, low, close, i) - cr(SHADOW_VERY_SHORT, open, high, low, close, i - SHADOW_VERY_SHORT.avg_period);
        body_short_sum += cr(BODY_SHORT, open, high, low, close, i) - cr(BODY_SHORT, open, high, low, close, i - BODY_SHORT.avg_period);
    }
    Ok(output)
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
/// Compute the candle three white soldiers result for the supplied aligned series.
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
pub fn candle_three_white_soldiers(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = *[SHADOW_VERY_SHORT.avg_period, BODY_SHORT.avg_period, FAR.avg_period, NEAR.avg_period].iter().max().unwrap() + 2;
    if len <= lookback { return Ok(output); }

    let mut shadow_sum = [0.0f64; 3];
    let mut near_sum = [0.0f64; 3];
    let mut far_sum = [0.0f64; 3];
    let mut body_short_sum = 0.0;
    let start = lookback;

    for k in 0..3 {
        let bar = start - 2 + k;
        if bar >= SHADOW_VERY_SHORT.avg_period {
            for j in (bar - SHADOW_VERY_SHORT.avg_period)..bar { shadow_sum[k] += cr(SHADOW_VERY_SHORT, open, high, low, close, j); }
        }
        if bar >= NEAR.avg_period {
            for j in (bar - NEAR.avg_period)..bar { near_sum[k] += cr(NEAR, open, high, low, close, j); }
        }
        if bar >= FAR.avg_period {
            for j in (bar - FAR.avg_period)..bar { far_sum[k] += cr(FAR, open, high, low, close, j); }
        }
    }
    for i in (start - BODY_SHORT.avg_period)..start { body_short_sum += cr(BODY_SHORT, open, high, low, close, i); }

    for i in start..len {
        output[i] = (candle_color(open[i-2], close[i-2]) == 1
            && candle_color(open[i-1], close[i-1]) == 1
            && candle_color(open[i], close[i]) == 1
            && close[i-1] > close[i-2] && close[i] > close[i-1]
            // Short upper shadows
            && upper_shadow(open[i-2], high[i-2], close[i-2]) < ca(SHADOW_VERY_SHORT, shadow_sum[0], open, high, low, close, i-2)
            && upper_shadow(open[i-1], high[i-1], close[i-1]) < ca(SHADOW_VERY_SHORT, shadow_sum[1], open, high, low, close, i-1)
            && upper_shadow(open[i], high[i], close[i]) < ca(SHADOW_VERY_SHORT, shadow_sum[2], open, high, low, close, i)
            // Opens within or near previous body
            && open[i-1] > open[i-2] && open[i-1] <= close[i-2] + ca(NEAR, near_sum[1], open, high, low, close, i-1)
            && open[i] > open[i-1] && open[i] <= close[i-1] + ca(NEAR, near_sum[2], open, high, low, close, i)
            // Bodies not far shorter than prior
            && real_body(open[i-1], close[i-1]) > real_body(open[i-2], close[i-2]) - ca(FAR, far_sum[1], open, high, low, close, i-1)
            && real_body(open[i], close[i]) > real_body(open[i-1], close[i-1]) - ca(FAR, far_sum[2], open, high, low, close, i)
            // Last body not short
            && real_body(open[i], close[i]) > ca(BODY_SHORT, body_short_sum, open, high, low, close, i)) as i32 * 100;
        for k in 0..3 {
            let bar = i - 2 + k;
            if SHADOW_VERY_SHORT.avg_period > 0 && bar >= SHADOW_VERY_SHORT.avg_period {
                shadow_sum[k] += cr(SHADOW_VERY_SHORT, open, high, low, close, bar) - cr(SHADOW_VERY_SHORT, open, high, low, close, bar - SHADOW_VERY_SHORT.avg_period);
            }
            if NEAR.avg_period > 0 && bar >= NEAR.avg_period {
                near_sum[k] += cr(NEAR, open, high, low, close, bar) - cr(NEAR, open, high, low, close, bar - NEAR.avg_period);
            }
            if FAR.avg_period > 0 && bar >= FAR.avg_period {
                far_sum[k] += cr(FAR, open, high, low, close, bar) - cr(FAR, open, high, low, close, bar - FAR.avg_period);
            }
        }
        body_short_sum += cr(BODY_SHORT, open, high, low, close, i) - cr(BODY_SHORT, open, high, low, close, i - BODY_SHORT.avg_period);
    }
    Ok(output)
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
/// Compute the candle abandoned baby result for the supplied aligned series.
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
pub fn candle_abandoned_baby(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let penetration = 0.3;
    let lookback = *[BODY_DOJI.avg_period, BODY_LONG.avg_period, BODY_SHORT.avg_period].iter().max().unwrap() + 2;
    if len <= lookback { return Ok(output); }

    let mut body_long_sum = 0.0;
    let mut body_doji_sum = 0.0;
    let mut body_short_sum = 0.0;
    let start = lookback;
    for i in (start - 2 - BODY_LONG.avg_period)..(start - 2) { body_long_sum += cr(BODY_LONG, open, high, low, close, i); }
    for i in (start - 1 - BODY_DOJI.avg_period)..(start - 1) { body_doji_sum += cr(BODY_DOJI, open, high, low, close, i); }
    for i in (start - BODY_SHORT.avg_period)..start { body_short_sum += cr(BODY_SHORT, open, high, low, close, i); }

    for i in start..len {
        let base = real_body(open[i-2], close[i-2]) > ca(BODY_LONG, body_long_sum, open, high, low, close, i-2)
            && real_body(open[i-1], close[i-1]) <= ca(BODY_DOJI, body_doji_sum, open, high, low, close, i-1)
            && real_body(open[i], close[i]) > ca(BODY_SHORT, body_short_sum, open, high, low, close, i);
        // Bullish: 1st black, gap down doji, gap up white
        let bull = base
            && candle_color(open[i-2], close[i-2]) == -1
            && candle_color(open[i], close[i]) == 1
            && high[i-1] < low[i-2]
            && low[i] > high[i-1]
            && close[i] > close[i-2] + real_body(open[i-2], close[i-2]) * penetration;
        // Bearish: 1st white, gap up doji, gap down black
        let bear = base
            && candle_color(open[i-2], close[i-2]) == 1
            && candle_color(open[i], close[i]) == -1
            && low[i-1] > high[i-2]
            && high[i] < low[i-1]
            && close[i] < close[i-2] - real_body(open[i-2], close[i-2]) * penetration;
        output[i] = (bull as i32) * 100 - (bear as i32) * 100;
        body_long_sum += cr(BODY_LONG, open, high, low, close, i-2) - cr(BODY_LONG, open, high, low, close, i - 2 - BODY_LONG.avg_period);
        body_doji_sum += cr(BODY_DOJI, open, high, low, close, i-1) - cr(BODY_DOJI, open, high, low, close, i - 1 - BODY_DOJI.avg_period);
        body_short_sum += cr(BODY_SHORT, open, high, low, close, i) - cr(BODY_SHORT, open, high, low, close, i - BODY_SHORT.avg_period);
    }
    Ok(output)
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
/// Compute the candle advance block result for the supplied aligned series.
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
pub fn candle_advance_block(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = *[SHADOW_LONG.avg_period, SHADOW_SHORT.avg_period, FAR.avg_period, NEAR.avg_period, BODY_LONG.avg_period].iter().max().unwrap() + 2;
    if len <= lookback { return Ok(output); }

    let mut shadow_short_sum = [0.0f64; 3];
    let mut shadow_long_sum = [0.0f64; 3];
    let mut near_sum = [0.0f64; 3];
    let mut far_sum = [0.0f64; 3];
    let mut body_long_sum = 0.0;
    let start = lookback;

    for k in 0..3 {
        let bar = start - 2 + k;
        // SHADOW_LONG and SHADOW_SHORT init
        if SHADOW_SHORT.avg_period > 0 && bar >= SHADOW_SHORT.avg_period {
            for j in (bar - SHADOW_SHORT.avg_period)..bar { shadow_short_sum[k] += cr(SHADOW_SHORT, open, high, low, close, j); }
        }
        // SHADOW_LONG avg_period=0
        if NEAR.avg_period > 0 && bar >= NEAR.avg_period {
            for j in (bar - NEAR.avg_period)..bar { near_sum[k] += cr(NEAR, open, high, low, close, j); }
        }
        if FAR.avg_period > 0 && bar >= FAR.avg_period {
            for j in (bar - FAR.avg_period)..bar { far_sum[k] += cr(FAR, open, high, low, close, j); }
        }
    }
    for i in (start - 2 - BODY_LONG.avg_period)..(start - 2) { body_long_sum += cr(BODY_LONG, open, high, low, close, i); }

    for i in start..len {
        let base = candle_color(open[i-2], close[i-2]) == 1
            && candle_color(open[i-1], close[i-1]) == 1
            && candle_color(open[i], close[i]) == 1
            && close[i-1] > close[i-2] && close[i] > close[i-1]
            // Opens within/near previous body
            && open[i-1] > open[i-2] && open[i-1] <= close[i-2] + ca(NEAR, near_sum[1], open, high, low, close, i-1)
            && open[i] > open[i-1] && open[i] <= close[i-1] + ca(NEAR, near_sum[2], open, high, low, close, i)
            // 1st: long body, short upper shadow
            && real_body(open[i-2], close[i-2]) > ca(BODY_LONG, body_long_sum, open, high, low, close, i-2)
            && upper_shadow(open[i-2], high[i-2], close[i-2]) < ca(SHADOW_SHORT, shadow_short_sum[0], open, high, low, close, i-2);
        // Weakness: bodies getting smaller and/or shadows getting longer
        let weakness = base && (
            (real_body(open[i-1], close[i-1]) < real_body(open[i-2], close[i-2]) - ca(FAR, far_sum[1], open, high, low, close, i-1)
                && real_body(open[i], close[i]) < real_body(open[i-1], close[i-1]) + ca(NEAR, near_sum[2], open, high, low, close, i))
            || (real_body(open[i], close[i]) < real_body(open[i-1], close[i-1])
                && real_body(open[i-1], close[i-1]) < real_body(open[i-2], close[i-2])
                && (upper_shadow(open[i], high[i], close[i]) > ca(SHADOW_LONG, shadow_long_sum[2], open, high, low, close, i)
                    || upper_shadow(open[i-1], high[i-1], close[i-1]) > ca(SHADOW_LONG, shadow_long_sum[1], open, high, low, close, i-1)))
            || (real_body(open[i], close[i]) < real_body(open[i-1], close[i-1]) - ca(FAR, far_sum[2], open, high, low, close, i)));
        output[i] = weakness as i32 * -100;
        // Update sums
        for k in 0..3 {
            let bar = i - 2 + k;
            if SHADOW_SHORT.avg_period > 0 && bar >= SHADOW_SHORT.avg_period {
                shadow_short_sum[k] += cr(SHADOW_SHORT, open, high, low, close, bar) - cr(SHADOW_SHORT, open, high, low, close, bar - SHADOW_SHORT.avg_period);
            }
            if NEAR.avg_period > 0 && bar >= NEAR.avg_period {
                near_sum[k] += cr(NEAR, open, high, low, close, bar) - cr(NEAR, open, high, low, close, bar - NEAR.avg_period);
            }
            if FAR.avg_period > 0 && bar >= FAR.avg_period {
                far_sum[k] += cr(FAR, open, high, low, close, bar) - cr(FAR, open, high, low, close, bar - FAR.avg_period);
            }
        }
        body_long_sum += cr(BODY_LONG, open, high, low, close, i-2) - cr(BODY_LONG, open, high, low, close, i - 2 - BODY_LONG.avg_period);
    }
    Ok(output)
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
/// Compute the candle breakaway result for the supplied aligned series.
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
pub fn candle_breakaway(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_LONG.avg_period + 4;
    if len <= lookback { return Ok(output); }

    let mut body_sum = 0.0;
    let start = lookback;
    for i in (start - 4 - BODY_LONG.avg_period)..(start - 4) { body_sum += cr(BODY_LONG, open, high, low, close, i); }

    for i in start..len {
        let base = real_body(open[i-4], close[i-4]) > ca(BODY_LONG, body_sum, open, high, low, close, i-4)
            && candle_color(open[i-4], close[i-4]) == candle_color(open[i-3], close[i-3])
            && candle_color(open[i-3], close[i-3]) == candle_color(open[i-1], close[i-1])
            && candle_color(open[i-1], close[i-1]) == -candle_color(open[i], close[i]);
        // Bearish first (black): gap down, progressive lower H/L, 5th closes in gap
        let bear_first = base
            && candle_color(open[i-4], close[i-4]) == -1
            && real_body_gap_down(open, close, i-3, i-4)
            && high[i-2] < high[i-3] && low[i-2] < low[i-3]
            && high[i-1] < high[i-2] && low[i-1] < low[i-2]
            && close[i] > open[i-3] && close[i] < close[i-4];
        // Bullish first (white): gap up, progressive higher H/L, 5th closes in gap
        let bull_first = base
            && candle_color(open[i-4], close[i-4]) == 1
            && real_body_gap_up(open, close, i-3, i-4)
            && high[i-2] > high[i-3] && low[i-2] > low[i-3]
            && high[i-1] > high[i-2] && low[i-1] > low[i-2]
            && close[i] < open[i-3] && close[i] > close[i-4];
        output[i] = (bear_first as i32 | bull_first as i32) * candle_color(open[i], close[i]) * 100;
        body_sum += cr(BODY_LONG, open, high, low, close, i-4) - cr(BODY_LONG, open, high, low, close, i - 4 - BODY_LONG.avg_period);
    }
    Ok(output)
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
/// Compute the candle conceal baby swall result for the supplied aligned series.
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
pub fn candle_conceal_baby_swall(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = SHADOW_VERY_SHORT.avg_period + 3;
    if len <= lookback { return Ok(output); }

    let mut shadow_sum = [0.0f64; 4];
    let start = lookback;
    for k in 0..4 {
        let bar = start - 3 + k;
        if bar >= SHADOW_VERY_SHORT.avg_period {
            for j in (bar - SHADOW_VERY_SHORT.avg_period)..bar {
                shadow_sum[k] += cr(SHADOW_VERY_SHORT, open, high, low, close, j);
            }
        }
    }

    for i in start..len {
        output[i] = (candle_color(open[i-3], close[i-3]) == -1
            && candle_color(open[i-2], close[i-2]) == -1
            && candle_color(open[i-1], close[i-1]) == -1
            && candle_color(open[i], close[i]) == -1
            // 1st and 2nd: marubozu (very short shadows)
            && upper_shadow(open[i-3], high[i-3], close[i-3]) < ca(SHADOW_VERY_SHORT, shadow_sum[0], open, high, low, close, i-3)
            && lower_shadow(open[i-3], low[i-3], close[i-3]) < ca(SHADOW_VERY_SHORT, shadow_sum[0], open, high, low, close, i-3)
            && upper_shadow(open[i-2], high[i-2], close[i-2]) < ca(SHADOW_VERY_SHORT, shadow_sum[1], open, high, low, close, i-2)
            && lower_shadow(open[i-2], low[i-2], close[i-2]) < ca(SHADOW_VERY_SHORT, shadow_sum[1], open, high, low, close, i-2)
            // 3rd: gaps down, upper shadow into 2nd body
            && real_body_gap_down(open, close, i-1, i-2)
            && high[i-1] > close[i-2]
            // 4th: engulfs 3rd including shadows
            && open[i] >= high[i-1] && close[i] <= low[i-1]) as i32 * 100;
        for k in 0..4 {
            let bar = i - 3 + k;
            if SHADOW_VERY_SHORT.avg_period > 0 && bar >= SHADOW_VERY_SHORT.avg_period {
                shadow_sum[k] += cr(SHADOW_VERY_SHORT, open, high, low, close, bar)
                    - cr(SHADOW_VERY_SHORT, open, high, low, close, bar - SHADOW_VERY_SHORT.avg_period);
            }
        }
    }
    Ok(output)
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
/// Compute the candle evening doji star result for the supplied aligned series.
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
pub fn candle_evening_doji_star(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let penetration = 0.3;
    let lookback = *[BODY_DOJI.avg_period, BODY_LONG.avg_period, BODY_SHORT.avg_period].iter().max().unwrap() + 2;
    if len <= lookback { return Ok(output); }

    let mut body_long_sum = 0.0;
    let mut body_doji_sum = 0.0;
    let mut body_short_sum = 0.0;
    let start = lookback;
    for i in (start - 2 - BODY_LONG.avg_period)..(start - 2) { body_long_sum += cr(BODY_LONG, open, high, low, close, i); }
    for i in (start - 1 - BODY_DOJI.avg_period)..(start - 1) { body_doji_sum += cr(BODY_DOJI, open, high, low, close, i); }
    for i in (start - BODY_SHORT.avg_period)..start { body_short_sum += cr(BODY_SHORT, open, high, low, close, i); }

    for i in start..len {
        output[i] = (candle_color(open[i-2], close[i-2]) == 1
            && real_body(open[i-2], close[i-2]) > ca(BODY_LONG, body_long_sum, open, high, low, close, i-2)
            && real_body(open[i-1], close[i-1]) <= ca(BODY_DOJI, body_doji_sum, open, high, low, close, i-1)
            && real_body_gap_up(open, close, i-1, i-2)
            && candle_color(open[i], close[i]) == -1
            && real_body(open[i], close[i]) > ca(BODY_SHORT, body_short_sum, open, high, low, close, i)
            && close[i] < close[i-2] - real_body(open[i-2], close[i-2]) * penetration) as i32 * -100;
        body_long_sum += cr(BODY_LONG, open, high, low, close, i-2) - cr(BODY_LONG, open, high, low, close, i - 2 - BODY_LONG.avg_period);
        body_doji_sum += cr(BODY_DOJI, open, high, low, close, i-1) - cr(BODY_DOJI, open, high, low, close, i - 1 - BODY_DOJI.avg_period);
        body_short_sum += cr(BODY_SHORT, open, high, low, close, i) - cr(BODY_SHORT, open, high, low, close, i - BODY_SHORT.avg_period);
    }
    Ok(output)
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
/// Compute the candle evening star result for the supplied aligned series.
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
pub fn candle_evening_star(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let penetration = 0.3;
    let lookback = BODY_SHORT.avg_period.max(BODY_LONG.avg_period) + 2;
    if len <= lookback { return Ok(output); }

    let mut body_long_sum = 0.0;
    let mut body_short_sum = 0.0;
    let mut body_short_sum2 = 0.0;
    let start = lookback;
    for i in (start - 2 - BODY_LONG.avg_period)..(start - 2) { body_long_sum += cr(BODY_LONG, open, high, low, close, i); }
    for i in (start - 1 - BODY_SHORT.avg_period)..(start - 1) { body_short_sum += cr(BODY_SHORT, open, high, low, close, i); }
    for i in (start - BODY_SHORT.avg_period)..start { body_short_sum2 += cr(BODY_SHORT, open, high, low, close, i); }

    for i in start..len {
        output[i] = (candle_color(open[i-2], close[i-2]) == 1
            && real_body(open[i-2], close[i-2]) > ca(BODY_LONG, body_long_sum, open, high, low, close, i-2)
            && real_body(open[i-1], close[i-1]) <= ca(BODY_SHORT, body_short_sum, open, high, low, close, i-1)
            && real_body_gap_up(open, close, i-1, i-2)
            && candle_color(open[i], close[i]) == -1
            && real_body(open[i], close[i]) > ca(BODY_SHORT, body_short_sum2, open, high, low, close, i)
            && close[i] < close[i-2] - real_body(open[i-2], close[i-2]) * penetration) as i32 * -100;
        body_long_sum += cr(BODY_LONG, open, high, low, close, i-2) - cr(BODY_LONG, open, high, low, close, i - 2 - BODY_LONG.avg_period);
        body_short_sum += cr(BODY_SHORT, open, high, low, close, i-1) - cr(BODY_SHORT, open, high, low, close, i - 1 - BODY_SHORT.avg_period);
        body_short_sum2 += cr(BODY_SHORT, open, high, low, close, i) - cr(BODY_SHORT, open, high, low, close, i - BODY_SHORT.avg_period);
    }
    Ok(output)
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
/// Compute the candle gap side side white result for the supplied aligned series.
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
pub fn candle_gap_side_side_white(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = NEAR.avg_period.max(EQUAL.avg_period) + 2;
    if len <= lookback { return Ok(output); }

    let mut near_sum = 0.0;
    let mut equal_sum = 0.0;
    let start = lookback;
    for i in (start - 1 - NEAR.avg_period)..(start - 1) { near_sum += cr(NEAR, open, high, low, close, i); }
    for i in (start - 1 - EQUAL.avg_period)..(start - 1) { equal_sum += cr(EQUAL, open, high, low, close, i); }

    for i in start..len {
        let base = candle_color(open[i-1], close[i-1]) == 1
            && candle_color(open[i], close[i]) == 1
            && (real_body(open[i-1], close[i-1]) - real_body(open[i], close[i])).abs() < ca(NEAR, near_sum, open, high, low, close, i-1)
            && (open[i-1] - open[i]).abs() < ca(EQUAL, equal_sum, open, high, low, close, i-1);
        // Upside gap
        let bull = base && real_body_gap_up(open, close, i-1, i-2);
        // Downside gap
        let bear = base && real_body_gap_down(open, close, i-1, i-2);
        output[i] = (bull as i32) * 100 - (bear as i32) * 100;
        near_sum += cr(NEAR, open, high, low, close, i-1) - cr(NEAR, open, high, low, close, i - 1 - NEAR.avg_period);
        equal_sum += cr(EQUAL, open, high, low, close, i-1) - cr(EQUAL, open, high, low, close, i - 1 - EQUAL.avg_period);
    }
    Ok(output)
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
/// Compute the candle identical three crows result for the supplied aligned series.
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
pub fn candle_identical_three_crows(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = SHADOW_VERY_SHORT.avg_period.max(EQUAL.avg_period) + 2;
    if len <= lookback { return Ok(output); }

    let mut shadow_sum = [0.0f64; 3];
    let mut equal_sum = [0.0f64; 3];
    let start = lookback;
    for k in 0..3 {
        let bar = start - 2 + k;
        if bar >= SHADOW_VERY_SHORT.avg_period {
            for j in (bar - SHADOW_VERY_SHORT.avg_period)..bar { shadow_sum[k] += cr(SHADOW_VERY_SHORT, open, high, low, close, j); }
        }
        if k < 2 && bar >= EQUAL.avg_period {
            for j in (bar - EQUAL.avg_period)..bar { equal_sum[k] += cr(EQUAL, open, high, low, close, j); }
        }
    }

    for i in start..len {
        output[i] = (candle_color(open[i-2], close[i-2]) == -1
            && candle_color(open[i-1], close[i-1]) == -1
            && candle_color(open[i], close[i]) == -1
            && close[i-1] < close[i-2] && close[i] < close[i-1]
            // Very short lower shadows
            && lower_shadow(open[i-2], low[i-2], close[i-2]) < ca(SHADOW_VERY_SHORT, shadow_sum[0], open, high, low, close, i-2)
            && lower_shadow(open[i-1], low[i-1], close[i-1]) < ca(SHADOW_VERY_SHORT, shadow_sum[1], open, high, low, close, i-1)
            && lower_shadow(open[i], low[i], close[i]) < ca(SHADOW_VERY_SHORT, shadow_sum[2], open, high, low, close, i)
            // Each opens equal to prior close
            && (open[i-1] - close[i-2]).abs() <= ca(EQUAL, equal_sum[0], open, high, low, close, i-2)
            && (open[i] - close[i-1]).abs() <= ca(EQUAL, equal_sum[1], open, high, low, close, i-1)) as i32 * -100;
        for k in 0..3 {
            let bar = i - 2 + k;
            if SHADOW_VERY_SHORT.avg_period > 0 && bar >= SHADOW_VERY_SHORT.avg_period {
                shadow_sum[k] += cr(SHADOW_VERY_SHORT, open, high, low, close, bar) - cr(SHADOW_VERY_SHORT, open, high, low, close, bar - SHADOW_VERY_SHORT.avg_period);
            }
        }
        for k in 0..2 {
            let bar = i - 2 + k;
            if EQUAL.avg_period > 0 && bar >= EQUAL.avg_period {
                equal_sum[k] += cr(EQUAL, open, high, low, close, bar) - cr(EQUAL, open, high, low, close, bar - EQUAL.avg_period);
            }
        }
    }
    Ok(output)
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
/// Compute the candle ladder bottom result for the supplied aligned series.
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
pub fn candle_ladder_bottom(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = SHADOW_VERY_SHORT.avg_period + 4;
    if len <= lookback { return Ok(output); }

    let mut shadow_sum = 0.0;
    let start = lookback;
    for i in (start - 1 - SHADOW_VERY_SHORT.avg_period)..(start - 1) { shadow_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i); }

    for i in start..len {
        output[i] = (candle_color(open[i-4], close[i-4]) == -1
            && candle_color(open[i-3], close[i-3]) == -1
            && candle_color(open[i-2], close[i-2]) == -1
            && candle_color(open[i-1], close[i-1]) == -1
            && close[i-3] < close[i-4] && close[i-2] < close[i-3]
            // 4th: upper shadow exceeds very short
            && upper_shadow(open[i-1], high[i-1], close[i-1]) > ca(SHADOW_VERY_SHORT, shadow_sum, open, high, low, close, i-1)
            // 5th: white, opens above 4th open, closes above 4th high
            && candle_color(open[i], close[i]) == 1
            && open[i] > open[i-1]
            && close[i] > high[i-1]) as i32 * 100;
        shadow_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i-1) - cr(SHADOW_VERY_SHORT, open, high, low, close, i - 1 - SHADOW_VERY_SHORT.avg_period);
    }
    Ok(output)
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
/// Compute the candle mat hold result for the supplied aligned series.
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
pub fn candle_mat_hold(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let penetration = 0.5;
    let lookback = BODY_SHORT.avg_period.max(BODY_LONG.avg_period) + 4;
    if len <= lookback { return Ok(output); }

    let mut body_sum = [0.0f64; 5];
    let start = lookback;
    // Init long body sum for i-4, short for i-3..i-1, long for i
    for i in (start - 4 - BODY_LONG.avg_period)..(start - 4) { body_sum[4] += cr(BODY_LONG, open, high, low, close, i); }
    for k in 1..4 {
        let bar = start - 4 + k;
        for j in (bar - BODY_SHORT.avg_period)..bar { body_sum[4 - k] += cr(BODY_SHORT, open, high, low, close, j); }
    }

    for i in start..len {
        output[i] = (real_body(open[i-4], close[i-4]) > ca(BODY_LONG, body_sum[4], open, high, low, close, i-4)
            && real_body(open[i-3], close[i-3]) < ca(BODY_SHORT, body_sum[3], open, high, low, close, i-3)
            && real_body(open[i-2], close[i-2]) < ca(BODY_SHORT, body_sum[2], open, high, low, close, i-2)
            && real_body(open[i-1], close[i-1]) < ca(BODY_SHORT, body_sum[1], open, high, low, close, i-1)
            // white, black, ?, ?, white
            && candle_color(open[i-4], close[i-4]) == 1
            && candle_color(open[i-3], close[i-3]) == -1
            && candle_color(open[i], close[i]) == 1
            // upside gap 1st to 2nd
            && real_body_gap_up(open, close, i-3, i-4)
            // 3rd and 4th hold within 1st close
            && open[i-2].min(close[i-2]) < close[i-4]
            && open[i-1].min(close[i-1]) < close[i-4]
            // penetration check
            && open[i-2].min(close[i-2]) > close[i-4] - real_body(open[i-4], close[i-4]) * penetration
            && open[i-1].min(close[i-1]) > close[i-4] - real_body(open[i-4], close[i-4]) * penetration
            // 2nd to 4th are falling
            && open[i-2].max(close[i-2]) < open[i-3]
            && open[i-1].max(close[i-1]) < open[i-2].max(close[i-2])
            // 5th opens above prior close, closes above highest reaction high
            && open[i] > close[i-1]
            && close[i] > high[i-3].max(high[i-2]).max(high[i-1])) as i32 * 100;
        body_sum[4] += cr(BODY_LONG, open, high, low, close, i-4) - cr(BODY_LONG, open, high, low, close, i - 4 - BODY_LONG.avg_period);
        for k in 1..4 {
            let bar = i - 4 + k;
            body_sum[4 - k] += cr(BODY_SHORT, open, high, low, close, bar) - cr(BODY_SHORT, open, high, low, close, bar - BODY_SHORT.avg_period);
        }
    }
    Ok(output)
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
/// Compute the candle morning doji star result for the supplied aligned series.
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
pub fn candle_morning_doji_star(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let penetration = 0.3;
    let lookback = *[BODY_DOJI.avg_period, BODY_LONG.avg_period, BODY_SHORT.avg_period].iter().max().unwrap() + 2;
    if len <= lookback { return Ok(output); }

    let mut body_long_sum = 0.0;
    let mut body_doji_sum = 0.0;
    let mut body_short_sum = 0.0;
    let start = lookback;
    for i in (start - 2 - BODY_LONG.avg_period)..(start - 2) { body_long_sum += cr(BODY_LONG, open, high, low, close, i); }
    for i in (start - 1 - BODY_DOJI.avg_period)..(start - 1) { body_doji_sum += cr(BODY_DOJI, open, high, low, close, i); }
    for i in (start - BODY_SHORT.avg_period)..start { body_short_sum += cr(BODY_SHORT, open, high, low, close, i); }

    for i in start..len {
        output[i] = (candle_color(open[i-2], close[i-2]) == -1
            && real_body(open[i-2], close[i-2]) > ca(BODY_LONG, body_long_sum, open, high, low, close, i-2)
            && real_body(open[i-1], close[i-1]) <= ca(BODY_DOJI, body_doji_sum, open, high, low, close, i-1)
            && real_body_gap_down(open, close, i-1, i-2)
            && candle_color(open[i], close[i]) == 1
            && real_body(open[i], close[i]) > ca(BODY_SHORT, body_short_sum, open, high, low, close, i)
            && close[i] > close[i-2] + real_body(open[i-2], close[i-2]) * penetration) as i32 * 100;
        body_long_sum += cr(BODY_LONG, open, high, low, close, i-2) - cr(BODY_LONG, open, high, low, close, i - 2 - BODY_LONG.avg_period);
        body_doji_sum += cr(BODY_DOJI, open, high, low, close, i-1) - cr(BODY_DOJI, open, high, low, close, i - 1 - BODY_DOJI.avg_period);
        body_short_sum += cr(BODY_SHORT, open, high, low, close, i) - cr(BODY_SHORT, open, high, low, close, i - BODY_SHORT.avg_period);
    }
    Ok(output)
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
/// Compute the candle morning star result for the supplied aligned series.
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
pub fn candle_morning_star(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let penetration = 0.3;
    let lookback = BODY_SHORT.avg_period.max(BODY_LONG.avg_period) + 2;
    if len <= lookback { return Ok(output); }

    let mut body_long_sum = 0.0;
    let mut body_short_sum = 0.0;
    let mut body_short_sum2 = 0.0;
    let start = lookback;
    for i in (start - 2 - BODY_LONG.avg_period)..(start - 2) { body_long_sum += cr(BODY_LONG, open, high, low, close, i); }
    for i in (start - 1 - BODY_SHORT.avg_period)..(start - 1) { body_short_sum += cr(BODY_SHORT, open, high, low, close, i); }
    for i in (start - BODY_SHORT.avg_period)..start { body_short_sum2 += cr(BODY_SHORT, open, high, low, close, i); }

    for i in start..len {
        output[i] = (candle_color(open[i-2], close[i-2]) == -1
            && real_body(open[i-2], close[i-2]) > ca(BODY_LONG, body_long_sum, open, high, low, close, i-2)
            && real_body(open[i-1], close[i-1]) <= ca(BODY_SHORT, body_short_sum, open, high, low, close, i-1)
            && real_body_gap_down(open, close, i-1, i-2)
            && candle_color(open[i], close[i]) == 1
            && real_body(open[i], close[i]) > ca(BODY_SHORT, body_short_sum2, open, high, low, close, i)
            && close[i] > close[i-2] + real_body(open[i-2], close[i-2]) * penetration) as i32 * 100;
        body_long_sum += cr(BODY_LONG, open, high, low, close, i-2) - cr(BODY_LONG, open, high, low, close, i - 2 - BODY_LONG.avg_period);
        body_short_sum += cr(BODY_SHORT, open, high, low, close, i-1) - cr(BODY_SHORT, open, high, low, close, i - 1 - BODY_SHORT.avg_period);
        body_short_sum2 += cr(BODY_SHORT, open, high, low, close, i) - cr(BODY_SHORT, open, high, low, close, i - BODY_SHORT.avg_period);
    }
    Ok(output)
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
/// Compute the candle rise fall three methods result for the supplied aligned series.
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
pub fn candle_rise_fall_three_methods(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_SHORT.avg_period.max(BODY_LONG.avg_period) + 4;
    if len <= lookback { return Ok(output); }

    let mut body_long_sum = [0.0f64; 2]; // [0]=i, [1]=i-4
    let mut body_short_sum = [0.0f64; 3]; // for i-3, i-2, i-1
    let start = lookback;
    for i in (start - 4 - BODY_LONG.avg_period)..(start - 4) { body_long_sum[1] += cr(BODY_LONG, open, high, low, close, i); }
    for i in (start - BODY_LONG.avg_period)..start { body_long_sum[0] += cr(BODY_LONG, open, high, low, close, i); }
    for k in 0..3 {
        let bar = start - 3 + k;
        for j in (bar - BODY_SHORT.avg_period)..bar { body_short_sum[k] += cr(BODY_SHORT, open, high, low, close, j); }
    }

    for i in start..len {
        let c4 = candle_color(open[i-4], close[i-4]);
        let c0 = candle_color(open[i], close[i]);
        if real_body(open[i-4], close[i-4]) > ca(BODY_LONG, body_long_sum[1], open, high, low, close, i-4)
            && real_body(open[i], close[i]) > ca(BODY_LONG, body_long_sum[0], open, high, low, close, i)
        {
            // 1st long, 3 short middle, 5th long
            let mid_short = real_body(open[i-3], close[i-3]) < ca(BODY_SHORT, body_short_sum[0], open, high, low, close, i-3)
                && real_body(open[i-2], close[i-2]) < ca(BODY_SHORT, body_short_sum[1], open, high, low, close, i-2)
                && real_body(open[i-1], close[i-1]) < ca(BODY_SHORT, body_short_sum[2], open, high, low, close, i-1);

            let bull = c4 == 1 && mid_short
                && candle_color(open[i-3], close[i-3]) == -1
                && candle_color(open[i-2], close[i-2]) == -1
                && candle_color(open[i-1], close[i-1]) == -1
                && close[i-3] < close[i-4]
                && close[i-2] < close[i-3]
                && close[i-1] < close[i-2]
                && low[i-3] > low[i-4] && low[i-2] > low[i-4] && low[i-1] > low[i-4]
                && high[i-3] < high[i-4] && high[i-2] < high[i-4] && high[i-1] < high[i-4]
                && c0 == 1
                && open[i] > close[i-1]
                && close[i] > close[i-4];
            let bear = c4 == -1 && mid_short
                && candle_color(open[i-3], close[i-3]) == 1
                && candle_color(open[i-2], close[i-2]) == 1
                && candle_color(open[i-1], close[i-1]) == 1
                && close[i-3] > close[i-4]
                && close[i-2] > close[i-3]
                && close[i-1] > close[i-2]
                && high[i-3] < high[i-4] && high[i-2] < high[i-4] && high[i-1] < high[i-4]
                && low[i-3] > low[i-4] && low[i-2] > low[i-4] && low[i-1] > low[i-4]
                && c0 == -1
                && open[i] < close[i-1]
                && close[i] < close[i-4];
            output[i] = (bull as i32) * 100 - (bear as i32) * 100;
        }
        body_long_sum[1] += cr(BODY_LONG, open, high, low, close, i-4) - cr(BODY_LONG, open, high, low, close, i - 4 - BODY_LONG.avg_period);
        body_long_sum[0] += cr(BODY_LONG, open, high, low, close, i) - cr(BODY_LONG, open, high, low, close, i - BODY_LONG.avg_period);
        for k in 0..3 {
            let bar = i - 3 + k;
            body_short_sum[k] += cr(BODY_SHORT, open, high, low, close, bar) - cr(BODY_SHORT, open, high, low, close, bar - BODY_SHORT.avg_period);
        }
    }
    Ok(output)
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
/// Compute the candle stalled pattern result for the supplied aligned series.
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
pub fn candle_stalled_pattern(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = *[BODY_LONG.avg_period, BODY_SHORT.avg_period, SHADOW_VERY_SHORT.avg_period, NEAR.avg_period].iter().max().unwrap() + 2;
    if len <= lookback { return Ok(output); }

    let mut body_long_sum = [0.0f64; 2]; // for i-2 and i-1
    let mut body_short_sum = 0.0;
    let mut shadow_sum = 0.0;
    let mut near_sum = [0.0f64; 3];
    let start = lookback;

    for i in (start - 2 - BODY_LONG.avg_period)..(start - 2) { body_long_sum[0] += cr(BODY_LONG, open, high, low, close, i); }
    for i in (start - 1 - BODY_LONG.avg_period)..(start - 1) { body_long_sum[1] += cr(BODY_LONG, open, high, low, close, i); }
    for i in (start - BODY_SHORT.avg_period)..start { body_short_sum += cr(BODY_SHORT, open, high, low, close, i); }
    for i in (start - 1 - SHADOW_VERY_SHORT.avg_period)..(start - 1) { shadow_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i); }
    for k in 0..3 {
        let bar = start - 2 + k;
        if NEAR.avg_period > 0 && bar >= NEAR.avg_period {
            for j in (bar - NEAR.avg_period)..bar { near_sum[k] += cr(NEAR, open, high, low, close, j); }
        }
    }

    for i in start..len {
        output[i] = (candle_color(open[i-2], close[i-2]) == 1
            && candle_color(open[i-1], close[i-1]) == 1
            && candle_color(open[i], close[i]) == 1
            && close[i-1] > close[i-2] && close[i] > close[i-1]
            && real_body(open[i-2], close[i-2]) > ca(BODY_LONG, body_long_sum[0], open, high, low, close, i-2)
            && real_body(open[i-1], close[i-1]) > ca(BODY_LONG, body_long_sum[1], open, high, low, close, i-1)
            && upper_shadow(open[i-1], high[i-1], close[i-1]) < ca(SHADOW_VERY_SHORT, shadow_sum, open, high, low, close, i-1)
            && open[i-1] > open[i-2]
            && open[i-1] <= close[i-2] + ca(NEAR, near_sum[0], open, high, low, close, i-2)
            && real_body(open[i], close[i]) < ca(BODY_SHORT, body_short_sum, open, high, low, close, i)
            && open[i] >= close[i-1] - real_body(open[i], close[i]) - ca(NEAR, near_sum[1], open, high, low, close, i-1)) as i32 * -100;
        body_long_sum[0] += cr(BODY_LONG, open, high, low, close, i-2) - cr(BODY_LONG, open, high, low, close, i - 2 - BODY_LONG.avg_period);
        body_long_sum[1] += cr(BODY_LONG, open, high, low, close, i-1) - cr(BODY_LONG, open, high, low, close, i - 1 - BODY_LONG.avg_period);
        body_short_sum += cr(BODY_SHORT, open, high, low, close, i) - cr(BODY_SHORT, open, high, low, close, i - BODY_SHORT.avg_period);
        shadow_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i-1) - cr(SHADOW_VERY_SHORT, open, high, low, close, i - 1 - SHADOW_VERY_SHORT.avg_period);
        for k in 0..3 {
            let bar = i - 2 + k;
            if NEAR.avg_period > 0 && bar >= NEAR.avg_period {
                near_sum[k] += cr(NEAR, open, high, low, close, bar) - cr(NEAR, open, high, low, close, bar - NEAR.avg_period);
            }
        }
    }
    Ok(output)
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
/// Compute the candle tasuki gap result for the supplied aligned series.
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
pub fn candle_tasuki_gap(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = NEAR.avg_period + 2;
    if len <= lookback { return Ok(output); }

    let mut near_sum = 0.0;
    let start = lookback;
    for i in (start - 1 - NEAR.avg_period)..(start - 1) { near_sum += cr(NEAR, open, high, low, close, i); }

    for i in start..len {
        let c1 = candle_color(open[i-1], close[i-1]);
        let c0 = candle_color(open[i], close[i]);

        // Bodies near same size
        let near_same = (real_body(open[i-1], close[i-1]) - real_body(open[i], close[i])).abs()
            < ca(NEAR, near_sum, open, high, low, close, i-1);

        // Bullish: upside gap, white bar then black bar
        let bull = real_body_gap_up(open, close, i-1, i-2)
            && c1 == 1 && c0 == -1
            && open[i] < close[i-1] && open[i] > open[i-1]
            && close[i] < open[i-1]
            && close[i] > open[i-2].max(close[i-2])
            && near_same;
        // Bearish: downside gap, black bar then white bar
        let bear = real_body_gap_down(open, close, i-1, i-2)
            && c1 == -1 && c0 == 1
            && open[i] < open[i-1] && open[i] > close[i-1]
            && close[i] > open[i-1]
            && close[i] < open[i-2].min(close[i-2])
            && near_same;
        output[i] = (bull as i32 | bear as i32) * c1 * 100;
        near_sum += cr(NEAR, open, high, low, close, i-1) - cr(NEAR, open, high, low, close, i - 1 - NEAR.avg_period);
    }
    Ok(output)
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
/// Compute the candle tri star result for the supplied aligned series.
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
pub fn candle_tri_star(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_DOJI.avg_period + 2;
    if len <= lookback { return Ok(output); }

    let mut body_sum = 0.0;
    let start = lookback;
    for i in (start - 2 - BODY_DOJI.avg_period)..(start - 2) { body_sum += cr(BODY_DOJI, open, high, low, close, i); }

    for i in start..len {
        let base = real_body(open[i-2], close[i-2]) <= ca(BODY_DOJI, body_sum, open, high, low, close, i-2)
            && real_body(open[i-1], close[i-1]) <= ca(BODY_DOJI, body_sum, open, high, low, close, i-1)
            && real_body(open[i], close[i]) <= ca(BODY_DOJI, body_sum, open, high, low, close, i);
        // Bearish: 2nd gaps up
        let bear = base && real_body_gap_up(open, close, i-1, i-2)
            && !real_body_gap_up(open, close, i, i-1);
        // Bullish: 2nd gaps down
        let bull = base && real_body_gap_down(open, close, i-1, i-2)
            && !real_body_gap_down(open, close, i, i-1);
        output[i] = (bull as i32) * 100 - (bear as i32) * 100;
        body_sum += cr(BODY_DOJI, open, high, low, close, i-2) - cr(BODY_DOJI, open, high, low, close, i - 2 - BODY_DOJI.avg_period);
    }
    Ok(output)
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
/// Compute the candle unique three river result for the supplied aligned series.
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
pub fn candle_unique_three_river(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_SHORT.avg_period.max(BODY_LONG.avg_period) + 2;
    if len <= lookback { return Ok(output); }

    let mut body_long_sum = 0.0;
    let mut body_short_sum = 0.0;
    let start = lookback;
    for i in (start - 2 - BODY_LONG.avg_period)..(start - 2) { body_long_sum += cr(BODY_LONG, open, high, low, close, i); }
    for i in (start - BODY_SHORT.avg_period)..start { body_short_sum += cr(BODY_SHORT, open, high, low, close, i); }

    for i in start..len {
        output[i] = (candle_color(open[i-2], close[i-2]) == -1
            && real_body(open[i-2], close[i-2]) > ca(BODY_LONG, body_long_sum, open, high, low, close, i-2)
            // 2nd: black, harami, lower low
            && candle_color(open[i-1], close[i-1]) == -1
            && open[i-1].max(close[i-1]) < open[i-2].max(close[i-2])
            && open[i-1].min(close[i-1]) > open[i-2].min(close[i-2])
            && low[i-1] < low[i-2]
            // 3rd: small white, close <= 2nd close
            && candle_color(open[i], close[i]) == 1
            && real_body(open[i], close[i]) < ca(BODY_SHORT, body_short_sum, open, high, low, close, i)
            && close[i] > close[i-1]) as i32 * 100;
        body_long_sum += cr(BODY_LONG, open, high, low, close, i-2) - cr(BODY_LONG, open, high, low, close, i - 2 - BODY_LONG.avg_period);
        body_short_sum += cr(BODY_SHORT, open, high, low, close, i) - cr(BODY_SHORT, open, high, low, close, i - BODY_SHORT.avg_period);
    }
    Ok(output)
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
/// Compute the candle upside gap two crows result for the supplied aligned series.
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
pub fn candle_upside_gap_two_crows(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_SHORT.avg_period.max(BODY_LONG.avg_period) + 2;
    if len <= lookback { return Ok(output); }

    let mut body_long_sum = 0.0;
    let mut body_short_sum = 0.0;
    let start = lookback;
    for i in (start - 2 - BODY_LONG.avg_period)..(start - 2) { body_long_sum += cr(BODY_LONG, open, high, low, close, i); }
    for i in (start - 1 - BODY_SHORT.avg_period)..(start - 1) { body_short_sum += cr(BODY_SHORT, open, high, low, close, i); }

    for i in start..len {
        output[i] = (candle_color(open[i-2], close[i-2]) == 1
            && real_body(open[i-2], close[i-2]) > ca(BODY_LONG, body_long_sum, open, high, low, close, i-2)
            // 2nd: short black, gap up
            && candle_color(open[i-1], close[i-1]) == -1
            && real_body(open[i-1], close[i-1]) <= ca(BODY_SHORT, body_short_sum, open, high, low, close, i-1)
            && real_body_gap_up(open, close, i-1, i-2)
            // 3rd: black, engulfs 2nd body, close above 1st close
            && candle_color(open[i], close[i]) == -1
            && open[i] > open[i-1]
            && close[i] < close[i-1]
            && close[i] > close[i-2]) as i32 * -100;
        body_long_sum += cr(BODY_LONG, open, high, low, close, i-2) - cr(BODY_LONG, open, high, low, close, i - 2 - BODY_LONG.avg_period);
        body_short_sum += cr(BODY_SHORT, open, high, low, close, i-1) - cr(BODY_SHORT, open, high, low, close, i - 1 - BODY_SHORT.avg_period);
    }
    Ok(output)
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
/// Compute the candle xside gap three methods result for the supplied aligned series.
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
pub fn candle_xside_gap_three_methods(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    // lookback = 2
    if len < 3 { return Ok(output); }

    for i in 2..len {
        let c2 = candle_color(open[i-2], close[i-2]);
        let c1 = candle_color(open[i-1], close[i-1]);
        let c0 = candle_color(open[i], close[i]);

        // 3rd opens within 2nd body, closes within 1st body
        let opens_within = open[i] > open[i-1].min(close[i-1]) && open[i] < open[i-1].max(close[i-1]);
        let closes_within = close[i] > open[i-2].min(close[i-2]) && close[i] < open[i-2].max(close[i-2]);
        let base = c2 == c1 && c0 != c2 && opens_within && closes_within;
        // Upside gap
        let bull = base && c2 == 1 && real_body_gap_up(open, close, i-1, i-2);
        // Downside gap
        let bear = base && c2 == -1 && real_body_gap_down(open, close, i-1, i-2);
        output[i] = (bull as i32) * 100 - (bear as i32) * 100;
    }
    Ok(output)
}
