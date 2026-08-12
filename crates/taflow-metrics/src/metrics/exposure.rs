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
    input_kind: Option<ExposureInputKind>,
    nan_policy: NanPolicy,
    exposed: usize,
    usable: usize,
    position: usize,
}

impl Exposure {
    /// Construct an empty state for an explicit return-proxy or position domain.
    pub fn new(nan_policy: NanPolicy) -> MetricResult<Self> {
        Ok(Self {
            input_kind: None,
            nan_policy,
            exposed: 0,
            usable: 0,
            position: 0,
        })
    }

    pub fn from_returns(&mut self, returns: &[f64]) -> MetricResult<&mut Self> {
        self.bind(ExposureInputKind::Returns)?;
        self.extend(returns)?;
        Ok(self)
    }

    pub fn from_positions(&mut self, positions: &[f64]) -> MetricResult<&mut Self> {
        self.bind(ExposureInputKind::Positions)?;
        self.extend(positions)?;
        Ok(self)
    }

    fn bind(&mut self, kind: ExposureInputKind) -> MetricResult<()> {
        match self.input_kind {
            None => self.input_kind = Some(kind),
            Some(selected) if selected == kind => {}
            Some(_) => {
                return Err(MetricError::InvalidParameter {
                    name: "input_kind",
                    value: format!("{kind:?}"),
                    reason: "exposure input domain is already selected",
                })
            }
        }
        Ok(())
    }

    fn ingest(&mut self, value: f64) -> MetricResult<()> {
        let input_kind = self.input_kind.ok_or(MetricError::InvalidParameter {
            name: "input_kind",
            value: "unbound".to_owned(),
            reason: "call from_returns or from_positions before append or extend",
        })?;
        let position = self.position;
        if value.is_nan() {
            if self.nan_policy == NanPolicy::Raise {
                return Err(MetricError::InvalidObservation {
                    domain: match input_kind {
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
                domain: match input_kind {
                    ExposureInputKind::Returns => "return",
                    ExposureInputKind::Positions => "position",
                },
                position,
                value: value.to_string(),
                reason: "infinite values are not supported",
            });
        }
        if input_kind == ExposureInputKind::Returns && value < -1.0 {
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
