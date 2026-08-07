"""Persistent True Strength Index."""
from typing import Any
import numpy as np
from ._native import TsiOperator as _Native
from ._series import as_float64_series
class TrueStrengthIndex:
    """Stateful TrueStrengthIndex indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """
    def __init__(self,fast:int=13,slow:int=25,input:Any|None=None):self._state=_Native(fast,slow);self.extend(input) if input is not None else None
    def append(self,input:float):self._state.append(input);return self
    def extend(self,input:Any):self._state.extend(as_float64_series(input));return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self):return self._state.value
    def reset(self):self._state.reset();return self
