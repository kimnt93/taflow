use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};
use crate::indicators::RollingStandardDeviation;
use crate::indicators::TrueRange;

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
