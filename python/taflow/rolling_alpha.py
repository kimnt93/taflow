"""Rolling regression alpha and information ratio features."""
from typing import Any
import numpy as np
from ._native import RollingAlphaOperator, RollingInformationRatioOperator
from ._series import as_float64_series


class RollingAlpha:
    """Stateful RollingAlpha indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """
    def __init__(self, _input: Any | None = None, benchmark: Any | None = None, timeperiod: int = 20):
        self._state = RollingAlphaOperator(timeperiod)
        self.extend(_input, benchmark) if _input is not None or benchmark is not None else None
    def append(self, _input: float, benchmark: float): self._state.append(_input, benchmark); return self
    def extend(self, _input: Any, benchmark: Any): self._state.extend(as_float64_series(_input), as_float64_series(benchmark)); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self): return self._state.value
    def reset(self): self._state.reset(); return self


class RollingInformationRatio:
    """Stateful RollingInformationRatio indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """
    def __init__(self, _input: Any | None = None, benchmark: Any | None = None, timeperiod: int = 20):
        self._state = RollingInformationRatioOperator(timeperiod)
        self.extend(_input, benchmark) if _input is not None or benchmark is not None else None
    def append(self, _input: float, benchmark: float): self._state.append(_input, benchmark); return self
    def extend(self, _input: Any, benchmark: Any): self._state.extend(as_float64_series(_input), as_float64_series(benchmark)); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self): return self._state.value
    def reset(self): self._state.reset(); return self
