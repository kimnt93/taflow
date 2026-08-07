from typing import Any
import numpy as np
from ._native import MinusDirectionalIndicator as _Native
from ._series import as_float64_series
class MinusDirectionalIndicator:
    """Stateful MinusDirectionalIndicator indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """
    def __init__(self, high:Any|None=None, low:Any|None=None, close:Any|None=None, timeperiod:int=14):
        self._state=_Native(timeperiod)
        if high is not None or low is not None or close is not None:self.extend(high,low,close)
    def append(self,h:float,l:float,c:float):self._state.append(h,l,c);return self
    def extend(self,h:Any,l:Any|None=None,c:Any|None=None):
        if l is None or c is None:raise ValueError("high, low, and close must be provided together")
        self._state.extend(as_float64_series(h),as_float64_series(l),as_float64_series(c));return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self):return self._state.value
    def reset(self):self._state.reset();return self
