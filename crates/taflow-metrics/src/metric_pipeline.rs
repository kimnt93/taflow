use crate::{
    metrics::{
        AnnualizedReturn, AnnualizedVolatility, AverageDrawdown, AverageLoss, AverageWin,
        BreakevenRate, CalmarRatio, CommonSenseRatio, CompositeProfitabilityConsistencyIndex,
        ConditionalDrawdownAtRisk, DownsideDeviation, EntropicValueAtRisk, Exposure,
        ExposureInputKind, GainToPainRatio, HistoricalExpectedShortfall, HistoricalValueAtRisk,
        KellyCriterion, LongestLosingStreak, LongestWinningStreak, MaximumDrawdown,
        MaximumDrawdownDuration, ModifiedSharpeRatio, OmegaRatio, PainIndex, PainRatio,
        ParametricExpectedShortfall, ParametricValueAtRisk, PayoffRatio, ProbabilisticSharpeRatio,
        ProfitFactor, RecoveryFactor, SharpeRatio, SortinoRatio, StabilityOfTimeSeries, TailRatio,
        TotalReturn, UlcerIndex, UlcerPerformanceIndex, WinRate,
    },
    MetricError, MetricInputKind, MetricInputState, MetricResult, NanPolicy,
};

/// Shared configuration applied when constructing compatible pipeline metrics.
#[derive(Debug, Clone, Copy)]
pub struct MetricPipelineConfiguration {
    pub periods_per_year: f64,
    pub annual_risk_free_rate: f64,
    pub annual_required_return: f64,
    pub annual_benchmark_sharpe_ratio: f64,
    pub cutoff: f64,
    pub confidence_level: f64,
}

impl Default for MetricPipelineConfiguration {
    fn default() -> Self {
        Self {
            periods_per_year: 252.0,
            annual_risk_free_rate: 0.0,
            annual_required_return: 0.0,
            annual_benchmark_sharpe_ratio: 0.0,
            cutoff: 0.05,
            confidence_level: 0.95,
        }
    }
}

macro_rules! metric_nodes {
    ($(($variant:ident, $state:ty, $name:literal)),+ $(,)?) => {
        #[derive(Debug, Clone)]
        enum MetricNode {
            $($variant($state)),+
        }

        impl MetricNode {
            fn name(&self) -> &'static str {
                match self { $(Self::$variant(_) => $name),+ }
            }

            fn append(&mut self, value: f64) -> MetricResult<()> {
                match self {
                    $(Self::$variant(state) => state.append(value).map(|_| ())),+
                }
            }

            fn value(&mut self) -> Option<f64> {
                match self {
                    $(Self::$variant(state) => state.value()),+
                }
            }

            fn reset(&mut self) {
                match self { $(Self::$variant(state) => state.reset()),+ }
            }
        }
    };
}

metric_nodes!(
    (TotalReturn, TotalReturn, "TotalReturn"),
    (AnnualizedReturn, AnnualizedReturn, "AnnualizedReturn"),
    (
        AnnualizedVolatility,
        AnnualizedVolatility,
        "AnnualizedVolatility"
    ),
    (MaximumDrawdown, MaximumDrawdown, "MaximumDrawdown"),
    (DownsideDeviation, DownsideDeviation, "DownsideDeviation"),
    (SharpeRatio, SharpeRatio, "SharpeRatio"),
    (SortinoRatio, SortinoRatio, "SortinoRatio"),
    (CalmarRatio, CalmarRatio, "CalmarRatio"),
    (OmegaRatio, OmegaRatio, "OmegaRatio"),
    (
        HistoricalValueAtRisk,
        HistoricalValueAtRisk,
        "HistoricalValueAtRisk"
    ),
    (
        HistoricalExpectedShortfall,
        HistoricalExpectedShortfall,
        "HistoricalExpectedShortfall"
    ),
    (TailRatio, TailRatio, "TailRatio"),
    (UlcerIndex, UlcerIndex, "UlcerIndex"),
    (
        UlcerPerformanceIndex,
        UlcerPerformanceIndex,
        "UlcerPerformanceIndex"
    ),
    (RecoveryFactor, RecoveryFactor, "RecoveryFactor"),
    (GainToPainRatio, GainToPainRatio, "GainToPainRatio"),
    (PainIndex, PainIndex, "PainIndex"),
    (PainRatio, PainRatio, "PainRatio"),
    (AverageDrawdown, AverageDrawdown, "AverageDrawdown"),
    (
        StabilityOfTimeSeries,
        StabilityOfTimeSeries,
        "StabilityOfTimeSeries"
    ),
    (BreakevenRate, BreakevenRate, "BreakevenRate"),
    (WinRate, WinRate, "WinRate"),
    (AverageWin, AverageWin, "AverageWin"),
    (AverageLoss, AverageLoss, "AverageLoss"),
    (PayoffRatio, PayoffRatio, "PayoffRatio"),
    (ProfitFactor, ProfitFactor, "ProfitFactor"),
    (
        LongestWinningStreak,
        LongestWinningStreak,
        "LongestWinningStreak"
    ),
    (CommonSenseRatio, CommonSenseRatio, "CommonSenseRatio"),
    (
        CompositeProfitabilityConsistencyIndex,
        CompositeProfitabilityConsistencyIndex,
        "CompositeProfitabilityConsistencyIndex"
    ),
    (KellyCriterion, KellyCriterion, "KellyCriterion"),
    (
        ModifiedSharpeRatio,
        ModifiedSharpeRatio,
        "ModifiedSharpeRatio"
    ),
    (
        ProbabilisticSharpeRatio,
        ProbabilisticSharpeRatio,
        "ProbabilisticSharpeRatio"
    ),
    (
        ParametricValueAtRisk,
        ParametricValueAtRisk,
        "ParametricValueAtRisk"
    ),
    (
        ParametricExpectedShortfall,
        ParametricExpectedShortfall,
        "ParametricExpectedShortfall"
    ),
    (
        ConditionalDrawdownAtRisk,
        ConditionalDrawdownAtRisk,
        "ConditionalDrawdownAtRisk"
    ),
    (
        EntropicValueAtRisk,
        EntropicValueAtRisk,
        "EntropicValueAtRisk"
    ),
    (Exposure, Exposure, "Exposure"),
);

