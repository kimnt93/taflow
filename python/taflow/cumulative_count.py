"""Persistent cumulative observation count adapter."""

from typing import Any
import numpy as np
from ._native import CumulativeCount as _NativeCumulativeCount
from ._series import as_float64_series


class CumulativeCount:
    """Emit the one-based count of observations in chronological order."""
    def __init__(self, _input: Any) -> None:
        self._state = _NativeCumulativeCount()
        self.extend(_input)
    def append(self, _input: float) -> "CumulativeCount": self._state.append(float(_input)); return self
    def extend(self, _input: Any) -> "CumulativeCount": self._state.extend(as_float64_series(_input)); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "CumulativeCount": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
