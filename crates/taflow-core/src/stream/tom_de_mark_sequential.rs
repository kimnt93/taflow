//! Stateful Tom DeMark Sequential setup counts.

use std::collections::VecDeque;

/// Causal four-bar comparison with capped buy and sell setup counts.
#[derive(Debug, Clone)]
pub struct TomDeMarkSequential {
    closes: VecDeque<f64>,
    buy: i32,
    sell: i32,
    value: Option<(i32, i32)>,
}

impl TomDeMarkSequential {
    /// Creates an empty setup-count state.
    pub fn new() -> Self { Self { closes: VecDeque::with_capacity(5), buy: 0, sell: 0, value: None } }

    /// Appends one close and returns buy and sell setup counts.
    pub fn append(&mut self, close: f64) -> (i32, i32) {
        self.closes.push_back(close);
        let result = if self.closes.len() <= 4 {
            (0, 0)
        } else {
            let comparison = self.closes.front().copied().unwrap();
            if close < comparison { self.buy = (self.buy + 1).min(9); self.sell = 0; }
            else if close > comparison { self.sell = (self.sell + 1).min(9); self.buy = 0; }
            else { self.buy = 0; self.sell = 0; }
            (self.buy, self.sell)
        };
        if self.closes.len() > 5 { self.closes.pop_front(); }
        self.value = Some(result); result
    }

    /// Returns the latest buy/sell setup counts.
    pub fn value(&self) -> Option<(i32, i32)> { self.value }
    /// Clears setup counts and close history.
    pub fn reset(&mut self) { self.closes.clear(); self.buy = 0; self.sell = 0; self.value = None; }
}
