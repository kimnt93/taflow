"""Native-backed rolling cointegration diagnostic."""
from typing import Any
import numpy as np
from .._native import RollingCointegration as _Native
from .._series import as_float64_series

class RollingCointegration:
    """Rolling OLS residual scale for two aligned price series."""
    def __init__(self, a: Any, b: Any, period: int = 30) -> None: self._state = _Native(period); self.extend(a, b)
    def append(self, a: float, b: float) -> "RollingCointegration": self._state.append(float(a), float(b)); return self
    def extend(self, a: Any, b: Any) -> "RollingCointegration":
        x, y = as_float64_series(a), as_float64_series(b)
        if len(x) != len(y): raise ValueError("a and b must have equal lengths")
        self._state.extend(x, y); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "RollingCointegration": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
