"""Native-backed rolling Kelly criterion adapter."""
from typing import Any
import numpy as np
from .._native import RollingKellyCriterion as _Native
from .._series import as_float64_series

class RollingKellyCriterion:
    """Rolling win-probability and payoff estimate for Kelly sizing.

    Required chronological return observations use a 14-bar default window.
    Rust owns the estimate, warm-up, and aligned NaNs; fluent lifecycle methods
    return ``self`` and ``compute`` returns float64. Oracle mapping: Wickra
    ``KellyCriterion``.
    """
    def __init__(self, values: Any, timeperiod: int = 14) -> None: self._state = _Native(int(timeperiod)); self.extend(values)
    def append(self, values: float) -> "RollingKellyCriterion": self._state.append(float(values)); return self
    def extend(self, values: Any) -> "RollingKellyCriterion": self._state.extend(as_float64_series(values)); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "RollingKellyCriterion": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
__all__ = ["RollingKellyCriterion"]
