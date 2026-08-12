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
    def __init__(self, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(nan_policy)

    def from_returns(self, returns: Any, *, column: str | None = None) -> "MaximumDrawdownDuration":
        """Append chronological returns observations and return this metric."""
        self._state.from_returns(as_metric_series(returns, column=column))
        return self

    def from_log_returns(self, log_returns: Any, *, column: str | None = None) -> "MaximumDrawdownDuration":
        """Append chronological log returns observations and return this metric."""
        self._state.from_log_returns(as_metric_series(log_returns, column=column))
        return self

    def from_equity(self, equity: Any, *, column: str | None = None) -> "MaximumDrawdownDuration":
        """Append chronological equity observations and return this metric."""
        self._state.from_equity(as_metric_series(equity, column=column))
        return self

    def from_pnl(self, pnl: Any, initial_capital: float, *, column: str | None = None) -> "MaximumDrawdownDuration":
        """Append chronological pnl observations and return this metric."""
        self._state.from_pnl(as_metric_series(pnl, column=column), float(initial_capital))
        return self

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
