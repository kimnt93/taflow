"""Canonical native-backed rolling sum adapter."""
from typing import Any
import numpy as np
from .._native import RollingSum as _NativeRollingSum
from .._series import as_float64_series

class RollingSum:
    """Compute trailing sums from required ``values`` in Rust."""
    def __init__(self, values: Any, timeperiod: int = 30) -> None:
        self._state = _NativeRollingSum(timeperiod); self.extend(values)
    def append(self, value: float) -> "RollingSum": self._state.append(float(value)); return self
    def extend(self, values: Any) -> "RollingSum": self._state.extend(as_float64_series(values)); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "RollingSum": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
__all__ = ["RollingSum"]
