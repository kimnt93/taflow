use crate::error::TaResult;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FibonacciArcsValue {
    pub radius_382: f64,
    pub radius_500: f64,
    pub radius_618: f64,
}

#[derive(Debug, Clone)]
pub struct FibonacciArcs {
    minimum: f64,
    maximum: f64,
    count: usize,
    value: Option<FibonacciArcsValue>,
}

impl FibonacciArcs {
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            minimum: f64::INFINITY,
            maximum: f64::NEG_INFINITY,
            count: 0,
            value: None,
        })
    }

    pub fn append(&mut self, high: f64, low: f64) -> Option<FibonacciArcsValue> {
        self.count += 1;
        self.maximum = self.maximum.max(high);
        self.minimum = self.minimum.min(low);
        self.value = (self.count >= 2).then(|| {
            let range = self.maximum - self.minimum;
            FibonacciArcsValue {
                radius_382: range * 0.382,
                radius_500: range * 0.5,
                radius_618: range * 0.618,
            }
        });
        self.value
    }

    pub fn value(&self) -> Option<FibonacciArcsValue> {
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
