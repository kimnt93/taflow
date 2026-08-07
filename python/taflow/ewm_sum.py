"""Persistent exponentially weighted moving sum."""

from typing import Any

import numpy as np

from ._native import EwmSumOperator as _Native
from ._series import as_float64_series


class ExponentiallyWeightedSum:
    """Compute a causal exponentially weighted sum using span ``timeperiod``.

    The smoothing factor is ``2 / (timeperiod + 1)`` and the recurrence is
    ``sum_t = x_t + (1 - alpha) * sum_(t-1)``.
    """

    def __init__(self, timeperiod: int, _input: Any | None = None) -> None:
        self._state = _Native(timeperiod)
        if _input is not None:
            self.extend(_input)

    def append(self, _input: float) -> "ExponentiallyWeightedSum":
        """Append one scalar observation."""
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "ExponentiallyWeightedSum":
        """Append an aligned input series."""
        self._state.extend(as_float64_series(_input))
        return self

    def compute(self) -> np.ndarray:
        """Return the complete aligned weighted-sum history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest weighted sum."""
        return self._state.value

    def reset(self) -> "ExponentiallyWeightedSum":
        """Reset the recurrence and clear output history."""
        self._state.reset()
        return self
