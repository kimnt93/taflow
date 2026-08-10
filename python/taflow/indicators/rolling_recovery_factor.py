"""Native-backed rolling recovery factor adapter."""
from typing import Any
import numpy as np
from .._native import RollingRecoveryFactor as _Native
from .._series import as_float64_series

class RollingRecoveryFactor:
    """Rolling net change divided by the maximum fractional drawdown.

    Required chronological equity ``values`` use a 14-bar default window.
    Rust owns drawdown state, warm-up, and aligned NaNs; lifecycle mutators
    return ``self`` and ``compute`` returns float64. Oracle mapping: Wickra
    ``RecoveryFactor``.
    """
    def __init__(self, values: Any, timeperiod: int = 14) -> None: self._state = _Native(int(timeperiod)); self.extend(values)
    def append(self, values: float) -> "RollingRecoveryFactor": self._state.append(float(values)); return self
    def extend(self, values: Any) -> "RollingRecoveryFactor": self._state.extend(as_float64_series(values)); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "RollingRecoveryFactor": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
__all__ = ["RollingRecoveryFactor"]
