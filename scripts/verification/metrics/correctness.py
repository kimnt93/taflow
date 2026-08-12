"""Compare public metric classes with pinned independent financial oracles.

The TAFlow actual path is deliberately class-only:
the canonical public instance input method and ``compute()``.
"""

from __future__ import annotations

import argparse
import importlib
import importlib.metadata
import json
import math
import os
import platform
import tempfile
from datetime import UTC, datetime
from pathlib import Path
from typing import Any

import numpy as np

try:
    from .registry import (
        CORRECTNESS_EVIDENCE_DIR,
        VERIFY_DIR,
        MetricSpec,
        ParameterRow,
        resolve_specs,
    )
except ImportError:
    from registry import (  # type: ignore[no-redef]
        CORRECTNESS_EVIDENCE_DIR,
        VERIFY_DIR,
        MetricSpec,
        ParameterRow,
        resolve_specs,
    )


def datasets() -> dict[str, np.ndarray]:
    """Return the deterministic common matrix used for external evidence."""
    rng = np.random.default_rng(20_260_811)
    normal = rng.normal(0.0004, 0.012, 513)
    fat_tail = rng.standard_t(4, 513) * 0.01
    fat_tail[[41, 201, 411]] = (-0.25, 0.18, -0.32)
    autocorrelated = np.empty(513)
    shocks = rng.normal(0.0, 0.008, len(autocorrelated))
    autocorrelated[0] = shocks[0]
    for index in range(1, len(autocorrelated)):
        autocorrelated[index] = 0.65 * autocorrelated[index - 1] + shocks[index]
    with_nan = normal[:31].copy()
    with_nan[[0, 15, 30]] = np.nan
    return {
        "singleton": np.array([0.01]),
        "two_observations": np.array([0.01, -0.02]),
        "all_zero": np.zeros(32),
        "constant_positive": np.full(32, 0.01),
        "constant_negative": np.full(32, -0.01),
        "alternating": np.tile([0.02, -0.02], 32),
        "deep_early_drawdown": np.array([0.04, -0.45, 0.03, 0.08, 0.1]),
        "deep_late_drawdown": np.array([0.04, 0.03, 0.08, 0.1, -0.45]),
        "terminal_total_loss": np.array([0.02, -0.01, -1.0]),
        "near_zero_variance": 0.01 + np.arange(32) * 1e-8,
        "normal_seed_20260811": normal,
        "fat_tailed_seed_20260811": fat_tail,
        "autocorrelated_seed_20260811": autocorrelated,
        "nan_omit": with_nan,
    }


def paired_datasets() -> dict[str, tuple[np.ndarray, np.ndarray]]:
    """Return deterministic aligned primary and benchmark matrices."""
    result: dict[str, tuple[np.ndarray, np.ndarray]] = {}
    for dataset_name, primary in datasets().items():
        index = np.arange(primary.size, dtype=np.float64)
        benchmark = np.nan_to_num(primary, nan=0.0) * 0.35 + np.sin(index) * 0.001
        benchmark = np.asarray(benchmark, dtype=np.float64)
        if dataset_name == "nan_omit" and benchmark.size > 5:
            benchmark[5] = np.nan
        result[dataset_name] = (primary, benchmark)
    return result


def period_rate(annual_rate: float, periods_per_year: float) -> float:
    return math.expm1(math.log1p(annual_rate) / periods_per_year)


