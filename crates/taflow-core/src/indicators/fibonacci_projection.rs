use crate::error::TaResult;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FibonacciProjectionValue {
    pub projection_100: f64,
    pub projection_1272: f64,
    pub projection_1618: f64,
    pub projection_200: f64,
}

#[derive(Debug, Clone)]
pub struct FibonacciProjection {
    minimum: f64,
    maximum: f64,
    count: usize,
    value: Option<FibonacciProjectionValue>,
}

impl FibonacciProjection {
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            minimum: f64::INFINITY,
            maximum: f64::NEG_INFINITY,
            count: 0,
            value: None,
        })
    }

    pub fn append(&mut self, high: f64, low: f64) -> Option<FibonacciProjectionValue> {
        self.count += 1;
        self.maximum = self.maximum.max(high);
        self.minimum = self.minimum.min(low);
        self.value = (self.count >= 3).then(|| {
            let range = self.maximum - self.minimum;
            FibonacciProjectionValue {
                projection_100: self.maximum + range,
                projection_1272: self.maximum + range * 1.272,
                projection_1618: self.maximum + range * 1.618,
                projection_200: self.maximum + range * 2.0,
            }
        });
        self.value
    }

    pub fn value(&self) -> Option<FibonacciProjectionValue> {
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
