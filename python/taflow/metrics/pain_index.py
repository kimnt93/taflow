"""Compute mean absolute percentage drawdown over the wealth path."""
from __future__ import annotations
from typing import Any
from .._native.metrics import PainIndex as _Native
from ._input import as_metric_series

class PainIndex:
    """Compute mean absolute percentage drawdown over the wealth path.

    Warm-up returns ``None`` until the registered minimum sample is met. The
    independent oracle mapping is ``PerformanceAnalytics::PainIndex`` from
    numpy 2.4.6.

    Construction stores configuration only. An instance ``from_*`` method
    selects and ingests returns, log returns, equity, or period P&L. Period P&L
    additionally requires positive ``initial_capital``. Rust owns semantic
    conversion and all metric arithmetic. ``append``, ``extend``, ``reset``,
    and every ``from_*`` method mutate and return this metric.
    """

    def __init__(self, nan_policy: str='omit') -> None:
        """Create an empty configured metric without processing a series."""
        self._state = _Native(nan_policy)

    def from_returns(self, returns: Any, *, column: str | None=None) -> 'PainIndex':
        """Select decimal simple returns, append them, and return this metric."""
        self._state.from_returns(as_metric_series(returns, column=column))
        return self

    def from_log_returns(self, log_returns: Any, *, column: str | None=None) -> 'PainIndex':
        """Select log returns, append them, and return this metric."""
        self._state.from_log_returns(as_metric_series(log_returns, column=column))
        return self

    def from_equity(self, equity: Any, *, column: str | None=None) -> 'PainIndex':
        """Select positive equity levels, append them, and return this metric."""
        self._state.from_equity(as_metric_series(equity, column=column))
        return self

    def from_pnl(self, pnl: Any, initial_capital: float, *, column: str | None=None) -> 'PainIndex':
        """Select period P&L, append it, and return this metric."""
        self._state.from_pnl(as_metric_series(pnl, column=column), float(initial_capital))
        return self

    def append(self, value: float) -> 'PainIndex':
        """Append one observation in the selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any, *, column: str | None=None) -> 'PainIndex':
        """Append observations in the selected domain and return this metric."""
        self._state.extend(as_metric_series(values, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return the current native metric value, or ``None`` when undefined."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current native metric value without replaying input."""
        return self._state.compute()

    def reset(self) -> 'PainIndex':
        """Clear observations while preserving configuration and input domain."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable observations processed by Rust."""
        return len(self._state)
__all__ = ['PainIndex']
