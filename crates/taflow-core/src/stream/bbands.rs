//! Stateful Bollinger Bands.
//!
//! The selected moving-average type controls only the middle band.  As in
//! TA-Lib, both outer bands use population deviation around the rolling SMA.

use crate::error::TaResult;
use crate::ma_type::MaType;

use super::{moving_average::MovingAverageDispatcher, RollingStandardDeviation, StreamingIndicator};

/// Computes aligned upper, middle, and lower Bollinger Band vectors.
pub fn bollinger_bands(
    input: &[f64],
    timeperiod: usize,
    nbdevup: f64,
    nbdevdn: f64,
    matype: MaType,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    let mut state = BollingerBands::new(timeperiod, nbdevup, nbdevdn, matype)?;
    let mut upper = Vec::with_capacity(input.len());
    let mut middle = Vec::with_capacity(input.len());
    let mut lower = Vec::with_capacity(input.len());
    for &value in input {
        if let Some(output) = state.append(value) {
            upper.push(output.upper);
            middle.push(output.middle);
            lower.push(output.lower);
        } else {
            upper.push(f64::NAN);
            middle.push(f64::NAN);
            lower.push(f64::NAN);
        }
    }
    Ok((upper, middle, lower))
}

/// One aligned upper, middle, and lower Bollinger Bands observation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BollingerBandsValue {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
}

/// Incremental Bollinger Bands with constant per-bar work.
pub struct BollingerBands {
    middle: MovingAverageDispatcher,
    deviation: RollingStandardDeviation,
    deviations_up: f64,
    deviations_down: f64,
    value: Option<BollingerBandsValue>,
}

impl BollingerBands {
    /// Creates a BBANDS state for a period of at least two bars.
    pub fn new(
        period: usize,
        deviations_up: f64,
        deviations_down: f64,
        ma_type: MaType,
    ) -> TaResult<Self> {
        Ok(Self {
            middle: MovingAverageDispatcher::new(period, ma_type)?,
            deviation: RollingStandardDeviation::new(period, 1.0)?,
            deviations_up,
            deviations_down,
            value: None,
        })
    }
}

impl StreamingIndicator for BollingerBands {
    type Output = BollingerBandsValue;

    fn append(&mut self, input: f64) -> Option<BollingerBandsValue> {
        let middle = self.middle.append(input);
        let deviation = self.deviation.append(input);
        self.value = middle
            .zip(deviation)
            .map(|(middle, deviation)| BollingerBandsValue {
                upper: middle + self.deviations_up * deviation,
                middle,
                lower: middle - self.deviations_down * deviation,
            });
        self.value
    }

    fn value(&self) -> Option<BollingerBandsValue> {
        self.value
    }

    fn reset(&mut self) {
        self.middle.reset();
        self.deviation.reset();
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
            let (upper, middle, lower) = overlap::bollinger_bands(&input, 13, 2.0, 1.5, ma_type).unwrap();
            let mut state = BollingerBands::new(13, 2.0, 1.5, ma_type).unwrap();
            for (index, &input) in input.iter().enumerate() {
                match state.append(input) {
                    Some(actual) => {
                        assert!((actual.upper - upper[index]).abs() < 1e-8, "MA type {code}");
                        assert!(
                            (actual.middle - middle[index]).abs() < 1e-9,
                            "MA type {code}"
                        );
                        assert!((actual.lower - lower[index]).abs() < 1e-8, "MA type {code}");
                    }
                    None => {
                        assert!(upper[index].is_nan(), "MA type {code}");
                        assert!(middle[index].is_nan(), "MA type {code}");
                        assert!(lower[index].is_nan(), "MA type {code}");
                    }
                }
            }
        }
    }
}
