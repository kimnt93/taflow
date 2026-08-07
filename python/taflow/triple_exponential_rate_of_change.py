"""Persistent Triple Exponential Rate of Change (TRIX)."""
from typing import Any
import numpy as np
from ._native import TripleExponentialRateOfChange as _Native
from ._series import as_float64_series

class TripleExponentialRateOfChange:
    """Stateful TripleExponentialRateOfChange indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """
    def __init__(self, _input: Any | None = None, timeperiod: int = 30) -> None:
        self._state = _Native(timeperiod)
        if _input is not None: self.extend(_input)
    def append(self, value: float) -> "TripleExponentialRateOfChange": self._state.append(float(value)); return self
    def extend(self, values: Any) -> "TripleExponentialRateOfChange": self._state.extend(as_float64_series(values)); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "TripleExponentialRateOfChange": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
