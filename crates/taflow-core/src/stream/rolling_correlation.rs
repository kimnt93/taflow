//! Stateful implementation for `rolling_correlation`.

use super::rolling_statistics::CORREL_DENOMINATOR_EPSILON;
use super::rolling_statistics::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Stateful Pearson correlation over paired observations.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingCorrelation`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingCorrelation {
    period: f64,
    moments: RollingPairMoments,
    value: Option<f64>,
}

impl RollingCorrelation {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            period: period as f64,
            moments: RollingPairMoments::new(period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, x: f64, y: f64) -> Option<f64> {
        let period = self.period;
        self.value = self
            .moments
            .append(x, y)
            .map(|m| correl_of(m.sx, m.sy, m.sxx, m.syy, m.sxy, period));
        self.value
    }

    /// Bulk kernel: `TA_CORREL`'s remove-trailing / add-new / emit recurrence
    /// indexing the input slices directly. Bit-identical to per-bar
    /// [`Self::append`] in outputs and post-run state.
    pub fn extend_slices_into(
        &mut self,
        input0: &[f64],
        input1: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        if input0.len() != input1.len() {
            return Err(crate::TaError::LengthMismatch {
                expected: input0.len(),
                got: input1.len(),
            });
        }
        let period = self.moments.period;
        let n = input0.len();
        output.reserve(n);
        // Warm-up prologue: after `period` appends the pair ring holds exactly
        // the first `period` slice pairs, regardless of prior state.
        let prologue = n.min(period);
        for i in 0..prologue {
            output.push(self.append(input0[i], input1[i]).unwrap_or(f64::NAN));
        }
        if n <= period {
            return Ok(());
        }
        // Branch-free steady loop: identical arithmetic in identical order to
        // `RollingPairMoments::append`, but every access is a zipped slice
        // iterator, so there is no ring traffic and no bounds check. The
        // `TrustedLen` `extend` writes each result straight into the output's
        // spare capacity — a `resize` prologue would cost a second full pass
        // over the buffer just to pre-fill it.
        let period_f = self.period;
        let mut m = self.moments.moments;
        let steady = n - period;
        output.extend(
            input0[period..]
                .iter()
                .zip(&input1[period..])
                .zip(&input0[..steady])
                .zip(&input1[..steady])
                .map(|(((&x, &y), &tx), &ty)| {
                    // "Remove trailing values", "add new values", then emit.
                    m.sx -= tx;
                    m.sxx -= tx * tx;
                    m.sxy -= tx * ty;
                    m.sy -= ty;
                    m.syy -= ty * ty;
                    m.sx += x;
                    m.sxx += x * x;
                    m.sxy += x * y;
                    m.sy += y;
                    m.syy += y * y;
                    correl_of(m.sx, m.sy, m.sxx, m.syy, m.sxy, period_f)
                }),
        );
        self.moments.moments = m;
        self.value = output.last().copied();
        // Rebuild the pair ring so subsequent appends continue bit-identically.
        self.moments.window.clear();
        for i in n - period..n {
            self.moments.window.push(input0[i], input1[i]);
        }
        Ok(())
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.moments.reset();
        self.value = None;
    }
}
