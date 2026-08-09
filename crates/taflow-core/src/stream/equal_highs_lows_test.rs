use super::equal_highs_lows::EqualHighsLows;
use crate::stream::swing_highs_lows_test::{lcg_series, ReferenceSwing};
use crate::stream::AverageTrueRange;

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
