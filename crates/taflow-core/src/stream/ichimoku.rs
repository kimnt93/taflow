//! Batch implementation for `ichimoku`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `ichimoku` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn ichimoku(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    tenkan: usize,
    kijun: usize,
    senkou: usize,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)> {
    if high.len() != low.len() || high.len() != close.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().min(close.len()),
        });
    }
    let mut state = Ichimoku::new(tenkan, kijun, senkou)?;
    let mut tenkan_sen = Vec::with_capacity(high.len());
    let mut kijun_sen = Vec::with_capacity(high.len());
    let mut span_a = Vec::with_capacity(high.len());
    let mut span_b = Vec::with_capacity(high.len());
    let mut chikou_span = Vec::with_capacity(high.len());
    for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
        let value = state.append(high, low, close);
        tenkan_sen.push(value.tenkan_sen);
        kijun_sen.push(value.kijun_sen);
        span_a.push(value.span_a);
        span_b.push(value.span_b);
        chikou_span.push(value.chikou_span);
    }
    Ok((tenkan_sen, kijun_sen, span_a, span_b, chikou_span))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// Pre-optimization midprice state (rolling max/min rescan) oracle.
    struct Midprice {
        highs: VecDeque<f64>,
        lows: VecDeque<f64>,
        period: usize,
    }

    impl Midprice {
        fn new(period: usize) -> Self {
            Self {
                highs: VecDeque::with_capacity(period),
                lows: VecDeque::with_capacity(period),
                period,
            }
        }

        fn append(&mut self, high: f64, low: f64) -> Option<f64> {
            if self.highs.len() == self.period {
                self.highs.pop_front();
                self.lows.pop_front();
            }
            self.highs.push_back(high);
            self.lows.push_back(low);
            (self.highs.len() == self.period).then(|| {
                let maximum = self.highs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                let minimum = self.lows.iter().copied().fold(f64::INFINITY, f64::min);
                (maximum + minimum) * 0.5
            })
        }
    }

    /// Pre-optimization `Ichimoku::append` oracle (three separate midprices).
    struct Reference {
        tenkan: Midprice,
        kijun: Midprice,
        senkou: Midprice,
    }

    impl Reference {
        fn new(tenkan: usize, kijun: usize, senkou: usize) -> Self {
            Self {
                tenkan: Midprice::new(tenkan),
                kijun: Midprice::new(kijun),
                senkou: Midprice::new(senkou),
            }
        }

        fn append(&mut self, high: f64, low: f64, close: f64) -> [f64; 5] {
            let tenkan = self.tenkan.append(high, low).unwrap_or(f64::NAN);
            let kijun = self.kijun.append(high, low).unwrap_or(f64::NAN);
            let span_b = self.senkou.append(high, low).unwrap_or(f64::NAN);
            let span_a = if tenkan.is_nan() || kijun.is_nan() {
                f64::NAN
            } else {
                0.5 * (tenkan + kijun)
            };
            [tenkan, kijun, span_a, span_b, close]
        }
    }

    fn lcg_series(len: usize, seed: u64) -> Vec<f64> {
        let mut state = seed;
        (0..len)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                90.0 + ((state >> 11) as f64 / (1u64 << 53) as f64) * 20.0
            })
            .collect()
    }

    fn assert_same(want: [f64; 5], got: IchimokuValue, label: &str) {
        let got = [
            got.tenkan_sen,
            got.kijun_sen,
            got.span_a,
            got.span_b,
            got.chikou_span,
        ];
        for (i, (w, g)) in want.iter().zip(&got).enumerate() {
            assert_eq!(w.to_bits(), g.to_bits(), "{label} output {i}");
        }
    }

    #[test]
    fn matches_reference_bitwise_and_survives_chunking() {
        let close = lcg_series(5_000, 0xE1_5EED_B1);
        let high: Vec<f64> = close.iter().map(|v| v + 0.9).collect();
        let low: Vec<f64> = close.iter().map(|v| v - 0.9).collect();
        // Nested, equal and inverted period orderings.
        for (tenkan, kijun, senkou) in [
            (9usize, 26usize, 52usize),
            (1, 1, 1),
            (2, 3, 5),
            (30, 5, 60),
            (52, 26, 9),
        ] {
            let mut reference = Reference::new(tenkan, kijun, senkou);
            let mut state = Ichimoku::new(tenkan, kijun, senkou).unwrap();
            for i in 0..close.len() {
                let want = reference.append(high[i], low[i], close[i]);
                let got = state.append(high[i], low[i], close[i]);
                assert_same(want, got, &format!("{tenkan}/{kijun}/{senkou} bar {i}"));
            }
            state.reset();
            let mut fresh = Reference::new(tenkan, kijun, senkou);
            for i in 0..512 {
                let want = fresh.append(high[i], low[i], close[i]);
                let got = state.append(high[i], low[i], close[i]);
                assert_same(want, got, "post-reset");
            }
        }
    }

    #[test]
    fn batch_matches_streaming() {
        let close = lcg_series(1_000, 0xE2_5EED_B2);
        let high: Vec<f64> = close.iter().map(|v| v + 0.5).collect();
        let low: Vec<f64> = close.iter().map(|v| v - 0.5).collect();
        let (tenkan, kijun, span_a, span_b, chikou) =
            ichimoku(&high, &low, &close, 9, 26, 52).unwrap();
        let mut state = Ichimoku::new(9, 26, 52).unwrap();
        for i in 0..close.len() {
            let got = state.append(high[i], low[i], close[i]);
            assert_eq!(tenkan[i].to_bits(), got.tenkan_sen.to_bits());
            assert_eq!(kijun[i].to_bits(), got.kijun_sen.to_bits());
            assert_eq!(span_a[i].to_bits(), got.span_a.to_bits());
            assert_eq!(span_b[i].to_bits(), got.span_b.to_bits());
            assert_eq!(chikou[i].to_bits(), got.chikou_span.to_bits());
        }
    }
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

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
