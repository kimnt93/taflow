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
    def __init__(self)->None: raise TypeError("use LongestLosingStreak.from_returns/from_pnl/from_trades")
    @classmethod
    def _create(cls,values:Any,mode:str,*,nan_policy:str="omit",column:str|None=None)->"LongestLosingStreak": state=cls.__new__(cls);state._state=_Native(mode,nan_policy);return state.extend(values,column=column)
    @classmethod
    def from_returns(cls,returns:Any,*,nan_policy:str="omit",column:str|None=None)->"LongestLosingStreak":
        """Construct from decimal period returns."""
        return cls._create(returns,"returns",nan_policy=nan_policy,column=column)
    @classmethod
    def from_pnl(cls,pnl:Any,*,nan_policy:str="omit",column:str|None=None)->"LongestLosingStreak":
        """Construct from raw period P&L."""
        return cls._create(pnl,"pnl",nan_policy=nan_policy,column=column)
    @classmethod
    def from_trades(cls,trades:Any,*,nan_policy:str="omit",column:str|None=None)->"LongestLosingStreak":
        """Construct from closed-trade P&L."""
        return cls._create(trades,"trades",nan_policy=nan_policy,column=column)
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
