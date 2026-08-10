"""Native-backed rolling Conditional Value at Risk adapter."""
from typing import Any
import numpy as np
from .._native import RollingConditionalValueAtRisk as _Native
from .._series import as_float64_series

class RollingConditionalValueAtRisk:
    """Rolling mean loss among observations below the confidence quantile.

    Required chronological ``values`` use a 14-bar default ``timeperiod`` and
    0.95 ``confidence``. Rust owns tail selection, warm-up, and aligned NaNs;
    fluent lifecycle methods return ``self`` and ``compute`` returns float64.
    Oracle mapping: Wickra ``ConditionalValueAtRisk``.
    """
    def __init__(self, values: Any, timeperiod: int = 14, confidence: float = 0.95) -> None:
        self._state = _Native(int(timeperiod), float(confidence)); self.extend(values)
    def append(self, values: float) -> "RollingConditionalValueAtRisk": self._state.append(float(values)); return self
    def extend(self, values: Any) -> "RollingConditionalValueAtRisk": self._state.extend(as_float64_series(values)); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "RollingConditionalValueAtRisk": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
__all__ = ["RollingConditionalValueAtRisk"]
