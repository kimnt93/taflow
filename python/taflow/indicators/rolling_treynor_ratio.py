"""Native-backed rolling Treynor ratio adapter."""
from typing import Any
import numpy as np
from .._native import RollingTreynorRatio as _Native
from .._series import as_float64_series

class RollingTreynorRatio:
    """Rolling mean return divided by covariance-based benchmark beta.

    Required aligned inputs are ``values`` and ``benchmark`` in that order;
    ``timeperiod`` defaults to 14. Native Rust owns validation, warm-up, and
    arithmetic; lifecycle mutators return ``self`` and ``compute`` returns one
    float64 array. Oracle mapping: Wickra ``TreynorRatio``.
    """
    def __init__(self, values: Any, benchmark: Any, timeperiod: int = 14) -> None:
        self._state = _Native(int(timeperiod)); self.extend(values, benchmark)
    def append(self, values: float, benchmark: float) -> "RollingTreynorRatio": self._state.append(float(values), float(benchmark)); return self
    def extend(self, values: Any, benchmark: Any) -> "RollingTreynorRatio":
        arrays = (as_float64_series(values), as_float64_series(benchmark))
        if len(arrays[0]) != len(arrays[1]): raise ValueError("values and benchmark must have equal lengths")
        self._state.extend(*arrays); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "RollingTreynorRatio": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
__all__ = ["RollingTreynorRatio"]
