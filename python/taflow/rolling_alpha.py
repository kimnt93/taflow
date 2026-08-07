"""Rolling regression alpha and information ratio features."""
from typing import Any
import numpy as np
from ._native import RollingAlphaOperator, RollingInformationRatioOperator
from ._series import as_float64_series


class RollingAlpha:
    def __init__(self, input: Any | None = None, benchmark: Any | None = None, timeperiod: int = 20):
        self._state = RollingAlphaOperator(timeperiod)
        self.extend(input, benchmark) if input is not None or benchmark is not None else None
    def append(self, input: float, benchmark: float): self._state.append(input, benchmark); return self
    def extend(self, input: Any, benchmark: Any): self._state.extend(as_float64_series(input), as_float64_series(benchmark)); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self): return self._state.value
    def reset(self): self._state.reset(); return self


class RollingInformationRatio:
    def __init__(self, input: Any | None = None, benchmark: Any | None = None, timeperiod: int = 20):
        self._state = RollingInformationRatioOperator(timeperiod)
        self.extend(input, benchmark) if input is not None or benchmark is not None else None
    def append(self, input: float, benchmark: float): self._state.append(input, benchmark); return self
    def extend(self, input: Any, benchmark: Any): self._state.extend(as_float64_series(input), as_float64_series(benchmark)); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self): return self._state.value
    def reset(self): self._state.reset(); return self
