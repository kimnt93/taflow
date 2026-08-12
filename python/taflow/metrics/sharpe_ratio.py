"""Annualized excess-return Sharpe ratio metric."""
from __future__ import annotations
from typing import Any
from .._native.metrics import SharpeRatio as _Native
from ._input import as_metric_series

class SharpeRatio:
    """Persistent annualized mean excess-return to sample-deviation ratio.

    Warm-up returns ``None`` until the registered minimum sample is met. The
    independent oracle mapping is ``empyrical.stats.sharpe_ratio`` from
    empyrical-reloaded 0.5.12.

    Construction stores configuration only. Select and ingest the semantic
    input domain with an instance ``from_*`` method before using ``append`` or
    ``extend``. P&L inputs require positive ``initial_capital`` for causal
    conversion to simple returns. Rust owns conversion and arithmetic.
    """

    def __init__(self, periods_per_year: float=252.0, annual_risk_free_rate: float=0.0, nan_policy: str='omit') -> None:
        """Create an empty configured Sharpe-ratio state."""
        self._state = _Native(float(periods_per_year), float(annual_risk_free_rate), nan_policy)

    def from_returns(self, returns: Any, *, column: str | None=None) -> 'SharpeRatio':
        """Select decimal simple returns, append them, and return this metric."""
        self._state.from_returns(as_metric_series(returns, column=column))
        return self

    def from_log_returns(self, log_returns: Any, *, column: str | None=None) -> 'SharpeRatio':
        """Select log returns, append them, and return this metric."""
        self._state.from_log_returns(as_metric_series(log_returns, column=column))
        return self

    def from_equity(self, equity: Any, *, column: str | None=None) -> 'SharpeRatio':
        """Select positive equity levels, append them, and return this metric."""
        self._state.from_equity(as_metric_series(equity, column=column))
        return self

    def from_pnl(self, pnl: Any, initial_capital: float, *, column: str | None=None) -> 'SharpeRatio':
        """Select period P&L, append it, and return this metric."""
        self._state.from_pnl(as_metric_series(pnl, column=column), float(initial_capital))
        return self

    def append(self, value: float) -> 'SharpeRatio':
        """Append one observation in the selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any, *, column: str | None=None) -> 'SharpeRatio':
        """Append observations in the selected domain and return this metric."""
        self._state.extend(as_metric_series(values, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return the current ratio, or ``None`` until it is defined."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying prior observations."""
        return self._state.compute()

    def reset(self) -> 'SharpeRatio':
        """Clear observations while preserving configuration and input domain."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable normalized returns processed by Rust."""
        return len(self._state)
__all__ = ['SharpeRatio']