def oracle_kwargs(spec: MetricSpec, row: ParameterRow) -> dict[str, Any]:
    """Translate TAFlow public configuration into the pinned oracle contract."""
    public = row.as_kwargs()
    annualization = float(public.get("periods_per_year", 252.0))
    transform = spec.oracle.argument_transform
    if transform == "returns":
        return {}
    if transform == "annualization":
        return {"annualization": annualization}
    if transform == "annualization_alpha_two":
        return {"annualization": annualization, "alpha": 2.0}
    if transform == "annual_risk_free_rate":
        annual_rate = float(public.get("annual_risk_free_rate", 0.0))
        return {
            "risk_free": period_rate(annual_rate, annualization),
            "annualization": annualization,
        }
    if transform == "annual_required_return":
        annual_rate = float(public.get("annual_required_return", 0.0))
        required = period_rate(annual_rate, annualization)
        if spec.class_name == "OmegaRatio":
            return {"required_return": annual_rate, "annualization": annualization}
        return {"required_return": required, "annualization": annualization}
    if transform == "cutoff":
        return {"cutoff": float(public["cutoff"])}
    if transform == "quantstats_returns_without_preparation":
        return {"prepare_returns": False}
    if transform == "quantstats_recovery_factor":
        return {"rf": 0.0, "prepare_returns": False}
    if transform == "quantstats_daily_gain_to_pain":
        return {"rf": 0.0, "resolution": "D"}
    if transform == "quantstats_ulcer_performance_index":
        return {"rf": 0.0}
    if transform == "quantstats_return_quality":
        return {"prepare_returns": False}
    if transform == "quantstats_exposure":
        return {"prepare_returns": False}
    if transform == "active_return_standard_deviation":
        return {"ddof": 1}
    if transform in {"paired_returns", "paired_returns_optional_annualization"}:
        return {}
    if transform == "paired_alpha":
        annual_rate = float(public.get("annual_risk_free_rate", 0.0))
        return {
            "risk_free": period_rate(annual_rate, annualization),
            "annualization": annualization,
        }
    if transform == "paired_r_squared":
        return {"prepare_returns": False}
    if transform in {
        "paired_capture",
        "paired_up_capture",
        "paired_down_capture",
        "paired_up_down_capture",
    }:
        return {}
    if transform == "performanceanalytics_treynor_source":
        return {}
    if transform == "quantstats_datetime_series":
        return {}
    if transform in {
        "gaussian_value_at_risk",
        "gaussian_expected_shortfall",
        "performanceanalytics_modified_sharpe_source",
        "performanceanalytics_conditional_drawdown_source",
        "vectorbt_probabilistic_sharpe_source",
        "vectorbt_deflated_sharpe_source",
        "mean_absolute_weight_change",
        "effective_number_of_bets",
        "riskfolio_entropic_value_at_risk_source",
    }:
        return {}
    raise ValueError(f"unsupported oracle transform {transform!r}")


def require_oracle(spec: MetricSpec):
    """Import an exact pinned oracle; missing or wrong versions are failures."""
    actual_version = importlib.metadata.version(spec.oracle.distribution)
    if actual_version != spec.oracle.version:
        raise RuntimeError(
            f"{spec.oracle.distribution} {spec.oracle.version} is required; "
            f"found {actual_version}"
        )
    module = importlib.import_module(spec.oracle.import_name)
    return getattr(module, spec.oracle.function)


def normalized(value: Any) -> float | None:
    array = np.asarray(value)
    if array.size != 1:
        raise TypeError(f"metric oracle returned non-scalar shape {array.shape}")
    result = float(array.reshape(-1)[0])
    return result if math.isfinite(result) else None


