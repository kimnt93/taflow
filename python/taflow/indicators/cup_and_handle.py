from typing import Any
import numpy as np
from .._native import CupAndHandle as _Native
from .._series import as_float64_series
class CupAndHandle:
    """Causal 20-bar cup-recovery pattern signal from aligned OHLC bars."""
    def __init__(self,open:Any,high:Any,low:Any,close:Any)->None:self._state=_Native();self.extend(open,high,low,close)
    def append(self,open:float,high:float,low:float,close:float)->"CupAndHandle":self._state.append(float(open),float(high),float(low),float(close));return self
    def extend(self,open:Any,high:Any,low:Any,close:Any)->"CupAndHandle":
        a=tuple(as_float64_series(x) for x in(open,high,low,close))
        if len({len(x) for x in a})!=1:raise ValueError("OHLC inputs must have equal lengths")
        self._state.extend(*a);return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self)->float|None:return self._state.value
    def reset(self)->"CupAndHandle":self._state.reset();return self
    def __len__(self)->int:return len(self._state)
