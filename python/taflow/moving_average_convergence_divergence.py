"""Canonical native-backed MACD interface."""

from typing import Any

import numpy as np

from ._native import StatefulMacd
from ._series import as_float64_series


class MovingAverageConvergenceDivergence:
    """Compute MACD, signal, and histogram with persistent native state."""

    def __init__(
        self,
        _input: Any,
        fast_period: int = 12,
        slow_period: int = 26,
        signal_period: int = 9,
    ) -> None:
        self._state = StatefulMacd(fast_period, slow_period, signal_period)
        if _input is not None:
            self.extend(_input)

    def append(self, _input: float) -> "MovingAverageConvergenceDivergence":
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "MovingAverageConvergenceDivergence":
        self._state.extend(as_float64_series(_input))
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        return self._state.compute()

    @property
    def value(self):
        return self._state.value

    def reset(self) -> "MovingAverageConvergenceDivergence":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)
