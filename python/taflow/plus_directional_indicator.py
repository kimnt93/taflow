from typing import Any
import numpy as np
from ._native import PlusDirectionalIndicator as _Native
from ._series import as_float64_series
class PlusDirectionalIndicator:
    """Stateful PlusDirectionalIndicator indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """
    def __init__(self, high:Any|None=None,low:Any|None=None,close:Any|None=None,timeperiod:int=14)->None:
        self._state=_Native(timeperiod)
        if high is not None or low is not None or close is not None:self.extend(high,low,close)
    def append(self,high:float,low:float,close:float)->"PlusDirectionalIndicator":self._state.append(high,low,close);return self
    def extend(self,high:Any,low:Any|None=None,close:Any|None=None)->"PlusDirectionalIndicator":
        if low is None or close is None:raise ValueError("high, low, and close must be provided together")
        self._state.extend(as_float64_series(high),as_float64_series(low),as_float64_series(close));return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self)->float|None:return self._state.value
    def reset(self)->"PlusDirectionalIndicator":self._state.reset();return self
