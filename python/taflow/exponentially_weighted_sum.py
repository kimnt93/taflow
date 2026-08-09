"""Persistent exponentially weighted moving sum."""

from typing import Any

import numpy as np

from ._native import ExponentiallyWeightedSumOperator as _Native
from ._series import as_float64_series


class ExponentiallyWeightedSum:
    """Compute a causal exponentially weighted sum using span ``timeperiod``.

    The smoothing factor is ``2 / (timeperiod + 1)`` and the recurrence is
    ``sum_t = x_t + (1 - alpha) * sum_(t-1)``.
    """

    def __init__(
        self,
        _input: Any,
        timeperiod: int = 14,
    ) -> None:
        self._state = _Native(timeperiod)
        self._length = 0
        self.extend(_input)

    def append(self, _input: float) -> "ExponentiallyWeightedSum":
        """Append one scalar observation."""
        self._state.append(float(_input))
        self._length += 1
        return self

    def extend(self, _input: Any) -> "ExponentiallyWeightedSum":
        """Append an aligned input series."""
        values = as_float64_series(_input)
        self._state.extend(values)
        self._length += len(values)
        return self

    def compute(self) -> np.ndarray:
        """Return the complete aligned weighted-sum history."""
        return self._state.compute()

    def __len__(self) -> int:
        """Return the number of observations consumed by this state."""
        return self._length

    @property
    def value(self) -> float | None:
        """Return the latest weighted sum."""
        return self._state.value

    def reset(self) -> "ExponentiallyWeightedSum":
        """Reset the recurrence and clear output history."""
        self._state.reset()
        self._length = 0
        return self
