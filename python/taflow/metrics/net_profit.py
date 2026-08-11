"""Net raw profit and loss metric."""
from __future__ import annotations
from typing import Any
from .._native.metrics import NetProfit as _Native
from ._input import as_metric_series


class NetProfit:
    """Sum raw P&L observations with their original signs.

    The independent oracle is NumPy ``sum`` and the result equals Gross Profit
    plus signed Gross Loss. Warm-up requires one usable observation; a nonempty
    all-breakeven sample returns zero. Only raw non-cumulative period P&L and
    realized closed-trade P&L are accepted. No capital conversion or
    annualization occurs. Rust owns compensated O(1) streaming state.
    """
    def __init__(self)->None: raise TypeError("use NetProfit.from_pnl/from_trades")
    @classmethod
    def _create(cls,values:Any,mode:str,*,nan_policy:str="omit",column:str|None=None)->"NetProfit": state=cls.__new__(cls);state._state=_Native(mode,nan_policy);return state.extend(values,column=column)
    @classmethod
    def from_pnl(cls,pnl:Any,*,nan_policy:str="omit",column:str|None=None)->"NetProfit":
        """Construct from raw non-cumulative period P&L."""
        return cls._create(pnl,"pnl",nan_policy=nan_policy,column=column)
    @classmethod
    def from_trades(cls,trades:Any,*,nan_policy:str="omit",column:str|None=None)->"NetProfit":
        """Construct from realized closed-trade P&L."""
        return cls._create(trades,"trades",nan_policy=nan_policy,column=column)
    def append(self,value:float)->"NetProfit":
        """Append one P&L observation and return this metric."""
        self._state.append(float(value));return self
    def extend(self,values:Any,*,column:str|None=None)->"NetProfit":
        """Append P&L observations and return this metric."""
        self._state.extend(as_metric_series(values,column=column));return self
    @property
    def value(self)->float|None:
        """Return net profit, or ``None`` during warm-up."""
        return self._state.value
    def compute(self)->float|None:
        """Return current scalar without replaying input."""
        return self._state.compute()
    def reset(self)->"NetProfit":
        """Clear state, preserve the P&L domain, and return this metric."""
        self._state.reset();return self
    def __len__(self)->int:
        """Return valid P&L observation count delegated to Rust."""
        return len(self._state)


__all__=["NetProfit"]
