"""Native-backed rolling profit factor adapter."""
from typing import Any
import numpy as np
from .._native import RollingProfitFactor as _Native
from .._series import as_float64_series

class RollingProfitFactor:
    """Rolling sum of positive observations divided by absolute losses.

    Required chronological ``values`` use a 14-bar default window. Rust owns
    rolling state and warm-up; all lifecycle mutators return ``self`` and
    ``compute`` returns one aligned float64 array. Oracle mapping: Wickra
    ``ProfitFactor``.
    """
    def __init__(self, values: Any, timeperiod: int = 14) -> None: self._state = _Native(int(timeperiod)); self.extend(values)
    def append(self, values: float) -> "RollingProfitFactor": self._state.append(float(values)); return self
    def extend(self, values: Any) -> "RollingProfitFactor": self._state.extend(as_float64_series(values)); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "RollingProfitFactor": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
__all__ = ["RollingProfitFactor"]
