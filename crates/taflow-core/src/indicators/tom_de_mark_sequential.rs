//! Stateful Tom DeMark Sequential setup counts.

use std::collections::VecDeque;

/// Buy and sell setup counts for one bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TomDeMarkSequentialValue {
    pub buy: i32,
    pub sell: i32,
}

/// Causal four-bar comparison with capped buy and sell setup counts.
#[derive(Debug, Clone)]
pub struct TomDeMarkSequential {
    closes: VecDeque<f64>,
    buy: i32,
    sell: i32,
    output: TomDeMarkSequentialValue,
    value: Option<TomDeMarkSequentialValue>,
}

impl TomDeMarkSequential {
    /// Creates an empty setup-count state.
    pub fn new() -> Self {
        Self {
            closes: VecDeque::with_capacity(5),
            buy: 0,
            sell: 0,
            output: TomDeMarkSequentialValue { buy: 0, sell: 0 },
            value: None,
        }
    }

    /// Appends one close and returns buy and sell setup counts.
    pub fn append(&mut self, close: f64) -> Option<TomDeMarkSequentialValue> {
        self.closes.push_back(close);
        if self.closes.len() > 5 {
            self.closes.pop_front();
        }
        let result = if self.closes.len() <= 4 {
            None
        } else {
            let comparison = self.closes.front().copied().unwrap();
            if close < comparison {
                self.buy = (self.buy + 1).min(9);
                self.sell = 0;
            } else if close > comparison {
                self.sell = (self.sell + 1).min(9);
                self.buy = 0;
            } else {
                self.buy = 0;
                self.sell = 0;
            }
            Some(TomDeMarkSequentialValue {
                buy: self.buy,
                sell: self.sell,
            })
        };
        self.output = result.unwrap_or(TomDeMarkSequentialValue { buy: 0, sell: 0 });
        self.value = result;
        result
    }

    /// Returns aligned buy/sell outputs for the latest bar.
    pub fn outputs(&self) -> TomDeMarkSequentialValue {
        self.output
    }

    /// Extends close history through the scalar state machine.
    pub fn extend_slice_into(&mut self, close: &[f64], buy: &mut Vec<i32>, sell: &mut Vec<i32>) {
        for &close in close {
            self.append(close);
            let output = self.outputs();
            buy.push(output.buy);
            sell.push(output.sell);
        }
    }

    /// Returns the latest buy/sell setup counts.
    pub fn value(&self) -> Option<TomDeMarkSequentialValue> {
        self.value
    }
    /// Clears setup counts and close history.
    pub fn reset(&mut self) {
        self.closes.clear();
        self.buy = 0;
        self.sell = 0;
        self.output = TomDeMarkSequentialValue { buy: 0, sell: 0 };
        self.value = None;
    }
}
