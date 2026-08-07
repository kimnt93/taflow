// Candlestick pattern recognition — 61 patterns
// Exact reimplementation of C TA-Lib's CandleAverage system
// Output: Vec<i32>, -100 = bearish, 0 = no signal, +100 = bullish

use crate::error::{TaError, TaResult};

// ========== C TA-Lib Candle Setting System ==========

/// Range types used by candle settings
#[derive(Clone, Copy)]
pub(crate) enum RangeType {
    RealBody,
    HighLow,
    Shadows,
}

/// A candle setting definition matching C TA-Lib defaults
#[derive(Clone, Copy)]
pub(crate) struct CandleSetting {
    pub(crate) range_type: RangeType,
    pub(crate) avg_period: usize,
    pub(crate) factor: f64,
}

// Default candle settings exactly matching C TA-Lib ta_common.c
pub(crate) const BODY_LONG: CandleSetting = CandleSetting {
    range_type: RangeType::RealBody,
    avg_period: 10,
    factor: 1.0,
};
pub(crate) const BODY_VERY_LONG: CandleSetting = CandleSetting {
    range_type: RangeType::RealBody,
    avg_period: 10,
    factor: 3.0,
};
pub(crate) const BODY_SHORT: CandleSetting = CandleSetting {
    range_type: RangeType::RealBody,
    avg_period: 10,
    factor: 1.0,
};
pub(crate) const BODY_DOJI: CandleSetting = CandleSetting {
    range_type: RangeType::HighLow,
    avg_period: 10,
    factor: 0.1,
};
pub(crate) const SHADOW_LONG: CandleSetting = CandleSetting {
    range_type: RangeType::RealBody,
    avg_period: 0,
    factor: 1.0,
};
pub(crate) const SHADOW_VERY_LONG: CandleSetting = CandleSetting {
    range_type: RangeType::RealBody,
    avg_period: 0,
    factor: 2.0,
};
pub(crate) const SHADOW_SHORT: CandleSetting = CandleSetting {
    range_type: RangeType::Shadows,
    avg_period: 10,
    factor: 1.0,
};
pub(crate) const SHADOW_VERY_SHORT: CandleSetting = CandleSetting {
    range_type: RangeType::HighLow,
    avg_period: 10,
    factor: 0.1,
};
pub(crate) const NEAR: CandleSetting = CandleSetting {
    range_type: RangeType::HighLow,
    avg_period: 5,
    factor: 0.2,
};
pub(crate) const FAR: CandleSetting = CandleSetting {
    range_type: RangeType::HighLow,
    avg_period: 5,
    factor: 0.6,
};
pub(crate) const EQUAL: CandleSetting = CandleSetting {
    range_type: RangeType::HighLow,
    avg_period: 5,
    factor: 0.05,
};

// ========== Helper Functions ==========

#[inline(always)]
pub(crate) fn real_body(open: f64, close: f64) -> f64 {
    (close - open).abs()
}

#[inline(always)]
pub(crate) fn upper_shadow(open: f64, high: f64, close: f64) -> f64 {
    high - open.max(close)
}

#[inline(always)]
pub(crate) fn lower_shadow(open: f64, low: f64, close: f64) -> f64 {
    open.min(close) - low
}

#[inline(always)]
pub(crate) fn candle_color(open: f64, close: f64) -> i32 {
    if close >= open {
        1
    } else {
        -1
    }
}

// ========== Monomorphized range/average functions ==========
// Each RangeType gets its own function to eliminate runtime match dispatch.
// These inline to 2-3 instructions — equivalent to C TA-Lib macros.

#[inline(always)]
pub(crate) fn range_realbody(_o: f64, _h: f64, _l: f64, c: f64, o: f64) -> f64 {
    (c - o).abs()
}
#[inline(always)]
pub(crate) fn range_highlow(_o: f64, h: f64, l: f64, _c: f64, _o2: f64) -> f64 {
    h - l
}
#[inline(always)]
pub(crate) fn range_shadows(_o: f64, h: f64, l: f64, c: f64, o: f64) -> f64 {
    (h - l) - (c - o).abs()
}

