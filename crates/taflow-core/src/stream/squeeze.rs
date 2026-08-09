//! Batch implementation for `squeeze`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `squeeze` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn squeeze(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    bb_length: usize,
    bb_std: f64,
    kc_length: usize,
    kc_scalar: f64,
    mom_length: usize,
    mom_smooth: usize,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)> {
    if high.len() != low.len() || high.len() != close.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().min(close.len()),
        });
    }
    let mut state = Squeeze::new(
        bb_length, bb_std, kc_length, kc_scalar, mom_length, mom_smooth,
    )?;
    let mut out = (0..4)
        .map(|_| Vec::with_capacity(high.len()))
        .collect::<Vec<_>>();
    for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
        let value = state.append(high, low, close);
        out[0].push(value.squeeze);
        out[1].push(value.on);
        out[2].push(value.off);
        out[3].push(value.no);
    }
    let mut out = out.into_iter();
    Ok((
        out.next().unwrap(),
        out.next().unwrap(),
        out.next().unwrap(),
        out.next().unwrap(),
    ))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::stream::{
        RollingStandardDeviation, SimpleMovingAverage, StreamingIndicator, TrueRange, Window,
    };

    /// Pre-optimization SMA-of-TR band (pandas-ta NaN-at-bar-0 convention).
    pub(crate) struct ReferenceTrBand {
        period: usize,
        window: Window,
        sum: f64,
        value: Option<f64>,
    }

    impl ReferenceTrBand {
        pub(crate) fn new(period: usize) -> Self {
            Self {
                period,
                window: Window::new(period).unwrap(),
                sum: 0.0,
                value: None,
            }
        }

        pub(crate) fn append(&mut self, tr: f64) -> Option<f64> {
            if !tr.is_nan() {
                if let Some(old) = self.window.push(tr) {
                    self.sum -= old;
                }
                self.sum += tr;
            }
            self.value = self.window.is_full().then(|| self.sum / self.period as f64);
            self.value
        }
    }

    /// Pre-optimization `Squeeze::append`: the Keltner basis is maintained by
    /// its own SMA instead of being read off the Bollinger midline.
    struct Reference {
        bb_std: f64,
        kc_scalar: f64,
        bb_mid: SimpleMovingAverage,
        bb_dev: RollingStandardDeviation,
        kc_basis: SimpleMovingAverage,
        tr_band: ReferenceTrBand,
        trange: TrueRange,
        close_window: Window,
        mom_smooth_sma: SimpleMovingAverage,
    }

    impl Reference {
        fn new(
            bb_length: usize,
            bb_std: f64,
            kc_length: usize,
            kc_scalar: f64,
            mom_length: usize,
            mom_smooth: usize,
        ) -> Self {
            Self {
                bb_std,
                kc_scalar,
                bb_mid: SimpleMovingAverage::new(bb_length).unwrap(),
                bb_dev: RollingStandardDeviation::new(bb_length, 1.0).unwrap(),
                kc_basis: SimpleMovingAverage::new(kc_length).unwrap(),
                tr_band: ReferenceTrBand::new(kc_length),
                trange: TrueRange::default(),
                close_window: Window::new(mom_length).unwrap(),
                mom_smooth_sma: SimpleMovingAverage::new(mom_smooth).unwrap(),
            }
        }

        fn append(&mut self, high: f64, low: f64, close: f64) -> [f64; 4] {
            let (bb_lower, bb_upper) = match (self.bb_mid.append(close), self.bb_dev.append(close))
            {
                (Some(mid), Some(std)) => (mid - self.bb_std * std, mid + self.bb_std * std),
                _ => (f64::NAN, f64::NAN),
            };
            let kc_basis = self.kc_basis.append(close);
            let tr = self.trange.append(high, low, close).unwrap_or(f64::NAN);
            let kc_band = self.tr_band.append(tr);
            let (kc_lower, kc_upper) = match (kc_basis, kc_band) {
                (Some(basis), Some(band)) => {
                    (basis - self.kc_scalar * band, basis + self.kc_scalar * band)
                }
                _ => (f64::NAN, f64::NAN),
            };
            let mom = self.close_window.push(close).map(|old| close - old);
            let squeeze = mom
                .and_then(|mom| self.mom_smooth_sma.append(mom))
                .unwrap_or(f64::NAN);
            let on = (bb_lower > kc_lower && bb_upper < kc_upper) as u8 as f64;
            let off = (bb_lower < kc_lower && bb_upper > kc_upper) as u8 as f64;
            let no = if on == 0.0 && off == 0.0 { 1.0 } else { 0.0 };
            [squeeze, on, off, no]
        }
    }

    pub(crate) fn lcg_series(len: usize, seed: u64) -> Vec<f64> {
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

    #[test]
    fn matches_reference_bitwise_and_survives_chunking() {
        let close = lcg_series(5_000, 0x31_5EED_F1);
        let high: Vec<f64> = close.iter().map(|v| v + 0.6).collect();
        let low: Vec<f64> = close.iter().map(|v| v - 0.6).collect();
        // Shared-basis case (bb_length == kc_length) and distinct-length case.
        for (bb_length, kc_length) in [(20usize, 20usize), (20, 30), (5, 5), (2, 3)] {
            let mut reference = Reference::new(bb_length, 2.0, kc_length, 1.5, 12, 6);
            let mut state = Squeeze::new(bb_length, 2.0, kc_length, 1.5, 12, 6).unwrap();
            for i in 0..close.len() {
                let want = reference.append(high[i], low[i], close[i]);
                let value = state.append(high[i], low[i], close[i]);
                let got = [value.squeeze, value.on, value.off, value.no];
                for (k, (w, g)) in want.iter().zip(&got).enumerate() {
                    assert_eq!(
                        w.to_bits(),
                        g.to_bits(),
                        "bb={bb_length} kc={kc_length} bar {i} output {k}"
                    );
                }
            }
            state.reset();
            let mut fresh = Reference::new(bb_length, 2.0, kc_length, 1.5, 12, 6);
            for i in 0..512 {
                let want = fresh.append(high[i], low[i], close[i]);
                let value = state.append(high[i], low[i], close[i]);
                let got = [value.squeeze, value.on, value.off, value.no];
                for (w, g) in want.iter().zip(&got) {
                    assert_eq!(w.to_bits(), g.to_bits(), "post-reset bb={bb_length}");
                }
            }
        }
    }

    #[test]
    fn batch_matches_streaming() {
        let close = lcg_series(1_000, 0x32_5EED_F2);
        let high: Vec<f64> = close.iter().map(|v| v + 0.6).collect();
        let low: Vec<f64> = close.iter().map(|v| v - 0.6).collect();
        let (squeeze_out, on, off, no) =
            squeeze(&high, &low, &close, 20, 2.0, 20, 1.5, 12, 6).unwrap();
        let mut state = Squeeze::new(20, 2.0, 20, 1.5, 12, 6).unwrap();
        for i in 0..close.len() {
            let value = state.append(high[i], low[i], close[i]);
            assert_eq!(squeeze_out[i].to_bits(), value.squeeze.to_bits());
            assert_eq!(on[i].to_bits(), value.on.to_bits());
            assert_eq!(off[i].to_bits(), value.off.to_bits());
            assert_eq!(no[i].to_bits(), value.no.to_bits());
        }
    }
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `SqueezeValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct SqueezeValue {
    pub squeeze: f64,
    pub on: f64,
    pub off: f64,
    pub no: f64,
}

/// Stateful TTM Squeeze (pandas-ta classic `momentum/squeeze.py`, theory:
/// John Carter). A Bollinger Bands envelope (SMA basis, population std) is
/// compared against a Keltner Channel (SMA of close, SMA of true range) to
/// classify compression states; the momentum line is an SMA of the
/// `close − close[mom_length]` difference.
///
/// All four band components are O(1) incremental states; `on`/`off`/`no` are
/// `0/1` booleans and, like pandas-ta's `&` against NaN, report `no = 1`
/// during warm-up (before both envelopes are defined).
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Squeeze`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Squeeze {
    bb_length: usize,
    bb_std: f64,
    kc_length: usize,
    kc_scalar: f64,
    mom_length: usize,
    mom_smooth: usize,
    bb_mid: SimpleMovingAverage,
    bb_dev: RollingStandardDeviation,
    /// `None` when `kc_length == bb_length`: the Keltner basis is then the
    /// same SMA of close as the Bollinger midline, so it is read from
    /// `bb_mid` rather than maintained a second time (M4).
    kc_basis: Option<SimpleMovingAverage>,
    tr_band: SqueezeTrBand,
    trange: TrueRange,
    close_window: Window,
    mom_smooth_sma: SimpleMovingAverage,
    value: Option<SqueezeValue>,
}

impl Squeeze {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(
        bb_length: usize,
        bb_std: f64,
        kc_length: usize,
        kc_scalar: f64,
        mom_length: usize,
        mom_smooth: usize,
    ) -> TaResult<Self> {
        validate_period(bb_length)?;
        validate_period(kc_length)?;
        validate_period(mom_length)?;
        validate_period(mom_smooth)?;
        if !(bb_std > 0.0) {
            return Err(TaError::InvalidParameter {
                name: "bb_std",
                value: bb_std.to_string(),
                reason: "must be > 0",
            });
        }
        if !(kc_scalar > 0.0) {
            return Err(TaError::InvalidParameter {
                name: "kc_scalar",
                value: kc_scalar.to_string(),
                reason: "must be > 0",
            });
        }
        Ok(Self {
            bb_length,
            bb_std,
            kc_length,
            kc_scalar,
            mom_length,
            mom_smooth,
            bb_mid: SimpleMovingAverage::new(bb_length)?,
            bb_dev: RollingStandardDeviation::new(bb_length, 1.0)?,
            kc_basis: (kc_length != bb_length)
                .then(|| SimpleMovingAverage::new(kc_length))
                .transpose()?,
            tr_band: SqueezeTrBand::new(kc_length)?,
            trange: TrueRange::default(),
            close_window: Window::new(mom_length)?,
            mom_smooth_sma: SimpleMovingAverage::new(mom_smooth)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    ///
    /// M4: with the default `bb_length == kc_length` the Keltner basis is
    /// literally the Bollinger midline — the same SMA of close over the same
    /// window — so only one of the two is maintained. Same inputs, same
    /// period, same recurrence, therefore the same bits.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> SqueezeValue {
        let bb_mid = self.bb_mid.append(close);
        let (bb_lower, bb_upper) = match (bb_mid, self.bb_dev.append(close)) {
            (Some(mid), Some(std)) => (mid - self.bb_std * std, mid + self.bb_std * std),
            _ => (f64::NAN, f64::NAN),
        };

        let kc_basis = match self.kc_basis.as_mut() {
            Some(kc_basis) => kc_basis.append(close),
            None => bb_mid,
        };
        let tr = self.trange.append(high, low, close).unwrap_or(f64::NAN);
        let kc_band = self.tr_band.append(tr);
        let (kc_lower, kc_upper) = match (kc_basis, kc_band) {
            (Some(basis), Some(band)) => {
                (basis - self.kc_scalar * band, basis + self.kc_scalar * band)
            }
            _ => (f64::NAN, f64::NAN),
        };

        let mom = self.close_window.push(close).map(|old| close - old);
        let squeeze = mom
            .and_then(|mom| self.mom_smooth_sma.append(mom))
            .unwrap_or(f64::NAN);

        let on = (bb_lower > kc_lower && bb_upper < kc_upper) as u8 as f64;
        let off = (bb_lower < kc_lower && bb_upper > kc_upper) as u8 as f64;
        let no = if on == 0.0 && off == 0.0 { 1.0 } else { 0.0 };

        let value = SqueezeValue {
            squeeze,
            on,
            off,
            no,
        };
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<SqueezeValue> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.bb_mid.reset();
        self.bb_dev.reset();
        if let Some(kc_basis) = self.kc_basis.as_mut() {
            kc_basis.reset();
        }
        self.tr_band.reset();
        self.trange.reset();
        self.close_window.clear();
        self.mom_smooth_sma.reset();
        self.value = None;
    }
}
