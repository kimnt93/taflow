use crate::error::TaResult;
use crate::stream::operator_states::*;
use crate::stream::*;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `IchimokuValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct IchimokuValue {
    pub tenkan_sen: f64,
    pub kijun_sen: f64,
    pub span_a: f64,
    pub span_b: f64,
    pub chikou_span: f64,
}

/// Stateful Ichimoku Kinkō Hyō (pandas-ta classic `overlap/ichimoku.py`).
///
/// Tenkan/Kijun are rolling `(max high + min low)/2` over their windows;
/// `span_a = 0.5·(tenkan + kijun)`; `span_b` is the same midpoint over the
/// Senkou window. All components are emitted **causally** at bar `i`: the
/// package displaces `span_a`/`span_b` forward `kijun` bars and chikou
/// backward `kijun` bars for plotting — that shift is presentation, so
/// taflow keeps the raw values and documents the displacement constants
/// instead (re-align in tests by `span.shift(kijun)`, `chikou.shift(-kijun)`).
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Ichimoku`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Ichimoku {
    /// One shared max/min staircase per side serving all three windows (M1),
    /// instead of three `RollingMidprice` states with six deques between them.
    highs: MultiPeriodStaircase,
    lows: MultiPeriodStaircase,
    tenkan_period: usize,
    kijun_period: usize,
    senkou_period: usize,
    value: Option<IchimokuValue>,
}

impl Ichimoku {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(tenkan: usize, kijun: usize, senkou: usize) -> TaResult<Self> {
        validate_period(tenkan)?;
        validate_period(kijun)?;
        validate_period(senkou)?;
        let longest = tenkan.max(kijun).max(senkou);
        Ok(Self {
            highs: MultiPeriodStaircase::new(longest, true),
            lows: MultiPeriodStaircase::new(longest, false),
            tenkan_period: tenkan,
            kijun_period: kijun,
            senkou_period: senkou,
            value: None,
        })
    }

    /// Midpoint of the rolling high max and low min over `period` bars.
    #[inline]
    fn midprice(&self, period: usize) -> f64 {
        match (self.highs.extremum(period), self.lows.extremum(period)) {
            (Some(high), Some(low)) => (high + low) * 0.5,
            _ => f64::NAN,
        }
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    ///
    /// The three extrema pairs share two scans: each bar is pushed once per
    /// side and each window's midprice is read off the shared staircase. The
    /// extrema themselves are comparison-only, so tenkan/kijun/span_b are the
    /// same numbers the three separate `RollingMidprice` states produced.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> IchimokuValue {
        self.highs.push(high);
        self.lows.push(low);
        let tenkan = self.midprice(self.tenkan_period);
        let kijun = self.midprice(self.kijun_period);
        let span_b = self.midprice(self.senkou_period);
        let span_a = if tenkan.is_nan() || kijun.is_nan() {
            f64::NAN
        } else {
            0.5 * (tenkan + kijun)
        };
        let value = IchimokuValue {
            tenkan_sen: tenkan,
            kijun_sen: kijun,
            span_a,
            span_b,
            chikou_span: close,
        };
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<IchimokuValue> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.highs.reset();
        self.lows.reset();
        self.value = None;
    }
}
