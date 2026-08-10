"""Public adapter for the native Demand Index state."""

from typing import Any

import numpy as np

from .._native import DemandIndex as _Native
from .._series import as_float64_series


class DemandIndex:
    """Measure EMA-smoothed price and volume pressure from OHLCV bars.

    Rust combines the close return, high-low range, and volume for each bar,
    then smooths pressure with an SMA-seeded EMA. The first close establishes
    the return baseline. This definition maps to Wickra ``DemandIndex`` 0.9.9;
    TA-Lib has no direct equivalent.

    Args:
        high: Initial chronological high-price series.
        low: Initial chronological low-price series.
        close: Initial chronological closing-price series.
        volume: Initial chronological volume series.
        timeperiod: EMA smoothing period. Defaults to 10.

    Raises:
        ValueError: If the four series have different lengths or the period is
            zero.
    """

    def __init__(
        self,
        high: Any,
        low: Any,
        close: Any,
        volume: Any,
        timeperiod: int = 10,
    ) -> None:
        """Initialize the index and process aligned OHLCV history."""
        self._state = _Native(int(timeperiod))
        self.extend(high, low, close, volume)

    def append(
        self,
        high: float,
        low: float,
        close: float,
        volume: float,
    ) -> "DemandIndex":
        """Append one high/low/close/volume bar and return this instance."""
        self._state.append(float(high), float(low), float(close), float(volume))
        return self

    def extend(
        self,
        high: Any,
        low: Any,
        close: Any,
        volume: Any,
    ) -> "DemandIndex":
        """Append aligned high, low, close, and volume series.

        Returns:
            This instance, allowing method chaining.

        Raises:
            ValueError: If the four input series have different lengths.
        """
        series = tuple(
            as_float64_series(item) for item in (high, low, close, volume)
        )
        if len({len(item) for item in series}) != 1:
            raise ValueError("high, low, close, and volume must have equal lengths")
        self._state.extend(*series)
        return self

    @property
    def value(self) -> float | None:
        """Return the latest demand value, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return aligned demand values, including warm-up ``NaN``."""
        return self._state.compute()

    def reset(self) -> "DemandIndex":
        """Clear the previous close and EMA state, then return this instance."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of aligned bars processed by Rust."""
        return len(self._state)


__all__ = ["DemandIndex"]
