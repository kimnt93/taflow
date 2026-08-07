//! Incremental Modified Hikkake recognition (CDLHIKKAKEMOD).
use std::collections::VecDeque;
#[derive(Clone, Copy)]
struct Candle {
    high: f64,
    low: f64,
    close: f64,
}
impl Candle {
    fn range(self) -> f64 {
        self.high - self.low
    }
}
/// Incremental CDLHIKKAKEMOD state.
pub struct CandleHikkakeModified {
    candles: VecDeque<Candle>,
    index: usize,
    pending: Option<(usize, i32, f64)>,
    value: Option<i32>,
}
impl Default for CandleHikkakeModified {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleHikkakeModified {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(8),
            index: 0,
            pending: None,
            value: None,
        }
    }
    fn near(&self) -> f64 {
        self.candles
            .iter()
            .skip(1)
            .take(5)
            .map(|c| c.range())
            .sum::<f64>()
            * 0.04
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, _open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let current = Candle { high, low, close };
        let i = self.index;
        self.index += 1;
        let mut result = 0;
        if i >= 10 && self.candles.len() == 8 {
            let a = self.candles[5];
            let b = self.candles[6];
            let c = self.candles[7];
            if c.high < b.high && c.low > b.low && b.high < a.high && b.low > a.low {
                let near = self.near();
                if high < c.high && low < c.low && b.close <= b.low + near {
                    self.pending = Some((i, 100, c.high));
                } else if high > c.high && low > c.low && b.close >= b.high - near {
                    self.pending = Some((i, -100, c.low));
                }
            }
            if let Some((setup, direction, threshold)) = self.pending {
                if i <= setup + 3
                    && ((direction > 0 && close > threshold)
                        || (direction < 0 && close < threshold))
                {
                    result = direction + direction.signum() * 100;
                    self.pending = None;
                } else if i > setup + 3 {
                    self.pending = None;
                }
            }
        }
        if self.candles.len() == 8 {
            self.candles.pop_front();
        }
        self.candles.push_back(current);
        self.value = (i >= 10).then_some(result);
        self.value
    }
    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<i32> {
        self.value
    }
    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        *self = Self::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_batch() {
        let open = vec![10.; 20];
        let high: Vec<f64> = (0..20).map(|i| 20. - i as f64 * 0.2).collect();
        let low: Vec<f64> = (0..20).map(|i| i as f64 * 0.1).collect();
        let close = high.clone();
        let e = crate::pattern::cdl_hikkakemod(&open, &high, &low, &close).unwrap();
        let mut s = CandleHikkakeModified::new();
        for (((&o, &h), &l), (&c, &e)) in open.iter().zip(&high).zip(&low).zip(close.iter().zip(&e))
        {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
