use crate::{MetricError, MetricResult, NanPolicy};

/// Semantic observation domain for [`Exposure`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExposureInputKind {
    /// Period returns, where exact non-zero return is an activity proxy.
    Returns,
    /// Explicit position, weight, or exposure state, where non-zero means invested.
    Positions,
}

/// Share of usable periods with a non-zero return or position state.
#[derive(Debug, Clone)]
pub struct Exposure {
    input_kind: ExposureInputKind,
    nan_policy: NanPolicy,
    exposed: usize,
    usable: usize,
    position: usize,
}

impl Exposure {
    /// Construct an empty state for an explicit return-proxy or position domain.
    pub fn new(input_kind: ExposureInputKind, nan_policy: NanPolicy) -> MetricResult<Self> {
        Ok(Self {
            input_kind,
            nan_policy,
            exposed: 0,
            usable: 0,
            position: 0,
        })
    }

    fn ingest(&mut self, value: f64) -> MetricResult<()> {
        let position = self.position;
        if value.is_nan() {
            if self.nan_policy == NanPolicy::Raise {
                return Err(MetricError::InvalidObservation {
                    domain: match self.input_kind {
                        ExposureInputKind::Returns => "return",
                        ExposureInputKind::Positions => "position",
                    },
                    position,
                    value: value.to_string(),
                    reason: "NaN is forbidden by nan_policy='raise'",
                });
            }
            self.position += 1;
            return Ok(());
        }
        if !value.is_finite() {
            return Err(MetricError::InvalidObservation {
                domain: match self.input_kind {
                    ExposureInputKind::Returns => "return",
                    ExposureInputKind::Positions => "position",
                },
                position,
                value: value.to_string(),
                reason: "infinite values are not supported",
            });
        }
        if self.input_kind == ExposureInputKind::Returns && value < -1.0 {
            return Err(MetricError::InvalidObservation {
                domain: "return",
                position,
                value: value.to_string(),
                reason: "simple returns must be greater than or equal to -1",
            });
        }

        self.exposed += usize::from(value != 0.0);
        self.usable += 1;
        self.position += 1;
        Ok(())
    }

    /// Append one chronological observation and return exposure to date.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        self.ingest(value)?;
        Ok(self.value())
    }

    /// Append a chronological slice through the same bounded native state.
    pub fn extend(&mut self, values: &[f64]) -> MetricResult<Option<f64>> {
        for &value in values {
            self.ingest(value)?;
        }
        Ok(self.value())
    }

    /// Return exposure rounded upward to the next percentage point.
    ///
    /// The raw fraction is non-zero usable observations divided by all usable
    /// observations. Applying `ceil(fraction * 100) / 100` preserves the
    /// QuantStats 0.0.81 oracle contract. This is O(1) from two counters.
    pub fn value(&self) -> Option<f64> {
        (self.usable != 0).then(|| {
            let raw = self.exposed as f64 / self.usable as f64;
            (raw * 100.0).ceil() / 100.0
        })
    }

    /// Return the current O(1) scalar without replaying observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving domain and missing policy.
    pub fn reset(&mut self) {
        self.exposed = 0;
        self.usable = 0;
        self.position = 0;
    }

    /// Return the number of usable observations processed.
    pub fn len(&self) -> usize {
        self.usable
    }

    /// Return whether no usable observations have been processed.
    pub fn is_empty(&self) -> bool {
        self.usable == 0
    }
}