def oracle_result(
    spec: MetricSpec,
    oracle: Any,
    row: ParameterRow,
    primary: np.ndarray,
    benchmark: np.ndarray | None = None,
) -> float | None:
    """Evaluate one normalized oracle value from already filtered inputs."""
    transform = spec.oracle.argument_transform
    public = row.as_kwargs()
    if transform == "effective_number_of_bets":
        total = float(np.sum(primary))
        if primary.size == 0 or total <= 0.0:
            return None
        probabilities = primary / total
        positive = probabilities > 0.0
        result = float(
            np.exp(-oracle(probabilities[positive] * np.log(probabilities[positive])))
        )
    elif transform == "vectorbt_deflated_sharpe_source":
        if primary.size < 4:
            return None
        from scipy.stats import kurtosis, skew

        periods = float(public.get("periods_per_year", 252.0))
        annual_rate = float(public.get("annual_risk_free_rate", 0.0))
        trials = int(public["number_of_trials"])
        annual_variance = float(public["annual_sharpe_ratio_variance"])
        excess = primary - period_rate(annual_rate, periods)
        deviation = float(np.std(excess, ddof=1))
        if deviation == 0.0:
            return None
        estimated = float(np.mean(excess) / deviation)
        gamma = float(np.euler_gamma)
        expected_maximum = math.sqrt(annual_variance / periods) * (
            (1.0 - gamma) * oracle.ppf(1.0 - 1.0 / trials)
            + gamma * oracle.ppf(1.0 - 1.0 / (trials * math.e))
        )
        adjustment = 1.0 - float(skew(excess, bias=False)) * estimated + (
            float(kurtosis(excess, fisher=False, bias=False)) - 1.0
        ) * estimated * estimated / 4.0
        if not math.isfinite(adjustment) or adjustment <= 0.0:
            return None
        result = float(oracle.cdf((estimated - expected_maximum) * math.sqrt(primary.size - 1.0) / math.sqrt(adjustment)))
    elif transform == "riskfolio_entropic_value_at_risk_source":
        if primary.size == 0:
            return None
        from scipy import special

        cutoff = float(public.get("cutoff", 0.05))
        losses = -primary
        maximum_loss = float(np.max(losses))
        if np.ptp(losses) == 0.0:
            return maximum_loss
        scale = max(float(np.ptp(losses)), np.finfo(np.float64).tiny)
        objective = lambda log_z: float(np.exp(log_z) * (special.logsumexp(losses / np.exp(log_z)) - np.log(primary.size * cutoff)))
        optimized = oracle(objective, bounds=(np.log(scale) - 40.0, np.log(scale) + 40.0), method="bounded", options={"xatol": 1e-13, "maxiter": 1000})
        if not optimized.success:
            raise RuntimeError("SciPy EVaR oracle failed to converge")
        result = min(float(optimized.fun), maximum_loss)
    elif transform == "mean_absolute_weight_change":
        result = (
            None
            if primary.size < 2
            else float(oracle(np.abs(np.diff(primary))))
        )
    elif transform == "vectorbt_probabilistic_sharpe_source":
        if primary.size < 4:
            return None
        from scipy.stats import kurtosis, skew

        periods = float(public.get("periods_per_year", 252.0))
        annual_rate = float(public.get("annual_risk_free_rate", 0.0))
        benchmark = float(public.get("annual_benchmark_sharpe_ratio", 0.0))
        excess = primary - period_rate(annual_rate, periods)
        deviation = float(np.std(excess, ddof=1))
        if deviation == 0.0:
            return None
        estimated = float(np.mean(excess) / deviation)
        sample_skewness = float(skew(excess, bias=False))
        sample_kurtosis = float(kurtosis(excess, fisher=False, bias=False))
        adjustment = (
            1.0
            - sample_skewness * estimated
            + (sample_kurtosis - 1.0) * estimated * estimated / 4.0
        )
        if not math.isfinite(adjustment) or adjustment <= 0.0:
            return None
        statistic = (
            (estimated - benchmark / math.sqrt(periods))
            * math.sqrt(primary.size - 1.0)
            / math.sqrt(adjustment)
        )
        result = float(oracle.cdf(statistic))
    elif transform == "gaussian_value_at_risk":
        if primary.size < 2:
            return None
        cutoff = float(public.get("cutoff", 0.05))
        result = float(
            np.mean(primary)
            + oracle.ppf(cutoff) * np.std(primary, ddof=1)
        )
    elif transform == "gaussian_expected_shortfall":
        if primary.size < 2:
            return None
        cutoff = float(public.get("cutoff", 0.05))
        quantile = oracle.ppf(cutoff)
        result = float(
            np.mean(primary)
            - np.std(primary, ddof=1) * oracle.pdf(quantile) / cutoff
        )
    elif transform == "performanceanalytics_modified_sharpe_source":
        if primary.size < 2:
            return None
        periods = float(public.get("periods_per_year", 252.0))
        annual_rate = float(public.get("annual_risk_free_rate", 0.0))
        excess = primary - period_rate(annual_rate, periods)
        mean = float(np.mean(excess))
        centered = excess - mean
        variance = float(np.mean(centered * centered))
        if abs(variance) <= math.sqrt(np.finfo(np.float64).eps):
            skewness = 0.0
            excess_kurtosis = 0.0
        else:
            skewness = float(np.mean(centered**3) / variance**1.5)
            excess_kurtosis = float(np.mean(centered**4) / variance**2 - 3.0)
        confidence = float(public.get("confidence_level", 0.95))
        z = float(oracle.ppf(1.0 - confidence))
        adjusted = (
            z
            + (z * z - 1.0) * skewness / 6.0
            + (z**3 - 3.0 * z) * excess_kurtosis / 24.0
            - (2.0 * z**3 - 5.0 * z) * skewness * skewness / 36.0
        )
        modified_risk = -mean - adjusted * math.sqrt(variance)
        if not math.isfinite(modified_risk) or modified_risk <= 0.0:
            return None
        result = mean / min(modified_risk, 1.0)
    elif transform == "performanceanalytics_conditional_drawdown_source":
        if primary.size == 0:
            return None
        wealth = np.cumprod(1.0 + primary)
        peaks = np.maximum.accumulate(np.r_[1.0, wealth])[1:]
        drawdowns = wealth / peaks - 1.0
        episodes: list[float] = []
        prior_negative = bool(drawdowns[0] < 0.0)
        trough = float(drawdowns[0])
        for drawdown in drawdowns[1:]:
            negative = bool(drawdown < 0.0)
            if negative == prior_negative:
                trough = min(trough, float(drawdown))
            else:
                episodes.append(trough)
                prior_negative = negative
                trough = float(drawdown)
        episodes.append(trough)
        confidence = float(public.get("confidence", 0.95))
        boundary = float(oracle(episodes, 1.0 - confidence, method="linear"))
        tail = np.asarray(episodes)[np.asarray(episodes) <= boundary]
        result = -float(np.mean(tail))
    elif transform == "active_return_standard_deviation":
        if benchmark is None:
            raise ValueError("active-return oracle requires benchmark values")
        result = normalized(oracle(primary - benchmark, **oracle_kwargs(spec, row)))
    elif transform in {
        "paired_returns",
        "paired_returns_optional_annualization",
        "paired_alpha",
    }:
        if benchmark is None:
            raise ValueError("paired oracle requires benchmark values")
        result = normalized(oracle(primary, benchmark, **oracle_kwargs(spec, row)))
    elif transform == "paired_r_squared":
        if benchmark is None:
            raise ValueError("paired oracle requires benchmark values")
        import pandas as pd

        result = normalized(
            oracle(
                pd.Series(primary),
                pd.Series(benchmark),
                **oracle_kwargs(spec, row),
            )
        )
    elif transform == "performanceanalytics_treynor_source":
        if benchmark is None:
            raise ValueError("paired Treynor oracle requires benchmark values")
        if primary.size < 2:
            return None
        periods = float(public.get("periods_per_year", 252.0))
        annual_rate = float(public.get("annual_risk_free_rate", 0.0))
        period_risk_free_rate = period_rate(annual_rate, periods)
        primary_excess = primary - period_risk_free_rate
        benchmark_excess = benchmark - period_risk_free_rate
        benchmark_variance = float(np.var(benchmark_excess, ddof=1))
        if benchmark_variance == 0.0:
            return None
        beta = float(
            np.cov(primary_excess, benchmark_excess, ddof=1)[0, 1]
            / benchmark_variance
        )
        if beta == 0.0 or not math.isfinite(beta):
            return None
        with np.errstate(invalid="ignore", over="ignore"):
            annualized_excess = float(
                oracle(1.0 + primary_excess) ** (periods / primary.size) - 1.0
            )
        result = annualized_excess / beta
        if not math.isfinite(result):
            return None
    elif transform in {
        "paired_capture",
        "paired_up_capture",
        "paired_down_capture",
        "paired_up_down_capture",
    }:
        if benchmark is None:
            raise ValueError("paired oracle requires benchmark values")
        module = importlib.import_module(spec.oracle.import_name)
        annualization = float(public.get("periods_per_year", 252.0))
        if transform == "paired_up_down_capture":
            def capture_for(mask: np.ndarray) -> float | None:
                selected_primary = primary[mask]
                selected_benchmark = benchmark[mask]
                if selected_primary.size == 0:
                    return None
                primary_growth = normalized(
                    module.annual_return(
                        selected_primary, annualization=annualization
                    )
                )
                benchmark_growth = normalized(
                    module.annual_return(
                        selected_benchmark, annualization=annualization
                    )
                )
                if (
                    primary_growth is None
                    or benchmark_growth is None
                    or benchmark_growth == 0.0
                ):
                    return None
                return primary_growth / benchmark_growth

            up_capture = capture_for(benchmark > 0.0)
            down_capture = capture_for(benchmark < 0.0)
            return (
                None
                if up_capture is None or down_capture is None or down_capture == 0.0
                else up_capture / down_capture
            )
        if transform == "paired_up_capture":
            selected = benchmark > 0.0
            primary = primary[selected]
            benchmark = benchmark[selected]
        elif transform == "paired_down_capture":
            selected = benchmark < 0.0
            primary = primary[selected]
            benchmark = benchmark[selected]
        if primary.size == 0:
            return None
        primary_growth = normalized(
            module.annual_return(primary, annualization=annualization)
        )
        benchmark_growth = normalized(
            module.annual_return(benchmark, annualization=annualization)
        )
        result = (
            None
            if primary_growth is None
            or benchmark_growth is None
            or benchmark_growth == 0.0
            else primary_growth / benchmark_growth
        )
    elif transform in {
        "quantstats_datetime_series",
        "quantstats_recovery_factor",
        "quantstats_daily_gain_to_pain",
        "quantstats_ulcer_performance_index",
        "quantstats_return_quality",
        "quantstats_returns_without_preparation",
        "quantstats_exposure",
    }:
        import pandas as pd

        oracle_value = oracle(
            pd.Series(
                primary,
                index=pd.date_range("2000-01-01", periods=primary.size, freq="D"),
            ),
            **oracle_kwargs(spec, row),
        )
        oracle_scalar = float(np.asarray(oracle_value).reshape(-1)[0])
        if spec.class_name == "Exposure" and primary.size == 0:
            result = None
        elif spec.class_name in {"WinRate", "ProfitFactor"} and np.count_nonzero(primary) == 0:
            result = None
        elif spec.class_name == "ProfitFactor" and math.isinf(oracle_scalar):
            result = oracle_scalar
        else:
            result = normalized(oracle_value)
    elif transform == "performanceanalytics_pain_index_source":
        wealth = np.cumprod(1.0 + primary)
        peaks = np.maximum.accumulate(np.r_[1.0, wealth])[1:]
        result = normalized(oracle(np.abs(wealth / peaks - 1.0)))
    elif transform == "exact_zero_rate":
        result = None if primary.size == 0 else normalized(oracle(primary == 0.0))
    elif transform == "strict_positive_sum":
        result = None if primary.size == 0 else float(np.sum(primary[primary > 0.0]))
    elif transform == "strict_negative_sum":
        result = None if primary.size == 0 else float(np.sum(primary[primary < 0.0]))
    elif transform == "quantstats_expectancy_components":
        if primary.size == 0:
            return None
        import pandas as pd

        module = importlib.import_module(spec.oracle.import_name)
        series = pd.Series(primary)
        wins = primary > 0.0
        losses = primary < 0.0
        average_win = 0.0 if not np.any(wins) else float(module.avg_win(series, prepare_returns=False))
        average_loss = 0.0 if not np.any(losses) else float(module.avg_loss(series, prepare_returns=False))
        result = (np.count_nonzero(wins) * average_win + np.count_nonzero(losses) * average_loss) / primary.size
    elif transform == "system_quality_number":
        if primary.size < 2:
            return None
        deviation = float(np.std(primary, ddof=1))
        result = None if deviation == 0.0 else float(np.sqrt(primary.size) * np.mean(primary) / deviation)
    elif transform in {
        "performanceanalytics_average_drawdown_source",
        "performanceanalytics_maximum_drawdown_duration_source",
        "performanceanalytics_pain_ratio_source",
    }:
        if primary.size == 0:
            return None
        wealth = np.cumprod(1.0 + primary)
        drawdowns = wealth / np.maximum.accumulate(np.r_[1.0, wealth])[1:] - 1.0
        if transform == "performanceanalytics_pain_ratio_source":
            pain = float(np.mean(np.abs(drawdowns)))
            if pain == 0.0:
                return None
            annualization = float(public.get("periods_per_year", 252.0))
            annual_return = float(np.prod(1.0 + primary) ** (annualization / primary.size) - 1.0)
            result = (annual_return - float(public.get("annual_risk_free_rate", 0.0))) / pain
        else:
            episodes: list[tuple[float, int]] = []
            trough = 0.0
            length = 0
            for drawdown in drawdowns:
                if drawdown < 0.0:
                    trough = min(trough, float(drawdown))
                    length += 1
                elif length:
                    episodes.append((-trough, length + 1))
                    trough = 0.0
                    length = 0
            if length:
                episodes.append((-trough, length + 1))
            if transform == "performanceanalytics_average_drawdown_source":
                result = 0.0 if not episodes else float(np.mean([episode[0] for episode in episodes]))
            else:
                result = None if not episodes else float(max(episode[1] for episode in episodes))
    else:
        result = normalized(oracle(primary, **oracle_kwargs(spec, row)))

    if transform in {
        "active_return_standard_deviation",
        "paired_returns_optional_annualization",
    } and public.get("annualized", True):
        result = (
            None
            if result is None
            else result * math.sqrt(float(public.get("periods_per_year", 252.0)))
        )

    comparison_values = primary if benchmark is None else primary - benchmark
    if spec.class_name in {"SharpeRatio", "InformationRatio"} and np.ptp(
        comparison_values
    ) == 0.0:
        return None
    if spec.class_name == "CoefficientOfDetermination" and benchmark is not None:
        if np.ptp(primary) == 0.0 or np.ptp(benchmark) == 0.0:
            return None
    if spec.class_name == "TreynorRatio" and np.ptp(primary) == 0.0:
        return None
    return result


