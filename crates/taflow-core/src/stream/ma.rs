//! Stateful selectable moving average.
//!
//! MA exposes TA-Lib's moving-average type selector over the nine incremental
//! implementations while retaining the selected type's native warm-up.

use crate::error::TaResult;
use crate::ma_type::{compute_ma, MaType};

use super::{moving_average::MovingAverageDispatcher, StreamingIndicator};

/// Incremental moving average selected by [`MaType`].
/// Persistent Rust state or aligned output type for `MovingAverage`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct MovingAverage {
    inner: MovingAverageDispatcher,
    value: Option<f64>,
}

impl MovingAverage {
    /// Creates a selectable moving average with TA-Lib-compatible defaults at
    /// the binding layer.
    pub fn new(period: usize, ma_type: MaType) -> TaResult<Self> {
        Ok(Self {
            inner: MovingAverageDispatcher::new(period, ma_type)?,
            value: None,
        })
    }
}

impl StreamingIndicator for MovingAverage {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self.inner.append(input);
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_batch_for_all_moving_average_types() {
        let input: Vec<f64> = (0..200)
            .map(|index| 100.0 + (index as f64 * 0.29).sin() * 6.0 + index as f64 * 0.02)
            .collect();
        for code in 0..=8 {
            let ma_type = MaType::try_from(code).unwrap();
            let expected = crate::stream::moving_average(&input, 13, ma_type).unwrap();
            let mut state = MovingAverage::new(13, ma_type).unwrap();
            for (&input, expected) in input.iter().zip(expected) {
                let actual = state.append(input);
                if expected.is_nan() {
                    assert_eq!(actual, None, "MA type {code}");
                } else {
                    let actual = actual.unwrap();
                    assert!(
                        (actual - expected).abs() < 1e-9,
                        "MA type {code}: actual={actual}, expected={expected}"
                    );
                }
            }
        }
    }
}

/// MA - Moving Average (selectable type)
///
/// Wrapper that dispatches to SMA, EMA, WMA, DEMA, TEMA, TRIMA, KAMA, MAMA, or TripleExponentialAverage
/// based on the `matype` parameter.
///
/// C TA-Lib signature: MA(input, timeperiod=30, matype=0)
/// Compute the moving average result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
/// * `matype` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn moving_average(input: &[f64], timeperiod: usize, matype: MaType) -> TaResult<Vec<f64>> {
    compute_ma(input, timeperiod, matype)
}
