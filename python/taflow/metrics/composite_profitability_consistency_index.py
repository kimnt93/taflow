"""Composite profitability and consistency metric."""
from __future__ import annotations
from typing import Any
from .._native.metrics import CompositeProfitabilityConsistencyIndex as _Native
from ._input import as_metric_series


class CompositeProfitabilityConsistencyIndex:
    """Multiply Profit Factor, decisive Win Rate, and Payoff Ratio.

    The independent oracle is QuantStats 0.0.81 ``cpc_index``. The complete
    TAFlow class name describes its formula and does not claim that “CPC” has a
    standardized historical expansion. Wins and losses are strict; breakevens
    remain in native length but are excluded from decisive win probability.
    Warm-up requires at least one win and one loss; otherwise the result is
    ``None``. Inputs are decimal returns or realized closed-trade P&L. Rust
    owns all allocation-free O(1) arithmetic and bulk work releases the GIL.
    """
    def __init__(self)->None:raise TypeError("use CompositeProfitabilityConsistencyIndex.from_returns/from_trades")
    @classmethod
    def _create(cls,values:Any,mode:str,*,nan_policy:str="omit",column:str|None=None)->"CompositeProfitabilityConsistencyIndex":state=cls.__new__(cls);state._state=_Native(mode,nan_policy);return state.extend(values,column=column)
    @classmethod
    def from_returns(cls,returns:Any,*,nan_policy:str="omit",column:str|None=None)->"CompositeProfitabilityConsistencyIndex":
        """Construct from decimal period returns."""
        return cls._create(returns,"returns",nan_policy=nan_policy,column=column)
    @classmethod
    def from_trades(cls,trades:Any,*,nan_policy:str="omit",column:str|None=None)->"CompositeProfitabilityConsistencyIndex":
        """Construct from realized closed-trade P&L."""
        return cls._create(trades,"trades",nan_policy=nan_policy,column=column)
    def append(self,value:float)->"CompositeProfitabilityConsistencyIndex":
        """Append one observation and return this metric."""
        self._state.append(float(value));return self
    def extend(self,values:Any,*,column:str|None=None)->"CompositeProfitabilityConsistencyIndex":
        """Append observations and return this metric."""
        self._state.extend(as_metric_series(values,column=column));return self
    @property
    def value(self)->float|None:
        """Return the composite, or ``None`` during warm-up."""
        return self._state.value
    def compute(self)->float|None:
        """Return current scalar without replaying input."""
        return self._state.compute()
    def reset(self)->"CompositeProfitabilityConsistencyIndex":
        """Clear state, preserve domain, and return this metric."""
        self._state.reset();return self
    def __len__(self)->int:
        """Return valid observation count delegated to Rust."""
        return len(self._state)


__all__=["CompositeProfitabilityConsistencyIndex"]
