use crate::{
    primitives::OnlineMoments, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Signed lower-tail Gaussian quantile estimated from sample moments.
#[derive(Debug, Clone)]
pub struct ParametricValueAtRisk {
    input: MetricInputState,
    moments: OnlineMoments,
    standard_normal_quantile: f64,
}

impl ParametricValueAtRisk {
    /// Construct an empty Gaussian value-at-risk estimator.
    pub fn new(
        input_kind: MetricInputKind,
        cutoff: f64,
        nan_policy: NanPolicy,
    ) -> MetricResult<Self> {
        if !cutoff.is_finite() || cutoff <= 0.0 || cutoff >= 1.0 {
            return Err(MetricError::InvalidParameter {
                name: "cutoff",
                value: cutoff.to_string(),
                reason: "must be finite and strictly between zero and one",
            });
        }
        if matches!(
            input_kind,
            MetricInputKind::RawPnl | MetricInputKind::Trades
        ) {
            return Err(MetricError::InvalidParameter {
                name: "input_kind",
                value: format!("{input_kind:?}"),
                reason: "parametric value at risk requires returns, log returns, equity, or period P&L with initial equity",
            });
        }

        Ok(Self {
            input: MetricInputState::new(input_kind, nan_policy)?,
            moments: OnlineMoments::new(),
            standard_normal_quantile: Self::inverse_standard_normal(cutoff),
        })
    }

    /// Append one chronological observation and return the current signed quantile.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(simple_return) = self.input.append(value)? {
            self.moments.append(simple_return);
        }
        Ok(self.value())
    }

    /// Append a chronological slice through the same persistent state.
    pub fn extend(&mut self, values: &[f64]) -> MetricResult<Option<f64>> {
        self.input.extend(values, |simple_return| {
            self.moments.append(simple_return);
            Ok(())
        })?;
        Ok(self.value())
    }

    /// Return `sample_mean + normal_ppf(cutoff) * sample_standard_deviation`.
    pub fn value(&self) -> Option<f64> {
        Some(
            self.moments.mean()?
                + self.standard_normal_quantile * self.moments.standard_deviation(1)?,
        )
    }

    /// Return the current result in O(1) without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving input mode and cutoff.
    pub fn reset(&mut self) {
        self.input.reset();
        self.moments.reset();
    }

    /// Return the number of usable normalized returns processed.
    pub fn len(&self) -> usize {
        self.input.len()
    }

    /// Return whether no usable normalized returns have been processed.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }

    // Peter J. Acklam's inverse-normal rational approximation. Its maximum
    // absolute error is below 1.2e-9 over the open probability interval.
    fn inverse_standard_normal(probability: f64) -> f64 {
        const A: [f64; 6] = [
            -3.969_683_028_665_376e1,
            2.209_460_984_245_205e2,
            -2.759_285_104_469_687e2,
            1.383_577_518_672_69e2,
            -3.066_479_806_614_716e1,
            2.506_628_277_459_239,
        ];
        const B: [f64; 5] = [
            -5.447_609_879_822_406e1,
            1.615_858_368_580_409e2,
            -1.556_989_798_598_866e2,
            6.680_131_188_771_972e1,
            -1.328_068_155_288_572e1,
        ];
        const C: [f64; 6] = [
            -7.784_894_002_430_293e-3,
            -3.223_964_580_411_365e-1,
            -2.400_758_277_161_838,
            -2.549_732_539_343_734,
            4.374_664_141_464_968,
            2.938_163_982_698_783,
        ];
        const D: [f64; 4] = [
            7.784_695_709_041_462e-3,
            3.224_671_290_700_398e-1,
            2.445_134_137_142_996,
            3.754_408_661_907_416,
        ];
        const LOWER_BREAKPOINT: f64 = 0.02425;
        const UPPER_BREAKPOINT: f64 = 1.0 - LOWER_BREAKPOINT;

        if probability < LOWER_BREAKPOINT {
            let q = (-2.0 * probability.ln()).sqrt();
            (((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
                / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
        } else if probability <= UPPER_BREAKPOINT {
            let q = probability - 0.5;
            let r = q * q;
            (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r + A[4]) * r + A[5]) * q
                / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r + B[4]) * r + 1.0)
        } else {
            let q = (-2.0 * (1.0 - probability).ln()).sqrt();
            -(((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
                / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
        }
    }
}
