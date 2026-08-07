"""Persistent cumulative product operator."""
from typing import Any
import numpy as np
from ._native import CumprodOperator as _Native
from ._series import as_float64_series

class Cumprod:
    def __init__(self, input: Any | None = None):
        self._state = _Native()
        if input is not None: self.extend(input)
    def append(self, input: float): self._state.append(input); return self
    def extend(self, input: Any): self._state.extend(as_float64_series(input)); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self): return self._state.value
    def reset(self): self._state.reset(); return self
