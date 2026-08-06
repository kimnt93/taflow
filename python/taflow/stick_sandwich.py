"""Persistent Stick Sandwich recognition (CDLSTICKSANDWICH)."""
from typing import Any
import numpy as np
from ._native import StickSandwich as _Native
from ._series import as_float64_series
class StickSandwich:
    def __init__(self,open:Any|None=None,high:Any|None=None,low:Any|None=None,close:Any|None=None):
        self._state=_Native()
        if any(value is not None for value in(open,high,low,close)):self.extend(open,high,low,close)
    def append(self,open:float,high:float,low:float,close:float):self._state.append(open,high,low,close);return self
    def extend(self,open:Any,high:Any,low:Any,close:Any):self._state.extend(as_float64_series(open),as_float64_series(high),as_float64_series(low),as_float64_series(close));return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self):return self._state.value
    def reset(self):self._state.reset();return self
CDLSTICKSANDWICH=StickSandwich
