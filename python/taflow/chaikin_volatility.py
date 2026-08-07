"""Persistent Chaikin Volatility."""
from typing import Any
import numpy as np
from ._native import ChaikinVolatilityOperator as _Native
from ._series import as_float64_series
class ChaikinVolatility:
    def __init__(self,high:Any|None=None,low:Any|None=None,timeperiod:int=10,roc_period:int=10):self._state=_Native(timeperiod,roc_period);self.extend(high,low) if high is not None or low is not None else None
    def append(self,high:float,low:float):self._state.append(high,low);return self
    def extend(self,high:Any,low:Any):self._state.extend(as_float64_series(high),as_float64_series(low));return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self):return self._state.value
    def reset(self):self._state.reset();return self
