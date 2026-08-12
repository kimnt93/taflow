"""Up-market benchmark capture ratio metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import UpMarketCaptureRatio as _Native
from ._input import as_paired_metric_series


class UpMarketCaptureRatio:
    """Compute capture using only periods with positive benchmark return.

    Aligned pairs are first converted to simple returns and missing values are
    omitted pairwise. Pairs whose normalized benchmark return is not strictly
    positive are then excluded. For the remaining observations, the result is
    primary CAGR divided by benchmark CAGR, where each CAGR is
    ``product(1 + return) ** (periods_per_year / eligible_count) - 1``.
    The independent oracle is Empyrical Reloaded 0.5.12 ``up_capture``, with
    the explicit default ``periods_per_year=252.0`` daily convention.

    One eligible observation is sufficient. Warm-up returns ``None`` for empty
    state or while no positive benchmark period has appeared; a zero benchmark
    CAGR is also ``None``. Under
    ``nan_policy="omit"`` a NaN removes its entire aligned pair; ``"raise"``
    rejects it. Infinities and mismatched lengths are rejected before metric
    mutation. Inputs may be aligned simple returns, log returns, positive equity
    levels, or non-cumulative period P&L. Period P&L requires separate positive
    initial capital for both streams. The first equity pair establishes the
    baselines and does not count as an observation. ``len(metric)`` counts all
    usable normalized aligned pairs; the up-market filter affects only the
    formula state. Mutating lifecycle methods are fluent; Rust owns conversion,
    filtering, compounding, and O(1)-memory state.
    """

    def __init__(self, periods_per_year: float = 252.0, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(float(periods_per_year), nan_policy)

    def from_returns(self, returns: Any, benchmark_returns: Any) -> "UpMarketCaptureRatio":
        """Append aligned returns series and return this metric."""
        primary, benchmark = as_paired_metric_series(returns, benchmark_returns)
        self._state.from_returns(primary, benchmark)
        return self

    def from_log_returns(self, log_returns: Any, benchmark_log_returns: Any) -> "UpMarketCaptureRatio":
        """Append aligned log returns series and return this metric."""
        primary, benchmark = as_paired_metric_series(log_returns, benchmark_log_returns)
        self._state.from_log_returns(primary, benchmark)
        return self

    def from_equity(self, equity: Any, benchmark_equity: Any) -> "UpMarketCaptureRatio":
        """Append aligned equity series and return this metric."""
        primary, benchmark = as_paired_metric_series(equity, benchmark_equity)
        self._state.from_equity(primary, benchmark)
        return self

    def from_pnl(self, pnl: Any, benchmark_pnl: Any, initial_capital: float, benchmark_initial_capital: float) -> "UpMarketCaptureRatio":
        """Append aligned period P&L with separate initial capitals."""
        primary, benchmark = as_paired_metric_series(pnl, benchmark_pnl)
        self._state.from_pnl(primary, benchmark, float(initial_capital), float(benchmark_initial_capital))
        return self

    def append(
        self, value: float, benchmark_value: float
    ) -> "UpMarketCaptureRatio":
        """Append one aligned pair in the selected domain and return this metric."""
        self._state.append(float(value), float(benchmark_value))
        return self

    def extend(
        self, values: Any, benchmark_values: Any
    ) -> "UpMarketCaptureRatio":
        """Append equal-length aligned series and return this metric."""
        primary, benchmark = as_paired_metric_series(values, benchmark_values)
        self._state.extend(primary, benchmark)
        return self

    @property
    def value(self) -> float | None:
        """Return the ratio, or ``None`` until an eligible result is defined."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "UpMarketCaptureRatio":
        """Clear observations, preserve configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable normalized aligned pairs."""
        return len(self._state)


__all__ = ["UpMarketCaptureRatio"]
