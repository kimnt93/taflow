//! Stateful Intraday Momentum Index.
//!
//! IMI separates each candle body into an intraday gain or loss and maintains
//! rolling sums for the most recent `timeperiod` bars in constant time.

use crate::error::TaResult;

use super::{invalid_period, Window};

/// Incremental Intraday Momentum Index with TA-Lib-compatible warm-up.
#[derive(Debug, Clone)]
pub struct IntradayMomentumIndex {
    gains: Window,
    losses: Window,
    gain_sum: f64,
    loss_sum: f64,
    value: Option<f64>,
}

impl IntradayMomentumIndex {
    /// Creates an IMI state with a period of at least two bars.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            gains: Window::new(period)?,
            losses: Window::new(period)?,
            gain_sum: 0.0,
            loss_sum: 0.0,
            value: None,
        })
    }

    /// Appends one candle's open and close.
    pub fn append(&mut self, open: f64, close: f64) -> Option<f64> {
        let movement = close - open;
        let (gain, loss) = if movement > 0.0 {
            (movement, 0.0)
        } else {
            (0.0, -movement)
        };
        if let Some(expired) = self.gains.push(gain) {
            self.gain_sum -= expired;
        }
        if let Some(expired) = self.losses.push(loss) {
            self.loss_sum -= expired;
        }
        self.gain_sum += gain;
        self.loss_sum += loss;
        self.value = self.gains.is_full().then(|| {
            let total = self.gain_sum + self.loss_sum;
            if total == 0.0 {
                50.0
            } else {
                100.0 * self.gain_sum / total
            }
        });
        self.value
    }

    /// Returns the latest warm value.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restores the post-construction state while retaining window capacity.
    pub fn reset(&mut self) {
        self.gains.clear();
        self.losses.clear();
        self.gain_sum = 0.0;
        self.loss_sum = 0.0;
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn matches_batch_and_reset_replay() {
        let open: Vec<f64> = (0..300)
            .map(|index| 100.0 + (index as f64 * 0.17).sin() * 8.0)
            .collect();
        let close: Vec<f64> = open
            .iter()
            .enumerate()
            .map(|(index, open)| open + (index as f64 * 0.31).cos() * 1.7)
            .collect();
        let expected = crate::stream::intraday_momentum_index(&open, &close, 14).unwrap();
        let mut state = IntradayMomentumIndex::new(14).unwrap();
        for index in 0..open.len() {
            match state.append(open[index], close[index]) {
                Some(actual) => assert!((actual - expected[index]).abs() < 1e-10),
                None => assert!(expected[index].is_nan()),
            }
        }
        let expected_final = state.value();
        state.reset();
        for index in 0..open.len() {
            state.append(open[index], close[index]);
        }
        assert_eq!(state.value(), expected_final);
    }

    #[test]
    fn flat_candles_return_neutral_value() {
        let mut state = IntradayMomentumIndex::new(2).unwrap();
        assert_eq!(state.append(10.0, 10.0), None);
        assert_eq!(state.append(10.0, 10.0), Some(50.0));
    }
}
use crate::error::TaError;

/// Compute the intraday momentum index result for the supplied aligned series.
///
/// # Parameters
///
/// * `open` - Input series or configuration value.
/// * `close` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn intraday_momentum_index(open: &[f64], close: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let len = open.len();
    if len != close.len() {
        return Err(TaError::LengthMismatch {
            expected: len,
            got: close.len(),
        });
    }
    if timeperiod < 2 {
        return Err(TaError::InvalidParameter {
            name: "timeperiod",
            value: timeperiod.to_string(),
            reason: "must be >= 2",
        });
    }
    if len < timeperiod {
        return Err(TaError::InsufficientData {
            need: timeperiod,
            got: len,
        });
    }

    let lookback = timeperiod - 1;
    let mut output = vec![f64::NAN; len];
    for today in lookback..len {
        let mut gains = 0.0;
        let mut losses = 0.0;
        for index in (today + 1 - timeperiod)..=today {
            let movement = close[index] - open[index];
            if movement > 0.0 {
                gains += movement;
            } else {
                losses -= movement;
            }
        }
        output[today] = if gains + losses == 0.0 {
            50.0
        } else {
            100.0 * gains / (gains + losses)
        };
    }
    Ok(output)
}
