use super::pattern_swing::{approximately_equal, SwingTracker, LEVEL_TOLERANCE, SWING_THRESHOLD};
use crate::error::TaResult;

/// Price and strength of the densest recent Fibonacci level cluster.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FibonacciConfluenceValue {
    pub price: f64,
    pub strength: f64,
}

/// Allocation-free Fibonacci confluence search across at most five swing legs.
#[derive(Debug, Clone)]
pub struct FibonacciConfluence {
    swing: SwingTracker,
    count: usize,
    value: Option<FibonacciConfluenceValue>,
}

impl FibonacciConfluence {
    /// Create an empty confluence tracker.
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            swing: SwingTracker::new(SWING_THRESHOLD, 6),
            count: 0,
            value: None,
        })
    }

    /// Append one high/low bar and search the bounded retracement set.
    pub fn append(&mut self, high: f64, low: f64) -> Option<FibonacciConfluenceValue> {
        self.count += 1;
        self.swing.append(high, low);
        let pivots = self.swing.pivots();
        if pivots.len() < 3 {
            self.value = None;
            return None;
        }

        // Six pivots produce at most five legs and three retracements per leg.
        let mut levels = [0.0; 15];
        let mut used = 0;
        for leg in pivots.windows(2) {
            for ratio in [0.382, 0.500, 0.618] {
                levels[used] = leg[1].price + ratio * (leg[0].price - leg[1].price);
                used += 1;
            }
        }

        let (mut best_count, mut best_total) = (0_usize, 0.0);
        for center in levels[..used].iter().copied() {
            let (mut cluster_count, mut cluster_total) = (0_usize, 0.0);
            for level in levels[..used].iter().copied() {
                if approximately_equal(level, center, LEVEL_TOLERANCE) {
                    cluster_count += 1;
                    cluster_total += level;
                }
            }
            // Wickra's max-by selection keeps the later center for equal counts.
            if cluster_count >= best_count {
                best_count = cluster_count;
                best_total = cluster_total;
            }
        }
        self.value = Some(FibonacciConfluenceValue {
            price: best_total / best_count as f64,
            strength: best_count as f64,
        });
        self.value
    }

    /// Return the latest confluence cluster.
    pub fn value(&self) -> Option<FibonacciConfluenceValue> {
        self.value
    }
    /// Return the processed-bar count.
    pub fn len(&self) -> usize {
        self.count
    }
    /// Return whether no bars were processed.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    /// Clear pivots and output.
    pub fn reset(&mut self) {
        self.swing.reset();
        self.count = 0;
        self.value = None;
    }
}
