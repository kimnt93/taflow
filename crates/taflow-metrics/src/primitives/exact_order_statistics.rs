use crate::{MetricError, MetricResult};

/// Exact retained order statistics with a lazily refreshed sorted cache.
#[derive(Debug, Clone, Default)]
pub struct ExactOrderStatistics {
    values: Vec<f64>,
    sorted: Vec<f64>,
    dirty: bool,
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
            sorted: Vec::with_capacity(capacity),
            dirty: false,
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
        self.sorted.clear();
        self.dirty = false;
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
        self.refresh();
        let index = (self.sorted.len() - 1) as f64 * cutoff;
        let lower = index.floor() as usize;
        let upper = index.ceil() as usize;
        let weight = index - lower as f64;
        Ok(Some(
            self.sorted[lower] + (self.sorted[upper] - self.sorted[lower]) * weight,
        ))
    }

    /// Mean of the lowest `floor((n - 1) * cutoff) + 1` observations.
    pub fn lower_tail_mean(&mut self, cutoff: f64) -> MetricResult<Option<f64>> {
        self.validate_cutoff(cutoff, false)?;
        if self.values.is_empty() {
            return Ok(None);
        }
        self.refresh();
        let selected = ((self.sorted.len() - 1) as f64 * cutoff).floor() as usize + 1;
        Ok(Some(
            self.sorted[..selected].iter().sum::<f64>() / selected as f64,
        ))
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

    fn refresh(&mut self) {
        if self.dirty {
            self.sorted.clear();
            self.sorted.extend_from_slice(&self.values);
            self.sorted.sort_by(f64::total_cmp);
            self.dirty = false;
        }
    }
}
