from typing import Any
import numpy as np
from .._native import WilliamsAccumulationDistribution as _Native
from .._series import as_float64_series
class WilliamsAccumulationDistribution:
    """Cumulative Williams accumulation/distribution from OHLC bars."""
    def __init__(self,high:Any,low:Any,close:Any)->None:self._state=_Native();self.extend(high,low,close)
    def append(self,high:float,low:float,close:float)->"WilliamsAccumulationDistribution":self._state.append(float(high),float(low),float(close));return self
    def extend(self,high:Any,low:Any,close:Any)->"WilliamsAccumulationDistribution":
        a=tuple(as_float64_series(x) for x in(high,low,close))
        if len({len(x) for x in a})!=1:raise ValueError("high, low, and close must have equal lengths")
        self._state.extend(*a);return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self)->float|None:return self._state.value
    def reset(self)->"WilliamsAccumulationDistribution":self._state.reset();return self
    def __len__(self)->int:return len(self._state)
