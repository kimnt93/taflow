from typing import Any
import numpy as np
from .._native import TimeSegmentedVolume as _Native
from .._series import as_float64_series
class TimeSegmentedVolume:
    """Cumulative volume signed by close direction."""
    def __init__(self,close:Any,volume:Any)->None:self._state=_Native();self.extend(close,volume)
    def append(self,close:float,volume:float)->"TimeSegmentedVolume":self._state.append(float(close),float(volume));return self
    def extend(self,close:Any,volume:Any)->"TimeSegmentedVolume":
        a,b=as_float64_series(close),as_float64_series(volume)
        if len(a)!=len(b):raise ValueError("close and volume must have equal lengths")
        self._state.extend(a,b);return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self)->float|None:return self._state.value
    def reset(self)->"TimeSegmentedVolume":self._state.reset();return self
    def __len__(self)->int:return len(self._state)
