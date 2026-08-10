use crate::error::{TaError, TaResult};
use crate::stream::sorted_ring::SortedRing;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuartileBandsValue {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
}
#[derive(Debug, Clone)]
pub struct QuartileBands {
    period: usize,
    window: SortedRing,
    value: Option<QuartileBandsValue>,
}
impl QuartileBands {
    pub fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(TaError::InvalidParameter {
                name: "period",
                value: period.to_string(),
                reason: "must be positive",
            });
        }
        Ok(Self {
            period,
            window: SortedRing::new(period),
            value: None,
        })
    }
    fn q(sorted: &[f64], p: f64) -> f64 {
        let x = p * (sorted.len() - 1) as f64;
        let a = x.floor() as usize;
        let b = x.ceil() as usize;
        sorted[a] + (sorted[b] - sorted[a]) * (x - a as f64)
    }
    pub fn append(&mut self, x: f64) -> Option<QuartileBandsValue> {
        self.window.push(x);
        self.value = self.window.is_full().then(|| {
            let s = self.window.sorted();
            QuartileBandsValue {
                upper: Self::q(s, 0.75),
                middle: Self::q(s, 0.5),
                lower: Self::q(s, 0.25),
            }
        });
        self.value
    }
    pub fn value(&self) -> Option<QuartileBandsValue> {
        self.value
    }
    pub fn reset(&mut self) {
        self.window.clear();
        self.value = None;
    }
    pub fn period(&self) -> usize {
        self.period
    }
}
