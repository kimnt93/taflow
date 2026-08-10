from typing import Any
import numpy as np
from .._native import AdaptiveCycle as _Native
from .._series import as_float64_series
class AdaptiveCycle:
    """Causal adaptive cycle-change series."""
    def __init__(self,values:Any)->None:self._state=_Native();self.extend(values)
    def append(self,value:float)->"AdaptiveCycle":self._state.append(float(value));return self
    def extend(self,values:Any)->"AdaptiveCycle":self._state.extend(as_float64_series(values));return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self)->float|None:return self._state.value
    def reset(self)->"AdaptiveCycle":self._state.reset();return self
    def __len__(self)->int:return len(self._state)
