//! Incremental Belt Hold candlestick recognition (CDLBELTHOLD).
use std::collections::VecDeque;
/// Stateful CandleBeltHold candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleBeltHold {
    b: VecDeque<f64>,
    r: VecDeque<f64>,
    bs: f64,
    rs: f64,
    value: Option<i32>,
}
impl Default for CandleBeltHold {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleBeltHold {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            b: VecDeque::with_capacity(10),
            r: VecDeque::with_capacity(10),
            bs: 0.,
            rs: 0.,
            value: None,
        }
    }
    fn push(q: &mut VecDeque<f64>, s: &mut f64, v: f64) {
        if q.len() == 10 {
            *s -= q.pop_front().unwrap();
        }
        q.push_back(v);
        *s += v;
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, o: f64, h: f64, l: f64, c: f64) -> Option<i32> {
        let body = (c - o).abs();
        let range = h - l;
        let upper = h - o.max(c);
        let lower = o.min(c) - l;
        let v = if self.b.len() == 10 {
            let long = body > self.bs / 10.0;
            let lim = self.rs * 0.01;
            Some(if long && c >= o && lower < lim {
                100
            } else if long && c < o && upper < lim {
                -100
            } else {
                0
            })
        } else {
            None
        };
        Self::push(&mut self.b, &mut self.bs, body);
        Self::push(&mut self.r, &mut self.rs, range);
        self.value = v;
        v
    }
    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<i32> {
        self.value
    }
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_batch() {
        let o: Vec<f64> = (0..45).map(|i| 100.0 + i as f64 * 0.2).collect();
        let h: Vec<f64> = o.iter().map(|x| x + 2.0).collect();
        let l: Vec<f64> = o.iter().map(|x| x - 2.0).collect();
        let c: Vec<f64> = o
            .iter()
            .enumerate()
            .map(|(i, x)| x + if i % 3 == 0 { -1.0 } else { 1.0 })
            .collect();
        let e = crate::pattern::cdl_belthold(&o, &h, &l, &c).unwrap();
        let mut s = CandleBeltHold::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            assert_eq!(s.append(o, h, l, c).unwrap_or(0), e)
        }
    }
}
