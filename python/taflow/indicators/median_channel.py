from typing import Any
import numpy as np
from .._native import MedianChannel as _Native
from .._series import as_float64_series
class MedianChannel:
    """Rolling median channel with range-scaled upper and lower bands."""
    def __init__(self,prices:Any,period:int=20,multiplier:float=2.0)->None:self._state=_Native(period,multiplier);self.extend(prices)
    def append(self,price:float)->"MedianChannel":self._state.append(float(price));return self
    def extend(self,prices:Any)->"MedianChannel":self._state.extend(as_float64_series(prices));return self
    def compute(self)->tuple[np.ndarray,np.ndarray,np.ndarray]:return self._state.compute()
    @property
    def value(self)->tuple[float,float,float]|None:return self._state.value
    def reset(self)->"MedianChannel":self._state.reset();return self
    def __len__(self)->int:return len(self._state)
