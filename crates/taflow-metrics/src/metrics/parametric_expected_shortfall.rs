use crate::{
    primitives::OnlineMoments, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Signed lower-tail Gaussian expected shortfall estimated from sample moments.
#[derive(Debug, Clone)]
pub struct ParametricExpectedShortfall {
    input: MetricInputState,
    moments: OnlineMoments,
    cutoff: f64,
    density_at_quantile: f64,
}

impl ParametricExpectedShortfall {
    /// Construct an empty Gaussian expected-shortfall estimator.
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
                reason: "parametric expected shortfall requires a normalized return domain",
            });
        }
        let quantile = Self::standard_normal_quantile(cutoff);
        let density_at_quantile =
            (-0.5 * quantile * quantile).exp() / (2.0 * std::f64::consts::PI).sqrt();
        Ok(Self {
            input: MetricInputState::new(input_kind, nan_policy)?,
            moments: OnlineMoments::new(),
            cutoff,
            density_at_quantile,
        })
    }
    /// Append one chronological observation and return current signed expected shortfall.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        if let Some(simple_return) = self.input.append(value)? {
            self.moments.append(simple_return);
        }
        Ok(self.value())
    }
    /// Append a chronological slice through the same state.
    pub fn extend(&mut self, values: &[f64]) -> MetricResult<Option<f64>> {
        self.input.extend(values, |simple_return| {
            self.moments.append(simple_return);
            Ok(())
        })?;
        Ok(self.value())
    }
    /// Return `mean - sample_std * normal_pdf(normal_ppf(cutoff)) / cutoff`.
    pub fn value(&self) -> Option<f64> {
        Some(
            self.moments.mean()?
                - self.moments.standard_deviation(1)? * self.density_at_quantile / self.cutoff,
        )
    }
    /// Return current scalar without replaying input.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }
    /// Reset while preserving cutoff and input configuration.
    pub fn reset(&mut self) {
        self.input.reset();
        self.moments.reset();
    }
    /// Return usable normalized-return count.
    pub fn len(&self) -> usize {
        self.input.len()
    }
    /// Return whether empty.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }

    // Peter J. Acklam's inverse-normal rational approximation.
    fn standard_normal_quantile(probability: f64) -> f64 {
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
        let (value, sign) = if probability < 0.02425 {
            ((-2.0 * probability.ln()).sqrt(), 1.0)
        } else if probability > 0.97575 {
            ((-2.0 * (1.0 - probability).ln()).sqrt(), -1.0)
        } else {
            let q = probability - 0.5;
            let r = q * q;
            return (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r + A[4]) * r + A[5]) * q
                / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r + B[4]) * r + 1.0);
        };
        sign * (((((C[0] * value + C[1]) * value + C[2]) * value + C[3]) * value + C[4]) * value
            + C[5])
            / ((((D[0] * value + D[1]) * value + D[2]) * value + D[3]) * value + 1.0)
    }
}
