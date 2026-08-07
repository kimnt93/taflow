"""Persistent Identical Three Crows recognition (CDLIDENTICAL3CROWS)."""
from typing import Any
import numpy as np
from ._native import IdenticalThreeCrows as _Native
from ._series import as_float64_series
class IdenticalThreeCrows:
 def __init__(self,open:Any|None=None,high:Any|None=None,low:Any|None=None,close:Any|None=None):self._state=_Native();self.extend(open,high,low,close) if any(x is not None for x in(open,high,low,close))else None
 def append(self,open:float,high:float,low:float,close:float):self._state.append(open,high,low,close);return self
 def extend(self,open:Any,high:Any,low:Any,close:Any):self._state.extend(as_float64_series(open),as_float64_series(high),as_float64_series(low),as_float64_series(close));return self
 def compute(self)->np.ndarray:return self._state.compute()
 @property
 def value(self):return self._state.value
 def reset(self):self._state.reset();return self
CDLIDENTICAL3CROWS=IdenticalThreeCrows
