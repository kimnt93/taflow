from typing import Any
import numpy as np
from .._native import EmpiricalModeDecomposition as _Native
from .._series import as_float64_series

class EmpiricalModeDecomposition:
    """Causal finite-window empirical-mode residual; Wickra name is identical."""
    def __init__(self, prices: Any, period: int = 20, fraction: float = 0.5) -> None: self._state = _Native(period, fraction); self.extend(prices)
    def append(self, price: float) -> "EmpiricalModeDecomposition": self._state.append(float(price)); return self
    def extend(self, prices: Any) -> "EmpiricalModeDecomposition": self._state.extend(as_float64_series(prices)); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "EmpiricalModeDecomposition": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