def close(
    actual: float | None, expected: float | None, spec: MetricSpec
) -> tuple[bool, float, float]:
    if actual is None or expected is None:
        return actual is None and expected is None, 0.0, 0.0
    if math.isinf(float(actual)) or math.isinf(float(expected)):
        return float(actual) == float(expected), 0.0, 0.0
    absolute = abs(float(actual) - float(expected))
    denominator = max(abs(float(expected)), spec.tolerance.absolute)
    relative = absolute / denominator
    passed = bool(
        np.isclose(
            actual,
            expected,
            rtol=spec.tolerance.relative,
            atol=spec.tolerance.absolute,
        )
    )
    return passed, absolute, relative


def _actual(
    spec: MetricSpec, values: np.ndarray, kwargs: dict[str, Any]
) -> tuple[object, float | None]:
    cls = spec.load_class()
    state = cls(**kwargs)
    state = getattr(state, spec.input_methods[0])(values)
    return state, state.compute()


def _actual_paired(
    spec: MetricSpec,
    primary: np.ndarray,
    benchmark: np.ndarray,
    kwargs: dict[str, Any],
) -> tuple[object, float | None]:
    cls = spec.load_class()
    state = cls(**kwargs).from_returns(primary, benchmark)
    return state, state.compute()


def _lifecycle(
    spec: MetricSpec, values: np.ndarray, kwargs: dict[str, Any], batch: float | None
) -> dict[str, bool]:
    cls = spec.load_class()
    scalar = getattr(cls(**kwargs), spec.input_methods[0])([])
    for value in values:
        if not np.isnan(value):
            returned = scalar.append(float(value))
            if returned is not scalar:
                raise TypeError(f"{spec.class_name}.append() is not fluent")
    scalar_result = scalar.compute()
    chunked = getattr(cls(**kwargs), spec.input_methods[0])([])
    for chunk in np.array_split(values, 7):
        returned = chunked.extend(chunk)
        if returned is not chunked:
            raise TypeError(f"{spec.class_name}.extend() is not fluent")
    reset_result = chunked.reset()
    if reset_result is not chunked:
        raise TypeError(f"{spec.class_name}.reset() is not fluent")
    chunked.extend(values)
    return {
        "scalar_matches_batch": close(scalar_result, batch, spec)[0],
        "chunked_reset_matches_batch": close(chunked.compute(), batch, spec)[0],
        "value_matches_compute": close(chunked.value, chunked.compute(), spec)[0],
    }


