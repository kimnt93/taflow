from typing import Any
import numpy as np
from .._native import BetterVolume as _Native
from .._series import as_float64_series
class BetterVolume:
    """Causal high/low/close/volume activity classification value."""
    def __init__(self,high:Any,low:Any,close:Any,volume:Any)->None:self._state=_Native();self.extend(high,low,close,volume)
    def append(self,high:float,low:float,close:float,volume:float)->"BetterVolume":self._state.append(float(high),float(low),float(close),float(volume));return self
    def extend(self,high:Any,low:Any,close:Any,volume:Any)->"BetterVolume":
        a=tuple(as_float64_series(x) for x in (high,low,close,volume))
        if len({len(x) for x in a})!=1:raise ValueError("OHLCV inputs must have equal lengths")
        self._state.extend(*a);return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self)->float|None:return self._state.value
    def reset(self)->"BetterVolume":self._state.reset();return self
    def __len__(self)->int:return len(self._state)
