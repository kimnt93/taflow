"""Persistent Separating Lines recognition (CDLSEPARATINGLINES)."""
from typing import Any
import numpy as np
from ._native import CandleSeparatingLines as _Native
from ._series import as_float64_series
class CandleSeparatingLines:
    """Stateful CandleSeparatingLines indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """
    def __init__(self,_open:Any|None=None,high:Any|None=None,low:Any|None=None,close:Any|None=None):self._state=_Native();self.extend(_open,high,low,close) if any(x is not None for x in(_open,high,low,close))else None
    def append(self,_open:float,high:float,low:float,close:float):self._state.append(_open,high,low,close);return self
    def extend(self,_open:Any,high:Any,low:Any,close:Any):self._state.extend(as_float64_series(_open),as_float64_series(high),as_float64_series(low),as_float64_series(close));return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self):return self._state.value
    def reset(self):self._state.reset();return self
