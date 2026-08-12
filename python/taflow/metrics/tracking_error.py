"""Benchmark-relative active-return dispersion metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import TrackingError as _Native
from ._input import as_paired_metric_series


class TrackingError:
    """Compute sample standard deviation of aligned active returns.

    Active return is primary simple return minus benchmark simple return.
    Sample standard deviation uses one degree of freedom and is multiplied by
    ``sqrt(periods_per_year)`` when ``annualized=True`` (the default). Set
    ``annualized=False`` for per-period tracking error. The independent oracle
    is NumPy ``std(primary - benchmark, ddof=1)``; pandas ``Series.std`` and
    QuantStats' information-ratio denominator provide definition cross-checks.

    At least two usable aligned pairs are required; during this warm-up, empty
    and one-pair states yield ``None``. Missing values are omitted pairwise under the
    default ``nan_policy="omit"`` or rejected by ``"raise"``; infinities and
    mismatched series lengths are rejected. Inputs may be aligned decimal
    simple returns, log returns, positive equity levels, or non-cumulative
    period P&L. P&L conversion requires separate positive initial capital for
    the primary and benchmark series. The first pair of equity levels sets
    baselines and does not increment the length. Mutating lifecycle methods
    are fluent, and Rust owns conversion and O(1)-memory metric arithmetic.
    """

    def __init__(self, periods_per_year: float = 252.0, annualized: bool = True, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(float(periods_per_year), annualized, nan_policy)

    def from_returns(self, returns: Any, benchmark_returns: Any) -> "TrackingError":
        """Append aligned returns series and return this metric."""
        primary, benchmark = as_paired_metric_series(returns, benchmark_returns)
        self._state.from_returns(primary, benchmark)
        return self

    def from_log_returns(self, log_returns: Any, benchmark_log_returns: Any) -> "TrackingError":
        """Append aligned log returns series and return this metric."""
        primary, benchmark = as_paired_metric_series(log_returns, benchmark_log_returns)
        self._state.from_log_returns(primary, benchmark)
        return self

    def from_equity(self, equity: Any, benchmark_equity: Any) -> "TrackingError":
        """Append aligned equity series and return this metric."""
        primary, benchmark = as_paired_metric_series(equity, benchmark_equity)
        self._state.from_equity(primary, benchmark)
        return self

    def from_pnl(self, pnl: Any, benchmark_pnl: Any, initial_capital: float, benchmark_initial_capital: float) -> "TrackingError":
        """Append aligned period P&L with separate initial capitals."""
        primary, benchmark = as_paired_metric_series(pnl, benchmark_pnl)
        self._state.from_pnl(primary, benchmark, float(initial_capital), float(benchmark_initial_capital))
        return self

    def append(self, value: float, benchmark_value: float) -> "TrackingError":
        """Append one aligned pair in the selected domain and return this metric."""
        self._state.append(float(value), float(benchmark_value))
        return self

    def extend(self, values: Any, benchmark_values: Any) -> "TrackingError":
        """Append aligned series after validating equal length and return this metric."""
        primary, benchmark = as_paired_metric_series(values, benchmark_values)
        self._state.extend(primary, benchmark)
        return self

    @property
    def value(self) -> float | None:
        """Return current tracking error, or ``None`` before two usable pairs."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "TrackingError":
        """Clear observations, preserve configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable aligned return pairs processed by Rust."""
        return len(self._state)


__all__ = ["TrackingError"]
