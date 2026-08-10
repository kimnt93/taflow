from typing import Any
import numpy as np
from .._native import RoofingFilter as _Native
from .._series import as_float64_series
class RoofingFilter:
    """Causal high-pass then low-pass roofing filter."""
    def __init__(self,values:Any,low_period:int=10,high_period:int=48)->None:self._state=_Native(low_period,high_period);self.extend(values)
    def append(self,value:float)->"RoofingFilter":self._state.append(float(value));return self
    def extend(self,values:Any)->"RoofingFilter":self._state.extend(as_float64_series(values));return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self)->float|None:return self._state.value
    def reset(self)->"RoofingFilter":self._state.reset();return self
    def __len__(self)->int:return len(self._state)
