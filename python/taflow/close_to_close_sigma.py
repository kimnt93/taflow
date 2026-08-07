"""Rolling standard deviation of log returns (close-to-close volatility)."""
from typing import Any
import numpy as np
from ._native import CloseToCloseSigmaOperator as _Native
from ._series import as_float64_series


class CloseToCloseSigma:
    def __init__(self, close: Any | None = None, timeperiod: int = 20):
        self._state = _Native(timeperiod)
        self.extend(close) if close is not None else None

    def append(self, close: float):
        self._state.append(close)
        return self

    def extend(self, close: Any):
        self._state.extend(as_float64_series(close))
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
        return self
