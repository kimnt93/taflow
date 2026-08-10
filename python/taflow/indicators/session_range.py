from typing import Any
import numpy as np
from .._native import SessionRange as _Native
from .._series import as_float64_series, as_int64_series
class SessionRange:
    """Running high-minus-low range within UTC-offset calendar sessions."""
    def __init__(self,open:Any,high:Any,low:Any,close:Any,volume:Any,timestamp:Any,utc_offset_minutes:int=0)->None:self._state=_Native(utc_offset_minutes);self.extend(open,high,low,close,volume,timestamp)
    def append(self,open:float,high:float,low:float,close:float,volume:float,timestamp:int)->"SessionRange":self._state.append(float(open),float(high),float(low),float(close),float(volume),int(timestamp));return self
    def extend(self,open:Any,high:Any,low:Any,close:Any,volume:Any,timestamp:Any)->"SessionRange":
        a=tuple(as_float64_series(x) for x in(open,high,low,close,volume))+(as_int64_series(timestamp),)
        if len({len(x) for x in a})!=1:raise ValueError("OHLCV and timestamp must have equal lengths")
        self._state.extend(*a);return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self)->float|None:return self._state.value
    def reset(self)->"SessionRange":self._state.reset();return self
    def __len__(self)->int:return len(self._state)
