//! Stateful fixed-bin session volume profile levels.

use crate::error::{TaError, TaResult};

/// Computes point of control and value-area bounds for each session bar.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `SessionVolumeLevels`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct SessionVolumeLevels {
    bins: usize,
    value_area: f64,
    low: Option<f64>,
    high: f64,
    step: f64,
    histogram: Vec<f64>,
    value: Option<(f64, f64, f64)>,
}

impl SessionVolumeLevels {
    /// Creates a profile with a positive bin count and value-area fraction.
    pub fn new(bins: usize, value_area: f64) -> TaResult<Self> {
        if bins < 1 {
            return Err(super::invalid_period("bins", bins, 1));
        }
        if !(0.0..=1.0).contains(&value_area) || value_area == 0.0 {
            return Err(TaError::InvalidParameter {
                name: "value_area",
                value: value_area.to_string(),
                reason: "must be in (0, 1]",
            });
        }
        Ok(Self {
            bins,
            value_area,
            low: None,
            high: 0.0,
            step: 1.0,
            histogram: vec![0.0; bins],
            value: None,
        })
    }

    /// Appends one OHLCV bar and optionally starts a new anchored session.
    pub fn append(
        &mut self,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
        anchor: bool,
    ) -> (f64, f64, f64) {
        if anchor || self.low.is_none() {
            self.low = Some(low);
            self.high = high;
            self.step = ((high - low) / self.bins as f64).max(1.0e-12);
            self.histogram.fill(0.0);
        }
        let session_low = self.low.as_mut().expect("initialized above");
        *session_low = session_low.min(low);
        self.high = self.high.max(high);
        let low_value = *session_low;
        let index =
            (((close - low_value) / self.step) as isize).clamp(0, self.bins as isize - 1) as usize;
        self.histogram[index] += volume;
        let poc = self
            .histogram
            .iter()
            .enumerate()
            .max_by(|(a, x), (b, y)| {
                x.partial_cmp(y)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| b.cmp(a))
            })
            .map(|(i, _)| i)
            .unwrap_or(0);
        let total: f64 = self.histogram.iter().sum();
        let target = total * self.value_area;
        let (mut left, mut right, mut accumulated) = (poc, poc, self.histogram[poc]);
        while accumulated < target && (left > 0 || right + 1 < self.bins) {
            if left == 0 {
                right += 1;
            } else if right + 1 == self.bins {
                left -= 1;
            } else if self.histogram[left - 1] >= self.histogram[right + 1] {
                left -= 1;
            } else {
                right += 1;
            }
            accumulated = self.histogram[left..=right].iter().sum();
        }
        let result = (
            (poc as f64 + 0.5) * self.step + low_value,
            (right as f64 + 0.5) * self.step + low_value,
            (left as f64 + 0.5) * self.step + low_value,
        );
        self.value = Some(result);
        result
    }

    /// Returns point of control, value-area high, and value-area low.
    pub fn value(&self) -> Option<(f64, f64, f64)> {
        self.value
    }
    /// Clears profile and session state.
    pub fn reset(&mut self) {
        self.low = None;
        self.high = 0.0;
        self.step = 1.0;
        self.histogram.fill(0.0);
        self.value = None;
    }
}
