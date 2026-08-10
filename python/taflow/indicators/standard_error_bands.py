from typing import Any
import numpy as np
from .._native import StandardErrorBands as _Native
from .._series import as_float64_series
class StandardErrorBands:
    """Rolling standard-error band width."""
    def __init__(self,values:Any,period:int=20)->None:self._state=_Native(period);self.extend(values)
    def append(self,value:float)->"StandardErrorBands":self._state.append(float(value));return self
    def extend(self,values:Any)->"StandardErrorBands":self._state.extend(as_float64_series(values));return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self)->float|None:return self._state.value
    def reset(self)->"StandardErrorBands":self._state.reset();return self
    def __len__(self)->int:return len(self._state)
