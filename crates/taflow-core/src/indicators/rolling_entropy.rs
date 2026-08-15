use crate::error::TaResult;
use crate::stream::operator_states::validate_period;
use std::collections::HashMap;

#[derive(Debug, Clone)]
/// Rolling Shannon entropy over exact value frequencies.
pub struct RollingEntropy {
    ring: Box<[f64]>,
    head: usize,
    len: usize,
    counts: HashMap<u64, u32>,
    count_log_count: Box<[f64]>,
    weighted_log_sum: f64,
    weighted_log_compensation: f64,
    nan_count: usize,
    period: usize,
    value: Option<f64>,
}

impl RollingEntropy {
    #[inline]
    fn count_key(value: f64) -> u64 {
        if value == 0.0 {
            0.0f64.to_bits()
        } else {
            value.to_bits()
        }
    }

    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        let count_log_count = (0..=period)
            .map(|count| {
                if count == 0 {
                    0.0
                } else {
                    let count = count as f64;
                    count * count.ln()
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Ok(Self {
            ring: vec![0.0; period].into_boxed_slice(),
            head: 0,
            len: 0,
            counts: HashMap::with_capacity(period),
            count_log_count,
            weighted_log_sum: 0.0,
            weighted_log_compensation: 0.0,
            nan_count: 0,
            period,
            value: None,
        })
    }

    #[inline]
    fn add_weighted_log_delta(&mut self, delta: f64) {
        // Neumaier compensation prevents long-running add/remove streams from
        // accumulating enough rounding drift to affect oracle tolerance.
        let next = self.weighted_log_sum + delta;
        if self.weighted_log_sum.abs() >= delta.abs() {
            self.weighted_log_compensation += (self.weighted_log_sum - next) + delta;
        } else {
            self.weighted_log_compensation += (delta - next) + self.weighted_log_sum;
        }
        self.weighted_log_sum = next;
    }

    #[inline]
    fn remove_value(&mut self, value: f64) {
        if value.is_nan() {
            self.nan_count -= 1;
            return;
        }

        let key = Self::count_key(value);
        let old_count = *self.counts.get(&key).expect("evicted value counted") as usize;
        let new_count = old_count - 1;
        let delta = self.count_log_count[new_count] - self.count_log_count[old_count];
        self.add_weighted_log_delta(delta);
        if new_count == 0 {
            self.counts.remove(&key);
        } else {
            *self.counts.get_mut(&key).expect("existing count") = new_count as u32;
        }
    }

    #[inline]
    fn add_value(&mut self, value: f64) {
        if value.is_nan() {
            self.nan_count += 1;
            return;
        }

        let key = Self::count_key(value);
        let old_count = self.counts.get(&key).copied().unwrap_or(0) as usize;
        let new_count = old_count + 1;
        let delta = self.count_log_count[new_count] - self.count_log_count[old_count];
        self.add_weighted_log_delta(delta);
        self.counts.insert(key, new_count as u32);
    }

    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.len == self.period {
            let evicted = self.ring[self.head];
            let unchanged_bin = !evicted.is_nan()
                && !input.is_nan()
                && Self::count_key(evicted) == Self::count_key(input);
            if !unchanged_bin && !(evicted.is_nan() && input.is_nan()) {
                self.remove_value(evicted);
                self.add_value(input);
            }
        } else {
            self.len += 1;
            self.add_value(input);
        }
        self.ring[self.head] = input;
        self.head = (self.head + 1) % self.period;
        self.value = (self.len == self.period).then(|| {
            if self.nan_count != 0 {
                return f64::NAN;
            }
            let n = self.period as f64;
            n.ln() - (self.weighted_log_sum + self.weighted_log_compensation) / n
        });
        self.value
    }

    /// Extend the state with a chronological slice and aligned NaN warm-up.
    pub fn extend_slice_into(&mut self, input: &[f64], output: &mut Vec<f64>) {
        output.extend(
            input
                .iter()
                .copied()
                .map(|value| self.append(value).unwrap_or(f64::NAN)),
        );
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }

    pub fn reset(&mut self) {
        self.head = 0;
        self.len = 0;
        self.counts.clear();
        self.weighted_log_sum = 0.0;
        self.weighted_log_compensation = 0.0;
        self.nan_count = 0;
        self.value = None;
    }
}
