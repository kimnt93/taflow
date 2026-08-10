from typing import Any
import numpy as np
from .._native import TradeVolumeIndex as _Native
from .._series import as_float64_series
class TradeVolumeIndex:
    """Cumulative volume signed by close-to-close direction."""
    def __init__(self,close:Any,volume:Any)->None:self._state=_Native();self.extend(close,volume)
    def append(self,close:float,volume:float)->"TradeVolumeIndex":self._state.append(float(close),float(volume));return self
    def extend(self,close:Any,volume:Any)->"TradeVolumeIndex":
        a,b=as_float64_series(close),as_float64_series(volume)
        if len(a)!=len(b):raise ValueError("close and volume must have equal lengths")
        self._state.extend(a,b);return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self)->float|None:return self._state.value
    def reset(self)->"TradeVolumeIndex":self._state.reset();return self
    def __len__(self)->int:return len(self._state)
