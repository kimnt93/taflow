"""Persistent Average Directional Index adapter."""

from typing import Any
import numpy as np
from ._native import AverageDirectionalIndex as _NativeAverageDirectionalIndex
from ._series import as_float64_series


class AverageDirectionalIndex:
    """Compute Wilder ADX from aligned high, low, and close histories."""
    def __init__(self, high: Any, low: Any, close: Any, period: int = 14) -> None:
        self._state = _NativeAverageDirectionalIndex(period)
        self.extend(high, low, close)
    def append(self, high: float, low: float, close: float) -> "AverageDirectionalIndex": self._state.append(float(high), float(low), float(close)); return self
    def extend(self, high: Any, low: Any, close: Any) -> "AverageDirectionalIndex": self._state.extend(as_float64_series(high), as_float64_series(low), as_float64_series(close)); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "AverageDirectionalIndex": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
