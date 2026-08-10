from typing import Any
import numpy as np
from .._native import AbsoluteBreadthIndex as _Native
from .._series import as_float64_series
class AbsoluteBreadthIndex:
    """Absolute aggregate breadth change from aligned market-breadth inputs."""
    def __init__(self,change:Any,volume:Any,new_high:Any,new_low:Any)->None:self._state=_Native();self.extend(change,volume,new_high,new_low)
    def append(self,change:float,volume:float,new_high:float,new_low:float)->"AbsoluteBreadthIndex":self._state.append(float(change),float(volume),float(new_high),float(new_low));return self
    def extend(self,change:Any,volume:Any,new_high:Any,new_low:Any)->"AbsoluteBreadthIndex":
        a=tuple(as_float64_series(x) for x in(change,volume,new_high,new_low))
        if len({len(x) for x in a})!=1:raise ValueError("breadth inputs must have equal lengths")
        self._state.extend(*a);return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self)->float|None:return self._state.value
    def reset(self)->"AbsoluteBreadthIndex":self._state.reset();return self
    def __len__(self)->int:return len(self._state)
