"""Canonical native-backed MACD interface."""

from typing import Any

import numpy as np

from ._native import StatefulMacd
from ._series import as_float64_series


class MovingAverageConvergenceDivergence:
    """Compute MACD, signal, and histogram with persistent native state."""

    def __init__(
        self,
        fast_period: int = 12,
        slow_period: int = 26,
        signal_period: int = 9,
        _input: Any | None = None,
    ) -> None:
        self._state = StatefulMacd(fast_period, slow_period, signal_period)
        self._values: list[tuple[float, float, float]] = []
        if _input is not None:
            self.extend(_input)

    def append(self, _input: float) -> "MovingAverageConvergenceDivergence":
        value = self._state.append(float(_input))
        self._values.append(
            (np.nan, np.nan, np.nan) if value is None else tuple(value)
        )
        return self

    def extend(self, _input: Any) -> "MovingAverageConvergenceDivergence":
        values = self._state.extend(as_float64_series(_input))
        arrays = tuple(np.asarray(value, dtype=np.float64) for value in values)
        self._values.extend(zip(*arrays))
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        if not self._values:
            empty = np.empty(0, dtype=np.float64)
            return empty.copy(), empty.copy(), empty.copy()
        return tuple(
            np.asarray(values, dtype=np.float64)
            for values in zip(*self._values)
        )

    @property
    def value(self):
        return self._state.value

    def reset(self) -> "MovingAverageConvergenceDivergence":
        self._state.reset()
        self._values.clear()
        return self
