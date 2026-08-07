//! Stateful Directional Movement Index.

use crate::error::TaResult;

use super::directional::DirectionalMovement;

/// Computes an aligned Directional Movement Index vector from HLC slices.
pub fn directional_movement_index(high: &[f64], low: &[f64], close: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if high.len() != low.len() || high.len() != close.len() {
        return Err(crate::TaError::LengthMismatch { expected: high.len(), got: low.len().min(close.len()) });
    }
    let mut state = DirectionalMovementIndex::new(timeperiod)?;
    Ok(high.iter().zip(low).zip(close).map(|((high, low), close)| state.append(*high, *low, *close).unwrap_or(f64::NAN)).collect())
}

/// Incremental DX with TA-Lib-compatible Wilder smoothing and lookback.
pub struct DirectionalMovementIndex {
    directional: DirectionalMovement,
    value: Option<f64>,
}

impl DirectionalMovementIndex {
    /// Creates a DX state with a period of at least two bars.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            directional: DirectionalMovement::new(period)?,
            value: None,
        })
    }

    /// Appends one high, low, and close observation.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        self.value = self
            .directional
            .append(high, low, close)
            .map(|value| value.dx);
        self.value
    }

    /// Returns the latest warmed output.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restores the post-construction state.
    pub fn reset(&mut self) {
        self.directional.reset();
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
            .map(|i| 100.0 + (i as f64 * 0.17).sin() * 8.0 + i as f64 * 0.01)
            .collect();
        let high: Vec<f64> = close.iter().map(|value| value + 1.3).collect();
        let low: Vec<f64> = close.iter().map(|value| value - 1.1).collect();
        for period in [2, 3, 14, 30] {
            let expected = directional_movement_index(&high, &low, &close, period).unwrap();
            let mut state = DirectionalMovementIndex::new(period).unwrap();
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
}
