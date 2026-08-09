"""Canonical native-backed Double Exponential Moving Average adapter."""
from typing import Any
import numpy as np
from ._native import DoubleExponentialMovingAverage as _NativeDoubleExponentialMovingAverage
from ._series import as_float64_series

class DoubleExponentialMovingAverage:
    """Compute DEMA from required ``values``; Rust owns warm-up and recurrence."""
    def __init__(self, values: Any, timeperiod: int = 30) -> None:
        self._state = _NativeDoubleExponentialMovingAverage(timeperiod); self.extend(values)
    def append(self, value: float) -> "DoubleExponentialMovingAverage": self._state.append(float(value)); return self
    def extend(self, values: Any) -> "DoubleExponentialMovingAverage": self._state.extend(as_float64_series(values)); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "DoubleExponentialMovingAverage": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
__all__ = ["DoubleExponentialMovingAverage"]
