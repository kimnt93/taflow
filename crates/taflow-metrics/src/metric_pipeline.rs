use crate::{
    metrics::{
        AnnualizedReturn, AnnualizedVolatility, AverageDrawdown, CalmarRatio,
        ConditionalDrawdownAtRisk, DeflatedSharpeRatio, DownsideDeviation, EntropicValueAtRisk,
        GainToPainRatio, HistoricalExpectedShortfall, HistoricalValueAtRisk, MaximumDrawdown,
        ModifiedSharpeRatio, OmegaRatio, PainIndex, PainRatio, ParametricExpectedShortfall,
        ParametricValueAtRisk, ProbabilisticSharpeRatio, RecoveryFactor, SharpeRatio, SortinoRatio,
        StabilityOfTimeSeries, TailRatio, TotalReturn, UlcerIndex, UlcerPerformanceIndex,
    },
    MetricError, MetricResult,
};

/// Semantic input domain selected for all metrics in a pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricPipelineInputKind {
    Returns,
    LogReturns,
    Equity,
    PeriodPnl,
}

/// Type-erased lifecycle used by `MetricPipeline` to own configured metrics.
pub trait PipelineMetric: Send {
    fn supports(&self, input_kind: MetricPipelineInputKind) -> bool;
    fn from_returns(&mut self, returns: &[f64]) -> MetricResult<()>;
    fn from_log_returns(&mut self, log_returns: &[f64]) -> MetricResult<()>;
    fn from_equity(&mut self, equity: &[f64]) -> MetricResult<()>;
    fn from_pnl(&mut self, pnl: &[f64], initial_capital: f64) -> MetricResult<()>;
    fn append(&mut self, value: f64) -> MetricResult<()>;
    fn extend(&mut self, values: &[f64]) -> MetricResult<()>;
    fn compute(&mut self) -> Option<f64>;
    fn reset(&mut self);
    fn len(&self) -> usize;
}

macro_rules! impl_return_pipeline_metric {
    ($state:ty) => {
        impl PipelineMetric for $state {
            fn supports(&self, _: MetricPipelineInputKind) -> bool {
                true
            }
            fn from_returns(&mut self, values: &[f64]) -> MetricResult<()> {
                <$state>::from_returns(self, values).map(|_| ())
            }
            fn from_log_returns(&mut self, values: &[f64]) -> MetricResult<()> {
                <$state>::from_log_returns(self, values).map(|_| ())
            }
            fn from_equity(&mut self, values: &[f64]) -> MetricResult<()> {
                <$state>::from_equity(self, values).map(|_| ())
            }
            fn from_pnl(&mut self, values: &[f64], initial_capital: f64) -> MetricResult<()> {
                <$state>::from_pnl(self, values, initial_capital).map(|_| ())
            }
            fn append(&mut self, value: f64) -> MetricResult<()> {
                <$state>::append(self, value).map(|_| ())
            }
            fn extend(&mut self, values: &[f64]) -> MetricResult<()> {
                <$state>::extend(self, values).map(|_| ())
            }
            fn compute(&mut self) -> Option<f64> {
                <$state>::compute(self)
            }
            fn reset(&mut self) {
                <$state>::reset(self)
            }
            fn len(&self) -> usize {
                <$state>::len(self)
            }
        }
    };
}

impl_return_pipeline_metric!(SharpeRatio);
impl_return_pipeline_metric!(SortinoRatio);
impl_return_pipeline_metric!(TotalReturn);
impl_return_pipeline_metric!(AnnualizedReturn);
impl_return_pipeline_metric!(AnnualizedVolatility);
impl_return_pipeline_metric!(MaximumDrawdown);
impl_return_pipeline_metric!(DownsideDeviation);
impl_return_pipeline_metric!(CalmarRatio);
impl_return_pipeline_metric!(OmegaRatio);
impl_return_pipeline_metric!(GainToPainRatio);
impl_return_pipeline_metric!(PainIndex);
impl_return_pipeline_metric!(UlcerIndex);
impl_return_pipeline_metric!(AverageDrawdown);
impl_return_pipeline_metric!(ConditionalDrawdownAtRisk);
impl_return_pipeline_metric!(EntropicValueAtRisk);
impl_return_pipeline_metric!(HistoricalExpectedShortfall);
impl_return_pipeline_metric!(HistoricalValueAtRisk);
impl_return_pipeline_metric!(ParametricExpectedShortfall);
impl_return_pipeline_metric!(ParametricValueAtRisk);
impl_return_pipeline_metric!(RecoveryFactor);
impl_return_pipeline_metric!(StabilityOfTimeSeries);
impl_return_pipeline_metric!(TailRatio);
impl_return_pipeline_metric!(DeflatedSharpeRatio);
impl_return_pipeline_metric!(ModifiedSharpeRatio);
impl_return_pipeline_metric!(PainRatio);
impl_return_pipeline_metric!(ProbabilisticSharpeRatio);
impl_return_pipeline_metric!(UlcerPerformanceIndex);

struct NamedMetric {
    name: String,
    metric: Box<dyn PipelineMetric>,
}

