//! Incremental Triple Exponential Average Rate of Change (TRIX).

use crate::error::TaResult;

use super::{invalid_period, Ema, StreamingIndicator};

/// Persistent TRIX with a triple TA-Lib-seeded EMA cascade and O(1) updates.
#[derive(Debug, Clone)]
pub struct Trix {
    ema1: Ema,
    ema2: Ema,
    ema3: Ema,
    previous_ema3: Option<f64>,
    value: Option<f64>,
}

impl Trix {
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            ema1: Ema::new(period)?,
            ema2: Ema::new(period)?,
            ema3: Ema::new(period)?,
            previous_ema3: None,
            value: None,
        })
    }
}

impl StreamingIndicator for Trix {
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
        let expected = crate::momentum::triple_exponential_rate_of_change(&input, 7).unwrap();
        let mut state = Trix::new(7).unwrap();
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
