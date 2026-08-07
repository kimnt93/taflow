//! Stateful Average Directional Index.
//!
//! ADX advances Wilder-smoothed true range and directional movement, seeds
//! from the first full period of DX values, and then Wilder-smooths later DX.

use crate::error::TaResult;

use super::directional::DirectionalMovement;

/// Incremental ADX with TA-Lib-compatible seeding and lookback.
pub struct Adx {
    period: usize,
    period_f: f64,
    directional: DirectionalMovement,
    dx_sum: f64,
    dx_count: usize,
    value: Option<f64>,
}

impl Adx {
    /// Creates an ADX state with a period of at least two bars.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            period,
            period_f: period as f64,
            directional: DirectionalMovement::new(period)?,
            dx_sum: 0.0,
            dx_count: 0,
            value: None,
        })
    }

    /// Appends one high, low, and close observation.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let directional = self.directional.append(high, low, close)?;
        self.value = if self.dx_count < self.period {
            self.dx_sum += directional.dx;
            self.dx_count += 1;
            (self.dx_count == self.period).then_some(self.dx_sum / self.period_f)
        } else {
            Some(
                (self.value.expect("ADX is seeded") * (self.period_f - 1.0) + directional.dx)
                    / self.period_f,
            )
        };
        self.value
    }

    /// Returns the latest warmed output.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restores the post-construction state.
    pub fn reset(&mut self) {
        self.directional.reset();
        self.dx_sum = 0.0;
        self.dx_count = 0;
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::momentum;

    #[test]
    fn matches_batch_and_reset_replay() {
        let close: Vec<f64> = (0..700)
            .map(|index| 100.0 + (index as f64 * 0.17).sin() * 8.0 + index as f64 * 0.01)
            .collect();
        let high: Vec<f64> = close.iter().map(|value| value + 1.3).collect();
        let low: Vec<f64> = close.iter().map(|value| value - 1.1).collect();
        for period in [2, 3, 14, 30] {
            let expected = momentum::average_directional_index(&high, &low, &close, period).unwrap();
            let mut state = Adx::new(period).unwrap();
            for index in 0..close.len() {
                match state.append(high[index], low[index], close[index]) {
                    Some(actual) => assert!((actual - expected[index]).abs() < 1e-12),
                    None => assert!(expected[index].is_nan()),
                }
            }
            let final_value = state.value();
            state.reset();
            for index in 0..close.len() {
                state.append(high[index], low[index], close[index]);
            }
            assert_eq!(state.value(), final_value);
        }
    }

    #[test]
    fn flat_prices_return_zero_after_warmup() {
        let mut state = Adx::new(14).unwrap();
        let values: Vec<_> = (0..50).map(|_| state.append(10.0, 10.0, 10.0)).collect();
        assert_eq!(values[27], Some(0.0));
    }
}
