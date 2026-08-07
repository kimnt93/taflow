use std::collections::VecDeque;
/// Stateful CandleShortLine candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleShortLine {
    b: VecDeque<f64>,
    s: VecDeque<f64>,
    bs: f64,
    ss: f64,
    value: Option<i32>,
}
impl CandleShortLine {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            b: VecDeque::with_capacity(10),
            s: VecDeque::with_capacity(10),
            bs: 0.,
            ss: 0.,
            value: None,
        }
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, o: f64, h: f64, l: f64, c: f64) -> Option<i32> {
        let body = (c - o).abs();
        let sh = (h - o.max(c)) + (o.min(c) - l);
        let v = if self.b.len() == 10 {
            Some(
                (body < self.bs / 10.
                    && h - o.max(c) < self.ss / 10.
                    && o.min(c) - l < self.ss / 10.) as i32
                    * if c >= o { 100 } else { -100 },
            )
        } else {
            None
        };
        if self.b.len() == 10 {
            self.bs -= self.b.pop_front().unwrap();
            self.ss -= self.s.pop_front().unwrap();
        }
        self.b.push_back(body);
        self.s.push_back(sh);
        self.bs += body;
        self.ss += sh;
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
