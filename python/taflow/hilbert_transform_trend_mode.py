"""Persistent Hilbert Transform trend/cycle mode (HT_TRENDMODE)."""
from typing import Any
import numpy as np
from ._native import HilbertTransformTrendMode as _Native
from ._series import as_float64_series

class HilbertTransformTrendMode:
    """Stateful HilbertTransformTrendMode indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """
    def __init__(self, input: Any | None = None):
        self._state = _Native()
        if input is not None: self.extend(input)
    def append(self, value: float): self._state.append(float(value)); return self
    def extend(self, values: Any): self._state.extend(as_float64_series(values)); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self): return self._state.value
    def reset(self): self._state.reset(); return self
