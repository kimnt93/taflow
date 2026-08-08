//! Incremental Triple Exponential Average Rate of Change (TRIX).

use crate::error::TaResult;

use super::{invalid_period, ExponentialMovingAverage, StreamingIndicator};

/// Persistent TRIX with a triple TA-Lib-seeded EMA cascade and O(1) updates.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `TripleExponentialRateOfChange`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct TripleExponentialRateOfChange {
    ema1: ExponentialMovingAverage,
    ema2: ExponentialMovingAverage,
    ema3: ExponentialMovingAverage,
    previous_ema3: Option<f64>,
    value: Option<f64>,
}

impl TripleExponentialRateOfChange {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            ema1: ExponentialMovingAverage::new(period)?,
            ema2: ExponentialMovingAverage::new(period)?,
            ema3: ExponentialMovingAverage::new(period)?,
            previous_ema3: None,
            value: None,
        })
    }
}

impl StreamingIndicator for TripleExponentialRateOfChange {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        let e1 = self.ema1.append(input)?;
        let e2 = self.ema2.append(e1)?;
        let e3 = self.ema3.append(e2)?;
        let previous = self.previous_ema3.replace(e3)?;
        self.value = Some(if previous != 0.0 {
            (e3 - previous) / previous * 100.0
        } else {
            0.0
        });
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.ema1.reset();
        self.ema2.reset();
        self.ema3.reset();
        self.previous_ema3 = None;
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_batch_for_chunked_extend() {
        let input: Vec<f64> = (0..96)
            .map(|i| 100.0 + i as f64 * 0.2 + (i as f64 * 0.3).sin())
            .collect();
        let expected = crate::stream::triple_exponential_rate_of_change(&input, 7).unwrap();
        let mut state = TripleExponentialRateOfChange::new(7).unwrap();
        let mut actual = state.extend(input[..43].iter().copied());
        actual.extend(state.extend(input[43..].iter().copied()));
        for (actual, expected) in actual.iter().zip(&expected) {
            match actual {
                Some(actual) => assert!((actual - expected).abs() < 1e-10),
                None => assert!(expected.is_nan()),
            }
        }
    }
}
use crate::error::TaError;
use crate::simd::sum_f64;

/// TRIX — rate of change of a triple-exponentially smoothed series.
///
/// TRIX = ROC(EMA(EMA(EMA(input))))
/// lookback = 3*(timeperiod-1) + 1
///
/// Optimized version: three scalar EMA stages plus ROC, with one output Vec
/// and no intermediate allocation.
/// Compute the triple exponential rate of change result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn triple_exponential_rate_of_change(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if timeperiod < 2 {
        return Err(TaError::InvalidParameter {
            name: "timeperiod",
            value: timeperiod.to_string(),
            reason: "must be >= 2",
        });
    }
    let len = input.len();
    let lookback = 3 * (timeperiod - 1) + 1;
    if len <= lookback {
        return Err(TaError::InsufficientData {
            need: lookback + 1,
            got: len,
        });
    }

    let k = 2.0 / (timeperiod as f64 + 1.0);
    let p = timeperiod - 1;
    let tp = timeperiod as f64;

    let mut output = vec![0.0_f64; len];
    output[..lookback].fill(f64::NAN);

    // Phase 1: Build EMA1, indices [p .. 2p]. Accumulate SMA for EMA2 seed.
    let seed1 = sum_f64(&input[..timeperiod]) / tp;
    let mut e1 = seed1;
    let mut sum2 = seed1;
    for i in timeperiod..(2 * p + 1) {
        e1 = k.mul_add(input[i] - e1, e1);
        sum2 += e1;
    }

    // Phase 2: Build EMA2, indices [2p .. 3p]. Accumulate SMA for EMA3 seed.
    let seed2 = sum2 / tp;
    let mut e2 = seed2;
    let mut sum3 = seed2;
    for i in (2 * p + 1)..(3 * p + 1) {
        e1 = k.mul_add(input[i] - e1, e1);
        e2 = k.mul_add(e1 - e2, e2);
        sum3 += e2;
    }

    // Phase 3: EMA3 seed ready. First EMA3 value at index 3*p.
    // ROC needs previous EMA3 value, so first output at index 3*p + 1 = lookback.
    let seed3 = sum3 / tp;
    let mut e3_prev = seed3;

    // Compute one more step to get e3 at index 3*p + 1
    let i = 3 * p + 1;
    e1 = k.mul_add(input[i] - e1, e1);
    e2 = k.mul_add(e1 - e2, e2);
    let e3_cur = k.mul_add(e2 - e3_prev, e3_prev);
    if e3_prev != 0.0 {
        output[lookback] = ((e3_cur - e3_prev) / e3_prev) * 100.0;
    }
    e3_prev = e3_cur;

    // Steady state: cascade all 3 EMA layers + ROC
    for i in (lookback + 1)..len {
        e1 = k.mul_add(input[i] - e1, e1);
        e2 = k.mul_add(e1 - e2, e2);
        let e3_cur = k.mul_add(e2 - e3_prev, e3_prev);
        if e3_prev != 0.0 {
            output[i] = ((e3_cur - e3_prev) / e3_prev) * 100.0;
        }
        e3_prev = e3_cur;
    }

    Ok(output)
}
