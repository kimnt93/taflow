"""Whole-history compounded-return to ulcer-index ratio metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import UlcerPerformanceIndex as _Native
from ._input import as_metric_series


class UlcerPerformanceIndex:
    """Compute compounded whole-sample return divided by the Ulcer Index.

    The frozen formula is ``(prod(1 + simple_returns) - 1) / ulcer_index``.
    Ulcer Index is ``sqrt(sum(drawdown**2) / (n - 1))`` over a compounded
    wealth path with phantom starting wealth of one. The external oracle is
    QuantStats 0.0.81 ``ulcer_performance_index`` with ``rf=0.0``. That oracle
    uses compounded—not arithmetic—return, subtracts its risk-free input once
    as a whole-sample return, and performs no annualization. TAFlow freezes the
    risk-free input at zero and therefore exposes no ambiguous rate or period
    parameter. Warm-up requires two usable returns. ``value`` and ``compute``
    return ``None`` during warm-up or when the Ulcer Index denominator is zero.
    Negative compounded return produces a valid negative index.

    Select input meaning with ``from_returns``, ``from_log_returns``,
    ``from_equity``, or ``from_pnl``. Returns are decimal simple returns. Rust
    converts log returns with ``expm1`` and positive equity levels into causal
    returns. P&L is non-cumulative period P&L and requires positive initial
    equity. Raw P&L and closed trades are unsupported because the index requires
    a wealth-return path. NaNs are omitted by default or rejected with
    ``nan_policy="raise"``; infinities and simple returns below -100% are always
    rejected. ``append``, ``extend``, and ``reset`` mutate and fluently return
    this instance. Bulk work releases the GIL; arithmetic and state live in Rust.
    """

    def __init__(self, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(nan_policy)

    def from_returns(self, returns: Any, *, column: str | None = None) -> "UlcerPerformanceIndex":
        """Append chronological returns observations and return this metric."""
        self._state.from_returns(as_metric_series(returns, column=column))
        return self

    def from_log_returns(self, log_returns: Any, *, column: str | None = None) -> "UlcerPerformanceIndex":
        """Append chronological log returns observations and return this metric."""
        self._state.from_log_returns(as_metric_series(log_returns, column=column))
        return self

    def from_equity(self, equity: Any, *, column: str | None = None) -> "UlcerPerformanceIndex":
        """Append chronological equity observations and return this metric."""
        self._state.from_equity(as_metric_series(equity, column=column))
        return self

    def from_pnl(self, pnl: Any, initial_capital: float, *, column: str | None = None) -> "UlcerPerformanceIndex":
        """Append chronological pnl observations and return this metric."""
        self._state.from_pnl(as_metric_series(pnl, column=column), float(initial_capital))
        return self

    def append(self, value: float) -> "UlcerPerformanceIndex":
        """Append one value in the selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(
        self, values: Any, *, column: str | None = None
    ) -> "UlcerPerformanceIndex":
        """Append a series in the selected domain and return this metric."""
        self._state.extend(as_metric_series(values, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return the current index, or ``None`` until it is defined."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "UlcerPerformanceIndex":
        """Clear observations, preserve input configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable normalized returns processed by Rust."""
        return len(self._state)


__all__ = ["UlcerPerformanceIndex"]
