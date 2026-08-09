//! Stateful causal Heikin-Ashi OHLC transform.

use crate::error::TaResult;

/// Named Heikin-Ashi OHLC output for one chronological bar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HeikinAshiValue {
    /// Transformed open.
    pub open: f64,
    /// Transformed high.
    pub high: f64,
    /// Transformed low.
    pub low: f64,
    /// Transformed close.
    pub close: f64,
}

/// Computes transformed open, high, low, and close values from OHLC bars.
#[derive(Debug, Clone)]
pub struct HeikinAshi {
    previous_open: Option<f64>,
    previous_close: Option<f64>,
    value: Option<HeikinAshiValue>,
}

impl HeikinAshi {
    /// Creates an empty Heikin-Ashi state.
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            previous_open: None,
            previous_close: None,
            value: None,
        })
    }

    /// Appends one OHLC bar and returns transformed OHLC values.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> HeikinAshiValue {
        let transformed_close = (open + high + low + close) / 4.0;
        let transformed_open = match (self.previous_open, self.previous_close) {
            (Some(previous_open), Some(previous_close)) => (previous_open + previous_close) / 2.0,
            _ => (open + close) / 2.0,
        };
        let transformed_high = high.max(transformed_open).max(transformed_close);
        let transformed_low = low.min(transformed_open).min(transformed_close);
        let value = HeikinAshiValue {
            open: transformed_open,
            high: transformed_high,
            low: transformed_low,
            close: transformed_close,
        };
        self.previous_open = Some(transformed_open);
        self.previous_close = Some(transformed_close);
        self.value = Some(value);
        value
    }

    /// Appends aligned OHLC slices into separate named output histories.
    #[allow(clippy::too_many_arguments)]
    pub fn extend_slices_into(
        &mut self,
        open: &[f64],
        high: &[f64],
        low: &[f64],
        close: &[f64],
        open_output: &mut Vec<f64>,
        high_output: &mut Vec<f64>,
        low_output: &mut Vec<f64>,
        close_output: &mut Vec<f64>,
    ) -> TaResult<()> {
        let length = open.len();
        for actual in [high.len(), low.len(), close.len()] {
            if actual != length {
                return Err(crate::TaError::LengthMismatch {
                    expected: length,
                    got: actual,
                });
            }
        }
        open_output.reserve(length);
        high_output.reserve(length);
        low_output.reserve(length);
        close_output.reserve(length);
        for (((&open, &high), &low), &close) in open.iter().zip(high).zip(low).zip(close) {
            let value = self.append(open, high, low, close);
            open_output.push(value.open);
            high_output.push(value.high);
            low_output.push(value.low);
            close_output.push(value.close);
        }
        Ok(())
    }

    /// Returns the latest named transformed OHLC value.
    pub fn value(&self) -> Option<HeikinAshiValue> {
        self.value
    }

    /// Clears previous-candle state.
    pub fn reset(&mut self) {
        self.previous_open = None;
        self.previous_close = None;
        self.value = None;
    }
}
