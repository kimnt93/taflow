//! Private shared state for lagged stream indicators.

use crate::error::TaResult;

use super::invalid_period;

/// A pure delay line: a ring of `period` values with one cursor.
#[derive(Debug, Clone)]
pub(crate) struct LaggedValue {
    buf: Box<[f64]>,
    /// Slot holding the value from `period` bars ago once warm.
    cursor: usize,
    len: usize,
}

impl LaggedValue {
    pub(crate) fn new(period: usize) -> TaResult<Self> {
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
    pub(crate) fn append(&mut self, input: f64) -> Option<(f64, f64)> {
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

    /// Extend a pristine delay line with a full slice without routing every
    /// observation through the ring. Returns `false` when scalar replay is
    /// required (a warmed state or a slice that does not clear warm-up).
    pub(crate) fn extend_from_empty_into<F>(
        &mut self,
        input: &[f64],
        output: &mut Vec<f64>,
        mut transform: F,
    ) -> bool
    where
        F: FnMut(f64, f64) -> f64,
    {
        let period = self.buf.len();
        if self.len != 0 || input.len() <= period {
            return false;
        }

        output.reserve(input.len());
        output.extend(std::iter::repeat_n(f64::NAN, period));
        output.extend(
            input[period..]
                .iter()
                .zip(&input[..input.len() - period])
                .map(|(&current, &previous)| transform(current, previous)),
        );

        let start = input.len() - period;
        for (index, &value) in input[start..].iter().enumerate() {
            self.buf[(start + index) % period] = value;
        }
        self.cursor = input.len() % period;
        self.len = period;
        true
    }

    pub(crate) fn reset(&mut self) {
        self.cursor = 0;
        self.len = 0;
    }
}
