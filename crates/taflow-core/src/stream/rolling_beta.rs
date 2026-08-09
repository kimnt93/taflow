//! Batch implementation for `rolling_beta`.

use super::rolling_statistics::{beta_return, ta_is_zero};
use crate::error::{TaError, TaResult};

/// RollingBeta — O(n) sliding-window algorithm.
///
/// C TA-Lib BETA uses percentage returns:
///   x[i] = (input0[i] - input0[i-1]) / input0[i-1]  (input0 return)
///   y[i] = (input1[i] - input1[i-1]) / input1[i-1]  (input1 return)
///   beta = (n*sxy - sx*sy) / (n*sxx - sx*sx)
///
/// `input0` is the market benchmark and `input1` is the stock;
/// the denominator is `rolling_var(x)`, the variance of benchmark returns.
/// Compute the rolling beta result for the supplied aligned series.
///
/// # Parameters
///
/// * `input0` - Input series or configuration value.
/// * `input1` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_beta(input0: &[f64], input1: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let len = input0.len();
    if len != input1.len() {
        return Err(TaError::LengthMismatch {
            expected: len,
            got: input1.len(),
        });
    }
    if timeperiod < 2 {
        return Err(TaError::InvalidParameter {
            name: "timeperiod",
            value: timeperiod.to_string(),
            reason: "must be >= 2",
        });
    }
    if len <= timeperiod {
        return Err(TaError::InsufficientData {
            need: timeperiod + 1,
            got: len,
        });
    }
    let mut output = vec![0.0_f64; len];
    output[..timeperiod].fill(f64::NAN);
    let n = timeperiod as f64;

    // Returns are recomputed straight from the price slices — exactly what
    // the C loop does through its `lastPrice`/`trailingLastPrice` cursors —
    // so no `len`-sized return series is materialized. `rx[j]` below denotes
    // the return between bars `j` and `j + 1`.
    let mut sx = 0.0_f64;
    let mut sy = 0.0_f64;
    let mut sxx = 0.0_f64;
    let mut sxy = 0.0_f64;

    // TA_BETA seeds `timeperiod - 1` returns before the emit loop, then per
    // bar adds the incoming return, emits, and only then removes the trailing
    // one. That order — the mirror of TA_CORREL's — is load-bearing: it makes
    // BETA bitwise equal to TA-Lib. See `RollingReturnMoments`. No reseed.
    for j in 0..(timeperiod - 1) {
        let x = beta_return(input0[j + 1], input0[j]);
        let y = beta_return(input1[j + 1], input1[j]);
        sxx += x * x;
        sxy += x * y;
        sx += x;
        sy += y;
    }

    // Branch-free steady loop over zipped slices; output slot `k` is bar
    // `k + timeperiod`, whose incoming return spans bars `k + timeperiod - 1`
    // and `k + timeperiod`, and whose trailing return spans `k` and `k + 1`.
    let steady = len - timeperiod;
    let out = &mut output[timeperiod..];
    let new0 = &input0[timeperiod..];
    let new1 = &input1[timeperiod..];
    let previous0 = &input0[timeperiod - 1..len - 1];
    let previous1 = &input1[timeperiod - 1..len - 1];
    let trailing0 = &input0[1..steady + 1];
    let trailing1 = &input1[1..steady + 1];
    let trailing_previous0 = &input0[..steady];
    let trailing_previous1 = &input1[..steady];

    for (k, slot) in out.iter_mut().enumerate() {
        let x = beta_return(new0[k], previous0[k]);
        let y = beta_return(new1[k], previous1[k]);
        sxx += x * x;
        sxy += x * y;
        sx += x;
        sy += y;

        // TA-Lib reads the trailing return before writing the output.
        let trailing_x = beta_return(trailing0[k], trailing_previous0[k]);
        let trailing_y = beta_return(trailing1[k], trailing_previous1[k]);

        let denom = (n * sxx) - (sx * sx);
        *slot = if !ta_is_zero(denom) {
            ((n * sxy) - (sx * sy)) / denom
        } else {
            0.0
        };

        // Remove the trailing return, after the emit.
        sxx -= trailing_x * trailing_x;
        sxy -= trailing_x * trailing_y;
        sx -= trailing_x;
        sy -= trailing_y;
    }
    Ok(output)
}
use super::rolling_statistics::*;
use super::*;

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
