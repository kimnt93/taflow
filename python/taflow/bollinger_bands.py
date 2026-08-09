"""Persistent Bollinger Bands adapter."""

from typing import Any

import numpy as np

from ._native import BollingerBands as _NativeBollingerBands
from ._series import as_float64_series


class BollingerBands:
    """Compute upper, middle, and lower Bollinger bands in Rust.

    ``values`` is required and may be empty. ``period`` defaults to 5,
    deviations default to 2.0, and ``moving_average_type`` defaults to 0.
    The output order is ``(upper, middle, lower)`` with NaN warm-up values.
    """

    def __init__(self, values: Any, period: int = 5, deviations_up: float = 2.0,
                 deviations_down: float = 2.0, moving_average_type: int = 0) -> None:
        self._state = _NativeBollingerBands(period, deviations_up, deviations_down, moving_average_type)
        self.extend(values)

    def append(self, value: float) -> "BollingerBands":
        self._state.append(float(value)); return self

    def extend(self, values: Any) -> "BollingerBands":
        self._state.extend(as_float64_series(values)); return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float, float] | None:
        return self._state.value

    def reset(self) -> "BollingerBands":
        self._state.reset(); return self

    def __len__(self) -> int:
        return len(self._state)
