"""Persistent exponentially weighted moving sum."""

from typing import Any

import numpy as np

from .._native import ExponentiallyWeightedSum as _Native
from .._adapter_protocol import adapter_length
from .._series import as_float64_series


class ExponentiallyWeightedSum:
    """Compute a causal exponentially weighted sum using span ``timeperiod``.

    ``_input`` is the required chronological series and may be empty for a
    fresh stream. ``timeperiod`` defaults to 14 and must be positive. The
    smoothing factor is ``2 / (timeperiod + 1)`` and the Rust recurrence is
    ``sum_t = x_t + (1 - alpha) * sum_(t-1)``. ``compute`` returns one aligned
    float array, ``value`` is the latest scalar, and lifecycle mutators return
    ``self``. The independent oracle is pandas ``ExponentialMovingWindow.sum``.
    """

    def __init__(
        self,
        _input: Any,
        timeperiod: int = 14,
    ) -> None:
        self._state = _Native(int(timeperiod))
        self.extend(_input)

    def append(self, _input: float) -> "ExponentiallyWeightedSum":
        """Append one observation and return this adapter."""
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "ExponentiallyWeightedSum":
        """Append a chronological input series and return this adapter."""
        values = as_float64_series(_input)
        self._state.extend(values)
        return self

    def compute(self) -> np.ndarray:
        """Return the complete aligned weighted-sum history."""
        return self._state.compute()

    def __len__(self) -> int:
        """Return the number of observations consumed by this state."""
        return adapter_length(self)

    @property
    def value(self) -> float | None:
        """Return the latest weighted sum."""
        return self._state.value

    def reset(self) -> "ExponentiallyWeightedSum":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self


__all__ = ["ExponentiallyWeightedSum"]
