//! Batch implementation for `rolling_beta`.

use super::rolling_statistics::*;
use super::rolling_statistics::{beta_return, ta_is_zero};
use super::*;
use crate::error::{TaError, TaResult};

/// Stateful TA-Lib BETA over percentage returns of two input series.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingBeta`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingBeta {
    period: f64,
    previous: Option<(f64, f64)>,
    returns: RollingReturnMoments,
    value: Option<f64>,
}

impl RollingBeta {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            period: period as f64,
            previous: None,
            returns: RollingReturnMoments::new(period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, input0: f64, input1: f64) -> Option<f64> {
        let Some((previous0, previous1)) = self.previous.replace((input0, input1)) else {
            return None;
        };
        let x = beta_return(input0, previous0);
        let y = beta_return(input1, previous1);
        let period = self.period;
        self.value = self
            .returns
            .append(x, y)
            .map(|m| beta_of(m.sx, m.sy, m.sxx, m.sxy, period));
        self.value
    }

    /// Bulk kernel: `TA_BETA`'s add-new / emit / remove-trailing recurrence
    /// over returns recomputed straight from the input slices — exactly what
    /// the C loop does through its `trailingLastPrice` cursors, so no return
    /// series is materialized and no ring is touched inside the loop.
    /// Bit-identical to per-bar [`Self::append`] in outputs and post-run state.
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
        let period = self.returns.period;
        let n = input0.len();
        output.reserve(n);
        // Warm-up prologue: after `period` appends the retained ring holds
        // exactly the `period - 1` returns of `inputs[..period]`, and
        // `previous` is `inputs[period - 1]` — regardless of prior state.
        let prologue = n.min(period);
        for i in 0..prologue {
            output.push(self.append(input0[i], input1[i]).unwrap_or(f64::NAN));
        }
        if n <= period {
            return Ok(());
        }
        // Branch-free steady loop over eight zipped slices: per series, the
        // incoming bar and its predecessor (the new return) and the trailing
        // bar and its predecessor (the return leaving the window). Output slot
        // `k` is bar `i = k + period`. The `TrustedLen` `extend` writes into
        // the output's spare capacity without a pre-fill pass.
        let period_f = self.period;
        let mut m = self.returns.moments;
        let steady = n - period;
        let series0 = input0[period..]
            .iter()
            .zip(&input0[period - 1..n - 1])
            .zip(&input0[1..steady + 1])
            .zip(&input0[..steady]);
        let series1 = input1[period..]
            .iter()
            .zip(&input1[period - 1..n - 1])
            .zip(&input1[1..steady + 1])
            .zip(&input1[..steady]);
        output.extend(series0.zip(series1).map(
            |(
                (((&new0, &previous0), &trailing0), &trailing_previous0),
                (((&new1, &previous1), &trailing1), &trailing_previous1),
            )| {
                let x = beta_return(new0, previous0);
                let y = beta_return(new1, previous1);
                m.sxx += x * x;
                m.sxy += x * y;
                m.sx += x;
                m.sy += y;

                // TA-Lib reads the trailing return before writing the output.
                let tx = beta_return(trailing0, trailing_previous0);
                let ty = beta_return(trailing1, trailing_previous1);

                let out = beta_of(m.sx, m.sy, m.sxx, m.sxy, period_f);

                m.sxx -= tx * tx;
                m.sxy -= tx * ty;
                m.sx -= tx;
                m.sy -= ty;
                out
            },
        ));
        self.returns.moments = m;
        self.value = output.last().copied();
        // Rebuild the retained ring (the `period - 1` most recent returns) and
        // `previous`, so subsequent appends continue bit-identically.
        self.returns.window.clear();
        for i in n - period + 1..n {
            self.returns.window.push(
                beta_return(input0[i], input0[i - 1]),
                beta_return(input1[i], input1[i - 1]),
            );
        }
        self.previous = Some((input0[n - 1], input1[n - 1]));
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
        self.previous = None;
        self.returns.reset();
        self.value = None;
    }
}
