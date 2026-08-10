from typing import Any
import numpy as np
from .._native import TwiggsMoneyFlow as _Native
from .._series import as_float64_series
class TwiggsMoneyFlow:
    """Rolling Twiggs money-flow ratio from high, low, close, and volume."""
    def __init__(self,high:Any,low:Any,close:Any,volume:Any,period:int=21)->None:self._state=_Native(period);self.extend(high,low,close,volume)
    def append(self,high:float,low:float,close:float,volume:float)->"TwiggsMoneyFlow":self._state.append(float(high),float(low),float(close),float(volume));return self
    def extend(self,high:Any,low:Any,close:Any,volume:Any)->"TwiggsMoneyFlow":
        a=tuple(as_float64_series(x) for x in(high,low,close,volume))
        if len({len(x) for x in a})!=1:raise ValueError("OHLCV inputs must have equal lengths")
        self._state.extend(*a);return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self)->float|None:return self._state.value
    def reset(self)->"TwiggsMoneyFlow":self._state.reset();return self
    def __len__(self)->int:return len(self._state)
