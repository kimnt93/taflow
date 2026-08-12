use crate::{
    primitives::DrawdownState, MetricError, MetricInputKind, MetricInputState, MetricResult,
    NanPolicy,
};

/// Exact expected shortfall of discrete drawdown-episode troughs.
#[derive(Debug, Clone)]
pub struct ConditionalDrawdownAtRisk {
    input: MetricInputState,
    drawdown: DrawdownState,
    confidence: f64,
    completed_troughs: Vec<f64>,
    current_trough: Option<f64>,
    current_is_drawdown: bool,
    sorted_troughs: Vec<f64>,
    dirty: bool,
}

impl ConditionalDrawdownAtRisk {
    /// Construct an empty exact discrete-episode CDaR state.
    pub fn new(
        input_kind: MetricInputKind,
        confidence: f64,
        nan_policy: NanPolicy,
    ) -> MetricResult<Self> {
        if !confidence.is_finite() || confidence <= 0.0 || confidence >= 1.0 {
            return Err(MetricError::InvalidParameter {
                name: "confidence",
                value: confidence.to_string(),
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
                reason: "conditional drawdown at risk requires returns, log returns, equity, or period P&L with initial equity",
            });
        }

        Ok(Self {
            input: MetricInputState::new(input_kind, nan_policy)?,
            drawdown: DrawdownState::new(),
            confidence,
            completed_troughs: Vec::new(),
            current_trough: None,
            current_is_drawdown: false,
            sorted_troughs: Vec::new(),
            dirty: false,
        })
    }

    fn ingest(&mut self, value: f64) -> MetricResult<()> {
        let Some(simple_return) = self.input.append(value)? else {
            return Ok(());
        };
        self.drawdown.append(simple_return)?;
        let drawdown = self
            .drawdown
            .current_drawdown()
            .expect("an appended return always produces a drawdown");
        let is_drawdown = drawdown < 0.0;

        match self.current_trough {
            None => {
                self.current_trough = Some(drawdown);
                self.current_is_drawdown = is_drawdown;
            }
            Some(current) if self.current_is_drawdown == is_drawdown => {
                self.current_trough = Some(current.min(drawdown));
            }
            Some(current) => {
                self.completed_troughs.push(current);
                self.current_trough = Some(drawdown);
                self.current_is_drawdown = is_drawdown;
            }
        }
        self.dirty = true;
        Ok(())
    }

    /// Append one chronological observation and return the current positive CDaR.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        self.ingest(value)?;
        Ok(self.value())
    }

    /// Append a chronological slice and sort the episode distribution at most once.
    pub fn extend(&mut self, values: &[f64]) -> MetricResult<Option<f64>> {
        self.completed_troughs.reserve(values.len());
        self.sorted_troughs.reserve(values.len().saturating_add(1));
        for &value in values {
            self.ingest(value)?;
        }
        Ok(self.value())
    }

    /// Return exact discrete-episode CDaR, or `None` before a usable return.
    ///
    /// This matches PerformanceAnalytics 2.1.0 `CDD` with `method="discrete"`,
    /// `geometric=TRUE`, and `invert=TRUE`. The signed episode troughs use an R
    /// type-7 quantile at `1 - confidence`; all troughs at or below that
    /// interpolated boundary are averaged, including ties. The active final
    /// episode participates exactly once. Exactness requires O(e) retained
    /// storage for e completed sign episodes; sorting is lazy and only occurs
    /// after input changes.
    pub fn value(&mut self) -> Option<f64> {
        self.current_trough?;
        if self.dirty {
            self.sorted_troughs.clear();
            self.sorted_troughs
                .extend_from_slice(&self.completed_troughs);
            self.sorted_troughs.push(
                self.current_trough
                    .expect("nonempty input has a current drawdown episode"),
            );
            self.sorted_troughs.sort_by(f64::total_cmp);
            self.dirty = false;
        }

        let cutoff = 1.0 - self.confidence;
        let quantile_index = (self.sorted_troughs.len() - 1) as f64 * cutoff;
        let lower = quantile_index.floor() as usize;
        let upper = quantile_index.ceil() as usize;
        let weight = quantile_index - lower as f64;
        let boundary = self.sorted_troughs[lower]
            + (self.sorted_troughs[upper] - self.sorted_troughs[lower]) * weight;

        let mut tail_sum = 0.0;
        let mut tail_count = 0_usize;
        for &trough in &self.sorted_troughs {
            if trough > boundary {
                break;
            }
            tail_sum += trough;
            tail_count += 1;
        }
        Some(-(tail_sum / tail_count as f64))
    }

    /// Return the current exact scalar without replaying prior observations.
    pub fn compute(&mut self) -> Option<f64> {
        self.value()
    }

    /// Restore fresh-state behavior while retaining allocated episode buffers.
    pub fn reset(&mut self) {
        self.input.reset();
        self.drawdown.reset();
        self.completed_troughs.clear();
        self.current_trough = None;
        self.current_is_drawdown = false;
        self.sorted_troughs.clear();
        self.dirty = false;
    }

    /// Return the number of usable normalized returns processed.
    pub fn len(&self) -> usize {
        self.input.len()
    }

    /// Return whether no usable normalized returns have been processed.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }
}
