"""Persistent Positive Volume Index."""
from typing import Any
import numpy as np
from ._native import PviOperator as _Native
from ._series import as_float64_series
class Pvi:
    def __init__(self, close: Any|None=None, volume: Any|None=None): self._state=_Native(); self.extend(close,volume) if close is not None or volume is not None else None
    def append(self, close: float, volume: float): self._state.append(close,volume); return self
    def extend(self, close: Any, volume: Any): self._state.extend(as_float64_series(close),as_float64_series(volume)); return self
    def compute(self)->np.ndarray:return self._state.compute()
    @property
    def value(self):return self._state.value
    def reset(self):self._state.reset();return self
