"""Pairs-trading z-score of the rolling OLS spread ``y - beta*x``."""
from typing import Any
import numpy as np
from ._native import SpreadZscoreOperator as _Native
from ._series import as_float64_series


class SpreadZscore:
    """Stateful SpreadZscore indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """
    def __init__(self, x: Any | None = None, y: Any | None = None, timeperiod: int = 20):
        self._state = _Native(timeperiod)
        if x is not None or y is not None:
            self.extend(x, y)

    def append(self, x: float, y: float):
        self._state.append(x, y)
        return self

    def extend(self, x: Any, y: Any):
        self._state.extend(as_float64_series(x), as_float64_series(y))
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
        return self
