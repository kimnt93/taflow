"""Public adapter for the native Trade Volume Index state."""

from typing import Any

import numpy as np

from .._native import TradeVolumeIndex as _Native
from .._series import as_float64_series


class TradeVolumeIndex:
    """Accumulate volume using a persistent minimum-tick price direction.

    A close change above ``min_tick`` selects positive volume and a change
    below ``-min_tick`` selects negative volume. Smaller changes retain the
    previous direction. The first bar seeds its close and outputs ``NaN``.
    This class maps to Wickra ``TradeVolumeIndex``.

    Args:
        close: Initial chronological closing prices.
        volume: Initial chronological volumes.
        min_tick: Non-negative direction threshold, default 0.25.

    Raises:
        ValueError: If inputs are misaligned or ``min_tick`` is invalid.
    """

    def __init__(self, close: Any, volume: Any, min_tick: float = 0.25) -> None:
        """Initialize the state and process the supplied close/volume history."""
        self._state = _Native(min_tick)
        self.extend(close, volume)

    def append(self, close: float, volume: float) -> "TradeVolumeIndex":
        """Append one close/volume sample and return this instance."""
        self._state.append(float(close), float(volume))
        return self

    def extend(self, close: Any, volume: Any) -> "TradeVolumeIndex":
        """Append aligned close and volume histories and return this instance."""
        close_series = as_float64_series(close)
        volume_series = as_float64_series(volume)
        if len(close_series) != len(volume_series):
            raise ValueError("close and volume must have equal lengths")
        self._state.extend(close_series, volume_series)
        return self

    @property
    def value(self) -> float | None:
        """Return the latest cumulative index, or ``None`` after no output."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return aligned TVI history, including the initial warm-up ``NaN``."""
        return self._state.compute()

    def reset(self) -> "TradeVolumeIndex":
        """Reset close, direction, and total, then return this instance."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of samples stored by the native state."""
        return len(self._state)
