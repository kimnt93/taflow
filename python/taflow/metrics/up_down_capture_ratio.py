"""Combined up-market and down-market benchmark capture metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import UpDownCaptureRatio as _Native
from ._input import as_paired_metric_series


class UpDownCaptureRatio:
    """Compute up-market capture divided by down-market capture.

    Rust converts aligned streams to simple returns, omits missing values
    pairwise, and separates observations by benchmark sign. Up-market and
    down-market capture are each primary CAGR divided by benchmark CAGR over
    their respective eligible observations; this metric divides the former by
    the latter. Every CAGR uses
    ``product(1 + return) ** (periods_per_year / eligible_count) - 1``.
    The independent oracle is Empyrical Reloaded 0.5.12
    ``up_down_capture``, with the daily convention represented explicitly by
    ``periods_per_year=252.0``.

    Warm-up is ``None`` until both a strictly positive and a strictly negative
    benchmark observation exist. A zero benchmark CAGR on either side or zero
    down-market capture also yields ``None``. Benchmark-zero pairs count toward
    ``len(metric)`` but enter neither formula. Under ``nan_policy="omit"``, a
    NaN removes its whole aligned pair; ``"raise"`` rejects it. Infinities and
    mismatched lengths are rejected before metric mutation.

    Input methods accept aligned simple returns, log returns, positive equity
    levels, or non-cumulative period P&L. P&L conversion requires separate
    positive initial capital for both streams. The first equity pair establishes
    the two baselines and does not count as a normalized observation.
    ``append``, ``extend``, and ``reset`` are fluent. Rust owns all conversion,
    filtering, compounding, and constant-memory arithmetic.
    """

    def __init__(self, periods_per_year: float = 252.0, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(float(periods_per_year), nan_policy)

    def from_returns(self, returns: Any, benchmark_returns: Any) -> "UpDownCaptureRatio":
        """Append aligned returns series and return this metric."""
        primary, benchmark = as_paired_metric_series(returns, benchmark_returns)
        self._state.from_returns(primary, benchmark)
        return self

    def from_log_returns(self, log_returns: Any, benchmark_log_returns: Any) -> "UpDownCaptureRatio":
        """Append aligned log returns series and return this metric."""
        primary, benchmark = as_paired_metric_series(log_returns, benchmark_log_returns)
        self._state.from_log_returns(primary, benchmark)
        return self

    def from_equity(self, equity: Any, benchmark_equity: Any) -> "UpDownCaptureRatio":
        """Append aligned equity series and return this metric."""
        primary, benchmark = as_paired_metric_series(equity, benchmark_equity)
        self._state.from_equity(primary, benchmark)
        return self

    def from_pnl(self, pnl: Any, benchmark_pnl: Any, initial_capital: float, benchmark_initial_capital: float) -> "UpDownCaptureRatio":
        """Append aligned period P&L with separate initial capitals."""
        primary, benchmark = as_paired_metric_series(pnl, benchmark_pnl)
        self._state.from_pnl(primary, benchmark, float(initial_capital), float(benchmark_initial_capital))
        return self

    def append(
        self, value: float, benchmark_value: float
    ) -> "UpDownCaptureRatio":
        """Append one aligned pair in the selected domain and return this metric."""
        self._state.append(float(value), float(benchmark_value))
        return self

    def extend(
        self, values: Any, benchmark_values: Any
    ) -> "UpDownCaptureRatio":
        """Append equal-length aligned series and return this metric."""
        primary, benchmark = as_paired_metric_series(values, benchmark_values)
        self._state.extend(primary, benchmark)
        return self

    @property
    def value(self) -> float | None:
        """Return the ratio, or ``None`` while either side is undefined."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "UpDownCaptureRatio":
        """Clear observations, preserve configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable normalized aligned pairs."""
        return len(self._state)


__all__ = ["UpDownCaptureRatio"]
