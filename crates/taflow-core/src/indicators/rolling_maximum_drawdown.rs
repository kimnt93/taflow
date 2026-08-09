//! Persistent rolling maximum drawdown state.

use crate::error::TaResult;
use crate::stream::operator_states::validate_period;
use crate::stream::StreamingIndicator;

#[derive(Debug, Clone, Copy)]
struct DrawdownSummary {
    maximum_positive: f64,
    minimum: f64,
    maximum_drawdown: f64,
    present: bool,
}

impl DrawdownSummary {
    const EMPTY: Self = Self {
        maximum_positive: f64::NEG_INFINITY,
        minimum: f64::INFINITY,
        maximum_drawdown: 0.0,
        present: false,
    };

    fn single(value: f64) -> Self {
        Self {
            maximum_positive: if value > 0.0 {
                value
            } else {
                f64::NEG_INFINITY
            },
            minimum: value,
            maximum_drawdown: 0.0,
            present: true,
        }
    }

    fn combine(left: Self, right: Self) -> Self {
        if !left.present {
            return right;
        }
        if !right.present {
            return left;
        }
        let cross_drawdown = if left.maximum_positive > 0.0 {
            ((left.maximum_positive - right.minimum) / left.maximum_positive).max(0.0)
        } else {
            0.0
        };
        Self {
            maximum_positive: left.maximum_positive.max(right.maximum_positive),
            minimum: left.minimum.min(right.minimum),
            maximum_drawdown: left
                .maximum_drawdown
                .max(right.maximum_drawdown)
                .max(cross_drawdown),
            present: true,
        }
    }
}

/// Deepest peak-to-trough decline in a fixed trailing window.
#[derive(Debug, Clone)]
pub struct RollingMaximumDrawdown {
    period: usize,
    tree_base: usize,
    tree: Vec<DrawdownSummary>,
    next_index: usize,
    finite_count: usize,
    value: Option<f64>,
}

impl RollingMaximumDrawdown {
    /// Create a state with a positive trailing-window period.
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        let tree_base = period.next_power_of_two();
        Ok(Self {
            period,
            tree_base,
            tree: vec![DrawdownSummary::EMPTY; tree_base * 2],
            next_index: 0,
            finite_count: 0,
            value: None,
        })
    }

    /// Append one equity sample and return the maximum drawdown once warm.
    ///
    /// Non-finite samples are ignored and return the previously available
    /// value, matching the Wickra `MaxDrawdown` streaming contract.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if !input.is_finite() {
            return self.value;
        }

        self.update_leaf(self.next_index, DrawdownSummary::single(input));
        self.next_index = (self.next_index + 1) % self.period;
        self.finite_count = self.finite_count.saturating_add(1);

        if self.finite_count >= self.period {
            let older = self.query(self.next_index, self.period);
            let newer = self.query(0, self.next_index);
            self.value = Some(DrawdownSummary::combine(older, newer).maximum_drawdown);
        }
        self.value
    }

    /// Return the latest maximum-drawdown fraction, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Return the configured trailing period.
    pub fn period(&self) -> usize {
        self.period
    }

    /// Reset to fresh-state behavior while retaining allocated storage.
    pub fn reset(&mut self) {
        self.tree.fill(DrawdownSummary::EMPTY);
        self.next_index = 0;
        self.finite_count = 0;
        self.value = None;
    }

    fn update_leaf(&mut self, index: usize, summary: DrawdownSummary) {
        let mut node = self.tree_base + index;
        self.tree[node] = summary;
        while node > 1 {
            node /= 2;
            self.tree[node] =
                DrawdownSummary::combine(self.tree[node * 2], self.tree[node * 2 + 1]);
        }
    }

    fn query(&self, start: usize, end: usize) -> DrawdownSummary {
        if start == end {
            return DrawdownSummary::EMPTY;
        }
        let mut left = start + self.tree_base;
        let mut right = end + self.tree_base;
        let mut left_summary = DrawdownSummary::EMPTY;
        let mut right_summary = DrawdownSummary::EMPTY;
        while left < right {
            if left % 2 == 1 {
                left_summary = DrawdownSummary::combine(left_summary, self.tree[left]);
                left += 1;
            }
            if right % 2 == 1 {
                right -= 1;
                right_summary = DrawdownSummary::combine(self.tree[right], right_summary);
            }
            left /= 2;
            right /= 2;
        }
        DrawdownSummary::combine(left_summary, right_summary)
    }
}

impl StreamingIndicator for RollingMaximumDrawdown {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        Self::append(self, input)
    }

    fn value(&self) -> Option<f64> {
        Self::value(self)
    }

    fn reset(&mut self) {
        Self::reset(self);
    }
}
