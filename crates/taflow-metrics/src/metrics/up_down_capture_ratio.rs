use crate::{
    primitives::CompoundedGrowth, MetricError, MetricInputKind, MetricResult, NanPolicy,
    PairedMetricInputState,
};

/// Up-market capture divided by down-market capture.
#[derive(Debug, Clone)]
pub struct UpDownCaptureRatio {
    input: PairedMetricInputState,
    up_primary_growth: CompoundedGrowth,
    up_benchmark_growth: CompoundedGrowth,
    down_primary_growth: CompoundedGrowth,
    down_benchmark_growth: CompoundedGrowth,
    periods_per_year: f64,
}

impl UpDownCaptureRatio {
    /// Construct an empty up/down capture state for aligned return-like inputs.
    pub fn new(
        primary_input_kind: MetricInputKind,
        benchmark_input_kind: MetricInputKind,
        periods_per_year: f64,
        nan_policy: NanPolicy,
    ) -> MetricResult<Self> {
        if !periods_per_year.is_finite() || periods_per_year <= 0.0 {
            return Err(MetricError::InvalidParameter {
                name: "periods_per_year",
                value: periods_per_year.to_string(),
                reason: "must be finite and greater than zero",
            });
        }
        if matches!(
            primary_input_kind,
            MetricInputKind::RawPnl | MetricInputKind::Trades
        ) || matches!(
            benchmark_input_kind,
            MetricInputKind::RawPnl | MetricInputKind::Trades
        ) {
            return Err(MetricError::InvalidParameter {
                name: "input_kind",
                value: format!("{primary_input_kind:?}/{benchmark_input_kind:?}"),
                reason: "up/down capture ratio requires returns, log returns, equity, or period P&L with initial equity",
            });
        }

        Ok(Self {
            input: PairedMetricInputState::new(
                primary_input_kind,
                benchmark_input_kind,
                nan_policy,
            )?,
            up_primary_growth: CompoundedGrowth::new(),
            up_benchmark_growth: CompoundedGrowth::new(),
            down_primary_growth: CompoundedGrowth::new(),
            down_benchmark_growth: CompoundedGrowth::new(),
            periods_per_year,
        })
    }

    fn consume(&mut self, primary_return: f64, benchmark_return: f64) -> MetricResult<()> {
        if benchmark_return > 0.0 {
            self.up_primary_growth.append(primary_return)?;
            self.up_benchmark_growth.append(benchmark_return)?;
        } else if benchmark_return < 0.0 {
            self.down_primary_growth.append(primary_return)?;
            self.down_benchmark_growth.append(benchmark_return)?;
        }
        Ok(())
    }

    /// Append one aligned pair and return the current up/down capture ratio.
    pub fn append(&mut self, primary: f64, benchmark: f64) -> MetricResult<Option<f64>> {
        if let Some((primary_return, benchmark_return)) = self.input.append(primary, benchmark)? {
            self.consume(primary_return, benchmark_return)?;
        }
        Ok(self.value())
    }

    /// Append equal-length aligned slices through the persistent state.
    pub fn extend(&mut self, primary: &[f64], benchmark: &[f64]) -> MetricResult<Option<f64>> {
        if primary.len() != benchmark.len() {
            return Err(MetricError::LengthMismatch {
                expected: primary.len(),
                got: benchmark.len(),
            });
        }
        for (&primary, &benchmark) in primary.iter().zip(benchmark) {
            self.append(primary, benchmark)?;
        }
        Ok(self.value())
    }

    fn capture(&self, primary: &CompoundedGrowth, benchmark: &CompoundedGrowth) -> Option<f64> {
        let count = primary.len();
        if count == 0 {
            return None;
        }
        let exponent = self.periods_per_year / count as f64;
        let primary_return = (primary.logarithmic_growth()? * exponent).exp_m1();
        let benchmark_return = (benchmark.logarithmic_growth()? * exponent).exp_m1();
        if benchmark_return == 0.0 {
            return None;
        }
        Some(primary_return / benchmark_return)
    }

    /// Return up-market capture divided by down-market capture.
    ///
    /// Both benchmark regimes and non-zero benchmark annual returns are
    /// required. A zero down-market capture denominator is also undefined.
    pub fn value(&self) -> Option<f64> {
        let up_capture = self.capture(&self.up_primary_growth, &self.up_benchmark_growth)?;
        let down_capture = self.capture(&self.down_primary_growth, &self.down_benchmark_growth)?;
        if down_capture == 0.0 {
            return None;
        }
        Some(up_capture / down_capture)
    }

    /// Return the current result without replaying prior observations.
    pub fn compute(&self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while preserving configuration.
    pub fn reset(&mut self) {
        self.input.reset();
        self.up_primary_growth.reset();
        self.up_benchmark_growth.reset();
        self.down_primary_growth.reset();
        self.down_benchmark_growth.reset();
    }

    /// Return the number of usable normalized aligned pairs processed.
    pub fn len(&self) -> usize {
        self.input.len()
    }

    /// Return whether no usable normalized aligned pairs have been processed.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }

    /// Return the configured annualization frequency.
    pub fn periods_per_year(&self) -> f64 {
        self.periods_per_year
    }
}
