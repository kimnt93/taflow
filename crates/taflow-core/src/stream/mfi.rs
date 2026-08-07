//! Incremental Money Flow Index (MFI).

use crate::error::{TaError, TaResult};

use super::{invalid_period, Window};

/// Computes an aligned Money Flow Index vector from HLCV slices.
pub fn money_flow_index(high: &[f64], low: &[f64], close: &[f64], volume: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if high.len() != low.len() || high.len() != close.len() || high.len() != volume.len() {
        return Err(crate::TaError::LengthMismatch { expected: high.len(), got: low.len().min(close.len()).min(volume.len()) });
    }
    let mut state = MoneyFlowIndex::new(timeperiod)?;
    Ok(high.iter().zip(low).zip(close).zip(volume).map(|(((&high, &low), &close), &volume)| state.append(high, low, close, volume).unwrap_or(f64::NAN)).collect())
}

/// Persistent Money Flow Index with O(1) updates after each HLCV bar.
#[derive(Debug, Clone)]
pub struct MoneyFlowIndex {
    previous_typical_price: Option<f64>,
    positive_flow: Window,
    negative_flow: Window,
    positive_sum: f64,
    negative_sum: f64,
    value: Option<f64>,
}

impl MoneyFlowIndex {
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
            previous_typical_price: None,
            positive_flow: Window::new(period)?,
            negative_flow: Window::new(period)?,
            positive_sum: 0.0,
            negative_sum: 0.0,
            value: None,
        })
    }

    /// Appends one HLCV bar and returns MFI after `timeperiod` price changes.
    pub fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> Option<f64> {
        let typical_price = (high + low + close) / 3.0;
        let Some(previous) = self.previous_typical_price.replace(typical_price) else {
            return None;
        };

        let money_flow = typical_price * volume;
        let (positive, negative) = if typical_price > previous {
            (money_flow, 0.0)
        } else if typical_price < previous {
            (0.0, money_flow)
        } else {
            (0.0, 0.0)
        };
        if let Some(old) = self.positive_flow.push(positive) {
            self.positive_sum -= old;
        }
        if let Some(old) = self.negative_flow.push(negative) {
            self.negative_sum -= old;
        }
        self.positive_sum += positive;
        self.negative_sum += negative;

        self.value = self.positive_flow.is_full().then(|| {
            if self.negative_sum > 0.0 {
                100.0 - 100.0 / (1.0 + self.positive_sum / self.negative_sum)
            } else {
                100.0
            }
        });
        self.value
    }

    /// Computes or updates `extend_slice` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn extend_slice(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        volume: &[f64],
    ) -> TaResult<Vec<Option<f64>>> {
        if high.len() != low.len() || high.len() != close.len() || high.len() != volume.len() {
            return Err(TaError::LengthMismatch {
                expected: high.len(),
                got: low.len().min(close.len()).min(volume.len()),
            });
        }
        Ok(high
            .iter()
            .zip(low)
            .zip(close)
            .zip(volume)
            .map(|(((&high, &low), &close), &volume)| self.append(high, low, close, volume))
            .collect())
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
        self.previous_typical_price = None;
        self.positive_flow.clear();
        self.negative_flow.clear();
        self.positive_sum = 0.0;
        self.negative_sum = 0.0;
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_batch_for_chunked_extend_and_replay() {
        let close: Vec<f64> = (0..96)
            .map(|index| 100.0 + index as f64 * 0.11 + (index as f64 * 0.39).sin() * 3.0)
            .collect();
        let high: Vec<f64> = close.iter().map(|value| value + 1.3).collect();
        let low: Vec<f64> = close.iter().map(|value| value - 0.7).collect();
        let volume: Vec<f64> = (0..96).map(|index| 1_000.0 + index as f64 * 17.0).collect();
        let expected = crate::stream::money_flow_index(&high, &low, &close, &volume, 14).unwrap();

        let mut state = MoneyFlowIndex::new(14).unwrap();
        let mut actual = state
            .extend_slice(&high[..41], &low[..41], &close[..41], &volume[..41])
            .unwrap();
        actual.extend(
            state
                .extend_slice(&high[41..], &low[41..], &close[41..], &volume[41..])
                .unwrap(),
        );
        for (actual, expected) in actual.iter().zip(&expected) {
            match actual {
                Some(actual) => assert!((actual - expected).abs() < 1e-10),
                None => assert!(expected.is_nan()),
            }
        }
    }
}
