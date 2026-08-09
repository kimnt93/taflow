use super::swing_high_low::SwingHighLow;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub(crate) struct ReferenceSwing {
    highs: VecDeque<f64>,
    lows: VecDeque<f64>,
    length: usize,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ReferenceSwingValue {
    pub signal: f64,
    pub level: f64,
    pub bars_since: f64,
}

impl ReferenceSwing {
    pub(crate) fn new(length: usize) -> Self {
        Self {
            highs: VecDeque::new(),
            lows: VecDeque::new(),
            length,
        }
    }

    pub(crate) fn append(&mut self, high: f64, low: f64) -> Option<ReferenceSwingValue> {
        let capacity = self.length * 2 + 1;
        if self.highs.len() == capacity {
            self.highs.pop_front();
            self.lows.pop_front();
        }
        self.highs.push_back(high);
        self.lows.push_back(low);
        if self.highs.len() < capacity {
            return None;
        }
        let center_high = self.highs[self.length];
        let center_low = self.lows[self.length];
        let is_high = center_high >= self.highs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let is_low = center_low <= self.lows.iter().copied().fold(f64::INFINITY, f64::min);
        let (signal, level) = match (is_high, is_low) {
            (true, false) => (1.0, center_high),
            (false, true) => (-1.0, center_low),
            _ => (f64::NAN, f64::NAN),
        };
        Some(ReferenceSwingValue {
            signal,
            level,
            bars_since: f64::NAN,
        })
    }
}

pub(crate) fn lcg_series(len: usize, mut seed: u64) -> Vec<f64> {
    (0..len)
        .map(|_| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            90.0 + ((seed >> 11) as f64 / (1u64 << 53) as f64) * 20.0
        })
        .collect()
}

#[test]
fn lifecycle_and_reset_are_consistent() {
    let mut state = SwingHighLow::new(2).unwrap();
    assert!(state.append(10.0, 8.0).is_none());
    assert!(state.append(11.0, 7.0).is_none());
    assert!(state.append(12.0, 6.0).is_none());
    assert!(state.append(11.0, 7.0).is_none());
    assert!(state.append(10.0, 8.0).is_some());
    state.reset();
    assert!(state.value().is_none());
}
