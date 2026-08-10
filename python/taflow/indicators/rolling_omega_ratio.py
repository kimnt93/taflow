"""Native-backed rolling Omega ratio adapter."""
from typing import Any
import numpy as np
from .._native import RollingOmegaRatio as _Native
from .._series import as_float64_series

class RollingOmegaRatio:
    """Rolling gain-above-threshold divided by loss-below-threshold ratio.

    ``values`` is required and ordered chronologically; ``timeperiod`` defaults
    to 14 and ``threshold`` to 0.0. Rust owns the rolling arithmetic and NaN
    warm-up; ``compute`` returns one aligned float64 array and lifecycle
    mutators return ``self``. Oracle mapping: Wickra ``OmegaRatio``.
    """
    def __init__(self, values: Any, timeperiod: int = 14, threshold: float = 0.0) -> None:
        self._state = _Native(int(timeperiod), float(threshold)); self.extend(values)
    def append(self, values: float) -> "RollingOmegaRatio": self._state.append(float(values)); return self
    def extend(self, values: Any) -> "RollingOmegaRatio": self._state.extend(as_float64_series(values)); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "RollingOmegaRatio": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
__all__ = ["RollingOmegaRatio"]
