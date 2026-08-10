"""Native-backed rolling Granger causality diagnostic."""
from typing import Any
import numpy as np
from .._native import RollingGrangerCausality as _Native
from .._series import as_float64_series

class RollingGrangerCausality:
    """Rolling lagged regression coefficient for two aligned series."""
    def __init__(self, a: Any, b: Any, period: int = 60, lag: int = 1) -> None: self._state = _Native(period, lag); self.extend(a, b)
    def append(self, a: float, b: float) -> "RollingGrangerCausality": self._state.append(float(a), float(b)); return self
    def extend(self, a: Any, b: Any) -> "RollingGrangerCausality":
        x, y = as_float64_series(a), as_float64_series(b)
        if len(x) != len(y): raise ValueError("a and b must have equal lengths")
        self._state.extend(x, y); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "RollingGrangerCausality": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
