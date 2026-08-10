from typing import Any
import numpy as np
from .._native import HurstChannel as _Native
from .._series import as_float64_series
class HurstChannel:
    """Return rolling range bands around the simple mean of close.

    Output order is ``(upper, middle, lower)`` and width is ``multiplier``
    times the trailing highest-high minus lowest-low. This maps to Wickra
    ``HurstChannel``; Rust owns warm-up and aligned history.
    """
    def __init__(self,high:Any,low:Any,close:Any,period:int=10,multiplier:float=.5)->None:self._state=_Native(period,multiplier);self.extend(high,low,close)
    def append(self,high:float,low:float,close:float)->"HurstChannel":self._state.append(float(high),float(low),float(close));return self
    def extend(self,high:Any,low:Any,close:Any)->"HurstChannel":
        a=tuple(as_float64_series(x) for x in(high,low,close))
        if len({len(x) for x in a})!=1:raise ValueError("high, low, and close must have equal lengths")
        self._state.extend(*a);return self
    def compute(self)->tuple[np.ndarray,np.ndarray,np.ndarray]:return self._state.compute()
    @property
    def value(self)->tuple[float,float,float]|None:return self._state.value
    def reset(self)->"HurstChannel":self._state.reset();return self
    def __len__(self)->int:return len(self._state)
