"""Persistent volume-weighted moving average."""
from typing import Any
import numpy as np
from ._native import VwmaOperator as _Native
from ._series import as_float64_series
class VolumeWeightedMovingAverage:
    """Stateful VolumeWeightedMovingAverage indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """
    def __init__(self,timeperiod:int,price:Any|None=None,volume:Any|None=None):self._state=_Native(timeperiod);self.extend(price,volume) if price is not None or volume is not None else None
    def append(self,price:float,volume:float):self._state.append(price,volume);return self
    def extend(self,price:Any,volume:Any):self._state.extend(as_float64_series(price),as_float64_series(volume));return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self):return self._state.value
    def reset(self):self._state.reset();return self
