"""Native-backed rolling variance ratio."""
from typing import Any
import numpy as np
from .._native import RollingVarianceRatio as _Native
from .._series import as_float64_series

class RollingVarianceRatio:
    """Rolling variance ratio for paired series, with period and aggregation ``q``."""
    def __init__(self, a: Any, b: Any, period: int = 60, q: int = 2) -> None: self._state = _Native(period, q); self.extend(a, b)
    def append(self, a: float, b: float) -> "RollingVarianceRatio": self._state.append(float(a), float(b)); return self
    def extend(self, a: Any, b: Any) -> "RollingVarianceRatio":
        x, y = as_float64_series(a), as_float64_series(b)
        if len(x) != len(y): raise ValueError("a and b must have equal lengths")
        self._state.extend(x, y); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "RollingVarianceRatio": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
