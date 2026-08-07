"""Rolling OLS hedge ratio over price levels."""
from typing import Any
import numpy as np
from ._native import HedgeRatioOperator as _Native
from ._series import as_float64_series


class HedgeRatio:
    def __init__(self, x: Any | None = None, y: Any | None = None, timeperiod: int = 20):
        self._state = _Native(timeperiod)
        self.extend(x, y) if x is not None or y is not None else None

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