/// Compute the range value for a single bar based on the setting's range_type
#[inline(always)]
pub(crate) fn candle_range(
    setting: CandleSetting,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
) -> f64 {
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
pub(crate) fn candle_average(
    setting: CandleSetting,
    sum: f64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
) -> f64 {
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
pub(crate) fn cr_realbody(o: &[f64], _h: &[f64], _l: &[f64], c: &[f64], i: usize) -> f64 {
    (c[i] - o[i]).abs()
}
#[inline(always)]
pub(crate) fn ca_realbody(
    setting: CandleSetting,
    sum: f64,
    o: &[f64],
    _h: &[f64],
    _l: &[f64],
    c: &[f64],
    i: usize,
) -> f64 {
    if setting.avg_period > 0 {
        setting.factor * (sum / setting.avg_period as f64)
    } else {
        setting.factor * (c[i] - o[i]).abs()
    }
}

// HighLow types: BODY_DOJI, SHADOW_VERY_SHORT, NEAR, FAR, EQUAL
#[inline(always)]
pub(crate) fn cr_highlow(_o: &[f64], h: &[f64], l: &[f64], _c: &[f64], i: usize) -> f64 {
    h[i] - l[i]
}
#[inline(always)]
pub(crate) fn ca_highlow(
    setting: CandleSetting,
    sum: f64,
    _o: &[f64],
    h: &[f64],
    l: &[f64],
    _c: &[f64],
    i: usize,
) -> f64 {
    if setting.avg_period > 0 {
        setting.factor * (sum / setting.avg_period as f64)
    } else {
        setting.factor * (h[i] - l[i])
    }
}

// Shadows type: SHADOW_SHORT
#[inline(always)]
pub(crate) fn cr_shadows(o: &[f64], h: &[f64], l: &[f64], c: &[f64], i: usize) -> f64 {
    (h[i] - l[i]) - (c[i] - o[i]).abs()
}
#[inline(always)]
pub(crate) fn ca_shadows(
    setting: CandleSetting,
    sum: f64,
    o: &[f64],
    h: &[f64],
    l: &[f64],
    c: &[f64],
    i: usize,
) -> f64 {
    if setting.avg_period > 0 {
        setting.factor * (sum / setting.avg_period as f64) / 2.0
    } else {
        setting.factor * ((h[i] - l[i]) - (c[i] - o[i]).abs()) / 2.0
    }
}

/// Generic cr/ca — still available for rare/complex patterns
#[inline(always)]
pub(crate) fn cr(
    setting: CandleSetting,
    o: &[f64],
    h: &[f64],
    l: &[f64],
    c: &[f64],
    i: usize,
) -> f64 {
    candle_range(setting, o[i], h[i], l[i], c[i])
}

#[inline(always)]
pub(crate) fn ca(
    setting: CandleSetting,
    sum: f64,
    o: &[f64],
    h: &[f64],
    l: &[f64],
    c: &[f64],
    i: usize,
) -> f64 {
    candle_average(setting, sum, o[i], h[i], l[i], c[i])
}

/// Helper: real body gap up (min(o,c) of bar2 > max(o,c) of bar1)
#[inline]
pub(crate) fn real_body_gap_up(o: &[f64], c: &[f64], bar2: usize, bar1: usize) -> bool {
    o[bar2].min(c[bar2]) > o[bar1].max(c[bar1])
}

/// Helper: real body gap down (max(o,c) of bar2 < min(o,c) of bar1)
#[inline]
pub(crate) fn real_body_gap_down(o: &[f64], c: &[f64], bar2: usize, bar1: usize) -> bool {
    o[bar2].max(c[bar2]) < o[bar1].min(c[bar1])
}

/// Validate OHLC arrays have same length
pub(crate) fn validate_ohlc(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<usize> {
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

// ========== Two-candle patterns ==========

// ========== Three+ candle patterns ==========
