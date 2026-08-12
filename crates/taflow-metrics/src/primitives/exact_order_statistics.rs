use crate::{MetricError, MetricResult};

/// Exact retained order statistics with a lazily refreshed sorted cache.
#[derive(Debug, Clone, Default)]
pub struct ExactOrderStatistics {
    values: Vec<f64>,
    working: Vec<f64>,
    dirty: bool,
    quantile_cache: [Option<(u64, f64)>; 2],
    next_quantile_slot: usize,
    lower_tail_cache: Option<(u64, f64)>,
}

impl ExactOrderStatistics {
    /// Construct an empty state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct an empty state with retained-history capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
            working: Vec::with_capacity(capacity),
            dirty: false,
            quantile_cache: [None, None],
            next_quantile_slot: 0,
            lower_tail_cache: None,
        }
    }

    /// Append one already validated finite observation.
    pub fn append(&mut self, value: f64) {
        self.values.push(value);
        self.dirty = true;
    }

    /// Clear observations without shrinking allocated buffers.
    pub fn reset(&mut self) {
        self.values.clear();
        self.working.clear();
        self.dirty = false;
        self.quantile_cache = [None, None];
        self.next_quantile_slot = 0;
        self.lower_tail_cache = None;
    }

    /// Number of retained observations.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Whether no observations are retained.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Linear-interpolated quantile matching NumPy's default convention.
    pub fn quantile(&mut self, cutoff: f64) -> MetricResult<Option<f64>> {
        self.validate_cutoff(cutoff, true)?;
        if self.values.is_empty() {
            return Ok(None);
        }
        self.refresh_working();
        let key = cutoff.to_bits();
        if let Some((_, value)) = self
            .quantile_cache
            .iter()
            .flatten()
            .find(|(cached, _)| *cached == key)
        {
            return Ok(Some(*value));
        }
        let index = (self.working.len() - 1) as f64 * cutoff;
        let lower = index.floor() as usize;
        let upper = index.ceil() as usize;
        let weight = index - lower as f64;
        let lower_value = *self.working.select_nth_unstable_by(lower, f64::total_cmp).1;
        let upper_value = if upper == lower {
            lower_value
        } else {
            *self.working.select_nth_unstable_by(upper, f64::total_cmp).1
        };
        let value = lower_value + (upper_value - lower_value) * weight;
        self.quantile_cache[self.next_quantile_slot] = Some((key, value));
        self.next_quantile_slot = (self.next_quantile_slot + 1) % self.quantile_cache.len();
        Ok(Some(value))
    }

    /// Mean of the lowest `floor((n - 1) * cutoff) + 1` observations.
    pub fn lower_tail_mean(&mut self, cutoff: f64) -> MetricResult<Option<f64>> {
        self.validate_cutoff(cutoff, false)?;
        if self.values.is_empty() {
            return Ok(None);
        }
        self.refresh_working();
        let key = cutoff.to_bits();
        if let Some((cached, value)) = self.lower_tail_cache {
            if cached == key {
                return Ok(Some(value));
            }
        }
        let selected = ((self.working.len() - 1) as f64 * cutoff).floor() as usize + 1;
        if selected < self.working.len() {
            self.working
                .select_nth_unstable_by(selected - 1, f64::total_cmp);
        }
        let value = self.working[..selected].iter().sum::<f64>() / selected as f64;
        self.lower_tail_cache = Some((key, value));
        Ok(Some(value))
    }

    fn validate_cutoff(&self, cutoff: f64, allow_endpoints: bool) -> MetricResult<()> {
        let valid = cutoff.is_finite()
            && if allow_endpoints {
                (0.0..=1.0).contains(&cutoff)
            } else {
                cutoff > 0.0 && cutoff < 1.0
            };
        if valid {
            Ok(())
        } else {
            Err(MetricError::InvalidParameter {
                name: "cutoff",
                value: cutoff.to_string(),
                reason: if allow_endpoints {
                    "must be finite and between zero and one inclusive"
                } else {
                    "must be finite and strictly between zero and one"
                },
            })
        }
    }

    fn refresh_working(&mut self) {
        if self.dirty {
            self.working.clear();
            self.working.extend_from_slice(&self.values);
            self.quantile_cache = [None, None];
            self.next_quantile_slot = 0;
            self.lower_tail_cache = None;
            self.dirty = false;
        }
    }
}
