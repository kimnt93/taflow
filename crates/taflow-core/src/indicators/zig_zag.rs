use crate::error::{TaError, TaResult};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ZigZagValue {
    pub high: f64,
    pub low: f64,
}

#[derive(Debug, Clone)]
pub struct ZigZag {
    threshold: f64,
    pivot: Option<f64>,
    rising: bool,
    value: Option<ZigZagValue>,
}

impl ZigZag {
    pub fn new(threshold: f64) -> TaResult<Self> {
        if !threshold.is_finite() || threshold <= 0.0 {
            return Err(TaError::InvalidParameter {
                name: "threshold",
                value: threshold.to_string(),
                reason: "must be finite and positive",
            });
        }
        Ok(Self {
            threshold,
            pivot: None,
            rising: true,
            value: None,
        })
    }
    pub fn append(&mut self, high: f64, low: f64) -> Option<ZigZagValue> {
        let mut out = ZigZagValue {
            high: f64::NAN,
            low: f64::NAN,
        };
        match self.pivot {
            None => self.pivot = Some((high + low) * 0.5),
            Some(pivot) if self.rising && low <= pivot * (1.0 - self.threshold) => {
                out.high = pivot;
                self.pivot = Some(low);
                self.rising = false;
            }
            Some(pivot) if !self.rising && high >= pivot * (1.0 + self.threshold) => {
                out.low = pivot;
                self.pivot = Some(high);
                self.rising = true;
            }
            Some(pivot) if self.rising && high > pivot => self.pivot = Some(high),
            Some(pivot) if !self.rising && low < pivot => self.pivot = Some(low),
            _ => {}
        }
        self.value = Some(out);
        self.value
    }
    pub fn value(&self) -> Option<ZigZagValue> {
        self.value
    }
    pub fn reset(&mut self) {
        self.pivot = None;
        self.rising = true;
        self.value = None;
    }
}
