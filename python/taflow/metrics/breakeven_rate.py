"""Exact-zero observation rate metric."""
from __future__ import annotations
from typing import Any
from .._native.metrics import BreakevenRate as _Native
from ._input import as_metric_series


class BreakevenRate:
    """Compute the fraction of valid observations exactly equal to zero.

    The independent oracle is NumPy exact-zero counting. Strictly positive
    observations are wins, strictly negative observations are losses, and both
    positive and negative floating zero are breakeven. Warm-up requires one
    usable observation. Inputs are explicitly period returns, raw period P&L,
    or closed-trade P&L; no annualization or capital conversion is performed.
    Rust owns the allocation-free O(1) state and NaNs are omitted by default.
    """
    def __init__(self, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(nan_policy)

    def from_returns(
        self, returns: Any, *, column: str | None = None
    ) -> "BreakevenRate":
        """Append chronological returns observations and return this metric."""
        self._state.from_returns(as_metric_series(returns, column=column))
        return self

    def from_pnl(
        self, pnl: Any, *, column: str | None = None
    ) -> "BreakevenRate":
        """Append chronological pnl observations and return this metric."""
        self._state.from_pnl(as_metric_series(pnl, column=column))
        return self

    def from_trades(
        self, trades: Any, *, column: str | None = None
    ) -> "BreakevenRate":
        """Append chronological trades observations and return this metric."""
        self._state.from_trades(as_metric_series(trades, column=column))
        return self

    def append(self, value: float) -> "BreakevenRate":
        """Append one observation and return this metric."""
        self._state.append(float(value)); return self
    def extend(self, values: Any, *, column: str | None = None) -> "BreakevenRate":
        """Append observations and return this metric."""
        self._state.extend(as_metric_series(values, column=column)); return self
    @property
    def value(self) -> float | None:
        """Return the breakeven fraction, or ``None`` during warm-up."""
        return self._state.value
    def compute(self) -> float | None:
        """Return the current scalar without replaying input."""
        return self._state.compute()
    def reset(self) -> "BreakevenRate":
        """Clear observations, preserve the domain, and return this metric."""
        self._state.reset(); return self
    def __len__(self) -> int:
        """Return valid observation count delegated to Rust."""
        return len(self._state)


__all__ = ["BreakevenRate"]