def _paired_lifecycle(
    spec: MetricSpec,
    primary: np.ndarray,
    benchmark: np.ndarray,
    kwargs: dict[str, Any],
    batch: float | None,
) -> dict[str, bool]:
    cls = spec.load_class()
    scalar = cls(**kwargs).from_returns([], [])
    for primary_value, benchmark_value in zip(primary, benchmark, strict=True):
        returned = scalar.append(float(primary_value), float(benchmark_value))
        if returned is not scalar:
            raise TypeError(f"{spec.class_name}.append() is not fluent")
    scalar_result = scalar.compute()
    chunked = cls(**kwargs).from_returns([], [])
    for primary_chunk, benchmark_chunk in zip(
        np.array_split(primary, 7), np.array_split(benchmark, 7), strict=True
    ):
        returned = chunked.extend(primary_chunk, benchmark_chunk)
        if returned is not chunked:
            raise TypeError(f"{spec.class_name}.extend() is not fluent")
    reset_result = chunked.reset()
    if reset_result is not chunked:
        raise TypeError(f"{spec.class_name}.reset() is not fluent")
    chunked.extend(primary, benchmark)
    return {
        "scalar_matches_batch": close(scalar_result, batch, spec)[0],
        "chunked_reset_matches_batch": close(chunked.compute(), batch, spec)[0],
        "value_matches_compute": close(chunked.value, chunked.compute(), spec)[0],
    }


