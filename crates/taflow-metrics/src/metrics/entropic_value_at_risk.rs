use crate::{MetricError, MetricInputKind, MetricInputState, MetricResult, NanPolicy};

/// Positive-loss empirical Entropic Value at Risk with lazy exact optimization.
#[derive(Debug, Clone)]
pub struct EntropicValueAtRisk {
    input: MetricInputState,
    returns: Vec<f64>,
    cutoff: f64,
    cached_value: Option<f64>,
    dirty: bool,
}

impl EntropicValueAtRisk {
    /// Construct an empty empirical EVaR state.
    pub fn new(cutoff: f64, nan_policy: NanPolicy) -> MetricResult<Self> {
        if !cutoff.is_finite() || cutoff <= 0.0 || cutoff >= 1.0 {
            return Err(MetricError::InvalidParameter {
                name: "cutoff",
                value: cutoff.to_string(),
                reason: "must be finite and strictly between zero and one",
            });
        }

        Ok(Self {
            input: MetricInputState::unbound(nan_policy),
            returns: Vec::new(),
            cutoff,
            cached_value: None,
            dirty: false,
        })
    }

    /// Append one chronological observation in amortized O(1) time.
    pub fn append(&mut self, value: f64) -> MetricResult<()> {
        if let Some(simple_return) = self.input.append(value)? {
            self.returns.push(simple_return);
            self.dirty = true;
        }
        Ok(())
    }

    /// Append a chronological slice through one native loop.
    pub fn extend(&mut self, values: &[f64]) -> MetricResult<()> {
        self.returns.reserve(values.len());
        for &value in values {
            self.append(value)?;
        }
        Ok(())
    }

    /// Return exact empirical EVaR as a positive-loss measure.
    ///
    /// For losses `L=-return`, this minimizes
    /// `z * (log(mean(exp(L / z))) + log(1 / cutoff))` over `z > 0`.
    /// The monotone first-order condition is bracketed and bisected with a
    /// relative interval tolerance of `1e-12` and at most 256 iterations.
    /// Log-sum-exp evaluation is shifted by the worst loss. If the empirical
    /// optimum is attained only as `z` approaches zero, the exact worst loss
    /// is returned. Exact recomputation requires O(n) retained history and
    /// O(n * iterations) work only after new observations make the cache dirty.
    pub fn value(&mut self) -> Option<f64> {
        if self.returns.is_empty() {
            return None;
        }
        if !self.dirty {
            return self.cached_value;
        }

        let value = self.optimize();
        self.cached_value = Some(value);
        self.dirty = false;
        Some(value)
    }

    /// Return the cached result, optimizing only after input changes.
    pub fn compute(&mut self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior without shrinking retained capacity.
    pub fn reset(&mut self) {
        self.input.reset();
        self.returns.clear();
        self.cached_value = None;
        self.dirty = false;
    }

    /// Return the number of usable normalized returns retained.
    pub fn len(&self) -> usize {
        self.input.len()
    }

    /// Return whether no usable normalized returns are retained.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }

    fn optimize(&self) -> f64 {
        const RELATIVE_TOLERANCE: f64 = 1e-12;
        const MAXIMUM_ITERATIONS: usize = 256;

        let maximum_loss = self
            .returns
            .iter()
            .map(|value| -*value)
            .max_by(f64::total_cmp)
            .expect("nonempty history has a worst loss");
        let minimum_loss = self
            .returns
            .iter()
            .map(|value| -*value)
            .min_by(f64::total_cmp)
            .expect("nonempty history has a best loss");
        let range = maximum_loss - minimum_loss;
        if range == 0.0 {
            return maximum_loss;
        }

        let worst_count = self
            .returns
            .iter()
            .filter(|value| -**value == maximum_loss)
            .count();
        if self.cutoff * self.returns.len() as f64 <= worst_count as f64 {
            return maximum_loss;
        }

        let log_inverse_cutoff = (1.0 / self.cutoff).ln();
        let condition = |scaled_inverse_temperature: f64| {
            self.condition(
                scaled_inverse_temperature,
                maximum_loss,
                range,
                log_inverse_cutoff,
            )
        };

        let mut lower = 0.0;
        let mut upper = 1.0;
        while condition(upper) < 0.0 {
            upper *= 2.0;
        }

        for _ in 0..MAXIMUM_ITERATIONS {
            let midpoint = lower + (upper - lower) * 0.5;
            if condition(midpoint) < 0.0 {
                lower = midpoint;
            } else {
                upper = midpoint;
            }
            if upper - lower <= RELATIVE_TOLERANCE * midpoint.max(1.0) {
                break;
            }
        }

        let optimum = lower + (upper - lower) * 0.5;
        let (log_mean_exponential, _) = self.shifted_log_moment(optimum, maximum_loss, range);
        maximum_loss + range * (log_mean_exponential + log_inverse_cutoff) / optimum
    }

    fn condition(
        &self,
        scaled_inverse_temperature: f64,
        maximum_loss: f64,
        range: f64,
        log_inverse_cutoff: f64,
    ) -> f64 {
        let (log_mean_exponential, weighted_scaled_loss) =
            self.shifted_log_moment(scaled_inverse_temperature, maximum_loss, range);
        scaled_inverse_temperature * weighted_scaled_loss
            - log_mean_exponential
            - log_inverse_cutoff
    }

    fn shifted_log_moment(
        &self,
        scaled_inverse_temperature: f64,
        maximum_loss: f64,
        range: f64,
    ) -> (f64, f64) {
        let mut exponential_sum = 0.0;
        let mut weighted_sum = 0.0;
        for &simple_return in &self.returns {
            let scaled_loss = (-simple_return - maximum_loss) / range;
            let weight = (scaled_inverse_temperature * scaled_loss).exp();
            exponential_sum += weight;
            weighted_sum += weight * scaled_loss;
        }
        (
            (exponential_sum / self.returns.len() as f64).ln(),
            weighted_sum / exponential_sum,
        )
    }
}

crate::impl_return_metric_lifecycle!(EntropicValueAtRisk);
