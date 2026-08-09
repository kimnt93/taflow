//! Persistent premium/discount zones relative to a rolling midpoint.

use crate::error::TaResult;

#[derive(Debug, Clone)]
struct Staircase {
    buf: Box<[(usize, f64)]>,
    head: usize,
    len: usize,
    index: usize,
    period: usize,
    maximum: bool,
}

impl Staircase {
    fn new(period: usize, maximum: bool) -> Self {
        Self {
            buf: vec![(0usize, 0.0f64); period].into_boxed_slice(),
            head: 0,
            len: 0,
            index: 0,
            period,
            maximum,
        }
    }

    #[inline]
    fn entry(&self, offset: usize) -> (usize, f64) {
        let capacity = self.buf.len();
        let mut slot = self.head + offset;
        if slot >= capacity {
            slot -= capacity;
        }
        self.buf[slot]
    }

    fn append(&mut self, value: f64) -> f64 {
        let capacity = self.buf.len();
        let index = self.index;
        self.index += 1;
        while self.len > 0 {
            let (_, back) = self.entry(self.len - 1);
            let dominated = if self.maximum {
                back <= value
            } else {
                back >= value
            };
            if !dominated {
                break;
            }
            self.len -= 1;
        }
        let first_valid = index.saturating_add(1).saturating_sub(self.period);
        while self.len > 0 && self.entry(0).0 < first_valid {
            self.head += 1;
            if self.head == capacity {
                self.head = 0;
            }
            self.len -= 1;
        }
        let mut tail = self.head + self.len;
        if tail >= capacity {
            tail -= capacity;
        }
        self.buf[tail] = (index, value);
        self.len += 1;
        self.entry(0).1
    }

    fn reset(&mut self) {
        self.head = 0;
        self.len = 0;
        self.index = 0;
    }
}

/// Rolling midpoint and signed premium/discount zone.
#[derive(Debug, Clone)]
pub struct PremiumDiscount {
    highs: Staircase,
    lows: Staircase,
    value: Option<PremiumDiscountValue>,
}

/// Premium/discount zone and equilibrium for one bar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PremiumDiscountValue {
    pub zone: i32,
    pub equilibrium: f64,
}

impl PremiumDiscount {
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 1 {
            return Err(crate::indicators::invalid_period("window", period, 1));
        }
        Ok(Self {
            highs: Staircase::new(period, true),
            lows: Staircase::new(period, false),
            value: None,
        })
    }

    pub fn append(&mut self, close: f64) -> PremiumDiscountValue {
        let high = self.highs.append(close);
        let low = self.lows.append(close);
        let equilibrium = (high + low) / 2.0;
        let zone = if close > equilibrium {
            1
        } else if close < equilibrium {
            -1
        } else {
            0
        };
        let value = PremiumDiscountValue { zone, equilibrium };
        self.value = Some(value);
        value
    }

    pub fn extend_slice_into(
        &mut self,
        close: &[f64],
        zones: &mut Vec<i32>,
        equilibrium: &mut Vec<f64>,
    ) {
        for &close in close {
            let value = self.append(close);
            zones.push(value.zone);
            equilibrium.push(value.equilibrium);
        }
    }

    pub fn value(&self) -> Option<PremiumDiscountValue> {
        self.value
    }

    pub fn reset(&mut self) {
        self.highs.reset();
        self.lows.reset();
        self.value = None;
    }
}
