use crate::error::TaResult;
use crate::stream::invalid_period;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct VolumeRelativeStrengthIndex {
    period: usize,
    previous_close: Option<f64>,
    changes: VecDeque<(f64, f64)>,
    value: Option<f64>,
}

impl VolumeRelativeStrengthIndex {
    pub fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("period", period, 1));
        }
        Ok(Self {
            period,
            previous_close: None,
            changes: VecDeque::with_capacity(period),
            value: None,
        })
    }
    pub fn append(&mut self, close: f64, volume: f64) -> Option<f64> {
        let Some(previous) = self.previous_close.replace(close) else {
            return None;
        };
        let change = close - previous;
        self.changes.push_back(if change >= 0.0 {
            (volume, 0.0)
        } else {
            (0.0, volume)
        });
        if self.changes.len() > self.period {
            self.changes.pop_front();
        }
        self.value = (self.changes.len() == self.period).then(|| {
            let (gain, loss) = self
                .changes
                .iter()
                .fold((0.0, 0.0), |(g, l), (x, y)| (g + x, l + y));
            if gain + loss == 0.0 {
                0.0
            } else {
                100.0 * gain / (gain + loss)
            }
        });
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.changes.clear();
        self.value = None;
    }
}
