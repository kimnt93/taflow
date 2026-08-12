"""Down-market benchmark capture ratio metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import DownMarketCaptureRatio as _Native
from ._input import as_paired_metric_series


class DownMarketCaptureRatio:
    """Compute primary CAGR divided by benchmark CAGR in down markets.

    After Rust normalizes the two aligned streams to simple returns, only pairs
    whose benchmark return is strictly negative are compounded. Each filtered
    stream is annualized as
    ``product(1 + return) ** (periods_per_year / eligible_count) - 1`` before
    division. The independent oracle is Empyrical Reloaded 0.5.12 ``down_capture`` with its
    daily default represented explicitly by ``periods_per_year=252.0``. During
    warm-up, before the first down-market pair, the value is ``None``; a zero
    benchmark CAGR also yields ``None``.

    Missing values are omitted pairwise under ``nan_policy="omit"`` or
    rejected under ``"raise"``. Infinities and mismatched input lengths are
    rejected before native mutation. Inputs may be aligned simple returns,
    log returns, positive equity levels, or non-cumulative period P&L. Period
    P&L requires separate positive initial capital for both streams. The first
    equity-level pair establishes baselines and does not increment length.
    ``append``, ``extend``, and ``reset`` are fluent; ``value`` and ``compute``
    expose the current scalar. Rust owns conversion, filtering, compounding,
    and the allocation-free-after-construction O(1)-memory state.
    """

    def __init__(self, periods_per_year: float = 252.0, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(float(periods_per_year), nan_policy)

    def from_returns(self, returns: Any, benchmark_returns: Any) -> "DownMarketCaptureRatio":
        """Append aligned returns series and return this metric."""
        primary, benchmark = as_paired_metric_series(returns, benchmark_returns)
        self._state.from_returns(primary, benchmark)
        return self

    def from_log_returns(self, log_returns: Any, benchmark_log_returns: Any) -> "DownMarketCaptureRatio":
        """Append aligned log returns series and return this metric."""
        primary, benchmark = as_paired_metric_series(log_returns, benchmark_log_returns)
        self._state.from_log_returns(primary, benchmark)
        return self

    def from_equity(self, equity: Any, benchmark_equity: Any) -> "DownMarketCaptureRatio":
        """Append aligned equity series and return this metric."""
        primary, benchmark = as_paired_metric_series(equity, benchmark_equity)
        self._state.from_equity(primary, benchmark)
        return self

    def from_pnl(self, pnl: Any, benchmark_pnl: Any, initial_capital: float, benchmark_initial_capital: float) -> "DownMarketCaptureRatio":
        """Append aligned period P&L with separate initial capitals."""
        primary, benchmark = as_paired_metric_series(pnl, benchmark_pnl)
        self._state.from_pnl(primary, benchmark, float(initial_capital), float(benchmark_initial_capital))
        return self

    def append(
        self, value: float, benchmark_value: float
    ) -> "DownMarketCaptureRatio":
        """Append one aligned pair in the selected domain and return this metric."""
        self._state.append(float(value), float(benchmark_value))
        return self

    def extend(
        self, values: Any, benchmark_values: Any
    ) -> "DownMarketCaptureRatio":
        """Append aligned series after validation and return this metric."""
        primary, benchmark = as_paired_metric_series(values, benchmark_values)
        self._state.extend(primary, benchmark)
        return self

    @property
    def value(self) -> float | None:
        """Return the ratio, or ``None`` until a valid denominator exists."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "DownMarketCaptureRatio":
        """Clear observations, preserve configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable aligned normalized return pairs."""
        return len(self._state)


__all__ = ["DownMarketCaptureRatio"]
