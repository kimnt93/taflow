use crate::error::TaResult;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FibonacciConfluenceValue {
    pub retracement: f64,
    pub extension: f64,
}

#[derive(Debug, Clone)]
pub struct FibonacciConfluence {
    minimum: f64,
    maximum: f64,
    count: usize,
    value: Option<FibonacciConfluenceValue>,
}

impl FibonacciConfluence {
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            minimum: f64::INFINITY,
            maximum: f64::NEG_INFINITY,
            count: 0,
            value: None,
        })
    }

    pub fn append(&mut self, high: f64, low: f64) -> Option<FibonacciConfluenceValue> {
        self.count += 1;
        self.maximum = self.maximum.max(high);
        self.minimum = self.minimum.min(low);
        self.value = (self.count >= 3).then(|| {
            let range = self.maximum - self.minimum;
            FibonacciConfluenceValue {
                retracement: self.minimum + range * 0.618,
                extension: self.maximum + range * 0.618,
            }
        });
        self.value
    }

    pub fn value(&self) -> Option<FibonacciConfluenceValue> {
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
