"""Longest observation-count drawdown episode metric."""
from __future__ import annotations
from typing import Any
from .._native.metrics import MaximumDrawdownDuration as _Native
from ._input import as_metric_series


class MaximumDrawdownDuration:
    """Return the longest drawdown length in observations.

    The oracle contract is PerformanceAnalytics 2.1.0 ``findDrawdowns`` with
    geometric wealth: only negative-drawdown episodes qualify, and length
    includes the peak/recovery boundary, so one underwater observation has
    length two. An unrecovered current episode uses the same convention.
    Warm-up ends at the first negative drawdown; a path that never falls below
    its running peak returns ``None``. NaNs are omitted by default. Rust owns
    the O(1) state and converts returns, log returns, equity, or period P&L.
    """
    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic factory."""
        raise TypeError("use MaximumDrawdownDuration.from_returns/from_equity/from_pnl/from_log_returns")
    @classmethod
    def _create(cls, values: Any, mode: str, *, initial_equity: float | None = None, nan_policy: str = "omit", column: str | None = None) -> "MaximumDrawdownDuration":
        state = cls.__new__(cls); state._state = _Native(mode, initial_equity=initial_equity, nan_policy=nan_policy); return state.extend(values, column=column)
    @classmethod
    def from_returns(cls, returns: Any, *, nan_policy: str = "omit", column: str | None = None) -> "MaximumDrawdownDuration":
        """Construct from chronological decimal simple returns."""
        return cls._create(returns, "returns", nan_policy=nan_policy, column=column)
    @classmethod
    def from_log_returns(cls, log_returns: Any, *, nan_policy: str = "omit", column: str | None = None) -> "MaximumDrawdownDuration":
        """Construct from chronological log returns converted by Rust."""
        return cls._create(log_returns, "log_returns", nan_policy=nan_policy, column=column)
    @classmethod
    def from_equity(cls, equity: Any, *, nan_policy: str = "omit", column: str | None = None) -> "MaximumDrawdownDuration":
        """Construct from positive chronological equity levels."""
        return cls._create(equity, "equity", nan_policy=nan_policy, column=column)
    @classmethod
    def from_pnl(cls, pnl: Any, *, initial_equity: float, nan_policy: str = "omit", column: str | None = None) -> "MaximumDrawdownDuration":
        """Construct from period P&L and required positive initial equity."""
        return cls._create(pnl, "pnl", initial_equity=float(initial_equity), nan_policy=nan_policy, column=column)
    def append(self, value: float) -> "MaximumDrawdownDuration":
        """Append one observation and return this metric."""
        self._state.append(float(value)); return self
    def extend(self, values: Any, *, column: str | None = None) -> "MaximumDrawdownDuration":
        """Append chronological observations and return this metric."""
        self._state.extend(as_metric_series(values, column=column)); return self
    @property
    def value(self) -> int | None:
        """Return the maximum episode length, or ``None`` when undefined."""
        return self._state.value
    def compute(self) -> int | None:
        """Return the current scalar without replaying input."""
        return self._state.compute()
    def reset(self) -> "MaximumDrawdownDuration":
        """Clear observations, preserve settings, and return this metric."""
        self._state.reset(); return self
    def __len__(self) -> int:
        """Return usable normalized-return count delegated to Rust."""
        return len(self._state)


__all__ = ["MaximumDrawdownDuration"]
