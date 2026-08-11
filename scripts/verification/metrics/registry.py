"""Explicit contracts for metric correctness and performance verification.

This registry is intentionally independent from the indicator registry.  A
metric produces a whole-history scalar and has input-domain factories and
annualization semantics that do not apply to aligned indicator histories.
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Any, Literal

ROOT = Path(__file__).resolve().parents[3]
VERIFY_DIR = ROOT / "verify" / "metrics"
CORRECTNESS_EVIDENCE_DIR = VERIFY_DIR / "evidence" / "correctness"
BENCHMARK_EVIDENCE_DIR = VERIFY_DIR / "evidence" / "benchmark"

Verdict = Literal["MATCH", "VARIANT"]


@dataclass(frozen=True)
class Tolerance:
    """Numerical comparison tolerance for one canonical metric."""

    relative: float
    absolute: float


@dataclass(frozen=True)
class OracleSpec:
    """Pinned external implementation and its semantic adapter."""

    distribution: str
    version: str
    import_name: str
    function: str
    source_url: str
    argument_transform: str
    output_normalization: str = "none_to_nan"


@dataclass(frozen=True)
class ParameterRow:
    """One public TAFlow configuration and matching oracle configuration."""

    name: str
    kwargs: tuple[tuple[str, Any], ...] = ()

    def as_kwargs(self) -> dict[str, Any]:
        return dict(self.kwargs)


@dataclass(frozen=True)
class MetricSpec:
    """Frozen public, mathematical, oracle, and benchmark contract."""

    class_name: str
    module: str
    phase: str
    family: str
    factories: tuple[str, ...]
    output_type: str
    minimum_observations: int
    formula: str
    edge_result: str
    oracle: OracleSpec
    tolerance: Tolerance
    parameter_rows: tuple[ParameterRow, ...] = (ParameterRow("default"),)
    expected: Verdict = "MATCH"
    variant_reason: str | None = None
    benchmark_eligible: bool = True
    paired: bool = False

    def load_class(self) -> type:
        """Load the canonical class from the public metrics package only."""
        import importlib

        module = importlib.import_module(f"taflow.metrics.{self.module}")
        cls = getattr(module, self.class_name)
        if cls.__module__ != module.__name__:
            raise TypeError(
                f"{self.class_name} must be defined in {module.__name__}, "
                f"not {cls.__module__}"
            )
        return cls


EMPYRICAL_SOURCE = (
    "https://github.com/stefan-jansen/empyrical-reloaded/blob/"
    "0.5.12/src/empyrical/stats.py"
)


def _empyrical(function: str, transform: str) -> OracleSpec:
    return OracleSpec(
        distribution="empyrical-reloaded",
        version="0.5.12",
        import_name="empyrical",
        function=function,
        source_url=EMPYRICAL_SOURCE,
        argument_transform=transform,
    )


def _quantstats(function: str, transform: str) -> OracleSpec:
    return OracleSpec(
        distribution="quantstats",
        version="0.0.81",
        import_name="quantstats.stats",
        function=function,
        source_url=(
            "https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py"
        ),
        argument_transform=transform,
    )


def _numpy(function: str, transform: str) -> OracleSpec:
    return OracleSpec(
        distribution="numpy",
        version="2.4.6",
        import_name="numpy",
        function=function,
        source_url="https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py",
        argument_transform=transform,
    )


ANNUAL_ROWS = tuple(
    ParameterRow(f"periods_per_year={value:g}", (("periods_per_year", float(value)),))
    for value in (1, 12, 52, 252, 365, 8760)
)
RATIO = Tolerance(relative=1e-11, absolute=1e-13)
GROWTH = Tolerance(relative=1e-12, absolute=1e-14)
QUANTILE = Tolerance(relative=1e-13, absolute=1e-15)
CORRELATION = Tolerance(relative=5e-11, absolute=1e-13)


METRICS: tuple[MetricSpec, ...] = (
    MetricSpec(
        "TotalReturn",
        "total_return",
        "P1",
        "return",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        1,
        "product(1 + return) - 1",
        "empty is None; return below -1 is rejected",
        _empyrical("cum_returns_final", "returns"),
        GROWTH,
    ),
    MetricSpec(
        "AnnualizedReturn",
        "annualized_return",
        "P1",
        "return",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        1,
        "product(1 + return) ** (periods_per_year / n) - 1",
        "empty is None",
        _empyrical("annual_return", "annualization"),
        GROWTH,
        ANNUAL_ROWS,
    ),
    MetricSpec(
        "AnnualizedVolatility",
        "annualized_volatility",
        "P1",
        "risk",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        2,
        "sample standard deviation * sqrt(periods_per_year)",
        "fewer than two is None; constant is 0",
        _empyrical("annual_volatility", "annualization_alpha_two"),
        RATIO,
        ANNUAL_ROWS,
    ),
    MetricSpec(
        "MaximumDrawdown",
        "maximum_drawdown",
        "P1",
        "drawdown",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        1,
        "minimum wealth / prior peak - 1 from phantom wealth 1",
        "empty is None; no decline is 0",
        _empyrical("max_drawdown", "returns"),
        GROWTH,
    ),
    MetricSpec(
        "DownsideDeviation",
        "downside_deviation",
        "P1",
        "risk",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        1,
        "root mean squared shortfall over all observations, annualized",
        "empty is None",
        _empyrical("downside_risk", "annual_required_return"),
        RATIO,
        tuple(
            ParameterRow(f"required={r:g}", (("annual_required_return", r),))
            for r in (0.0, 0.03)
        ),
    ),
    MetricSpec(
        "SharpeRatio",
        "sharpe_ratio",
        "P1",
        "ratio",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        2,
        "mean excess return / sample deviation * sqrt(periods_per_year)",
        "insufficient input or zero deviation is None",
        _empyrical("sharpe_ratio", "annual_risk_free_rate"),
        RATIO,
        tuple(
            ParameterRow(f"risk_free={r:g}", (("annual_risk_free_rate", r),))
            for r in (-0.01, 0.0, 0.03)
        ),
    ),
    MetricSpec(
        "SortinoRatio",
        "sortino_ratio",
        "P1",
        "ratio",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        2,
        "annual excess return / annualized downside deviation",
        "insufficient input or zero downside deviation is None",
        _empyrical("sortino_ratio", "annual_required_return"),
        RATIO,
        tuple(
            ParameterRow(f"required={r:g}", (("annual_required_return", r),))
            for r in (0.0, 0.03)
        ),
    ),
    MetricSpec(
        "CalmarRatio",
        "calmar_ratio",
        "P1",
        "ratio",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        1,
        "annualized return / absolute maximum drawdown",
        "zero drawdown is None",
        _empyrical("calmar_ratio", "annualization"),
        RATIO,
        ANNUAL_ROWS,
    ),
    MetricSpec(
        "OmegaRatio",
        "omega_ratio",
        "P1",
        "ratio",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        2,
        "sum above threshold / absolute sum below threshold",
        "no below-threshold mass is None",
        _empyrical("omega_ratio", "annual_required_return"),
        RATIO,
        tuple(
            ParameterRow(f"required={r:g}", (("annual_required_return", r),))
            for r in (0.0, 0.03)
        ),
    ),
    MetricSpec(
        "HistoricalValueAtRisk",
        "historical_value_at_risk",
        "P1",
        "tail risk",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        1,
        "signed linear lower-tail quantile",
        "empty is None; cutoff is strictly between zero and one",
        _empyrical("value_at_risk", "cutoff"),
        QUANTILE,
        tuple(
            ParameterRow(f"cutoff={c:g}", (("cutoff", c),))
            for c in (0.01, 0.05, 0.10, 0.50)
        ),
    ),
    MetricSpec(
        "HistoricalExpectedShortfall",
        "historical_expected_shortfall",
        "P1",
        "tail risk",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        1,
        "mean of the lowest floor((n - 1) * cutoff) + 1 observations",
        "empty is None; cutoff is strictly between zero and one",
        _empyrical("conditional_value_at_risk", "cutoff"),
        QUANTILE,
        tuple(
            ParameterRow(f"cutoff={c:g}", (("cutoff", c),))
            for c in (0.01, 0.05, 0.10, 0.50)
        ),
    ),
    MetricSpec(
        "TailRatio",
        "tail_ratio",
        "P1",
        "tail risk",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        1,
        "absolute 95th percentile / absolute 5th percentile",
        "empty or zero lower-tail magnitude is None",
        _empyrical("tail_ratio", "returns"),
        RATIO,
    ),
    MetricSpec(
        "TrackingError",
        "tracking_error",
        "P2",
        "benchmark relative",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        2,
        "sample standard deviation of primary minus benchmark returns, optionally annualized",
        "fewer than two aligned pairs is None; constant active return is zero",
        _numpy("std", "active_return_standard_deviation"),
        RATIO,
        (
            ParameterRow("default"),
            ParameterRow("unannualized", (("annualized", False),)),
            ParameterRow("periods_per_year=12", (("periods_per_year", 12.0),)),
        ),
        paired=True,
    ),
    MetricSpec(
        "InformationRatio",
        "information_ratio",
        "P2",
        "benchmark relative",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        2,
        "mean active return divided by sample active-return deviation, optionally annualized",
        "fewer than two aligned pairs or zero tracking error is None",
        _empyrical("excess_sharpe", "paired_returns_optional_annualization"),
        RATIO,
        (
            ParameterRow("default"),
            ParameterRow("unannualized", (("annualized", False),)),
            ParameterRow("periods_per_year=12", (("periods_per_year", 12.0),)),
        ),
        paired=True,
    ),
    MetricSpec(
        "Beta",
        "beta",
        "P2",
        "benchmark relative",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        2,
        "sample covariance divided by sample benchmark variance",
        "fewer than two aligned pairs or zero benchmark variance is None",
        _empyrical("beta_aligned", "paired_returns"),
        RATIO,
        paired=True,
    ),
    MetricSpec(
        "Alpha",
        "alpha",
        "P2",
        "benchmark relative",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        2,
        "annualized regression intercept after per-period risk-free adjustment",
        "fewer than two aligned pairs or zero benchmark variance is None",
        _empyrical("alpha_aligned", "paired_alpha"),
        RATIO,
        tuple(
            ParameterRow(f"risk_free={rate:g}", (("annual_risk_free_rate", rate),))
            for rate in (-0.01, 0.0, 0.03)
        ),
        paired=True,
    ),
    MetricSpec(
        "CoefficientOfDetermination",
        "coefficient_of_determination",
        "P2",
        "benchmark relative",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        2,
        "squared Pearson correlation of aligned returns",
        "fewer than two aligned pairs or zero variance in either series is None",
        _quantstats("r_squared", "paired_r_squared"),
        CORRELATION,
        paired=True,
    ),
    MetricSpec(
        "CaptureRatio",
        "capture_ratio",
        "P2",
        "benchmark relative",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        1,
        "primary geometric annual return divided by benchmark geometric annual return",
        "empty or zero benchmark annual return is None",
        _empyrical("capture", "paired_capture"),
        RATIO,
        ANNUAL_ROWS,
        paired=True,
    ),
    MetricSpec(
        "UpMarketCaptureRatio",
        "up_market_capture_ratio",
        "P2",
        "benchmark relative",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        1,
        "primary CAGR divided by benchmark CAGR where benchmark return is positive",
        "no eligible up-market pair or zero benchmark annual return is None",
        _empyrical("up_capture", "paired_up_capture"),
        RATIO,
        ANNUAL_ROWS,
        paired=True,
    ),
    MetricSpec(
        "DownMarketCaptureRatio",
        "down_market_capture_ratio",
        "P2",
        "benchmark relative",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        1,
        "primary CAGR divided by benchmark CAGR where benchmark return is negative",
        "no eligible down-market pair or zero benchmark annual return is None",
        _empyrical("down_capture", "paired_down_capture"),
        RATIO,
        ANNUAL_ROWS,
        paired=True,
    ),
    MetricSpec(
        "UpDownCaptureRatio",
        "up_down_capture_ratio",
        "P2",
        "benchmark relative",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        1,
        "up-market capture ratio divided by down-market capture ratio",
        "missing either eligible side or zero down-market capture is None",
        _empyrical("up_down_capture", "paired_up_down_capture"),
        RATIO,
        ANNUAL_ROWS,
        paired=True,
    ),
    MetricSpec(
        "TreynorRatio",
        "treynor_ratio",
        "P2",
        "benchmark relative",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        2,
        "CAGR of period excess returns divided by beta of excess primary versus excess benchmark",
        "fewer than two aligned pairs, zero beta, or invalid excess compounding is None",
        _quantstats("treynor_ratio", "paired_treynor_crosscheck"),
        CORRELATION,
        expected="VARIANT",
        variant_reason=(
            "The full convention is pinned to PerformanceAnalytics 2.1.0 source; "
            "the executable QuantStats cross-check is equivalent only at zero "
            "risk-free rate when periods_per_year equals the aligned sample length."
        ),
        benchmark_eligible=False,
        paired=True,
    ),
    MetricSpec(
        "UlcerIndex",
        "ulcer_index",
        "P3",
        "drawdown path quality",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        2,
        "square root of summed squared drawdowns divided by n minus one",
        "fewer than two usable returns is None",
        _quantstats("ulcer_index", "quantstats_datetime_series"),
        RATIO,
    ),
    MetricSpec(
        "UlcerPerformanceIndex",
        "ulcer_performance_index",
        "P3",
        "drawdown path quality",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        2,
        "compounded whole-sample return divided by ulcer index",
        "fewer than two or zero ulcer index is None",
        _quantstats("ulcer_performance_index", "quantstats_ulcer_performance_index"),
        RATIO,
    ),
    MetricSpec(
        "RecoveryFactor",
        "recovery_factor",
        "P3",
        "drawdown path quality",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        1,
        "absolute arithmetic return sum divided by absolute maximum drawdown",
        "empty or zero maximum drawdown is None",
        _quantstats("recovery_factor", "quantstats_recovery_factor"),
        RATIO,
    ),
    MetricSpec(
        "GainToPainRatio",
        "gain_to_pain_ratio",
        "P3",
        "drawdown path quality",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        1,
        "net arithmetic return sum divided by absolute negative return sum",
        "empty or zero negative-return sum is None",
        _quantstats("gain_to_pain_ratio", "quantstats_daily_gain_to_pain"),
        RATIO,
    ),
    MetricSpec(
        "PainIndex",
        "pain_index",
        "P3",
        "drawdown path quality",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        1,
        "mean absolute percentage drawdown over real observations",
        "empty is None",
        OracleSpec(
            distribution="numpy",
            version="2.4.6",
            import_name="numpy",
            function="mean",
            source_url=(
                "https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/"
                "PerformanceAnalytics_2.1.0.tar.gz"
            ),
            argument_transform="performanceanalytics_pain_index_source",
        ),
        GROWTH,
        expected="VARIANT",
        variant_reason=(
            "PerformanceAnalytics 2.1.0 source is pinned and translated for the "
            "executable comparison because R is unavailable in the test environment."
        ),
    ),
    MetricSpec(
        "PainRatio",
        "pain_ratio",
        "P3",
        "drawdown path quality",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        1,
        "geometric annualized excess return divided by mean absolute drawdown",
        "empty or zero pain index is None",
        OracleSpec(
            distribution="numpy", version="2.4.6", import_name="numpy", function="mean",
            source_url="https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/PerformanceAnalytics_2.1.0.tar.gz",
            argument_transform="performanceanalytics_pain_ratio_source",
        ),
        RATIO,
        ANNUAL_ROWS,
        expected="VARIANT",
        variant_reason="PerformanceAnalytics 2.1.0 source translation is used because R is unavailable.",
        benchmark_eligible=False,
    ),
    MetricSpec(
        "AverageDrawdown",
        "average_drawdown",
        "P3",
        "drawdown path quality",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None", 1, "mean trough magnitude across drawdown episodes", "empty is None; no drawdown is zero",
        OracleSpec(distribution="numpy", version="2.4.6", import_name="numpy", function="mean", source_url="https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/PerformanceAnalytics_2.1.0.tar.gz", argument_transform="performanceanalytics_average_drawdown_source"),
        GROWTH, expected="VARIANT", variant_reason="PerformanceAnalytics 2.1.0 source translation is used because R is unavailable.", benchmark_eligible=False,
    ),
    MetricSpec(
        "MaximumDrawdownDuration",
        "maximum_drawdown_duration",
        "P3",
        "drawdown path quality",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "int | None", 1, "maximum negative drawdown episode length including boundary", "no drawdown is None",
        OracleSpec(distribution="numpy", version="2.4.6", import_name="numpy", function="max", source_url="https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/PerformanceAnalytics_2.1.0.tar.gz", argument_transform="performanceanalytics_maximum_drawdown_duration_source"),
        GROWTH, expected="VARIANT", variant_reason="PerformanceAnalytics 2.1.0 findDrawdowns source translation is used because R is unavailable.", benchmark_eligible=False,
    ),
    MetricSpec(
        "StabilityOfTimeSeries",
        "stability_of_time_series",
        "P3",
        "drawdown path quality",
        ("from_returns", "from_equity", "from_pnl", "from_log_returns"),
        "float | None",
        2,
        "R-squared of cumulative log returns regressed on observation index",
        "fewer than two, constant cumulative path, or total loss is None",
        _empyrical("stability_of_timeseries", "returns"),
        CORRELATION,
    ),
    MetricSpec(
        "BreakevenRate",
        "breakeven_rate",
        "P4",
        "period and trade quality",
        ("from_returns", "from_pnl", "from_trades"),
        "float | None",
        1,
        "exact zero observation count divided by valid observation count",
        "empty is None",
        _numpy("mean", "exact_zero_rate"),
        GROWTH,
    ),
    MetricSpec(
        "WinRate",
        "win_rate",
        "P4",
        "period and trade quality",
        ("from_returns", "from_pnl", "from_trades"),
        "float | None", 1, "strict wins divided by decisive nonzero observations", "empty or all breakeven is None",
        _quantstats("win_rate", "quantstats_return_quality"), GROWTH,
    ),
    MetricSpec(
        "AverageWin",
        "average_win",
        "P4",
        "period and trade quality",
        ("from_returns", "from_pnl", "from_trades"),
        "float | None", 1, "mean strictly positive observation", "no win is None",
        _quantstats("avg_win", "quantstats_return_quality"), GROWTH,
    ),
    MetricSpec(
        "AverageLoss", "average_loss", "P4", "period and trade quality",
        ("from_returns", "from_pnl", "from_trades"), "float | None", 1,
        "mean strictly negative observation with signed output", "no loss is None",
        _quantstats("avg_loss", "quantstats_return_quality"), GROWTH,
    ),
    MetricSpec(
        "PayoffRatio", "payoff_ratio", "P4", "period and trade quality",
        ("from_returns", "from_pnl", "from_trades"), "float | None", 1,
        "average win divided by absolute average loss", "missing either side is None",
        _quantstats("payoff_ratio", "quantstats_return_quality"), RATIO,
    ),
    MetricSpec(
        "ProfitFactor",
        "profit_factor",
        "P4",
        "period and trade quality",
        ("from_returns", "from_pnl", "from_trades"),
        "float | None", 1, "gross positive sum divided by absolute gross negative sum", "positive-only is positive infinity; empty or all-zero is None",
        _quantstats("profit_factor", "quantstats_return_quality"), RATIO,
    ),
    MetricSpec(
        "LongestLosingStreak", "longest_losing_streak", "P4", "period and trade quality",
        ("from_returns", "from_pnl", "from_trades"), "int | None", 1,
        "longest consecutive run of strictly negative observations", "empty is None; no loss is zero",
        _quantstats("consecutive_losses", "quantstats_return_quality"), GROWTH,
    ),
    MetricSpec(
        "LongestWinningStreak", "longest_winning_streak", "P4", "period and trade quality",
        ("from_returns", "from_pnl", "from_trades"), "int | None", 1,
        "longest consecutive run of strictly positive observations", "empty is None; no win is zero",
        _quantstats("consecutive_wins", "quantstats_return_quality"), GROWTH,
    ),
    MetricSpec(
        "NetProfit", "net_profit", "P4", "period and trade quality",
        ("from_pnl", "from_trades"), "float | None", 1,
        "sum of raw signed profit and loss observations", "empty is None; all-zero is zero",
        _numpy("sum", "returns"), GROWTH,
    ),
    MetricSpec(
        "GrossProfit", "gross_profit", "P4", "period and trade quality",
        ("from_pnl", "from_trades"), "float | None", 1,
        "sum of strictly positive raw profit observations", "empty is None; no profit is zero",
        _numpy("sum", "strict_positive_sum"), GROWTH,
    ),
    MetricSpec(
        "GrossLoss", "gross_loss", "P4", "period and trade quality",
        ("from_pnl", "from_trades"), "float | None", 1,
        "signed sum of strictly negative raw loss observations", "empty is None; no loss is zero",
        _numpy("sum", "strict_negative_sum"), GROWTH,
    ),
    MetricSpec(
        "Expectancy", "expectancy", "P4", "period and trade quality",
        ("from_pnl", "from_trades"), "float | None", 1,
        "win probability times average win plus loss probability times average loss", "empty is None; breakevens contribute zero",
        _quantstats("avg_win", "quantstats_expectancy_components"), GROWTH,
    ),
    MetricSpec(
        "KellyCriterion", "kelly_criterion", "P4", "period and trade quality",
        ("from_returns", "from_trades"), "float | None", 1,
        "binary Kelly fraction from decisive win probability and payoff ratio", "missing win or loss is None",
        _quantstats("kelly_criterion", "quantstats_return_quality"), RATIO,
    ),
    MetricSpec(
        "SystemQualityNumber", "system_quality_number", "P4", "period and trade quality",
        ("from_trades",), "float | None", 2,
        "square root of trade count times mean trade P&L divided by sample deviation", "fewer than two or zero deviation is None",
        _numpy("mean", "system_quality_number"), RATIO,
        expected="VARIANT", variant_reason="Executable NumPy cross-check follows pinned vectorbt 0.28.1 Trades.sqn source because vectorbt is unavailable.", benchmark_eligible=False,
    ),
    MetricSpec(
        "CommonSenseRatio", "common_sense_ratio", "P4", "period and trade quality",
        ("from_returns",), "float | None", 1,
        "profit factor multiplied by absolute 95th-to-5th percentile tail ratio", "zero loss or zero lower-tail denominator is None",
        _quantstats("common_sense_ratio", "quantstats_return_quality"), RATIO,
    ),
    MetricSpec(
        "CompositeProfitabilityConsistencyIndex",
        "composite_profitability_consistency_index",
        "P4",
        "period and trade quality",
        ("from_returns", "from_trades"),
        "float | None",
        1,
        "profit factor * win rate * payoff ratio",
        "undefined when any component is undefined",
        _quantstats("cpc_index", "quantstats_returns_without_preparation"),
        RATIO,
        variant_reason=(
            "CompositeProfitabilityConsistencyIndex is TAFlow nomenclature "
            "derived from the formula, not a claimed historical expansion of "
            "the QuantStats cpc_index alias."
        ),
    ),
)

REGISTRY = {spec.class_name: spec for spec in METRICS}


def resolve_specs(
    names: list[str] | None, *, available_only: bool = False
) -> list[MetricSpec]:
    """Resolve canonical names and optionally retain only importable classes."""
    selected = list(METRICS) if not names else []
    for name in names or ():
        canonical = next(
            (key for key in REGISTRY if key.casefold() == name.casefold()), None
        )
        if canonical is None:
            raise KeyError(
                f"unknown metric {name!r}; choose from {', '.join(REGISTRY)}"
            )
        selected.append(REGISTRY[canonical])
    if not available_only:
        return selected
    available: list[MetricSpec] = []
    for spec in selected:
        try:
            spec.load_class()
        except (AttributeError, ImportError, ModuleNotFoundError):
            continue
        available.append(spec)
    return available
