"""Causal swing-point confirmation with confirmation-bar output."""
from typing import Any
import numpy as np
from ._native import SwingHighLowOperator as _Native
from ._series import as_float64_series


class SwingHighLow:
    def __init__(self, high: Any | None = None, low: Any | None = None, swing_length: int = 5):
        self._state = _Native(swing_length)
        self.extend(high, low) if high is not None or low is not None else None

    def append(self, high: float, low: float):
        self._state.append(high, low)
        return self

    def extend(self, high: Any, low: Any):
        self._state.extend(as_float64_series(high), as_float64_series(low))
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        return self._state.compute()

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
        return self


SwingHigh = SwingHighLow
SwingLow = SwingHighLow
