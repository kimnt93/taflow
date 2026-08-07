"""Rolling exact-value Shannon entropy."""
from typing import Any
import numpy as np
from ._native import RollingEntropyOperator as _Native
from ._series import as_float64_series


class RollingEntropy:
    """Stateful RollingEntropy indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """
    def __init__(self, _input: Any | None = None, timeperiod: int = 20):
        self._state = _Native(timeperiod)
        self.extend(_input) if _input is not None else None
    def append(self, _input: float): self._state.append(_input); return self
    def extend(self, _input: Any): self._state.extend(as_float64_series(_input)); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self): return self._state.value
    def reset(self): self._state.reset(); return self
