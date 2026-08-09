"""Persistent rolling median adapter."""

from typing import Any
import numpy as np
from .._native import RollingMedian as _NativeRollingMedian
from .._series import as_float64_series


class RollingMedian:
    """Compute a causal trailing median with NaN warm-up values."""
    def __init__(self, _input: Any, timeperiod: int = 14) -> None:
        self._state = _NativeRollingMedian(timeperiod)
        self.extend(_input)
    def append(self, _input: float) -> "RollingMedian": self._state.append(float(_input)); return self
    def extend(self, _input: Any) -> "RollingMedian": self._state.extend(as_float64_series(_input)); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "RollingMedian": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
