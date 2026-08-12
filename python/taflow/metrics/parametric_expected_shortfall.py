"""Gaussian lower-tail expected shortfall metric."""
from __future__ import annotations
from typing import Any
from .._native.metrics import ParametricExpectedShortfall as _Native
from ._input import as_metric_series


class ParametricExpectedShortfall:
    """Estimate signed Gaussian lower-tail expected shortfall.

    The formula is ``mean - sample_std * normal_pdf(normal_ppf(cutoff))/cutoff``.
    The executable oracle is SciPy's normal distribution using NumPy sample
    moments, corresponding to the Gaussian PerformanceAnalytics/Riskfolio
    convention. ``cutoff`` defaults to 0.05 and is a lower-tail probability.
    Warm-up requires two usable returns; constant returns produce their mean.
    Negative output denotes a loss-side return, rather than a positive loss
    magnitude. Rust owns O(1) state, conversion, and missing-value handling.
    """
    def __init__(self, cutoff: float = 0.05, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(float(cutoff), nan_policy)

    def from_returns(
        self, returns: Any, *, column: str | None = None
    ) -> "ParametricExpectedShortfall":
        """Append chronological decimal simple returns and return this metric."""
        self._state.from_returns(as_metric_series(returns, column=column))
        return self

    def from_log_returns(
        self, log_returns: Any, *, column: str | None = None
    ) -> "ParametricExpectedShortfall":
        """Append chronological log returns and return this metric."""
        self._state.from_log_returns(as_metric_series(log_returns, column=column))
        return self

    def from_equity(
        self, equity: Any, *, column: str | None = None
    ) -> "ParametricExpectedShortfall":
        """Append chronological positive equity levels and return this metric."""
        self._state.from_equity(as_metric_series(equity, column=column))
        return self

    def from_pnl(
        self,
        pnl: Any,
        initial_capital: float,
        *,
        column: str | None = None,
    ) -> "ParametricExpectedShortfall":
        """Append period P&L using required positive initial capital."""
        self._state.from_pnl(
            as_metric_series(pnl, column=column), float(initial_capital)
        )
        return self

    def append(self,value:float)->"ParametricExpectedShortfall":
        """Append one selected-domain observation and return this metric."""
        self._state.append(float(value));return self
    def extend(self,values:Any,*,column:str|None=None)->"ParametricExpectedShortfall":
        """Append observations and return this metric."""
        self._state.extend(as_metric_series(values,column=column));return self
    @property
    def value(self)->float|None:
        """Return signed expected shortfall, or ``None`` during warm-up."""
        return self._state.value
    def compute(self)->float|None:
        """Return current scalar without replaying input."""
        return self._state.compute()
    def reset(self)->"ParametricExpectedShortfall":
        """Clear observations, preserve settings, and return this metric."""
        self._state.reset();return self
    def __len__(self)->int:
        """Return usable normalized-return count delegated to Rust."""
        return len(self._state)


__all__=["ParametricExpectedShortfall"]
