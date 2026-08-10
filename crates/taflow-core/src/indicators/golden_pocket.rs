use crate::error::TaResult;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GoldenPocketValue {
    pub lower: f64,
    pub midpoint: f64,
    pub upper: f64,
}

#[derive(Debug, Clone)]
pub struct GoldenPocket {
    minimum: f64,
    maximum: f64,
    count: usize,
    value: Option<GoldenPocketValue>,
}

impl GoldenPocket {
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            minimum: f64::INFINITY,
            maximum: f64::NEG_INFINITY,
            count: 0,
            value: None,
        })
    }

    pub fn append(&mut self, high: f64, low: f64) -> Option<GoldenPocketValue> {
        self.count += 1;
        self.maximum = self.maximum.max(high);
        self.minimum = self.minimum.min(low);
        self.value = (self.count >= 2).then(|| {
            let range = self.maximum - self.minimum;
            GoldenPocketValue {
                lower: self.minimum + range * 0.618,
                midpoint: self.minimum + range * 0.634,
                upper: self.minimum + range * 0.65,
            }
        });
        self.value
    }

    pub fn value(&self) -> Option<GoldenPocketValue> {
        self.value
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn reset(&mut self) {
        self.minimum = f64::INFINITY;
        self.maximum = f64::NEG_INFINITY;
        self.count = 0;
        self.value = None;
    }
}
