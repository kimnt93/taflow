use super::{MetricInputKind, MetricInputState, NanPolicy};
use crate::{MetricError, MetricResult};

/// Two aligned input converters with transactional pairwise missing handling.
#[derive(Debug, Clone)]
pub struct PairedMetricInputState {
    primary: MetricInputState,
    benchmark: MetricInputState,
    len: usize,
}

impl PairedMetricInputState {
    /// Construct paired converters whose semantic domains match.
    pub fn new(
        primary_kind: MetricInputKind,
        benchmark_kind: MetricInputKind,
        nan_policy: NanPolicy,
    ) -> MetricResult<Self> {
        if !primary_kind.discriminant_matches(benchmark_kind) {
            return Err(MetricError::InputDomainMismatch {
                primary: primary_kind.domain(),
                benchmark: benchmark_kind.domain(),
            });
        }
        Ok(Self {
            primary: MetricInputState::new(primary_kind, nan_policy)?,
            benchmark: MetricInputState::new(benchmark_kind, nan_policy)?,
            len: 0,
        })
    }

    /// Append one aligned pair without partially mutating either converter.
    pub fn append(&mut self, primary: f64, benchmark: f64) -> MetricResult<Option<(f64, f64)>> {
        if primary.is_nan() || benchmark.is_nan() {
            if self.primary.nan_policy() == NanPolicy::Raise {
                let rejected = if primary.is_nan() { primary } else { benchmark };
                // A temporary converter gives the same domain-specific diagnostic
                // while preserving both live converter states.
                let mut probe = if primary.is_nan() {
                    self.primary.clone()
                } else {
                    self.benchmark.clone()
                };
                return probe.append(rejected).map(|_| None);
            }
            self.primary.skip_missing_pair();
            self.benchmark.skip_missing_pair();
            return Ok(None);
        }

        let mut primary_state = self.primary.clone();
        let mut benchmark_state = self.benchmark.clone();
        let primary_value = primary_state.append(primary)?;
        let benchmark_value = benchmark_state.append(benchmark)?;
        let pair = match (primary_value, benchmark_value) {
            (Some(primary), Some(benchmark)) => Some((primary, benchmark)),
            (None, None) => None,
            _ => {
                return Err(MetricError::InputDomainMismatch {
                    primary: self.primary.kind().domain(),
                    benchmark: self.benchmark.kind().domain(),
                });
            }
        };
        self.primary = primary_state;
        self.benchmark = benchmark_state;
        if pair.is_some() {
            self.len += 1;
        }
        Ok(pair)
    }

    /// Append equal-length slices and invoke `consume` for each usable pair.
    pub fn extend_slices(
        &mut self,
        primary: &[f64],
        benchmark: &[f64],
        mut consume: impl FnMut(f64, f64),
    ) -> MetricResult<()> {
        if primary.len() != benchmark.len() {
            return Err(MetricError::LengthMismatch {
                expected: primary.len(),
                got: benchmark.len(),
            });
        }
        for (&primary, &benchmark) in primary.iter().zip(benchmark) {
            if let Some((primary, benchmark)) = self.append(primary, benchmark)? {
                consume(primary, benchmark);
            }
        }
        Ok(())
    }

    /// Restore both converters to their configured fresh states.
    pub fn reset(&mut self) {
        self.primary.reset();
        self.benchmark.reset();
        self.len = 0;
    }

    /// Return the number of usable aligned pairs emitted.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Return whether no usable aligned pairs have been emitted.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
