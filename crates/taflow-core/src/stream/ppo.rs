//! Stateful Percentage Price Oscillator.
//!
//! PPO normalizes the distance between fast and slow moving averages by the
//! slow average and supports all nine TA-Lib moving-average types.

use crate::error::TaResult;
use crate::ma_type::MaType;

use super::{moving_average::MovingAverageDispatcher, StreamingIndicator};

/// Compute the percentage price oscillator result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `fastperiod` - Input series or configuration value.
/// * `slowperiod` - Input series or configuration value.
/// * `matype` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn percentage_price_oscillator(input: &[f64], fastperiod: usize, slowperiod: usize, matype: MaType) -> TaResult<Vec<f64>> {
    let mut state = PercentagePriceOscillator::new(fastperiod, slowperiod, matype)?;
    Ok(input.iter().map(|&value| state.append(value).unwrap_or(f64::NAN)).collect())
}

/// Incremental PPO driven by two selected moving-average states.
pub struct PercentagePriceOscillator {
    fast: MovingAverageDispatcher,
    slow: MovingAverageDispatcher,
    value: Option<f64>,
}

impl PercentagePriceOscillator {
    /// Creates a PPO state; period order is normalized to fast then slow.
    pub fn new(fast_period: usize, slow_period: usize, ma_type: MaType) -> TaResult<Self> {
        let (fast_period, slow_period) = if fast_period < slow_period {
            (fast_period, slow_period)
        } else {
            (slow_period, fast_period)
        };
        Ok(Self {
            fast: MovingAverageDispatcher::new(fast_period, ma_type)?,
            slow: MovingAverageDispatcher::new(slow_period, ma_type)?,
            value: None,
        })
    }
}

impl StreamingIndicator for PercentagePriceOscillator {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        let fast = self.fast.append(input);
        let slow = self.slow.append(input);
        self.value = fast
            .zip(slow)
            .and_then(|(fast, slow)| (slow != 0.0).then_some((fast - slow) / slow * 100.0));
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.fast.reset();
        self.slow.reset();
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn matches_batch_for_all_moving_average_types() {
        let input: Vec<f64> = (0..200)
            .map(|index| 100.0 + (index as f64 * 0.19).sin() * 7.0 + index as f64 * 0.03)
            .collect();
        for code in 0..=8 {
            let ma_type = MaType::try_from(code).unwrap();
            let expected = percentage_price_oscillator(&input, 7, 13, ma_type).unwrap();
            let mut state = PercentagePriceOscillator::new(7, 13, ma_type).unwrap();
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
