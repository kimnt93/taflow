from typing import Any
import numpy as np
from .._native import RollingLeadLagCrossCorrelation as _Native
from .._series import as_float64_series
class RollingLeadLagCrossCorrelation:
    """Find the strongest signed correlation across bounded lead/lag offsets.

    Output is ``(lag, correlation)``; positive lag means ``left`` leads
    ``right``. Zero lag is checked first so ties prefer the smallest absolute
    offset, matching Wickra ``LeadLagCrossCorrelation``.
    """
    def __init__(self,left:Any,right:Any,window:int=20,max_lag:int=10)->None:self._state=_Native(window,max_lag);self.extend(left,right)
    def append(self,left:float,right:float)->"RollingLeadLagCrossCorrelation":self._state.append(float(left),float(right));return self
    def extend(self,left:Any,right:Any)->"RollingLeadLagCrossCorrelation":
        a,b=as_float64_series(left),as_float64_series(right)
        if len(a)!=len(b):raise ValueError("left and right inputs must have equal lengths")
        self._state.extend(a,b);return self
    def compute(self)->tuple[np.ndarray,np.ndarray]:return self._state.compute()
    @property
    def value(self)->tuple[float,float]|None:return self._state.value
    def reset(self)->"RollingLeadLagCrossCorrelation":self._state.reset();return self
    def __len__(self)->int:return len(self._state)
