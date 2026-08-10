/// Fractional reversal required to confirm a chart-pattern pivot.
pub(crate) const SWING_THRESHOLD: f64 = 0.05;

/// Relative tolerance used when two support or resistance levels must match.
pub(crate) const LEVEL_TOLERANCE: f64 = 0.03;

/// One confirmed, non-repainting swing pivot.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Pivot {
    pub(crate) price: f64,
    pub(crate) direction: f64,
}

#[derive(Debug, Clone, Copy)]
struct RunningSwing {
    direction: f64,
    extreme: f64,
}

/// Bounded swing history shared by geometric chart-pattern detectors.
#[derive(Debug, Clone)]
pub(crate) struct SwingTracker {
    threshold: f64,
    capacity: usize,
    state: Option<RunningSwing>,
    pivots: Vec<Pivot>,
}

impl SwingTracker {
    /// Construct with storage allocated once so updates do not allocate.
    pub(crate) fn new(threshold: f64, capacity: usize) -> Self {
        Self {
            threshold,
            capacity,
            state: None,
            pivots: Vec::with_capacity(capacity),
        }
    }

    /// Append one high/low bar and report whether a pivot was confirmed.
    pub(crate) fn append(&mut self, high: f64, low: f64) -> bool {
        let Some(state) = self.state else {
            self.state = Some(RunningSwing {
                direction: 1.0,
                extreme: high,
            });
            return false;
        };

        if state.direction > 0.0 {
            if high > state.extreme {
                self.state = Some(RunningSwing {
                    direction: 1.0,
                    extreme: high,
                });
                return false;
            }
            if low <= state.extreme * (1.0 - self.threshold) {
                self.push(Pivot {
                    price: state.extreme,
                    direction: 1.0,
                });
                self.state = Some(RunningSwing {
                    direction: -1.0,
                    extreme: low,
                });
                return true;
            }
            false
        } else {
            if low < state.extreme {
                self.state = Some(RunningSwing {
                    direction: -1.0,
                    extreme: low,
                });
                return false;
            }
            if high >= state.extreme * (1.0 + self.threshold) {
                self.push(Pivot {
                    price: state.extreme,
                    direction: -1.0,
                });
                self.state = Some(RunningSwing {
                    direction: 1.0,
                    extreme: high,
                });
                return true;
            }
            false
        }
    }

    pub(crate) fn pivots(&self) -> &[Pivot] {
        &self.pivots
    }

    pub(crate) fn reset(&mut self) {
        self.state = None;
        self.pivots.clear();
    }

    fn push(&mut self, pivot: Pivot) {
        if self.pivots.len() == self.capacity {
            self.pivots.remove(0);
        }
        self.pivots.push(pivot);
    }
}

/// Return the last two highs and lows from four alternating pivots.
pub(crate) fn recent_legs(pivots: &[Pivot]) -> (f64, f64, f64, f64) {
    let length = pivots.len();
    if pivots[length - 1].direction > 0.0 {
        (
            pivots[length - 3].price,
            pivots[length - 1].price,
            pivots[length - 4].price,
            pivots[length - 2].price,
        )
    } else {
        (
            pivots[length - 4].price,
            pivots[length - 2].price,
            pivots[length - 3].price,
            pivots[length - 1].price,
        )
    }
}

/// Compare levels using a scale-relative tolerance.
pub(crate) fn approximately_equal(left: f64, right: f64, tolerance: f64) -> bool {
    let scale = left.abs().max(right.abs()).max(f64::MIN_POSITIVE);
    (left - right).abs() <= tolerance * scale
}
