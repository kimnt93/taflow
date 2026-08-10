use crate::error::TaResult;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FibonacciChannelValue {
    pub lower: f64,
    pub retracement_382: f64,
    pub retracement_618: f64,
    pub upper: f64,
}

#[derive(Debug, Clone)]
pub struct FibonacciChannel {
    minimum: f64,
    maximum: f64,
    count: usize,
    value: Option<FibonacciChannelValue>,
}

impl FibonacciChannel {
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            minimum: f64::INFINITY,
            maximum: f64::NEG_INFINITY,
            count: 0,
            value: None,
        })
    }

    pub fn append(&mut self, high: f64, low: f64) -> Option<FibonacciChannelValue> {
        self.count += 1;
        self.maximum = self.maximum.max(high);
        self.minimum = self.minimum.min(low);
        self.value = (self.count >= 3).then(|| {
            let range = self.maximum - self.minimum;
            FibonacciChannelValue {
                lower: self.minimum,
                retracement_382: self.minimum + range * 0.382,
                retracement_618: self.minimum + range * 0.618,
                upper: self.maximum,
            }
        });
        self.value
    }

    pub fn value(&self) -> Option<FibonacciChannelValue> {
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
