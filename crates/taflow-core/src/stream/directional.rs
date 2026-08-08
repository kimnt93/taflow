//! Shared incremental directional-movement recurrence.

use crate::error::{TaError, TaResult};

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub(super) struct DirectionalValue {
    pub(super) true_range: f64,
    pub(super) plus_dm: f64,
    pub(super) minus_dm: f64,
    pub(super) plus_di: f64,
    pub(super) minus_di: f64,
    pub(super) dx: f64,
}

pub(super) struct DirectionalMovement {
    // Fields are `pub(super)` so sibling bulk kernels (ADX/ADXR/+DI) can hold
    // the Wilder recurrence state in locals and write it back after a fused
    // loop; the arithmetic contract lives in `append` below.
    pub(super) period: usize,
    pub(super) period_f: f64,
    pub(super) index: usize,
    pub(super) previous: Option<(f64, f64, f64)>,
    pub(super) true_range: f64,
    pub(super) plus_dm: f64,
    pub(super) minus_dm: f64,
}

impl DirectionalMovement {
    pub(super) fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: period.to_string(),
                reason: "must be >= 1",
            });
        }
        Ok(Self {
            period,
            period_f: period as f64,
            index: 0,
            previous: None,
            true_range: 0.0,
            plus_dm: 0.0,
            minus_dm: 0.0,
        })
    }

    pub(super) fn append(&mut self, high: f64, low: f64, close: f64) -> Option<DirectionalValue> {
        let index = self.index;
        self.index += 1;
        let Some((previous_high, previous_low, previous_close)) = self.previous else {
            self.previous = Some((high, low, close));
            return None;
        };
        self.previous = Some((high, low, close));

        let true_range = (high - low)
            .max((high - previous_close).abs())
            .max((low - previous_close).abs());
        let up = high - previous_high;
        let down = previous_low - low;
        let plus_dm = if up > down && up > 0.0 { up } else { 0.0 };
        let minus_dm = if down > up && down > 0.0 { down } else { 0.0 };

        if index < self.period {
            self.true_range += true_range;
            self.plus_dm += plus_dm;
            self.minus_dm += minus_dm;
            return None;
        }

        self.true_range = self.true_range - self.true_range / self.period_f + true_range;
        self.plus_dm = self.plus_dm - self.plus_dm / self.period_f + plus_dm;
        self.minus_dm = self.minus_dm - self.minus_dm / self.period_f + minus_dm;
        let (plus_di, minus_di, dx) = if self.true_range > 0.0 {
            let plus_di = 100.0 * self.plus_dm / self.true_range;
            let minus_di = 100.0 * self.minus_dm / self.true_range;
            let sum = plus_di + minus_di;
            let dx = if sum > 0.0 {
                100.0 * (plus_di - minus_di).abs() / sum
            } else {
                0.0
            };
            (plus_di, minus_di, dx)
        } else {
            (0.0, 0.0, 0.0)
        };
        Some(DirectionalValue {
            true_range: self.true_range,
            plus_dm: self.plus_dm,
            minus_dm: self.minus_dm,
            plus_di,
            minus_di,
            dx,
        })
    }

    pub(super) fn reset(&mut self) {
        self.index = 0;
        self.previous = None;
        self.true_range = 0.0;
        self.plus_dm = 0.0;
        self.minus_dm = 0.0;
    }
}
