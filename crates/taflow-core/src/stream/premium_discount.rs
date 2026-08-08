//! Stateful premium/discount zones relative to a rolling midpoint.

use crate::error::TaResult;

/// Monotonic staircase over the last `period` closes, backed by a fixed ring.
///
/// Unlike the shared `MonotonicMax`/`MonotonicMin` states this one reports the
/// extremum of a *partial* window too, which is what premium/discount needs:
/// the indicator emits from bar 0. The deque front is always the extremum of
/// everything currently buffered, so warm-up bars match the historical
/// fold-over-the-whole-deque behaviour exactly.
#[derive(Debug, Clone)]
struct Staircase {
    buf: Box<[(usize, f64)]>,
    head: usize,
    len: usize,
    index: usize,
    period: usize,
    maximum: bool,
}

impl Staircase {
    fn new(period: usize, maximum: bool) -> Self {
        Self {
            buf: vec![(0usize, 0.0f64); period].into_boxed_slice(),
            head: 0,
            len: 0,
            index: 0,
            period,
            maximum,
        }
    }

    #[inline]
    fn entry(&self, offset: usize) -> (usize, f64) {
        let capacity = self.buf.len();
        let mut slot = self.head + offset;
        if slot >= capacity {
            slot -= capacity;
        }
        self.buf[slot]
    }

    /// Pushes one observation and returns the extremum of the live window.
    fn append(&mut self, value: f64) -> f64 {
        let capacity = self.buf.len();
        let index = self.index;
        self.index += 1;
        while self.len > 0 {
            let (_, back) = self.entry(self.len - 1);
            let dominated = if self.maximum {
                back <= value
            } else {
                back >= value
            };
            if !dominated {
                break;
            }
            self.len -= 1;
        }
        // Drop entries that have aged out *before* inserting, so the live
        // entries (all with distinct indices inside the window) never exceed
        // the ring's `period` slots.
        let first_valid = index.saturating_add(1).saturating_sub(self.period);
        while self.len > 0 && self.entry(0).0 < first_valid {
            self.head += 1;
            if self.head == capacity {
                self.head = 0;
            }
            self.len -= 1;
        }
        let mut tail = self.head + self.len;
        if tail >= capacity {
            tail -= capacity;
        }
        self.buf[tail] = (index, value);
        self.len += 1;
        self.entry(0).1
    }

    fn reset(&mut self) {
        self.head = 0;
        self.len = 0;
        self.index = 0;
    }
}

/// Rolling midpoint and signed premium/discount zone.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `PremiumDiscount`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct PremiumDiscount {
    highs: Staircase,
    lows: Staircase,
    value: Option<(i32, f64)>,
}

impl PremiumDiscount {
    /// Creates the indicator with a positive rolling window.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 1 {
            return Err(super::invalid_period("window", period, 1));
        }
        Ok(Self {
            highs: Staircase::new(period, true),
            lows: Staircase::new(period, false),
            value: None,
        })
    }

    /// Appends one close and returns `(zone, equilibrium)`.
    ///
    /// M1: the two O(period) rescans of a close `VecDeque` become
    /// amortized-O(1) monotonic staircases over fixed rings. Extrema are
    /// comparison-only, so the equilibrium and zone are bit-identical to the
    /// rescan implementation, warm-up bars included.
    pub fn append(&mut self, close: f64) -> (i32, f64) {
        let high = self.highs.append(close);
        let low = self.lows.append(close);
        let equilibrium = (high + low) / 2.0;
        let zone = if close > equilibrium {
            1
        } else if close < equilibrium {
            -1
        } else {
            0
        };
        self.value = Some((zone, equilibrium));
        (zone, equilibrium)
    }

    /// Returns the latest zone and equilibrium.
    pub fn value(&self) -> Option<(i32, f64)> {
        self.value
    }

    /// Clears all rolling history.
    pub fn reset(&mut self) {
        self.highs.reset();
        self.lows.reset();
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// Pre-optimization `PremiumDiscount::append` oracle (deque rescans).
    struct Reference {
        period: usize,
        closes: VecDeque<f64>,
    }

    impl Reference {
        fn new(period: usize) -> Self {
            Self {
                period,
                closes: VecDeque::with_capacity(period),
            }
        }

        fn append(&mut self, close: f64) -> (i32, f64) {
            self.closes.push_back(close);
            if self.closes.len() > self.period {
                self.closes.pop_front();
            }
            let low = self.closes.iter().copied().fold(f64::INFINITY, f64::min);
            let high = self
                .closes
                .iter()
                .copied()
                .fold(f64::NEG_INFINITY, f64::max);
            let equilibrium = (high + low) / 2.0;
            let zone = if close > equilibrium {
                1
            } else if close < equilibrium {
                -1
            } else {
                0
            };
            (zone, equilibrium)
        }
    }

    fn lcg_series(len: usize, seed: u64) -> Vec<f64> {
        let mut state = seed;
        (0..len)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                90.0 + ((state >> 11) as f64 / (1u64 << 53) as f64) * 20.0
            })
            .collect()
    }

    #[test]
    fn matches_reference_bitwise_and_survives_chunking() {
        let mut closes = lcg_series(5_000, 0xD1_5EED_A1);
        // A constant stretch exercises the zone == 0 tie branch.
        for slot in closes[2_000..2_050].iter_mut() {
            *slot = 100.0;
        }
        for period in [1usize, 2, 5, 20, 200] {
            let mut reference = Reference::new(period);
            let mut state = PremiumDiscount::new(period).unwrap();
            for (i, &close) in closes.iter().enumerate() {
                let (want_zone, want_eq) = reference.append(close);
                let (got_zone, got_eq) = state.append(close);
                assert_eq!(want_zone, got_zone, "p={period} bar {i}");
                assert_eq!(want_eq.to_bits(), got_eq.to_bits(), "p={period} bar {i}");
            }
            state.reset();
            let mut fresh = Reference::new(period);
            for &close in closes.iter().take(512) {
                let (want_zone, want_eq) = fresh.append(close);
                let (got_zone, got_eq) = state.append(close);
                assert_eq!(want_zone, got_zone, "p={period} post-reset");
                assert_eq!(want_eq.to_bits(), got_eq.to_bits(), "p={period} post-reset");
            }
        }
    }
}
