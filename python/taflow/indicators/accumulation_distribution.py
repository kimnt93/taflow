"""Persistent Accumulation/Distribution adapter."""

from typing import Any

import numpy as np

from .._native import AccumulationDistribution as _NativeAccumulationDistribution
from .._series import as_float64_series


class AccumulationDistribution:
    """Accumulate close-location value multiplied by volume in Rust.

    For each bar the increment is
    ``((close-low) - (high-close)) / (high-low) * volume``; a non-positive
    range contributes zero. The constructor requires aligned chronological
    high, low, close, and volume series. Pass four empty arrays for a fresh
    streaming state. There is no warm-up. This maps to TA-Lib ``AD``.
    """

    def __init__(self, high: Any, low: Any, close: Any, volume: Any) -> None:
        self._state = _NativeAccumulationDistribution()
        self.extend(high, low, close, volume)

    def append(self, high: float, low: float, close: float, volume: float) -> "AccumulationDistribution":
        """Append one high/low/close/volume tuple and return this indicator."""
        self._state.append(float(high), float(low), float(close), float(volume))
        return self

    def extend(self, high: Any, low: Any, close: Any, volume: Any) -> "AccumulationDistribution":
        """Append aligned price and volume histories and return this indicator."""
        self._state.extend(
            as_float64_series(high),
            as_float64_series(low),
            as_float64_series(close),
            as_float64_series(volume),
        )
        return self

    def compute(self) -> np.ndarray:
        """Return the complete aligned ``float64`` A/D history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest value, or ``None`` before the first tuple."""
        return self._state.value

    def reset(self) -> "AccumulationDistribution":
        """Restore fresh native state, clear history, and return this indicator."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed tuples."""
        return len(self._state)
