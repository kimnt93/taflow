/// Fills the first `lookback` output elements with NaN, matching TA-Lib.
#[inline]
pub fn fill_nan_prefix(output: &mut [f64], lookback: usize) {
    for v in output.iter_mut().take(lookback) {
        *v = f64::NAN;
    }
}

/// Validates that an input array is long enough for the requested lookback.
#[inline]
pub fn validate_length(len: usize, lookback: usize) -> bool {
    len > lookback
}

/// Borrowed OHLCV inputs. All fields are slices, so constructing this view is
/// zero-copy.
#[derive(Debug, Clone, Copy)]
pub struct OhlcvInputs<'a> {
    pub open: Option<&'a [f64]>,
    pub high: Option<&'a [f64]>,
    pub low: Option<&'a [f64]>,
    pub close: Option<&'a [f64]>,
    pub volume: Option<&'a [f64]>,
}

impl<'a> OhlcvInputs<'a> {
    /// Creates an input view containing only close prices.
    pub fn close_only(close: &'a [f64]) -> Self {
        Self {
            open: None,
            high: None,
            low: None,
            close: Some(close),
            volume: None,
        }
    }

    /// Creates an input view containing high, low, and close prices.
    pub fn hlc(high: &'a [f64], low: &'a [f64], close: &'a [f64]) -> Self {
        Self {
            open: None,
            high: Some(high),
            low: Some(low),
            close: Some(close),
            volume: None,
        }
    }

    /// Creates an input view containing the complete OHLCV set.
    pub fn full(
        open: &'a [f64],
        high: &'a [f64],
        low: &'a [f64],
        close: &'a [f64],
        volume: &'a [f64],
    ) -> Self {
        Self {
            open: Some(open),
            high: Some(high),
            low: Some(low),
            close: Some(close),
            volume: Some(volume),
        }
    }
}
