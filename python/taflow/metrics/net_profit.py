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
    def __init__(self, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(nan_policy)

    def from_pnl(self, pnl: Any, *, column: str | None = None) -> "NetProfit":
        """Append chronological pnl observations and return this metric."""
        self._state.from_pnl(as_metric_series(pnl, column=column))
        return self

    def from_trades(self, trades: Any, *, column: str | None = None) -> "NetProfit":
        """Append chronological trades observations and return this metric."""
        self._state.from_trades(as_metric_series(trades, column=column))
        return self

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
