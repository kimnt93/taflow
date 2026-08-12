"""Longest strictly losing observation run."""
from __future__ import annotations
from typing import Any
from .._native.metrics import LongestLosingStreak as _Native
from ._input import as_metric_series


class LongestLosingStreak:
    """Compute the longest consecutive run of negative observations.

    The independent oracle is QuantStats 0.0.81 ``consecutive_losses``.
    Strict negatives extend a streak; zero, positive values, and breakevens
    break it. Warm-up requires one usable observation, after which a no-loss
    sample returns integer zero. Inputs are explicit period returns, raw period
    P&L, or closed-trade P&L. Rust owns O(1) state; no annualization occurs.
    """
    def __init__(self, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(nan_policy)

    def from_returns(self, returns: Any, *, column: str | None = None) -> "LongestLosingStreak":
        """Append chronological returns observations and return this metric."""
        self._state.from_returns(as_metric_series(returns, column=column))
        return self

    def from_pnl(self, pnl: Any, *, column: str | None = None) -> "LongestLosingStreak":
        """Append chronological pnl observations and return this metric."""
        self._state.from_pnl(as_metric_series(pnl, column=column))
        return self

    def from_trades(self, trades: Any, *, column: str | None = None) -> "LongestLosingStreak":
        """Append chronological trades observations and return this metric."""
        self._state.from_trades(as_metric_series(trades, column=column))
        return self

    def append(self,value:float)->"LongestLosingStreak":
        """Append one observation and return this metric."""
        self._state.append(float(value));return self
    def extend(self,values:Any,*,column:str|None=None)->"LongestLosingStreak":
        """Append observations and return this metric."""
        self._state.extend(as_metric_series(values,column=column));return self
    @property
    def value(self)->int|None:
        """Return the longest run, or ``None`` during warm-up."""
        return self._state.value
    def compute(self)->int|None:
        """Return current scalar without replaying input."""
        return self._state.compute()
    def reset(self)->"LongestLosingStreak":
        """Clear state and return this metric."""
        self._state.reset();return self
    def __len__(self)->int:
        """Return valid observation count delegated to Rust."""
        return len(self._state)


__all__=["LongestLosingStreak"]
