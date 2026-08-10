from typing import Any
import numpy as np
from .._native import VolumeWeightedMovingAverageConvergenceDivergence as _Native
from .._series import as_float64_series
class VolumeWeightedMovingAverageConvergenceDivergence:
    """Volume-weighted MACD-style fast-minus-slow EMA."""
    def __init__(self,close:Any,volume:Any,fast:int=12,slow:int=26)->None:self._state=_Native(fast,slow);self.extend(close,volume)
    def append(self,close:float,volume:float)->"VolumeWeightedMovingAverageConvergenceDivergence":self._state.append(float(close),float(volume));return self
    def extend(self,close:Any,volume:Any)->"VolumeWeightedMovingAverageConvergenceDivergence":
        a,b=as_float64_series(close),as_float64_series(volume)
        if len(a)!=len(b):raise ValueError("close and volume must have equal lengths")
        self._state.extend(a,b);return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self)->float|None:return self._state.value
    def reset(self)->"VolumeWeightedMovingAverageConvergenceDivergence":self._state.reset();return self
    def __len__(self)->int:return len(self._state)
