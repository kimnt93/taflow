"""Rust-owned fan-out pipeline for whole-history metrics."""

from __future__ import annotations

from collections.abc import Sequence
from typing import Any

from .._native.metrics import MetricPipeline as _Native
from ._input import as_metric_series


class MetricPipeline:
    """Normalize one semantic input stream once and update many metrics.

    Rust owns input conversion, metric states, fan-out, lifecycle, and scalar
    computation. For example, ``from_pnl`` causally converts each period P&L
    observation to a simple return once, then sends that normalized return to
    every selected compatible metric. Python converts the input container once
    and maps native named results; it performs no financial arithmetic.

    ``metrics=None`` selects every supported single-return metric. A sequence
    of canonical class names selects an ordered subset. Configuration is shared
    by metrics that use it: annualization, risk-free/required returns, cutoff,
    confidence, and benchmark Sharpe. Paired benchmark metrics, trade-only
    metrics, weights/covariance metrics, and raw-P&L totals require different
    semantic inputs and are deliberately rejected rather than silently mixed.

    ``append``, ``extend``, and ``reset`` are fluent. ``value`` and ``compute``
    return ``dict[str, float | None]`` in selection order. Empty/warm-up values
    are ``None``. Bulk native fan-out releases the GIL.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError(
            "use MetricPipeline.from_returns/from_log_returns/from_equity/from_pnl"
        )

    @classmethod
    def _create(
        cls,
        values: Any,
        input_mode: str,
        *,
        metrics: Sequence[str] | None = None,
        periods_per_year: float = 252.0,
        annual_risk_free_rate: float = 0.0,
        annual_required_return: float = 0.0,
        annual_benchmark_sharpe_ratio: float = 0.0,
        cutoff: float = 0.05,
        confidence_level: float = 0.95,
        initial_equity: float | None = None,
        nan_policy: str = "omit",
    ) -> "MetricPipeline":
        state = cls.__new__(cls)
        selected = [] if metrics is None else [str(name) for name in metrics]
        state._state = _Native(
            input_mode,
            selected,
            float(periods_per_year),
            float(annual_risk_free_rate),
            float(annual_required_return),
            float(annual_benchmark_sharpe_ratio),
            float(cutoff),
            float(confidence_level),
            initial_equity,
            nan_policy,
        )
        return state.extend(values)

    @classmethod
    def from_returns(
        cls,
        returns: Any,
        *,
        metrics: Sequence[str] | None = None,
        periods_per_year: float = 252.0,
        annual_risk_free_rate: float = 0.0,
        annual_required_return: float = 0.0,
        annual_benchmark_sharpe_ratio: float = 0.0,
        cutoff: float = 0.05,
        confidence_level: float = 0.95,
        nan_policy: str = "omit",
    ) -> "MetricPipeline":
        """Construct from chronological decimal simple returns."""
        return cls._create(
            returns, "returns", metrics=metrics,
            periods_per_year=periods_per_year,
            annual_risk_free_rate=annual_risk_free_rate,
            annual_required_return=annual_required_return,
            annual_benchmark_sharpe_ratio=annual_benchmark_sharpe_ratio,
            cutoff=cutoff, confidence_level=confidence_level, nan_policy=nan_policy,
        )

    @classmethod
    def from_log_returns(
        cls,
        log_returns: Any,
        *,
        metrics: Sequence[str] | None = None,
        periods_per_year: float = 252.0,
        annual_risk_free_rate: float = 0.0,
        annual_required_return: float = 0.0,
        annual_benchmark_sharpe_ratio: float = 0.0,
        cutoff: float = 0.05,
        confidence_level: float = 0.95,
        nan_policy: str = "omit",
    ) -> "MetricPipeline":
        """Construct from chronological log returns normalized once in Rust."""
        return cls._create(
            log_returns, "log_returns", metrics=metrics,
            periods_per_year=periods_per_year,
            annual_risk_free_rate=annual_risk_free_rate,
            annual_required_return=annual_required_return,
            annual_benchmark_sharpe_ratio=annual_benchmark_sharpe_ratio,
            cutoff=cutoff, confidence_level=confidence_level, nan_policy=nan_policy,
        )

    @classmethod
    def from_equity(
        cls,
        equity: Any,
        *,
        metrics: Sequence[str] | None = None,
        periods_per_year: float = 252.0,
        annual_risk_free_rate: float = 0.0,
        annual_required_return: float = 0.0,
        annual_benchmark_sharpe_ratio: float = 0.0,
        cutoff: float = 0.05,
        confidence_level: float = 0.95,
        nan_policy: str = "omit",
    ) -> "MetricPipeline":
        """Construct from positive chronological equity or adjusted-price levels."""
        return cls._create(
            equity, "equity", metrics=metrics,
            periods_per_year=periods_per_year,
            annual_risk_free_rate=annual_risk_free_rate,
            annual_required_return=annual_required_return,
            annual_benchmark_sharpe_ratio=annual_benchmark_sharpe_ratio,
            cutoff=cutoff, confidence_level=confidence_level, nan_policy=nan_policy,
        )

    @classmethod
    def from_pnl(
        cls,
        pnl: Any,
        *,
        initial_equity: float,
        metrics: Sequence[str] | None = None,
        periods_per_year: float = 252.0,
        annual_risk_free_rate: float = 0.0,
        annual_required_return: float = 0.0,
        annual_benchmark_sharpe_ratio: float = 0.0,
        cutoff: float = 0.05,
        confidence_level: float = 0.95,
        nan_policy: str = "omit",
    ) -> "MetricPipeline":
        """Construct from period P&L, converting to returns once in native Rust."""
        return cls._create(
            pnl, "pnl", initial_equity=float(initial_equity), metrics=metrics,
            periods_per_year=periods_per_year,
            annual_risk_free_rate=annual_risk_free_rate,
            annual_required_return=annual_required_return,
            annual_benchmark_sharpe_ratio=annual_benchmark_sharpe_ratio,
            cutoff=cutoff, confidence_level=confidence_level, nan_policy=nan_policy,
        )

    @classmethod
    def supported_metrics(cls) -> tuple[str, ...]:
        """Return canonical metric names accepted by the native pipeline."""
        return tuple(_Native.supported_metrics())

    @property
    def metrics(self) -> tuple[str, ...]:
        """Return selected metric names in stable result order."""
        return tuple(self._state.metrics)

    def append(self, value: float) -> "MetricPipeline":
        """Append one observation in the factory-selected domain."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "MetricPipeline":
        """Append one chronological series through one native fan-out pass."""
        self._state.extend(as_metric_series(values))
        return self

    @property
    def value(self) -> dict[str, float | None]:
        """Return all current named native metric values."""
        return dict(self._state.value)

    def compute(self) -> dict[str, float | None]:
        """Return all current named values without replaying input."""
        return dict(self._state.compute())

    def reset(self) -> "MetricPipeline":
        """Reset the shared converter and every selected metric state."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return usable normalized-return count from the native converter."""
        return len(self._state)


__all__ = ["MetricPipeline"]
