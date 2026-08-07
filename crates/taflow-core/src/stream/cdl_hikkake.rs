//! Incremental Hikkake pattern recognition (CDLHIKKAKE).
use std::collections::VecDeque;
#[derive(Clone, Copy)]
struct Candle {
    high: f64,
    low: f64,
}
/// Incremental CDLHIKKAKE state.
pub struct CandleHikkake {
    candles: VecDeque<Candle>,
    index: usize,
    pending: Option<(usize, i32, f64)>,
    value: Option<i32>,
}
impl Default for CandleHikkake {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleHikkake {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(2),
            index: 0,
            pending: None,
            value: None,
        }
    }
    /// Appends OHLC data and returns a Hikkake setup/confirmation after warmup.
    pub fn append(&mut self, _open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let current = Candle { high, low };
        let i = self.index;
        self.index += 1;
        let mut result = 0;
        if self.candles.len() == 2 {
            let first = self.candles[0];
            let second = self.candles[1];
            if second.high < first.high
                && second.low > first.low
                && high < second.high
                && low < second.low
            {
                self.pending = Some((i, 100, second.high));
                result = 100;
            } else if second.high < first.high
                && second.low > first.low
                && high > second.high
                && low > second.low
            {
                self.pending = Some((i, -100, second.low));
                result = -100;
            } else if i >= 5 {
                if let Some((setup, direction, threshold)) = self.pending {
                    if i - setup <= 3
                        && ((direction > 0 && close > threshold)
                            || (direction < 0 && close < threshold))
                    {
                        result = direction + direction.signum() * 100;
                        self.pending = None;
                    } else if i - setup > 3 {
                        self.pending = None;
                    }
                }
            }
        }
        if self.candles.len() == 2 {
            self.candles.pop_front();
        }
        self.candles.push_back(current);
        self.value = (i >= 5).then_some(result);
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
        let open = vec![10.; 15];
        let high = vec![
            12., 11., 10., 11., 12., 13., 12., 11., 10., 11., 12., 13., 14., 15., 16.,
        ];
        let low = vec![
            8., 9., 8., 7., 6., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14.,
        ];
        let close = high.clone();
        let e = crate::pattern::cdl_hikkake(&open, &high, &low, &close).unwrap();
        let mut s = CandleHikkake::new();
        for (((&o, &h), &l), (&c, &e)) in open.iter().zip(&high).zip(&low).zip(close.iter().zip(&e))
        {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
