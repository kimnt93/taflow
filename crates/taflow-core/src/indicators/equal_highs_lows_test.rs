use super::equal_highs_lows::EqualHighsLows;
use crate::indicators::AverageTrueRange;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct ReferenceSwing {
    highs: VecDeque<f64>,
    lows: VecDeque<f64>,
    length: usize,
}

#[derive(Debug, Clone, Copy)]
struct ReferenceSwingValue {
    signal: f64,
    level: f64,
}

impl ReferenceSwing {
    fn new(length: usize) -> Self {
        Self {
            highs: VecDeque::new(),
            lows: VecDeque::new(),
            length,
        }
    }

    fn append(&mut self, high: f64, low: f64) -> Option<ReferenceSwingValue> {
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
        Some(ReferenceSwingValue { signal, level })
    }
}

fn lcg_series(len: usize, mut seed: u64) -> Vec<f64> {
    (0..len)
        .map(|_| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            90.0 + ((seed >> 11) as f64 / (1u64 << 53) as f64) * 20.0
        })
        .collect()
}

struct Reference {
    atr: AverageTrueRange,
    swing: ReferenceSwing,
    previous_high: Option<f64>,
    previous_low: Option<f64>,
    eq_threshold: f64,
}

impl Reference {
    fn new(eq_len: usize, atr_period: usize, eq_threshold: f64) -> Self {
        Self {
            atr: AverageTrueRange::new(atr_period).unwrap(),
            swing: ReferenceSwing::new(eq_len),
            previous_high: None,
            previous_low: None,
            eq_threshold,
        }
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> [f64; 3] {
        let atr = self.atr.append(high, low, close);
        let mut eqh = f64::NAN;
        let mut eql = f64::NAN;
        let mut level = f64::NAN;
        if let Some(swing) = self.swing.append(high, low) {
            if swing.signal > 0.0 {
                if let (Some(previous), Some(atr)) = (self.previous_high, atr) {
                    if (swing.level - previous).abs() < atr * self.eq_threshold {
                        eqh = 1.0;
                        level = swing.level;
                    }
                }
                self.previous_high = Some(swing.level);
            } else if swing.signal < 0.0 {
                if let (Some(previous), Some(atr)) = (self.previous_low, atr) {
                    if (swing.level - previous).abs() < atr * self.eq_threshold {
                        eql = 1.0;
                        level = swing.level;
                    }
                }
                self.previous_low = Some(swing.level);
            }
        }
        [eqh, eql, level]
    }
}

#[test]
fn matches_reference_bitwise_and_survives_reset() {
    let base = lcg_series(5_000, 0x11_5EED_D1);
    let high: Vec<f64> = base.iter().map(|v| v + 0.7).collect();
    let low: Vec<f64> = base.iter().map(|v| v - 0.7).collect();
    for (eq_len, atr_period, threshold) in [(1usize, 14usize, 0.1), (5, 14, 0.5), (20, 30, 1.0)] {
        let mut reference = Reference::new(eq_len, atr_period, threshold);
        let mut state = EqualHighsLows::new(eq_len, atr_period, threshold).unwrap();
        for i in 0..base.len() {
            let want = reference.append(high[i], low[i], base[i]);
            let got = state.append(high[i], low[i], base[i]);
            let got = [got.eqh, got.eql, got.level];
            for (k, (w, g)) in want.iter().zip(&got).enumerate() {
                assert_eq!(w.to_bits(), g.to_bits(), "l={eq_len} bar {i} output {k}");
            }
        }
        state.reset();
        let mut fresh = Reference::new(eq_len, atr_period, threshold);
        for i in 0..512 {
            let want = fresh.append(high[i], low[i], base[i]);
            let got = state.append(high[i], low[i], base[i]);
            let got = [got.eqh, got.eql, got.level];
            for (w, g) in want.iter().zip(&got) {
                assert_eq!(w.to_bits(), g.to_bits(), "post-reset l={eq_len}");
            }
        }
    }
}