#[derive(Debug, Clone)]
enum IntegralMetricNode {
    LongestLosingStreak(LongestLosingStreak),
    MaximumDrawdownDuration(MaximumDrawdownDuration),
}

impl IntegralMetricNode {
    fn name(&self) -> &'static str {
        match self {
            Self::LongestLosingStreak(_) => "LongestLosingStreak",
            Self::MaximumDrawdownDuration(_) => "MaximumDrawdownDuration",
        }
    }

    fn append(&mut self, value: f64) -> MetricResult<()> {
        match self {
            Self::LongestLosingStreak(state) => state.append(value).map(|_| ()),
            Self::MaximumDrawdownDuration(state) => state.append(value).map(|_| ()),
        }
    }

    fn value(&mut self) -> Option<f64> {
        match self {
            Self::LongestLosingStreak(state) => state.value().map(|value| value as f64),
            Self::MaximumDrawdownDuration(state) => state.value().map(|value| value as f64),
        }
    }

    fn reset(&mut self) {
        match self {
            Self::LongestLosingStreak(state) => state.reset(),
            Self::MaximumDrawdownDuration(state) => state.reset(),
        }
    }
}

#[derive(Debug, Clone)]
enum PipelineNode {
    Floating(MetricNode),
    Integral(IntegralMetricNode),
}

impl PipelineNode {
    fn name(&self) -> &'static str {
        match self {
            Self::Floating(node) => node.name(),
            Self::Integral(node) => node.name(),
        }
    }

    fn append(&mut self, value: f64) -> MetricResult<()> {
        match self {
            Self::Floating(node) => node.append(value),
            Self::Integral(node) => node.append(value),
        }
    }

    fn value(&mut self) -> Option<f64> {
        match self {
            Self::Floating(node) => node.value(),
            Self::Integral(node) => node.value(),
        }
    }

    fn reset(&mut self) {
        match self {
            Self::Floating(node) => node.reset(),
            Self::Integral(node) => node.reset(),
        }
    }
}

/// Native fan-out pipeline that normalizes one input stream once.
#[derive(Debug, Clone)]
pub struct MetricPipeline {
    input: MetricInputState,
    nodes: Vec<PipelineNode>,
}

