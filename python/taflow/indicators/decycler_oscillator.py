from typing import Any
import numpy as np
from .._native import DecyclerOscillator as _Native
from .._series import as_float64_series
class DecyclerOscillator:
    """Difference between fast and slow decycler components."""
    def __init__(self,values:Any,fast:int=10,slow:int=20)->None:self._state=_Native(fast,slow);self.extend(values)
    def append(self,value:float)->"DecyclerOscillator":self._state.append(float(value));return self
    def extend(self,values:Any)->"DecyclerOscillator":self._state.extend(as_float64_series(values));return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self)->float|None:return self._state.value
    def reset(self)->"DecyclerOscillator":self._state.reset();return self
    def __len__(self)->int:return len(self._state)
