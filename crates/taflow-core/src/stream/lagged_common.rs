//! Private shared state for lagged stream indicators.

use crate::error::TaResult;

use super::invalid_period;

/// A pure delay line: a ring of `period` values with one cursor.
#[derive(Debug, Clone)]
pub(super) struct LaggedValue {
    buf: Box<[f64]>,
    /// Slot holding the value from `period` bars ago once warm.
    cursor: usize,
    len: usize,
}

impl LaggedValue {
    pub(super) fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("timeperiod", period, 1));
        }
        Ok(Self {
            buf: vec![0.0; period].into_boxed_slice(),
            cursor: 0,
            len: 0,
        })
    }

    #[inline]
    pub(super) fn append(&mut self, input: f64) -> Option<(f64, f64)> {
        let period = self.buf.len();
        if self.len < period {
            self.buf[self.cursor] = input;
            self.cursor += 1;
            if self.cursor == period {
                self.cursor = 0;
            }
            self.len += 1;
            return None;
        }
        let previous = self.buf[self.cursor];
        self.buf[self.cursor] = input;
        self.cursor += 1;
        if self.cursor == period {
            self.cursor = 0;
        }
        Some((input, previous))
    }

    pub(super) fn reset(&mut self) {
        self.cursor = 0;
        self.len = 0;
    }
}

pub(super) fn validate_rate_of_change(input: &[f64], timeperiod: usize) -> TaResult<()> {
    if timeperiod == 0 {
        return Err(crate::TaError::InvalidParameter {
            name: "timeperiod",
            value: "0".to_string(),
            reason: "must be >= 1",
        });
    }
    if input.len() <= timeperiod {
        return Err(crate::TaError::InsufficientData {
            need: timeperiod + 1,
            got: input.len(),
        });
    }
    Ok(())
}
