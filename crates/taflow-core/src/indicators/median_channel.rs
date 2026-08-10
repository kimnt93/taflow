use crate::error::{TaError, TaResult};
use crate::stream::sorted_ring::SortedRing;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MedianChannelValue {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
}
#[derive(Debug, Clone)]
pub struct MedianChannel {
    window: SortedRing,
    deviations: Vec<f64>,
    multiplier: f64,
    value: Option<MedianChannelValue>,
}
impl MedianChannel {
    pub fn new(period: usize, multiplier: f64) -> TaResult<Self> {
        if period == 0 {
            return Err(TaError::InvalidParameter {
                name: "period",
                value: period.to_string(),
                reason: "must be positive",
            });
        }
        if !multiplier.is_finite() || multiplier <= 0.0 {
            return Err(TaError::InvalidParameter {
                name: "multiplier",
                value: multiplier.to_string(),
                reason: "must be finite and positive",
            });
        }
        Ok(Self {
            window: SortedRing::new(period),
            deviations: Vec::with_capacity(period),
            multiplier,
            value: None,
        })
    }
    pub fn append(&mut self, x: f64) -> Option<MedianChannelValue> {
        self.window.push(x);
        if !self.window.is_full() {
            self.value = None;
            return None;
        }
        self.value = Some({
            let s = self.window.sorted();
            let n = s.len();
            let m = if n % 2 == 1 {
                s[n / 2]
            } else {
                (s[n / 2 - 1] + s[n / 2]) * 0.5
            };
            self.deviations.clear();
            self.deviations
                .extend(s.iter().map(|value| (value - m).abs()));
            self.deviations.sort_by(f64::total_cmp);
            let mad = if n % 2 == 1 {
                self.deviations[n / 2]
            } else {
                (self.deviations[n / 2 - 1] + self.deviations[n / 2]) * 0.5
            };
            let width = mad * self.multiplier;
            MedianChannelValue {
                upper: m + width,
                middle: m,
                lower: m - width,
            }
        });
        self.value
    }
    pub fn value(&self) -> Option<MedianChannelValue> {
        self.value
    }
    pub fn reset(&mut self) {
        self.window.clear();
        self.deviations.clear();
        self.value = None;
    }
}
