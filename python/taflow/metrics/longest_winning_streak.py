"""Longest consecutive winning-observation metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import LongestWinningStreak as _Native
from ._input import as_metric_series


class LongestWinningStreak:
    """Compute the longest consecutive run of strictly positive observations.

    A win is strictly greater than zero. Both exact zero and a negative value
    terminate the current streak. The independent oracle is QuantStats 0.0.81
    ``consecutive_wins`` with no aggregation and ``prepare_returns=False``.
    TAFlow returns ``None`` during empty/all-missing warm-up and ``0.0`` for a
    non-empty sample with no win. With ``nan_policy="omit"``, missing values are
    removed and therefore do not break a run; ``"raise"`` rejects them.

    ``from_returns`` consumes decimal simple period returns. ``from_pnl``
    consumes raw, non-cumulative period P&L without initial-capital conversion,
    and ``from_trades`` consumes realized P&L for closed trades. No domain is
    annualized. Infinities are rejected, while return-domain observations below
    -100% are invalid. ``append``, ``extend``, and ``reset`` are fluent. Native
    bulk work releases the GIL, and Rust owns bounded allocation-free O(1)
    streaming state.
    """

    def __init__(self, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(nan_policy)

    def from_returns(self, returns: Any, *, column: str | None = None) -> "LongestWinningStreak":
        """Append chronological returns observations and return this metric."""
        self._state.from_returns(as_metric_series(returns, column=column))
        return self

    def from_pnl(self, pnl: Any, *, column: str | None = None) -> "LongestWinningStreak":
        """Append chronological pnl observations and return this metric."""
        self._state.from_pnl(as_metric_series(pnl, column=column))
        return self

    def from_trades(self, trades: Any, *, column: str | None = None) -> "LongestWinningStreak":
        """Append chronological trades observations and return this metric."""
        self._state.from_trades(as_metric_series(trades, column=column))
        return self

    def append(self, value: float) -> "LongestWinningStreak":
        """Append one value in the selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(
        self, values: Any, *, column: str | None = None
    ) -> "LongestWinningStreak":
        """Append chronological observations and return this metric."""
        self._state.extend(as_metric_series(values, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return the longest winning run, or ``None`` while empty."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "LongestWinningStreak":
        """Clear observations, preserve input configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable observations processed by Rust."""
        return len(self._state)


__all__ = ["LongestWinningStreak"]
