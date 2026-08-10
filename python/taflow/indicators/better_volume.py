"""Public adapter for the native Better Volume state."""

from typing import Any

import numpy as np

from .._native import BetterVolume as _Native
from .._series import as_float64_series


class BetterVolume:
    """Measure volume effort relative to the achieved high-low range.

    The oscillator is ``volume / SMA(volume) - range / SMA(range)``. Positive
    values describe unusually high effort for the resulting range. The first
    ``period - 1`` outputs are ``NaN``. This contract maps to Wickra
    ``BetterVolume``; arithmetic and warm-up are owned by Rust.

    Args:
        high: Initial chronological high prices.
        low: Initial chronological low prices.
        close: Initial chronological closes; retained for the OHLCV API order.
        volume: Initial chronological volumes.
        period: Simple-average lookback, default 20.

    Raises:
        ValueError: If inputs are misaligned or ``period`` is zero.
    """

    def __init__(self, high: Any, low: Any, close: Any, volume: Any, period: int = 20) -> None:
        """Initialize the native state and process the supplied history."""
        self._state = _Native(period)
        self.extend(high, low, close, volume)

    def append(self, high: float, low: float, close: float, volume: float) -> "BetterVolume":
        """Append one OHLCV bar and return this instance."""
        self._state.append(float(high), float(low), float(close), float(volume))
        return self

    def extend(self, high: Any, low: Any, close: Any, volume: Any) -> "BetterVolume":
        """Append aligned OHLCV histories and return this instance.

        Raises:
            ValueError: If the four converted float64 series differ in length.
        """
        series = tuple(as_float64_series(item) for item in (high, low, close, volume))
        if len({len(item) for item in series}) != 1:
            raise ValueError("OHLCV inputs must have equal lengths")
        self._state.extend(*series)
        return self

    @property
    def value(self) -> float | None:
        """Return the latest oscillator value, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return all aligned outputs with ``NaN`` at warm-up positions."""
        return self._state.compute()

    def reset(self) -> "BetterVolume":
        """Clear all observations and return this instance."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of bars stored by the native state."""
        return len(self._state)
