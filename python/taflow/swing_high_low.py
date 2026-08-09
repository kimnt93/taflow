"""Canonical causal swing-point confirmation adapter."""

from typing import Any

import numpy as np

from ._native import SwingHighLowOperator as _Native
from ._series import as_float64_series


class SwingHighLow:
    """Confirm swing highs/lows after a centered causal window."""

    def __init__(self, high: Any, low: Any, swing_length: int = 5) -> None:
        self._state = _Native(swing_length)
        self._length = 0
        self.extend(high, low)

    def append(self, high: float, low: float) -> "SwingHighLow":
        self._state.append(float(high), float(low))
        self._length += 1
        return self

    def extend(self, high: Any, low: Any) -> "SwingHighLow":
        high_values = as_float64_series(high)
        low_values = as_float64_series(low)
        if len(high_values) != len(low_values):
            raise ValueError("high and low must have equal lengths")
        self._state.extend(high_values, low_values)
        self._length += len(high_values)
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float, float] | None:
        return self._state.value

    def reset(self) -> "SwingHighLow":
        self._state.reset()
        self._length = 0
        return self

    def __len__(self) -> int:
        return self._length
