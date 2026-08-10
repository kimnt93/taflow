from typing import Any
import numpy as np
from .._native import MarketFacilitationIndex as _Native
from .._series import as_float64_series
class MarketFacilitationIndex:
    """High-low trading range divided by volume."""
    def __init__(self,high:Any,low:Any,volume:Any)->None:self._state=_Native();self.extend(high,low,volume)
    def append(self,high:float,low:float,volume:float)->"MarketFacilitationIndex":self._state.append(float(high),float(low),float(volume));return self
    def extend(self,high:Any,low:Any,volume:Any)->"MarketFacilitationIndex":
        a=tuple(as_float64_series(x) for x in(high,low,volume))
        if len({len(x) for x in a})!=1:raise ValueError("high, low, and volume must have equal lengths")
        self._state.extend(*a);return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self)->float|None:return self._state.value
    def reset(self)->"MarketFacilitationIndex":self._state.reset();return self
    def __len__(self)->int:return len(self._state)
