from typing import Any
import numpy as np
from .._native import ZigZag as _Native
from .._series import as_float64_series

class ZigZag:
    """Threshold-confirmed high/low pivots; outputs aligned high and low arrays."""
    def __init__(self, high: Any, low: Any, threshold: float = 0.05) -> None: self._state = _Native(threshold); self.extend(high, low)
    def append(self, high: float, low: float) -> "ZigZag": self._state.append(float(high), float(low)); return self
    def extend(self, high: Any, low: Any) -> "ZigZag":
        arrays = as_float64_series(high), as_float64_series(low)
        if len(arrays[0]) != len(arrays[1]): raise ValueError("high and low must have equal lengths")
        self._state.extend(*arrays); return self
    def compute(self) -> tuple[np.ndarray, np.ndarray]: return self._state.compute()
    @property
    def value(self) -> tuple[float, float] | None: return self._state.value
    def reset(self) -> "ZigZag": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
