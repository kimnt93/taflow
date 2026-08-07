"""Persistent Ease of Movement."""
from typing import Any
import numpy as np
from ._native import EaseOfMovementOperator as _Native
from ._series import as_float64_series
class EaseOfMovement:
    """Stateful EaseOfMovement indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """
    def __init__(self,high:Any|None=None,low:Any|None=None,volume:Any|None=None):self._state=_Native();self.extend(high,low,volume) if high is not None or low is not None or volume is not None else None
    def append(self,high:float,low:float,volume:float):self._state.append(high,low,volume);return self
    def extend(self,high:Any,low:Any,volume:Any):self._state.extend(as_float64_series(high),as_float64_series(low),as_float64_series(volume));return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self):return self._state.value
    def reset(self):self._state.reset();return self
