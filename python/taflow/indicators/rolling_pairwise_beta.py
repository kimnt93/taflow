from typing import Any
import numpy as np
from .._native import RollingPairwiseBeta as _Native
from .._series import as_float64_series
class RollingPairwiseBeta:
    """Rolling beta of the first aligned series against the second."""
    def __init__(self,x:Any,y:Any,period:int=20)->None:self._state=_Native(period);self.extend(x,y)
    def append(self,x:float,y:float)->"RollingPairwiseBeta":self._state.append(float(x),float(y));return self
    def extend(self,x:Any,y:Any)->"RollingPairwiseBeta":
        a,b=as_float64_series(x),as_float64_series(y)
        if len(a)!=len(b):raise ValueError("x and y must have equal lengths")
        self._state.extend(a,b);return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self)->float|None:return self._state.value
    def reset(self)->"RollingPairwiseBeta":self._state.reset();return self
    def __len__(self)->int:return len(self._state)
