"""Exact whole-history return tail-ratio metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import TailRatio as _Native
from ._input import as_metric_series


class TailRatio:
    """Compute the magnitude ratio of the 95th and 5th return percentiles.

    TAFlow uses linear-interpolated exact quantiles. The external oracle and
    name mapping is Empyrical Reloaded 0.5.12 ``tail_ratio`` for defined
    results. Scalar warm-up is one usable normalized return. An empty state,
    or a zero-magnitude 5th percentile,
    returns ``None`` rather than emitting a non-finite ratio. Inputs may be
    decimal simple returns, log returns, positive equity levels, or
    non-cumulative period P&L with positive initial capital. The first equity
    level establishes a baseline and does not increment metric length.
    ``nan_policy`` is ``"omit"`` or ``"raise"``; infinities are rejected.
    Mutating lifecycle methods are fluent. Rust retains O(n) observations and
    lazily refreshes the sorted cache; a bulk extension sorts only after all
    accepted observations have been appended. Python performs no arithmetic.
    """

    def __init__(self, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(nan_policy)

    def from_returns(
        self, returns: Any, *, column: str | None = None
    ) -> "TailRatio":
        """Append chronological decimal simple returns and return this metric."""
        self._state.from_returns(as_metric_series(returns, column=column))
        return self

    def from_log_returns(
        self, log_returns: Any, *, column: str | None = None
    ) -> "TailRatio":
        """Append chronological log returns and return this metric."""
        self._state.from_log_returns(as_metric_series(log_returns, column=column))
        return self

    def from_equity(
        self, equity: Any, *, column: str | None = None
    ) -> "TailRatio":
        """Append chronological positive equity levels and return this metric."""
        self._state.from_equity(as_metric_series(equity, column=column))
        return self

    def from_pnl(
        self,
        pnl: Any,
        initial_capital: float,
        *,
        column: str | None = None,
    ) -> "TailRatio":
        """Append period P&L using required positive initial capital."""
        self._state.from_pnl(
            as_metric_series(pnl, column=column), float(initial_capital)
        )
        return self

    def append(self, value: float) -> "TailRatio":
        """Append one value in the selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "TailRatio":
        """Append a chronological series in the selected domain and return this metric."""
        self._state.extend(as_metric_series(values))
        return self

    @property
    def value(self) -> float | None:
        """Return the current ratio, or ``None`` while undefined."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current exact scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "TailRatio":
        """Clear retained observations, preserve configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable normalized returns retained by Rust."""
        return len(self._state)


__all__ = ["TailRatio"]
