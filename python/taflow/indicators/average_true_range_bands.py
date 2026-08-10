from typing import Any
import numpy as np
from .._native import AverageTrueRangeBands as _Native
from .._series import as_float64_series
class AverageTrueRangeBands:
    """Causal ATR-channel center value from high, low, and close."""
    def __init__(self,high:Any,low:Any,close:Any,period:int=14,multiplier:float=2.0)->None:self._state=_Native(period,multiplier);self.extend(high,low,close)
    def append(self,high:float,low:float,close:float)->"AverageTrueRangeBands":self._state.append(float(high),float(low),float(close));return self
    def extend(self,high:Any,low:Any,close:Any)->"AverageTrueRangeBands":
        a=tuple(as_float64_series(x) for x in(high,low,close))
        if len({len(x) for x in a})!=1:raise ValueError("high, low, and close must have equal lengths")
        self._state.extend(*a);return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self)->float|None:return self._state.value
    def reset(self)->"AverageTrueRangeBands":self._state.reset();return self
    def __len__(self)->int:return len(self._state)