impl MetricPipeline {
    pub const SUPPORTED_METRICS: &'static [&'static str] = &[
        "TotalReturn",
        "AnnualizedReturn",
        "AnnualizedVolatility",
        "MaximumDrawdown",
        "DownsideDeviation",
        "SharpeRatio",
        "SortinoRatio",
        "CalmarRatio",
        "OmegaRatio",
        "HistoricalValueAtRisk",
        "HistoricalExpectedShortfall",
        "TailRatio",
        "UlcerIndex",
        "UlcerPerformanceIndex",
        "RecoveryFactor",
        "GainToPainRatio",
        "PainIndex",
        "PainRatio",
        "AverageDrawdown",
        "MaximumDrawdownDuration",
        "StabilityOfTimeSeries",
        "BreakevenRate",
        "WinRate",
        "AverageWin",
        "AverageLoss",
        "PayoffRatio",
        "ProfitFactor",
        "LongestLosingStreak",
        "LongestWinningStreak",
        "CommonSenseRatio",
        "CompositeProfitabilityConsistencyIndex",
        "KellyCriterion",
        "ModifiedSharpeRatio",
        "ProbabilisticSharpeRatio",
        "ParametricValueAtRisk",
        "ParametricExpectedShortfall",
        "ConditionalDrawdownAtRisk",
        "EntropicValueAtRisk",
        "Exposure",
    ];

    pub fn new(
        input_kind: MetricInputKind,
        metrics: &[String],
        configuration: MetricPipelineConfiguration,
        nan_policy: NanPolicy,
    ) -> MetricResult<Self> {
        if matches!(
            input_kind,
            MetricInputKind::RawPnl | MetricInputKind::Trades
        ) {
            return Err(MetricError::InvalidParameter {
                name: "input_kind",
                value: format!("{input_kind:?}"),
                reason: "metric pipeline requires returns, log returns, equity, or period P&L",
            });
        }
        let names: Vec<&str> = if metrics.is_empty() {
            Self::SUPPORTED_METRICS.to_vec()
        } else {
            metrics.iter().map(String::as_str).collect()
        };
        let mut nodes = Vec::with_capacity(names.len());
        for (index, name) in names.iter().enumerate() {
            if names[..index].contains(name) {
                return Err(MetricError::InvalidParameter {
                    name: "metrics",
                    value: (*name).to_owned(),
                    reason: "metric names must be unique",
                });
            }
            nodes.push(Self::build_node(name, configuration)?);
        }
        Ok(Self {
            input: MetricInputState::new(input_kind, nan_policy)?,
            nodes,
        })
    }

    fn build_node(name: &str, c: MetricPipelineConfiguration) -> MetricResult<PipelineNode> {
        let input = MetricInputKind::Returns;
        let policy = NanPolicy::Raise;
        let floating = match name {
            "TotalReturn" => MetricNode::TotalReturn(TotalReturn::new(input, policy)?),
            "AnnualizedReturn" => MetricNode::AnnualizedReturn(AnnualizedReturn::new(
                input,
                c.periods_per_year,
                policy,
            )?),
            "AnnualizedVolatility" => MetricNode::AnnualizedVolatility(AnnualizedVolatility::new(
                input,
                c.periods_per_year,
                policy,
            )?),
            "MaximumDrawdown" => MetricNode::MaximumDrawdown(MaximumDrawdown::new(input, policy)?),
            "DownsideDeviation" => MetricNode::DownsideDeviation(DownsideDeviation::new(
                input,
                c.periods_per_year,
                c.annual_required_return,
                policy,
            )?),
            "SharpeRatio" => MetricNode::SharpeRatio(SharpeRatio::new(
                input,
                c.periods_per_year,
                c.annual_risk_free_rate,
                policy,
            )?),
            "SortinoRatio" => MetricNode::SortinoRatio(SortinoRatio::new(
                input,
                c.periods_per_year,
                c.annual_required_return,
                policy,
            )?),
            "CalmarRatio" => {
                MetricNode::CalmarRatio(CalmarRatio::new(input, c.periods_per_year, policy)?)
            }
            "OmegaRatio" => MetricNode::OmegaRatio(OmegaRatio::new(
                input,
                c.periods_per_year,
                c.annual_required_return,
                policy,
            )?),
            "HistoricalValueAtRisk" => MetricNode::HistoricalValueAtRisk(
                HistoricalValueAtRisk::new(input, c.cutoff, policy)?,
            ),
            "HistoricalExpectedShortfall" => MetricNode::HistoricalExpectedShortfall(
                HistoricalExpectedShortfall::new(input, c.cutoff, policy)?,
            ),
            "TailRatio" => MetricNode::TailRatio(TailRatio::new(input, policy)?),
            "UlcerIndex" => MetricNode::UlcerIndex(UlcerIndex::new(input, policy)?),
            "UlcerPerformanceIndex" => {
                MetricNode::UlcerPerformanceIndex(UlcerPerformanceIndex::new(input, policy)?)
            }
            "RecoveryFactor" => MetricNode::RecoveryFactor(RecoveryFactor::new(input, policy)?),
            "GainToPainRatio" => MetricNode::GainToPainRatio(GainToPainRatio::new(input, policy)?),
            "PainIndex" => MetricNode::PainIndex(PainIndex::new(input, policy)?),
            "PainRatio" => MetricNode::PainRatio(PainRatio::new(
                input,
                c.periods_per_year,
                c.annual_risk_free_rate,
                policy,
            )?),
            "AverageDrawdown" => MetricNode::AverageDrawdown(AverageDrawdown::new(input, policy)?),
            "StabilityOfTimeSeries" => {
                MetricNode::StabilityOfTimeSeries(StabilityOfTimeSeries::new(input, policy)?)
            }
            "BreakevenRate" => MetricNode::BreakevenRate(BreakevenRate::new(input, policy)?),
            "WinRate" => MetricNode::WinRate(WinRate::new(input, policy)?),
            "AverageWin" => MetricNode::AverageWin(AverageWin::new(input, policy)?),
            "AverageLoss" => MetricNode::AverageLoss(AverageLoss::new(input, policy)?),
            "PayoffRatio" => MetricNode::PayoffRatio(PayoffRatio::new(input, policy)?),
            "ProfitFactor" => MetricNode::ProfitFactor(ProfitFactor::new(input, policy)?),
            "LongestWinningStreak" => {
                MetricNode::LongestWinningStreak(LongestWinningStreak::new(input, policy)?)
            }
            "CommonSenseRatio" => {
                MetricNode::CommonSenseRatio(CommonSenseRatio::new(input, policy)?)
            }
            "CompositeProfitabilityConsistencyIndex" => {
                MetricNode::CompositeProfitabilityConsistencyIndex(
                    CompositeProfitabilityConsistencyIndex::new(input, policy)?,
                )
            }
            "KellyCriterion" => MetricNode::KellyCriterion(KellyCriterion::new(input, policy)?),
            "ModifiedSharpeRatio" => MetricNode::ModifiedSharpeRatio(ModifiedSharpeRatio::new(
                input,
                c.periods_per_year,
                c.annual_risk_free_rate,
                c.confidence_level,
                policy,
            )?),
            "ProbabilisticSharpeRatio" => {
                MetricNode::ProbabilisticSharpeRatio(ProbabilisticSharpeRatio::new(
                    input,
                    c.periods_per_year,
                    c.annual_risk_free_rate,
                    c.annual_benchmark_sharpe_ratio,
                    policy,
                )?)
            }
            "ParametricValueAtRisk" => MetricNode::ParametricValueAtRisk(
                ParametricValueAtRisk::new(input, c.cutoff, policy)?,
            ),
            "ParametricExpectedShortfall" => MetricNode::ParametricExpectedShortfall(
                ParametricExpectedShortfall::new(input, c.cutoff, policy)?,
            ),
            "ConditionalDrawdownAtRisk" => MetricNode::ConditionalDrawdownAtRisk(
                ConditionalDrawdownAtRisk::new(input, c.confidence_level, policy)?,
            ),
            "EntropicValueAtRisk" => {
                MetricNode::EntropicValueAtRisk(EntropicValueAtRisk::new(input, c.cutoff, policy)?)
            }
            "Exposure" => MetricNode::Exposure(Exposure::new(ExposureInputKind::Returns, policy)?),
            "LongestLosingStreak" => {
                return Ok(PipelineNode::Integral(
                    IntegralMetricNode::LongestLosingStreak(LongestLosingStreak::new(
                        input, policy,
                    )?),
                ))
            }
            "MaximumDrawdownDuration" => {
                return Ok(PipelineNode::Integral(
                    IntegralMetricNode::MaximumDrawdownDuration(MaximumDrawdownDuration::new(
                        input, policy,
                    )?),
                ))
            }
            _ => {
                return Err(MetricError::InvalidParameter {
                    name: "metrics",
                    value: name.to_owned(),
                    reason: "unsupported or non-single-return metric",
                })
            }
        };
        Ok(PipelineNode::Floating(floating))
    }

    pub fn append(&mut self, value: f64) -> MetricResult<()> {
        if let Some(simple_return) = self.input.append(value)? {
            for node in &mut self.nodes {
                node.append(simple_return)?;
            }
        }
        Ok(())
    }

    pub fn extend(&mut self, values: &[f64]) -> MetricResult<()> {
        for &value in values {
            self.append(value)?;
        }
        Ok(())
    }

    pub fn value(&mut self) -> Vec<(&'static str, Option<f64>)> {
        self.nodes
            .iter_mut()
            .map(|node| {
                let name = node.name();
                (name, node.value())
            })
            .collect()
    }

    pub fn compute(&mut self) -> Vec<(&'static str, Option<f64>)> {
        self.value()
    }

    pub fn reset(&mut self) {
        self.input.reset();
        for node in &mut self.nodes {
            node.reset();
        }
    }

    pub fn len(&self) -> usize {
        self.input.len()
    }

    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }

    pub fn metric_names(&self) -> Vec<&'static str> {
        self.nodes.iter().map(PipelineNode::name).collect()
    }
}
