"""Persistent Minus Directional Movement (-DM)."""

from typing import Any

import numpy as np

from ._native import MinusDirectionalMovement as _Native
from ._series import as_float64_series


class MinusDirectionalMovement:
    def __init__(
        self,
        high: Any | None = None,
        low: Any | None = None,
        timeperiod: int = 14,
    ):
        self._state = _Native(timeperiod)
        if high is not None or low is not None:
            self.extend(high, low)

    def append(self, high: float, low: float):
        self._state.append(high, low)
        return self

    def extend(self, high: Any, low: Any | None = None):
        if low is None:
            raise ValueError("high and low must be provided together")
        self._state.extend(as_float64_series(high), as_float64_series(low))
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
        return self


MINUS_DM = MinusDirectionalMovement
