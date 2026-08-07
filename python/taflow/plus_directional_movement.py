from typing import Any
import numpy as np
from ._native import PlusDirectionalMovement as _Native
from ._series import as_float64_series
class PlusDirectionalMovement:
    """Stateful PlusDirectionalMovement indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """
    def __init__(self, high:Any|None=None,low:Any|None=None,timeperiod:int=14):
        self._state=_Native(timeperiod)
        if high is not None or low is not None:self.extend(high,low)
    def append(self,h:float,l:float):self._state.append(h,l);return self
    def extend(self,h:Any,l:Any|None=None):
        if l is None:raise ValueError("high and low must be provided together")
        self._state.extend(as_float64_series(h),as_float64_series(l));return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self):return self._state.value
    def reset(self):self._state.reset();return self
