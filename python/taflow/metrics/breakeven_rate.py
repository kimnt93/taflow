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
    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic factory."""
        raise TypeError("use BreakevenRate.from_returns/from_pnl/from_trades")
    @classmethod
    def _create(cls, values: Any, mode: str, *, nan_policy: str = "omit", column: str | None = None) -> "BreakevenRate":
        state = cls.__new__(cls); state._state = _Native(mode, nan_policy); return state.extend(values, column=column)
    @classmethod
    def from_returns(cls, returns: Any, *, nan_policy: str = "omit", column: str | None = None) -> "BreakevenRate":
        """Construct from decimal period returns."""
        return cls._create(returns, "returns", nan_policy=nan_policy, column=column)
    @classmethod
    def from_pnl(cls, pnl: Any, *, nan_policy: str = "omit", column: str | None = None) -> "BreakevenRate":
        """Construct from raw non-cumulative period P&L."""
        return cls._create(pnl, "pnl", nan_policy=nan_policy, column=column)
    @classmethod
    def from_trades(cls, trades: Any, *, nan_policy: str = "omit", column: str | None = None) -> "BreakevenRate":
        """Construct from closed-trade P&L observations."""
        return cls._create(trades, "trades", nan_policy=nan_policy, column=column)
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
