"""Canonical native-backed Kaufman Adaptive Moving Average adapter."""
from typing import Any
import numpy as np
from ._native import KaufmanAdaptiveMovingAverage as _NativeKaufmanAdaptiveMovingAverage
from ._series import as_float64_series

class KaufmanAdaptiveMovingAverage:
    """Compute KAMA from required ``values`` through the Rust recurrence."""
    def __init__(self, values: Any, timeperiod: int = 30) -> None:
        self._state = _NativeKaufmanAdaptiveMovingAverage(timeperiod); self.extend(values)
    def append(self, value: float) -> "KaufmanAdaptiveMovingAverage": self._state.append(float(value)); return self
    def extend(self, values: Any) -> "KaufmanAdaptiveMovingAverage": self._state.extend(as_float64_series(values)); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "KaufmanAdaptiveMovingAverage": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
__all__ = ["KaufmanAdaptiveMovingAverage"]
