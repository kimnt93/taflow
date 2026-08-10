from typing import Any
import numpy as np
from .._native import BreadthThrust as _Native
from .._series import as_float64_series
class BreadthThrust:
    """Rolling advancing breadth as a fraction of absolute breadth activity."""
    def __init__(self,change:Any,volume:Any,new_high:Any,new_low:Any,period:int)->None:self._state=_Native(period);self.extend(change,volume,new_high,new_low)
    def append(self,change:float,volume:float,new_high:float,new_low:float)->"BreadthThrust":self._state.append(float(change),float(volume),float(new_high),float(new_low));return self
    def extend(self,change:Any,volume:Any,new_high:Any,new_low:Any)->"BreadthThrust":
        a=tuple(as_float64_series(x) for x in(change,volume,new_high,new_low))
        if len({len(x) for x in a})!=1:raise ValueError("breadth inputs must have equal lengths")
        self._state.extend(*a);return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self)->float|None:return self._state.value
    def reset(self)->"BreadthThrust":self._state.reset();return self
    def __len__(self)->int:return len(self._state)
