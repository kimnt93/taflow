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
    #[allow(dead_code)]
    pub(crate) range_type: RangeType,
    pub(crate) avg_period: usize,
    pub(crate) factor: f64,
}

/// Trailing bars a candle streaming state replays to rebuild itself after a
/// bulk (batch-kernel) pass over a slice.
///
/// Every candle state's fields are a function of a bounded window of recent
/// bars: the deepest is a 10-bar average (`avg_period`) read at an offset of at
/// most 4 bars back, i.e. 15 bars. Replaying any tail at least that long from a
/// pristine state therefore reproduces the exact state a full per-bar run would
/// have left, so 64 is a comfortable margin at negligible cost.
pub(crate) const BULK_REPLAY_BARS: usize = 64;

// Default candle settings exactly matching C TA-Lib ta_common.c
pub(crate) const BODY_LONG: CandleSetting = CandleSetting {
    range_type: RangeType::RealBody,
    avg_period: 10,
    factor: 1.0,
};
#[allow(dead_code)]
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

// ---- Scalar monomorphized cr/ca (streaming states) ----
// Same arithmetic, operation for operation, as the slice variants above, so a
// streaming state that maintains its sums with these produces bit-identical
// thresholds to the batch loop.

#[inline(always)]
pub(crate) fn cr_realbody_scalar(o: f64, c: f64) -> f64 {
    (c - o).abs()
}

#[inline(always)]
pub(crate) fn cr_highlow_scalar(h: f64, l: f64) -> f64 {
    h - l
}

#[inline(always)]
pub(crate) fn cr_shadows_scalar(o: f64, h: f64, l: f64, c: f64) -> f64 {
    (h - l) - (c - o).abs()
}

#[inline(always)]
pub(crate) fn ca_realbody_scalar(setting: CandleSetting, sum: f64, o: f64, c: f64) -> f64 {
    if setting.avg_period > 0 {
        setting.factor * (sum / setting.avg_period as f64)
    } else {
        setting.factor * (c - o).abs()
    }
}

#[inline(always)]
pub(crate) fn ca_highlow_scalar(setting: CandleSetting, sum: f64, h: f64, l: f64) -> f64 {
    if setting.avg_period > 0 {
        setting.factor * (sum / setting.avg_period as f64)
    } else {
        setting.factor * (h - l)
    }
}

#[inline(always)]
pub(crate) fn ca_shadows_scalar(
    setting: CandleSetting,
    sum: f64,
    o: f64,
    h: f64,
    l: f64,
    c: f64,
) -> f64 {
    if setting.avg_period > 0 {
        setting.factor * (sum / setting.avg_period as f64) / 2.0
    } else {
        setting.factor * ((h - l) - (c - o).abs()) / 2.0
    }
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
