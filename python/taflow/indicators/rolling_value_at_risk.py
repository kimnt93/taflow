"""Native-backed rolling Value at Risk adapter."""
from typing import Any
import numpy as np
from .._native import RollingValueAtRisk as _Native
from .._series import as_float64_series

class RollingValueAtRisk:
    """Rolling lower-tail loss quantile with confidence defaulting to 0.95.

    Required ``values`` are chronological returns or P&L observations;
    ``timeperiod`` is 14 by default. Rust owns quantiles, warm-up, and aligned
    NaNs. Methods ``append``, ``extend``, and ``reset`` return ``self`` and
    ``compute`` returns a float64 array. Oracle mapping: Wickra ``ValueAtRisk``.
    """
    def __init__(self, values: Any, timeperiod: int = 14, confidence: float = 0.95) -> None:
        self._state = _Native(int(timeperiod), float(confidence)); self.extend(values)
    def append(self, values: float) -> "RollingValueAtRisk": self._state.append(float(values)); return self
    def extend(self, values: Any) -> "RollingValueAtRisk": self._state.extend(as_float64_series(values)); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "RollingValueAtRisk": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
__all__ = ["RollingValueAtRisk"]
