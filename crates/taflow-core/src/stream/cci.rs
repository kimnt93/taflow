//! Incremental Commodity Channel Index (CCI).
//!
//! CCI's moving mean is O(1), while its exact mean absolute deviation needs a
//! bounded window scan. The state retains no full price history and continues
//! from each appended HLC bar without replaying earlier input.

use crate::error::{TaError, TaResult};

use super::{invalid_period, Window};

/// Computes an aligned Commodity Channel Index vector from HLC slices.
pub fn commodity_channel_index(high: &[f64], low: &[f64], close: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if high.len() != low.len() || high.len() != close.len() {
        return Err(TaError::LengthMismatch { expected: high.len(), got: low.len().min(close.len()) });
    }
    let mut state = CommodityChannelIndex::new(timeperiod)?;
    Ok(high
        .iter()
        .zip(low)
        .zip(close)
        .map(|((high, low), close)| state.append(*high, *low, *close).unwrap_or(f64::NAN))
        .collect())
}

/// Persistent Commodity Channel Index with TA-Lib-compatible warm-up.
#[derive(Debug, Clone)]
pub struct CommodityChannelIndex {
    period: usize,
    window: Window,
    sum: f64,
    value: Option<f64>,
}

impl CommodityChannelIndex {
    /// Creates an empty CCI state. TA-Lib requires a period of at least two.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            period,
            window: Window::new(period)?,
            sum: 0.0,
            value: None,
        })
    }

    /// Appends one high/low/close bar and returns CCI after warm-up.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let typical_price = (high + low + close) / 3.0;
        if let Some(old) = self.window.push(typical_price) {
            self.sum -= old;
        }
        self.sum += typical_price;

        self.value = self.window.is_full().then(|| {
            let period = self.period as f64;
            let average = self.sum / period;
            let mean_deviation = self
                .window
                .iter()
                .map(|value| (*value - average).abs())
                .sum::<f64>()
                / period;
            if mean_deviation > 0.0 {
                (typical_price - average) / (0.015 * mean_deviation)
            } else {
                0.0
            }
        });
        self.value
    }

    /// Extends state with aligned HLC slices after validating all lengths.
    pub fn extend_slice(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
    ) -> TaResult<Vec<Option<f64>>> {
        if high.len() != low.len() || high.len() != close.len() {
            return Err(TaError::LengthMismatch {
                expected: high.len(),
                got: low.len().min(close.len()),
            });
        }
        Ok(high
            .iter()
            .zip(low)
            .zip(close)
            .map(|((&high, &low), &close)| self.append(high, low, close))
            .collect())
    }

    /// Returns the newest warm value without materializing history.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clears all accumulated state while retaining the allocated window.
    pub fn reset(&mut self) {
        self.window.clear();
        self.sum = 0.0;
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_batch_for_extend_chunk_and_reset_replay() {
        let close: Vec<f64> = (0..96)
            .map(|index| 100.0 + index as f64 * 0.17 + (index as f64 * 0.23).sin() * 2.0)
            .collect();
        let high: Vec<f64> = close.iter().map(|value| value + 1.2).collect();
        let low: Vec<f64> = close.iter().map(|value| value - 0.9).collect();
        let expected = crate::momentum::commodity_channel_index(&high, &low, &close, 14).unwrap();

        let mut state = CommodityChannelIndex::new(14).unwrap();
        let mut actual = state
            .extend_slice(&high[..37], &low[..37], &close[..37])
            .unwrap();
        actual.extend(
            state
                .extend_slice(&high[37..], &low[37..], &close[37..])
                .unwrap(),
        );
        for (actual, expected) in actual.iter().zip(expected.iter()) {
            match actual {
                Some(actual) => assert!((actual - expected).abs() < 1e-10),
                None => assert!(expected.is_nan()),
            }
        }

        state.reset();
        for ((&high, &low), (&close, expected)) in
            high.iter().zip(&low).zip(close.iter().zip(&expected))
        {
            match state.append(high, low, close) {
                Some(actual) => assert!((actual - expected).abs() < 1e-10),
                None => assert!(expected.is_nan()),
            }
        }
    }
}
