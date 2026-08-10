from typing import Any
import numpy as np
from .._native import PercentAboveMovingAverage as _Native
from .._series import as_float64_series
class PercentAboveMovingAverage:
    """Percentage of the supplied universe above its moving average; Wickra alias ``PercentAboveMa``."""
    def __init__(self,change:Any,volume:Any,new_high:Any,new_low:Any,above_moving_average:Any)->None:self._state=_Native();self.extend(change,volume,new_high,new_low,above_moving_average)
    def append(self,change:float,volume:float,new_high:float,new_low:float,above_moving_average:float)->"PercentAboveMovingAverage":self._state.append(float(change),float(volume),float(new_high),float(new_low),float(above_moving_average));return self
    def extend(self,change:Any,volume:Any,new_high:Any,new_low:Any,above_moving_average:Any)->"PercentAboveMovingAverage":
        a=tuple(as_float64_series(x) for x in(change,volume,new_high,new_low,above_moving_average))
        if len({len(x) for x in a})!=1:raise ValueError("breadth inputs must have equal lengths")
        self._state.extend(*a);return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self)->float|None:return self._state.value
    def reset(self)->"PercentAboveMovingAverage":self._state.reset();return self
    def __len__(self)->int:return len(self._state)
