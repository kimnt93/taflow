"""Persistent Keltner Channels."""
from typing import Any
import numpy as np
from ._native import KeltnerChannelsOperator as _Native
from ._series import as_float64_series
class KeltnerChannels:
    def __init__(self,high:Any|None=None,low:Any|None=None,close:Any|None=None,timeperiod:int=20,multiplier:float=2.):self._state=_Native(timeperiod,multiplier);self.extend(high,low,close) if high is not None or low is not None or close is not None else None
    def append(self,high:float,low:float,close:float):self._state.append(high,low,close);return self
    def extend(self,high:Any,low:Any,close:Any):self._state.extend(as_float64_series(high),as_float64_series(low),as_float64_series(close));return self
    def compute(self)->tuple[np.ndarray,np.ndarray,np.ndarray]:return self._state.compute()
    @property
    def value(self):return self._state.value
    def reset(self):self._state.reset();return self