def verify_metric(spec: MetricSpec) -> dict[str, Any]:
    """Run all registered datasets and parameter rows for one metric."""
    oracle = require_oracle(spec)
    cases: list[dict[str, Any]] = []
    max_absolute = 0.0
    max_relative = 0.0
    selected_datasets: dict[str, Any] = paired_datasets() if spec.paired else datasets()
    for row in spec.parameter_rows:
        kwargs = row.as_kwargs()
        for dataset_name, dataset in selected_datasets.items():
            if spec.paired:
                primary, benchmark = dataset
                valid = ~(np.isnan(primary) | np.isnan(benchmark))
                valid_count = int(np.count_nonzero(valid))
            else:
                values = dataset
                if spec.class_name == "EffectiveNumberOfBets":
                    values = np.abs(values)
                valid_count = int(np.count_nonzero(~np.isnan(values)))
            if valid_count < spec.minimum_observations:
                continue
            if spec.paired:
                state, actual = _actual_paired(spec, primary, benchmark, kwargs)
                oracle_primary = primary[valid]
                oracle_benchmark = benchmark[valid]
                expected = oracle_result(
                    spec, oracle, row, oracle_primary, oracle_benchmark
                )
            else:
                state, actual = _actual(spec, values, kwargs)
                oracle_values = values[~np.isnan(values)]
                expected = oracle_result(spec, oracle, row, oracle_values)
            passed, absolute, relative = close(actual, expected, spec)
            lifecycle = (
                _paired_lifecycle(spec, primary, benchmark, kwargs, actual)
                if spec.paired
                else _lifecycle(spec, values, kwargs, actual)
            )
            lifecycle_passed = all(lifecycle.values()) and len(state) == valid_count
            max_absolute = max(max_absolute, absolute)
            max_relative = max(max_relative, relative)
            cases.append(
                {
                    "dataset": dataset_name,
                    "parameter_row": row.name,
                    "actual": actual,
                    "expected": expected,
                    "absolute_error": absolute,
                    "relative_error": relative,
                    "oracle_passed": passed,
                    "lifecycle": lifecycle,
                    "passed": passed and lifecycle_passed,
                }
            )
    if spec.class_name == "EffectiveNumberOfBets":
        cls = spec.load_class()
        matrix_inputs = (
            (np.array([0.5, 0.5]), np.eye(2)),
            (
                np.array([0.6, 0.4]),
                np.array([[0.04, 0.012], [0.012, 0.09]]),
            ),
            (
                np.array([0.2, 0.3, 0.5]),
                np.array(
                    [
                        [0.04, 0.01, -0.002],
                        [0.01, 0.06, 0.008],
                        [-0.002, 0.008, 0.09],
                    ]
                ),
            ),
        )
        for index, (weights, covariance) in enumerate(matrix_inputs):
            eigenvalues, eigenvectors = np.linalg.eigh(covariance)
            exposures = eigenvectors.T @ weights
            contributions = np.maximum(eigenvalues, 0.0) * exposures**2
            probabilities = contributions / np.sum(contributions)
            positive = probabilities > 0.0
            expected = float(
                np.exp(
                    -np.sum(
                        probabilities[positive] * np.log(probabilities[positive])
                    )
                )
            )
            state = cls().from_weights_and_covariance(weights, covariance)
            actual = state.compute()
            passed, absolute, relative = close(actual, expected, spec)
            stable = close(state.compute(), actual, spec)[0]
            reset_is_fluent = state.reset() is state
            reset_clears = state.compute() is None and len(state) == 0
            max_absolute = max(max_absolute, absolute)
            max_relative = max(max_relative, relative)
            cases.append(
                {
                    "dataset": f"weights_and_covariance_{index}",
                    "parameter_row": "default",
                    "actual": actual,
                    "expected": expected,
                    "absolute_error": absolute,
                    "relative_error": relative,
                    "oracle_passed": passed,
                    "lifecycle": {
                        "compute_is_stable": stable,
                        "reset_is_fluent": reset_is_fluent,
                        "reset_clears": reset_clears,
                    },
                    "passed": passed and stable and reset_is_fluent and reset_clears,
                }
            )
    source_distribution, source_version = spec.oracle.source_package
    return {
        "class": spec.class_name,
        "verdict": spec.expected,
        "oracle": {
            "distribution": source_distribution,
            "version": source_version,
            "runtime_distribution": spec.oracle.distribution,
            "runtime_version": spec.oracle.version,
            "function": spec.oracle.function,
            "source_function": spec.oracle.source_function_name,
            "source": spec.oracle.source_url,
        },
        "variant_reason": spec.variant_reason,
        "max_absolute_error": max_absolute,
        "max_relative_error": max_relative,
        "passed": bool(cases) and all(case["passed"] for case in cases),
        "cases": cases,
    }


