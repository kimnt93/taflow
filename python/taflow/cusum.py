"""CUSUM drift-detection accumulator on a change series."""
from typing import Any
import numpy as np
from ._native import CusumOperator as _Native
from ._series import as_float64_series


class Cusum:
    def __init__(self, change: Any | None = None, threshold: float = 1.0):
        self._state = _Native(threshold)
        self.extend(change) if change is not None else None

    def append(self, change: float):
        self._state.append(change)
        return self

    def extend(self, change: Any):
        self._state.extend(as_float64_series(change))
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
        return self
