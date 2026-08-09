//! Batch implementation for `squeeze_pro`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `squeeze_pro` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn squeeze_pro(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    bb_length: usize,
    bb_std: f64,
    kc_length: usize,
    kc_scalar_wide: f64,
    kc_scalar_normal: f64,
    kc_scalar_narrow: f64,
    mom_length: usize,
    mom_smooth: usize,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)> {
    if high.len() != low.len() || high.len() != close.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().min(close.len()),
        });
    }
    let mut state = SqueezePro::new(
        bb_length,
        bb_std,
        kc_length,
        kc_scalar_wide,
        kc_scalar_normal,
        kc_scalar_narrow,
        mom_length,
        mom_smooth,
    )?;
    let mut squeeze = Vec::with_capacity(high.len());
    let mut on_wide = Vec::with_capacity(high.len());
    let mut on_normal = Vec::with_capacity(high.len());
    let mut on_narrow = Vec::with_capacity(high.len());
    let mut off = Vec::with_capacity(high.len());
    let mut no = Vec::with_capacity(high.len());
    for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
        let value = state.append(high, low, close);
        squeeze.push(value.squeeze);
        on_wide.push(value.on_wide);
        on_normal.push(value.on_normal);
        on_narrow.push(value.on_narrow);
        off.push(value.off);
        no.push(value.no);
    }
    Ok((squeeze, on_wide, on_normal, on_narrow, off, no))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::squeeze::tests::{lcg_series, ReferenceTrBand};
    use crate::stream::{
        RollingStandardDeviation, SimpleMovingAverage, StreamingIndicator, TrueRange, Window,
    };

    /// Pre-optimization `SqueezePro::append`: separate Keltner basis SMA.
    struct Reference {
        bb_std: f64,
        wide: f64,
        normal: f64,
        narrow: f64,
        bb_mid: SimpleMovingAverage,
        bb_dev: RollingStandardDeviation,
        kc_basis: SimpleMovingAverage,
        tr_band: ReferenceTrBand,
        trange: TrueRange,
        close_window: Window,
        mom_smooth_sma: SimpleMovingAverage,
    }

    impl Reference {
        #[allow(clippy::too_many_arguments)]
        fn new(
            bb_length: usize,
            bb_std: f64,
            kc_length: usize,
            wide: f64,
            normal: f64,
            narrow: f64,
            mom_length: usize,
            mom_smooth: usize,
        ) -> Self {
            Self {
                bb_std,
                wide,
                normal,
                narrow,
                bb_mid: SimpleMovingAverage::new(bb_length).unwrap(),
                bb_dev: RollingStandardDeviation::new(bb_length, 1.0).unwrap(),
                kc_basis: SimpleMovingAverage::new(kc_length).unwrap(),
                tr_band: ReferenceTrBand::new(kc_length),
                trange: TrueRange::default(),
                close_window: Window::new(mom_length).unwrap(),
                mom_smooth_sma: SimpleMovingAverage::new(mom_smooth).unwrap(),
            }
        }

        fn append(&mut self, high: f64, low: f64, close: f64) -> [f64; 6] {
            let (bb_lower, bb_upper) = match (self.bb_mid.append(close), self.bb_dev.append(close))
            {
                (Some(mid), Some(std)) => (mid - self.bb_std * std, mid + self.bb_std * std),
                _ => (f64::NAN, f64::NAN),
            };
            let kc_basis = self.kc_basis.append(close);
            let tr = self.trange.append(high, low, close).unwrap_or(f64::NAN);
            let kc_band = self.tr_band.append(tr);
            let bands = match (kc_basis, kc_band) {
                (Some(basis), Some(band)) => [
                    basis - self.wide * band,
                    basis + self.wide * band,
                    basis - self.normal * band,
                    basis + self.normal * band,
                    basis - self.narrow * band,
                    basis + self.narrow * band,
                ],
                _ => [f64::NAN; 6],
            };
            let mom = self.close_window.push(close).map(|old| close - old);
            let squeeze = mom
                .and_then(|mom| self.mom_smooth_sma.append(mom))
                .unwrap_or(f64::NAN);
            let on_wide = (bb_lower > bands[0] && bb_upper < bands[1]) as u8 as f64;
            let on_normal = (bb_lower > bands[2] && bb_upper < bands[3]) as u8 as f64;
            let on_narrow = (bb_lower > bands[4] && bb_upper < bands[5]) as u8 as f64;
            let off = (bb_lower < bands[0] && bb_upper > bands[1]) as u8 as f64;
            let no = if on_wide == 0.0 && off == 0.0 {
                1.0
            } else {
                0.0
            };
            [squeeze, on_wide, on_normal, on_narrow, off, no]
        }
    }

    #[test]
    fn matches_reference_bitwise_and_survives_chunking() {
        let close = lcg_series(5_000, 0x41_5EED_A2);
        let high: Vec<f64> = close.iter().map(|v| v + 0.6).collect();
        let low: Vec<f64> = close.iter().map(|v| v - 0.6).collect();
        for (bb_length, kc_length) in [(20usize, 20usize), (20, 30), (5, 5)] {
            let mut reference = Reference::new(bb_length, 2.0, kc_length, 2.0, 1.5, 1.0, 12, 6);
            let mut state =
                SqueezePro::new(bb_length, 2.0, kc_length, 2.0, 1.5, 1.0, 12, 6).unwrap();
            for i in 0..close.len() {
                let want = reference.append(high[i], low[i], close[i]);
                let value = state.append(high[i], low[i], close[i]);
                let got = [
                    value.squeeze,
                    value.on_wide,
                    value.on_normal,
                    value.on_narrow,
                    value.off,
                    value.no,
                ];
                for (k, (w, g)) in want.iter().zip(&got).enumerate() {
                    assert_eq!(
                        w.to_bits(),
                        g.to_bits(),
                        "bb={bb_length} kc={kc_length} bar {i} output {k}"
                    );
                }
            }
            state.reset();
            let mut fresh = Reference::new(bb_length, 2.0, kc_length, 2.0, 1.5, 1.0, 12, 6);
            for i in 0..512 {
                let want = fresh.append(high[i], low[i], close[i]);
                let value = state.append(high[i], low[i], close[i]);
                let got = [
                    value.squeeze,
                    value.on_wide,
                    value.on_normal,
                    value.on_narrow,
                    value.off,
                    value.no,
                ];
                for (w, g) in want.iter().zip(&got) {
                    assert_eq!(w.to_bits(), g.to_bits(), "post-reset bb={bb_length}");
                }
            }
        }
    }

    #[test]
    fn batch_matches_streaming() {
        let close = lcg_series(1_000, 0x42_5EED_A3);
        let high: Vec<f64> = close.iter().map(|v| v + 0.6).collect();
        let low: Vec<f64> = close.iter().map(|v| v - 0.6).collect();
        let outputs = squeeze_pro(&high, &low, &close, 20, 2.0, 20, 2.0, 1.5, 1.0, 12, 6).unwrap();
        let mut state = SqueezePro::new(20, 2.0, 20, 2.0, 1.5, 1.0, 12, 6).unwrap();
        for i in 0..close.len() {
            let value = state.append(high[i], low[i], close[i]);
            assert_eq!(outputs.0[i].to_bits(), value.squeeze.to_bits());
            assert_eq!(outputs.1[i].to_bits(), value.on_wide.to_bits());
            assert_eq!(outputs.2[i].to_bits(), value.on_normal.to_bits());
            assert_eq!(outputs.3[i].to_bits(), value.on_narrow.to_bits());
            assert_eq!(outputs.4[i].to_bits(), value.off.to_bits());
            assert_eq!(outputs.5[i].to_bits(), value.no.to_bits());
        }
    }
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `SqueezeProValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct SqueezeProValue {
    pub squeeze: f64,
    pub on_wide: f64,
    pub on_normal: f64,
    pub on_narrow: f64,
    pub off: f64,
    pub no: f64,
}

/// Stateful Squeeze PRO (pandas-ta classic `momentum/squeeze_pro.py`): the
/// TTM Squeeze with three Keltner scalar levels (`wide`/`normal`/`narrow`)
/// sharing one SMA basis and one SMA-of-TR band.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `SqueezePro`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct SqueezePro {
    bb_length: usize,
    bb_std: f64,
    kc_length: usize,
    kc_scalar_wide: f64,
    kc_scalar_normal: f64,
    kc_scalar_narrow: f64,
    mom_length: usize,
    mom_smooth: usize,
    bb_mid: SimpleMovingAverage,
    bb_dev: RollingStandardDeviation,
    /// See [`Squeeze::kc_basis`]: `None` reuses the Bollinger midline when
    /// `kc_length == bb_length` (M4).
    kc_basis: Option<SimpleMovingAverage>,
    tr_band: SqueezeTrBand,
    trange: TrueRange,
    close_window: Window,
    mom_smooth_sma: SimpleMovingAverage,
    value: Option<SqueezeProValue>,
}

impl SqueezePro {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(
        bb_length: usize,
        bb_std: f64,
        kc_length: usize,
        kc_scalar_wide: f64,
        kc_scalar_normal: f64,
        kc_scalar_narrow: f64,
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
        if !(kc_scalar_wide > 0.0 && kc_scalar_normal > 0.0 && kc_scalar_narrow > 0.0) {
            return Err(TaError::InvalidParameter {
                name: "kc_scalar",
                value: format!("{kc_scalar_wide}/{kc_scalar_normal}/{kc_scalar_narrow}"),
                reason: "must all be > 0",
            });
        }
        if !(kc_scalar_wide > kc_scalar_normal && kc_scalar_normal > kc_scalar_narrow) {
            return Err(TaError::InvalidParameter {
                name: "kc_scalar",
                value: format!("{kc_scalar_wide}/{kc_scalar_normal}/{kc_scalar_narrow}"),
                reason: "must satisfy wide > normal > narrow",
            });
        }
        Ok(Self {
            bb_length,
            bb_std,
            kc_length,
            kc_scalar_wide,
            kc_scalar_normal,
            kc_scalar_narrow,
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
    /// M4: the three Keltner levels already share one basis and one TR band;
    /// with `bb_length == kc_length` that basis is also the Bollinger midline,
    /// so the duplicate SMA of close is dropped (identical recurrence, so
    /// identical bits).
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> SqueezeProValue {
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
        let (
            kc_wide_lower,
            kc_wide_upper,
            kc_norm_lower,
            kc_norm_upper,
            kc_narr_lower,
            kc_narr_upper,
        ) = match (kc_basis, kc_band) {
            (Some(basis), Some(band)) => (
                basis - self.kc_scalar_wide * band,
                basis + self.kc_scalar_wide * band,
                basis - self.kc_scalar_normal * band,
                basis + self.kc_scalar_normal * band,
                basis - self.kc_scalar_narrow * band,
                basis + self.kc_scalar_narrow * band,
            ),
            _ => (f64::NAN, f64::NAN, f64::NAN, f64::NAN, f64::NAN, f64::NAN),
        };

        let mom = self.close_window.push(close).map(|old| close - old);
        let squeeze = mom
            .and_then(|mom| self.mom_smooth_sma.append(mom))
            .unwrap_or(f64::NAN);

        let on_wide = (bb_lower > kc_wide_lower && bb_upper < kc_wide_upper) as u8 as f64;
        let on_normal = (bb_lower > kc_norm_lower && bb_upper < kc_norm_upper) as u8 as f64;
        let on_narrow = (bb_lower > kc_narr_lower && bb_upper < kc_narr_upper) as u8 as f64;
        let off = (bb_lower < kc_wide_lower && bb_upper > kc_wide_upper) as u8 as f64;
        let no = if on_wide == 0.0 && off == 0.0 {
            1.0
        } else {
            0.0
        };

        let value = SqueezeProValue {
            squeeze,
            on_wide,
            on_normal,
            on_narrow,
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
    pub fn value(&self) -> Option<SqueezeProValue> {
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
