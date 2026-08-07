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
        self._values: list[float] = []
        if _input is not None:
            self.extend(_input)

    def append(self, _input: float) -> "TripleExponentialAverage":
        value = self._state.append(float(_input))
        self._values.append(np.nan if value is None else float(value))
        return self

    def extend(self, _input: Any) -> "TripleExponentialAverage":
        values = self._state.extend(as_float64_series(_input))
        self._values.extend(np.asarray(values, dtype=np.float64).tolist())
        return self

    def compute(self) -> np.ndarray:
        return np.asarray(self._values, dtype=np.float64)

    @property
    def value(self):
        return self._state.value

    def reset(self) -> "TripleExponentialAverage":
        self._state.reset()
        self._values.clear()
        return self
