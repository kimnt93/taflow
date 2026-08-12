"""Benchmark-relative coefficient of determination metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import CoefficientOfDetermination as _Native
from ._input import as_paired_metric_series


class CoefficientOfDetermination:
    """Compute squared Pearson correlation with a benchmark.

    The metric consumes aligned decimal simple returns and squares their
    Pearson correlation coefficient. The independent oracle is QuantStats 0.0.81
    ``r_squared`` and returns a value on the zero-to-one scale, subject only to
    floating-point rounding. Warm-up requires at least two usable aligned
    pairs. Empty and one-pair states, or either series having zero variance,
    yield ``None``.

    Missing values are omitted pairwise under ``nan_policy="omit"`` or
    rejected under ``"raise"``. Infinities and mismatched series lengths are
    rejected before native mutation. Inputs may be aligned simple returns,
    log returns, positive equity levels, or non-cumulative period P&L. Period
    P&L requires separate positive initial capital for each series. The first
    equity-level pair establishes conversion baselines and does not increment
    the length. Mutating lifecycle methods are fluent; Rust owns conversions
    and allocation-free O(1) paired moments.
    """

    def __init__(self, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(nan_policy)

    def from_returns(self, returns: Any, benchmark_returns: Any) -> "CoefficientOfDetermination":
        """Append aligned returns series and return this metric."""
        primary, benchmark = as_paired_metric_series(returns, benchmark_returns)
        self._state.from_returns(primary, benchmark)
        return self

    def from_log_returns(self, log_returns: Any, benchmark_log_returns: Any) -> "CoefficientOfDetermination":
        """Append aligned log returns series and return this metric."""
        primary, benchmark = as_paired_metric_series(log_returns, benchmark_log_returns)
        self._state.from_log_returns(primary, benchmark)
        return self

    def from_equity(self, equity: Any, benchmark_equity: Any) -> "CoefficientOfDetermination":
        """Append aligned equity series and return this metric."""
        primary, benchmark = as_paired_metric_series(equity, benchmark_equity)
        self._state.from_equity(primary, benchmark)
        return self

    def from_pnl(self, pnl: Any, benchmark_pnl: Any, initial_capital: float, benchmark_initial_capital: float) -> "CoefficientOfDetermination":
        """Append aligned period P&L with separate initial capitals."""
        primary, benchmark = as_paired_metric_series(pnl, benchmark_pnl)
        self._state.from_pnl(primary, benchmark, float(initial_capital), float(benchmark_initial_capital))
        return self

    def append(
        self, value: float, benchmark_value: float
    ) -> "CoefficientOfDetermination":
        """Append one aligned pair in the selected domain and return this metric."""
        self._state.append(float(value), float(benchmark_value))
        return self

    def extend(
        self, values: Any, benchmark_values: Any
    ) -> "CoefficientOfDetermination":
        """Append aligned series after validating length and return this metric."""
        primary, benchmark = as_paired_metric_series(values, benchmark_values)
        self._state.extend(primary, benchmark)
        return self

    @property
    def value(self) -> float | None:
        """Return current squared correlation, or ``None`` until defined."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "CoefficientOfDetermination":
        """Clear observations, preserve input settings, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the usable aligned-return count delegated to native state."""
        return len(self._state)


__all__ = ["CoefficientOfDetermination"]
