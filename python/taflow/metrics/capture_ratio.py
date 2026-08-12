"""Benchmark-relative geometric capture ratio metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import CaptureRatio as _Native
from ._input import as_paired_metric_series


class CaptureRatio:
    """Compute portfolio CAGR divided by benchmark CAGR.

    Each side is compounded geometrically and annualized as
    ``product(1 + return) ** (periods_per_year / n) - 1`` before division.
    The default ``periods_per_year=252.0`` matches the independent Empyrical
    Reloaded 0.5.12 ``capture`` oracle's daily convention. Warm-up requires one usable aligned
    pair is sufficient; empty state and a zero benchmark CAGR yield ``None``.

    Missing values are omitted pairwise under ``nan_policy="omit"`` or
    rejected under ``"raise"``. Infinities and mismatched input lengths are
    rejected before native mutation. Inputs may be aligned simple returns,
    log returns, positive equity levels, or non-cumulative period P&L. Period
    P&L requires separate positive initial capital for both streams. The first
    equity-level pair establishes baselines and does not increment length.
    Mutating lifecycle methods are fluent; Rust owns conversion and O(1)-memory
    compounded states.
    """

    def __init__(self, periods_per_year: float = 252.0, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(float(periods_per_year), nan_policy)

    def from_returns(self, returns: Any, benchmark_returns: Any) -> "CaptureRatio":
        """Append aligned returns series and return this metric."""
        primary, benchmark = as_paired_metric_series(returns, benchmark_returns)
        self._state.from_returns(primary, benchmark)
        return self

    def from_log_returns(self, log_returns: Any, benchmark_log_returns: Any) -> "CaptureRatio":
        """Append aligned log returns series and return this metric."""
        primary, benchmark = as_paired_metric_series(log_returns, benchmark_log_returns)
        self._state.from_log_returns(primary, benchmark)
        return self

    def from_equity(self, equity: Any, benchmark_equity: Any) -> "CaptureRatio":
        """Append aligned equity series and return this metric."""
        primary, benchmark = as_paired_metric_series(equity, benchmark_equity)
        self._state.from_equity(primary, benchmark)
        return self

    def from_pnl(self, pnl: Any, benchmark_pnl: Any, initial_capital: float, benchmark_initial_capital: float) -> "CaptureRatio":
        """Append aligned period P&L with separate initial capitals."""
        primary, benchmark = as_paired_metric_series(pnl, benchmark_pnl)
        self._state.from_pnl(primary, benchmark, float(initial_capital), float(benchmark_initial_capital))
        return self

    def append(self, value: float, benchmark_value: float) -> "CaptureRatio":
        """Append one aligned pair in the selected domain and return this metric."""
        self._state.append(float(value), float(benchmark_value))
        return self

    def extend(self, values: Any, benchmark_values: Any) -> "CaptureRatio":
        """Append aligned series after equal-length validation and return this metric."""
        primary, benchmark = as_paired_metric_series(values, benchmark_values)
        self._state.extend(primary, benchmark)
        return self

    @property
    def value(self) -> float | None:
        """Return the ratio, or ``None`` until its denominator is defined."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "CaptureRatio":
        """Clear observations, preserve configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable aligned return pairs processed by Rust."""
        return len(self._state)


__all__ = ["CaptureRatio"]
