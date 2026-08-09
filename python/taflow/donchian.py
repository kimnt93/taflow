"""Persistent Donchian channel adapter."""

from typing import Any

import numpy as np

from ._native import DonchianOperator as _Native
from ._series import as_float64_series


class Donchian:
    """Causal rolling high, low, and midpoint channel.

    Parameters
    ----------
    high, low : array-like
        Aligned high and low price histories. Empty arrays create a fresh
        stream for later ``append`` calls.
    timeperiod : int, default 20
        Number of bars in the trailing extrema window.

    The first ``timeperiod - 1`` aligned outputs are ``NaN``. ``compute``
    returns ``(upper, lower, middle)`` NumPy arrays. Lifecycle methods mutate
    and return this adapter; ``value`` exposes the latest tuple or ``None``.
    """

    def __init__(
        self,
        high: Any,
        low: Any,
        timeperiod: int = 20,
    ) -> None:
        """Create the native state and replay the aligned input histories."""
        self._state = _Native(timeperiod)
        self.extend(high, low)

    def append(self, high: float, low: float) -> "Donchian":
        """Append one high/low bar and return this adapter."""
        self._state.append(float(high), float(low))
        return self

    def extend(self, high: Any, low: Any) -> "Donchian":
        """Append aligned high and low histories and return this adapter."""
        high_array = as_float64_series(high)
        low_array = as_float64_series(low)
        if high_array.shape != low_array.shape:
            raise ValueError("high and low must have equal lengths")
        self._state.extend(high_array, low_array)
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        """Return ``(upper, lower, middle)`` aligned output arrays."""
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float, float] | None:
        """Return the latest ``(upper, lower, middle)`` tuple or ``None``."""
        return self._state.value

    def reset(self) -> "Donchian":
        """Reset native state and output history, returning this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return len(self._state.compute()[0])