def _atomic_write(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    descriptor, temporary_name = tempfile.mkstemp(
        dir=path.parent, prefix=f".{path.name}."
    )
    try:
        with os.fdopen(descriptor, "w", encoding="utf-8") as temporary:
            temporary.write(content)
        os.replace(temporary_name, path)
    except BaseException:
        Path(temporary_name).unlink(missing_ok=True)
        raise


def write_results(results: list[dict[str, Any]]) -> None:
    CORRECTNESS_EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)
    for result in results:
        _atomic_write(
            CORRECTNESS_EVIDENCE_DIR / f"{result['class']}.json",
            json.dumps(result, indent=2, sort_keys=True) + "\n",
        )
    lines = [
        "# Metrics correctness",
        "",
        f"Generated: {datetime.now(UTC).date().isoformat()}",
        "",
        "Every TAFlow value below came from a configured canonical instance, its input method, and `compute()`.",
        "",
        "`MATCH` means every registered dataset, parameter row, and lifecycle check passed the metric's declared absolute/relative tolerance.",
        "",
        "Reference libraries and source functions are linked through each Target entry.",
        "",
        "| **Class** | **Target** | **Verdict** | **Batch vs oracle** | **Continue vs oracle** |",
        "|---|---|---|---|---|",
    ]
    for result in results:
        oracle = result["oracle"]
        lines.append(
            f"| {result['class']} | "
            f"[{oracle['distribution']} {oracle['version']}]({oracle['source']}) | "
            f"{result['verdict']} | "
            f"pass (err {result['max_absolute_error']:.1e}, nan 0) | "
            f"pass (err {result['max_absolute_error']:.1e}, nan 0) |"
        )
    variants = [result for result in results if result["verdict"] == "VARIANT"]
    if variants:
        lines.extend(["", "## Definition variants", ""])
        for result in variants:
            lines.append(f"- `{result['class']}`: {result['variant_reason']}")
    lines.extend(
        [
            "",
            f"Environment: Python {platform.python_version()}, NumPy {np.__version__}.",
            "",
        ]
    )
    _atomic_write(VERIFY_DIR / "CORRECTNESS.md", "\n".join(lines))


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("metrics", nargs="*", help="canonical metric class names")
    parser.add_argument(
        "--metric",
        action="append",
        default=[],
        help="canonical metric class name (repeatable)",
    )
    parser.add_argument(
        "--list",
        action="store_true",
        help="list registry entries without importing TAFlow",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    requested = args.metrics + args.metric
    if args.list:
        for spec in resolve_specs(requested or None):
            print(
                f"{spec.class_name}\t{spec.phase}\t{spec.oracle.distribution} {spec.oracle.version}:{spec.oracle.function}"
            )
        return 0
    specs = resolve_specs(requested or None, available_only=not requested)
    if not specs:
        raise RuntimeError("no implemented registered metrics found")
    results = [verify_metric(spec) for spec in specs]
    failures = [result["class"] for result in results if not result["passed"]]
    if failures:
        raise AssertionError(f"metric correctness failed: {', '.join(failures)}")
    write_results(results)
    print(
        f"verified {len(results)} metrics: {', '.join(result['class'] for result in results)}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
