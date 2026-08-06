//! Stateful selectable moving average.
//!
//! MA exposes TA-Lib's moving-average type selector over the nine incremental
//! implementations while retaining the selected type's native warm-up.

use crate::error::TaResult;
use crate::ma_type::MaType;

use super::{moving_average::MovingAverage, StreamingIndicator};

/// Incremental moving average selected by [`MaType`].
pub struct Ma {
    inner: MovingAverage,
    value: Option<f64>,
}

impl Ma {
    /// Creates a selectable moving average with TA-Lib-compatible defaults at
    /// the binding layer.
    pub fn new(period: usize, ma_type: MaType) -> TaResult<Self> {
        Ok(Self {
            inner: MovingAverage::new(period, ma_type)?,
            value: None,
        })
    }
}

impl StreamingIndicator for Ma {
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
    use crate::overlap;

    #[test]
    fn matches_batch_for_all_moving_average_types() {
        let input: Vec<f64> = (0..200)
            .map(|index| 100.0 + (index as f64 * 0.29).sin() * 6.0 + index as f64 * 0.02)
            .collect();
        for code in 0..=8 {
            let ma_type = MaType::try_from(code).unwrap();
            let expected = overlap::ma(&input, 13, ma_type).unwrap();
            let mut state = Ma::new(13, ma_type).unwrap();
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
