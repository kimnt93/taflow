"""Benchmark-relative market beta metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import Beta as _Native
from ._input import as_paired_metric_series


class Beta:
    """Compute covariance with a benchmark divided by benchmark variance.

    The metric uses aligned decimal simple returns and the ratio of sample
    covariance to sample benchmark variance. This matches the independent
    Empyrical Reloaded 0.5.12 ``beta_aligned`` oracle; the degrees-of-freedom
    factors cancel in the ratio. Warm-up requires at least two usable aligned
    pairs. Empty and one-pair states, or a benchmark with zero variance, yield
    ``None``.

    Missing values are omitted pairwise under ``nan_policy="omit"`` or
    rejected under ``"raise"``. Infinities and mismatched series lengths are
    rejected before native mutation. Inputs may be aligned simple returns,
    log returns, positive equity levels, or non-cumulative period P&L. Period
    P&L requires separate positive initial capital for each series. The first
    equity-level pair establishes conversion baselines and does not increment
    the length. Mutating lifecycle methods are fluent; Rust owns conversions
    and allocation-free O(1) metric arithmetic.
    """

    def __init__(self, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(nan_policy)

    def from_returns(self, returns: Any, benchmark_returns: Any) -> "Beta":
        """Append aligned returns series and return this metric."""
        primary, benchmark = as_paired_metric_series(returns, benchmark_returns)
        self._state.from_returns(primary, benchmark)
        return self

    def from_log_returns(self, log_returns: Any, benchmark_log_returns: Any) -> "Beta":
        """Append aligned log returns series and return this metric."""
        primary, benchmark = as_paired_metric_series(log_returns, benchmark_log_returns)
        self._state.from_log_returns(primary, benchmark)
        return self

    def from_equity(self, equity: Any, benchmark_equity: Any) -> "Beta":
        """Append aligned equity series and return this metric."""
        primary, benchmark = as_paired_metric_series(equity, benchmark_equity)
        self._state.from_equity(primary, benchmark)
        return self

    def from_pnl(self, pnl: Any, benchmark_pnl: Any, initial_capital: float, benchmark_initial_capital: float) -> "Beta":
        """Append aligned period P&L with separate initial capitals."""
        primary, benchmark = as_paired_metric_series(pnl, benchmark_pnl)
        self._state.from_pnl(primary, benchmark, float(initial_capital), float(benchmark_initial_capital))
        return self

    def append(self, value: float, benchmark_value: float) -> "Beta":
        """Append one aligned pair in the selected domain and return this metric."""
        self._state.append(float(value), float(benchmark_value))
        return self

    def extend(self, values: Any, benchmark_values: Any) -> "Beta":
        """Append aligned series after validating length and return this metric."""
        primary, benchmark = as_paired_metric_series(values, benchmark_values)
        self._state.extend(primary, benchmark)
        return self

    @property
    def value(self) -> float | None:
        """Return current beta, or ``None`` until the ratio is defined."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "Beta":
        """Clear observations, preserve input settings, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable aligned return pairs processed by Rust."""
        return len(self._state)


__all__ = ["Beta"]
