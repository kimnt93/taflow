use super::operator_states::*;
use super::*;
use crate::error::TaResult;
use crate::indicators::RollingSum;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VortexValue {
    pub vp: f64,
    pub vn: f64,
}

/// Stateful Vortex indicator (bukosabino `ta` `trend.VortexIndicator`, theory:
/// Etienne Botes & Douglas Siepman, TASC Jan 2010). +VI/−VI are the ratio of
/// the rolling `n`-sum of positive/negative directional movement to the
/// rolling `n`-sum of true range.
///
/// The first bar's true range uses `close` as its own previous close (the
/// package fills bar 0 with the global close mean, but that value only feeds
/// outputs whose window is not yet complete, so the streaming choice is
/// output-equivalent); the movement terms are NaN at bar 0, so +VI/−VI are
/// first defined at bar `n`.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Vortex`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Vortex {
    period: usize,
    previous_close: Option<f64>,
    previous_low: Option<f64>,
    previous_high: Option<f64>,
    tr_sum: RollingSum,
    vmp_sum: RollingSum,
    vmm_sum: RollingSum,
    value: Option<VortexValue>,
}

impl Vortex {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            period,
            previous_close: None,
            previous_low: None,
            previous_high: None,
            tr_sum: RollingSum::new(period)?,
            vmp_sum: RollingSum::new(period)?,
            vmm_sum: RollingSum::new(period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> VortexValue {
        let (tr, movement) = match self.previous_close {
            Some(previous_close) => {
                let tr = (high - low)
                    .max((high - previous_close).abs())
                    .max((low - previous_close).abs());
                let vmp = (high - self.previous_low.unwrap()).abs();
                let vmm = (low - self.previous_high.unwrap()).abs();
                (tr, Some((vmp, vmm)))
            }
            None => {
                let tr = (high - low)
                    .max((high - close).abs())
                    .max((low - close).abs());
                (tr, None)
            }
        };
        self.previous_close = Some(close);
        self.previous_low = Some(low);
        self.previous_high = Some(high);

        let trn = self.tr_sum.append(tr);
        let (vmp_sum, vmm_sum) = movement.map_or((None, None), |(vmp, vmm)| {
            (self.vmp_sum.append(vmp), self.vmm_sum.append(vmm))
        });
        let vp = match (vmp_sum, trn) {
            (Some(numerator), Some(denominator)) => numerator / denominator,
            _ => f64::NAN,
        };
        let vn = match (vmm_sum, trn) {
            (Some(numerator), Some(denominator)) => numerator / denominator,
            _ => f64::NAN,
        };
        let value = VortexValue { vp, vn };
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<VortexValue> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.previous_low = None;
        self.previous_high = None;
        self.tr_sum.reset();
        self.vmp_sum.reset();
        self.vmm_sum.reset();
        self.value = None;
    }
}
