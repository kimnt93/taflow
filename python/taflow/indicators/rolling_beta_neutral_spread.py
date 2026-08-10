from typing import Any
import numpy as np
from .._native import RollingBetaNeutralSpread as _Native
from .._series import as_float64_series
class RollingBetaNeutralSpread:
    """Rolling beta-hedged spread of two aligned series."""
    def __init__(self,x:Any,y:Any,period:int=20)->None:self._state=_Native(period);self.extend(x,y)
    def append(self,x:float,y:float)->"RollingBetaNeutralSpread":self._state.append(float(x),float(y));return self
    def extend(self,x:Any,y:Any)->"RollingBetaNeutralSpread":
        a,b=as_float64_series(x),as_float64_series(y)
        if len(a)!=len(b):raise ValueError("x and y must have equal lengths")
        self._state.extend(a,b);return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self)->float|None:return self._state.value
    def reset(self)->"RollingBetaNeutralSpread":self._state.reset();return self
    def __len__(self)->int:return len(self._state)
