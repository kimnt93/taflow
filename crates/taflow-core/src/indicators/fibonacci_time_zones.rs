use crate::error::TaResult;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FibonacciTimeZonesValue {
    pub current_zone: f64,
    pub next_zone: f64,
}

#[derive(Debug, Clone)]
pub struct FibonacciTimeZones {
    anchor: usize,
    previous_high: Option<f64>,
    previous_low: Option<f64>,
    count: usize,
    value: Option<FibonacciTimeZonesValue>,
}

impl FibonacciTimeZones {
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            anchor: 0,
            previous_high: None,
            previous_low: None,
            count: 0,
            value: None,
        })
    }

    pub fn append(&mut self, high: f64, low: f64) -> Option<FibonacciTimeZonesValue> {
        if self.previous_high.is_some_and(|value| high > value)
            || self.previous_low.is_some_and(|value| low < value)
        {
            self.anchor = self.count;
        }
        self.previous_high = Some(high);
        self.previous_low = Some(low);
        self.count += 1;
        self.value = (self.count >= 2).then(|| {
            let elapsed = self.count - 1 - self.anchor;
            let mut previous = 1usize;
            let mut next = 2usize;
            while next <= elapsed {
                let following = previous + next;
                previous = next;
                next = following.max(1);
            }
            FibonacciTimeZonesValue {
                current_zone: (self.anchor + previous) as f64,
                next_zone: (self.anchor + next) as f64,
            }
        });
        self.value
    }

    pub fn value(&self) -> Option<FibonacciTimeZonesValue> {
        self.value
    }
    pub fn len(&self) -> usize {
        self.count
    }
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    pub fn reset(&mut self) {
        self.anchor = 0;
        self.previous_high = None;
        self.previous_low = None;
        self.count = 0;
        self.value = None;
    }
}