/// Pipeline of configured metrics addressed by caller-provided names.
pub struct MetricPipeline {
    metrics: Vec<NamedMetric>,
    input_kind: Option<MetricPipelineInputKind>,
}

impl Default for MetricPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricPipeline {
    /// Construct an empty pipeline. Add configured metrics before ingesting data.
    pub fn new() -> Self {
        Self {
            metrics: Vec::new(),
            input_kind: None,
        }
    }

    /// Add a configured metric under a unique caller-provided name.
    pub fn add<M>(&mut self, name: impl Into<String>, metric: M) -> MetricResult<&mut Self>
    where
        M: PipelineMetric + 'static,
    {
        if self.input_kind.is_some() {
            return Err(MetricError::InvalidParameter {
                name: "metric",
                value: "configured metric".to_owned(),
                reason: "metrics must be added before pipeline input is selected",
            });
        }
        let name = name.into();
        if name.is_empty() || self.metrics.iter().any(|entry| entry.name == name) {
            return Err(MetricError::InvalidParameter {
                name: "name",
                value: name,
                reason: "metric name must be non-empty and unique",
            });
        }
        self.metrics.push(NamedMetric {
            name,
            metric: Box::new(metric),
        });
        Ok(self)
    }

    pub fn from_returns(&mut self, returns: &[f64]) -> MetricResult<&mut Self> {
        self.ingest(MetricPipelineInputKind::Returns, returns, None)
    }

    pub fn from_log_returns(&mut self, values: &[f64]) -> MetricResult<&mut Self> {
        self.ingest(MetricPipelineInputKind::LogReturns, values, None)
    }

    pub fn from_equity(&mut self, values: &[f64]) -> MetricResult<&mut Self> {
        self.ingest(MetricPipelineInputKind::Equity, values, None)
    }

    pub fn from_pnl(&mut self, pnl: &[f64], initial_capital: f64) -> MetricResult<&mut Self> {
        self.ingest(
            MetricPipelineInputKind::PeriodPnl,
            pnl,
            Some(initial_capital),
        )
    }

    fn ingest(
        &mut self,
        input_kind: MetricPipelineInputKind,
        values: &[f64],
        initial_capital: Option<f64>,
    ) -> MetricResult<&mut Self> {
        self.select(input_kind)?;
        for entry in &mut self.metrics {
            match input_kind {
                MetricPipelineInputKind::Returns => entry.metric.from_returns(values)?,
                MetricPipelineInputKind::LogReturns => entry.metric.from_log_returns(values)?,
                MetricPipelineInputKind::Equity => entry.metric.from_equity(values)?,
                MetricPipelineInputKind::PeriodPnl => entry.metric.from_pnl(
                    values,
                    initial_capital.expect("period P&L requires initial capital"),
                )?,
            }
        }
        Ok(self)
    }

    fn select(&mut self, input_kind: MetricPipelineInputKind) -> MetricResult<()> {
        if self
            .input_kind
            .is_some_and(|selected| selected != input_kind)
        {
            return Err(MetricError::InvalidParameter {
                name: "input_kind",
                value: format!("{input_kind:?}"),
                reason: "pipeline input domain is already selected",
            });
        }
        if let Some(entry) = self
            .metrics
            .iter()
            .find(|entry| !entry.metric.supports(input_kind))
        {
            return Err(MetricError::InvalidParameter {
                name: "metric",
                value: entry.name.clone(),
                reason: "metric does not support the selected pipeline input domain",
            });
        }
        self.input_kind = Some(input_kind);
        Ok(())
    }

    pub fn append(&mut self, value: f64) -> MetricResult<&mut Self> {
        if self.input_kind.is_none() {
            return Err(MetricError::InvalidParameter {
                name: "input_kind",
                value: "unbound".to_owned(),
                reason: "call a semantic from_* method before append or extend",
            });
        }
        for entry in &mut self.metrics {
            entry.metric.append(value)?;
        }
        Ok(self)
    }

    pub fn extend(&mut self, values: &[f64]) -> MetricResult<&mut Self> {
        if self.input_kind.is_none() {
            return Err(MetricError::InvalidParameter {
                name: "input_kind",
                value: "unbound".to_owned(),
                reason: "call a semantic from_* method before append or extend",
            });
        }
        for entry in &mut self.metrics {
            entry.metric.extend(values)?;
        }
        Ok(self)
    }

    pub fn compute(&mut self) -> Vec<(&str, Option<f64>)> {
        self.metrics
            .iter_mut()
            .map(|entry| (entry.name.as_str(), entry.metric.compute()))
            .collect()
    }

    pub fn value(&mut self) -> Vec<(&str, Option<f64>)> {
        self.compute()
    }

    pub fn reset(&mut self) -> &mut Self {
        for entry in &mut self.metrics {
            entry.metric.reset();
        }
        self
    }

    pub fn len(&self) -> usize {
        self.metrics.first().map_or(0, |entry| entry.metric.len())
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn metric_names(&self) -> Vec<&str> {
        self.metrics
            .iter()
            .map(|entry| entry.name.as_str())
            .collect()
    }
}
