"""Canonical native-backed T3 moving-average interface."""

from typing import Any

import numpy as np

from ._native import StatefulT3
from ._series import as_float64_series


class TripleExponentialAverage:
    """Compute the T3 moving average with persistent native state."""

    def __init__(
        self,
        timeperiod: int = 5,
        volume_factor: float = 0.7,
        _input: Any | None = None,
    ) -> None:
        self._state = StatefulT3(timeperiod, volume_factor)
        if _input is not None:
            self.extend(_input)

    def append(self, _input: float) -> "TripleExponentialAverage":
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "TripleExponentialAverage":
        self._state.extend(as_float64_series(_input))
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self):
        return self._state.value

    def reset(self) -> "TripleExponentialAverage":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)
