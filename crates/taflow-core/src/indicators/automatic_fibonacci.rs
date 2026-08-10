use crate::error::TaResult;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutomaticFibonacciValue {
    pub level_000: f64,
    pub level_236: f64,
    pub level_382: f64,
    pub level_500: f64,
    pub level_618: f64,
    pub level_786: f64,
    pub level_100: f64,
}

#[derive(Debug, Clone)]
pub struct AutomaticFibonacci {
    minimum: f64,
    maximum: f64,
    count: usize,
    value: Option<AutomaticFibonacciValue>,
}

impl AutomaticFibonacci {
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            minimum: f64::INFINITY,
            maximum: f64::NEG_INFINITY,
            count: 0,
            value: None,
        })
    }

    pub fn append(&mut self, high: f64, low: f64) -> Option<AutomaticFibonacciValue> {
        self.count += 1;
        self.maximum = self.maximum.max(high);
        self.minimum = self.minimum.min(low);
        self.value = (self.count >= 2).then(|| {
            let range = self.maximum - self.minimum;
            AutomaticFibonacciValue {
                level_000: self.minimum,
                level_236: self.minimum + range * 0.236,
                level_382: self.minimum + range * 0.382,
                level_500: self.minimum + range * 0.5,
                level_618: self.minimum + range * 0.618,
                level_786: self.minimum + range * 0.786,
                level_100: self.maximum,
            }
        });
        self.value
    }

    pub fn value(&self) -> Option<AutomaticFibonacciValue> {
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
