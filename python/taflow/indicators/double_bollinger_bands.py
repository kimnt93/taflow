from typing import Any
import numpy as np
from .._native import DoubleBollingerBands as _Native
from .._series import as_float64_series
class DoubleBollingerBands:
    """Rolling double-Bollinger central average value."""
    def __init__(self,values:Any,period:int=20)->None:self._state=_Native(period);self.extend(values)
    def append(self,value:float)->"DoubleBollingerBands":self._state.append(float(value));return self
    def extend(self,values:Any)->"DoubleBollingerBands":self._state.extend(as_float64_series(values));return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self)->float|None:return self._state.value
    def reset(self)->"DoubleBollingerBands":self._state.reset();return self
    def __len__(self)->int:return len(self._state)
