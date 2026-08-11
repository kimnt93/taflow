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
    def __init__(self)->None:raise TypeError("use ParametricExpectedShortfall.from_returns/from_equity/from_pnl/from_log_returns")
    @classmethod
    def _create(cls,values:Any,mode:str,*,cutoff:float=.05,initial_equity:float|None=None,nan_policy:str="omit",column:str|None=None)->"ParametricExpectedShortfall":state=cls.__new__(cls);state._state=_Native(mode,float(cutoff),initial_equity,nan_policy);return state.extend(values,column=column)
    @classmethod
    def from_returns(cls,returns:Any,*,cutoff:float=.05,nan_policy:str="omit",column:str|None=None)->"ParametricExpectedShortfall":
        """Construct from decimal simple returns."""
        return cls._create(returns,"returns",cutoff=cutoff,nan_policy=nan_policy,column=column)
    @classmethod
    def from_log_returns(cls,log_returns:Any,*,cutoff:float=.05,nan_policy:str="omit",column:str|None=None)->"ParametricExpectedShortfall":
        """Construct from log returns converted by Rust."""
        return cls._create(log_returns,"log_returns",cutoff=cutoff,nan_policy=nan_policy,column=column)
    @classmethod
    def from_equity(cls,equity:Any,*,cutoff:float=.05,nan_policy:str="omit",column:str|None=None)->"ParametricExpectedShortfall":
        """Construct from positive equity levels converted by Rust."""
        return cls._create(equity,"equity",cutoff=cutoff,nan_policy=nan_policy,column=column)
    @classmethod
    def from_pnl(cls,pnl:Any,*,initial_equity:float,cutoff:float=.05,nan_policy:str="omit",column:str|None=None)->"ParametricExpectedShortfall":
        """Construct from period P&L and required positive initial equity."""
        return cls._create(pnl,"pnl",cutoff=cutoff,initial_equity=float(initial_equity),nan_policy=nan_policy,column=column)
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
