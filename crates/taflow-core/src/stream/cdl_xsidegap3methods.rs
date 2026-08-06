//! Incremental Upside/Downside Gap Three Methods recognition (CDLXSIDEGAP3METHODS).
use std::collections::VecDeque;
#[derive(Clone, Copy)]
struct Candle {
    open: f64,
    close: f64,
}
/// Incremental CDLXSIDEGAP3METHODS state.
pub struct CdlXSideGap3Methods {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CdlXSideGap3Methods {
    fn default() -> Self {
        Self::new()
    }
}
impl CdlXSideGap3Methods {
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(2),
            value: None,
        }
    }
    pub fn append(&mut self, open: f64, _high: f64, _low: f64, close: f64) -> Option<i32> {
        let output = if self.candles.len() == 2 {
            let first = self.candles[0];
            let second = self.candles[1];
            let first_color = if first.close >= first.open { 1 } else { -1 };
            let second_color = if second.close >= second.open { 1 } else { -1 };
            let current_color = if close >= open { 1 } else { -1 };
            let base = first_color == second_color
                && current_color != first_color
                && open > second.open.min(second.close)
                && open < second.open.max(second.close)
                && close > first.open.min(first.close)
                && close < first.open.max(first.close);
            let bull = base
                && first_color == 1
                && second.open.min(second.close) > first.open.max(first.close);
            let bear = base
                && first_color == -1
                && second.open.max(second.close) < first.open.min(first.close);
            Some((bull as i32) * 100 - (bear as i32) * 100)
        } else {
            None
        };
        if self.candles.len() == 2 {
            self.candles.pop_front();
        }
        self.candles.push_back(Candle { open, close });
        self.value = output;
        output
    }
    pub fn value(&self) -> Option<i32> {
        self.value
    }
    pub fn reset(&mut self) {
        *self = Self::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_batch() {
        let open = vec![10., 13., 13.5, 10., 7., 7.5];
        let high = vec![14.; 6];
        let low = vec![6.; 6];
        let close = vec![12., 14., 11., 8., 6., 9.];
        let e = crate::pattern::cdl_xsidegap3methods(&open, &high, &low, &close).unwrap();
        let mut s = CdlXSideGap3Methods::new();
        for (((&o, &h), &l), (&c, &e)) in open.iter().zip(&high).zip(&low).zip(close.iter().zip(&e))
        {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
