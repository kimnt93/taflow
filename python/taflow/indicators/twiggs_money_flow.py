"""Public adapter for native Twiggs Money Flow."""

from typing import Any

import numpy as np

from .._native import TwiggsMoneyFlow as _Native
from .._series import as_float64_series


class TwiggsMoneyFlow:
    """Compute true-range money flow with Wilder smoothing.

    Each bar uses the previous close to form true high and true low. Its signed
    accumulation/distribution flow and volume are independently Wilder-smoothed,
    then divided. One bar seeds the previous close and the next ``period`` bars
    seed both averages. This maps to Wickra ``TwiggsMoneyFlow``.

    Args:
        high: Initial chronological high prices.
        low: Initial chronological low prices.
        close: Initial chronological closing prices.
        volume: Initial chronological volumes.
        period: Wilder smoothing period, default 21.

    Raises:
        ValueError: If series lengths differ or ``period`` is zero.
    """

    def __init__(self, high: Any, low: Any, close: Any, volume: Any, period: int = 21) -> None:
        """Initialize native state and process the aligned OHLCV history."""
        self._state = _Native(period)
        self.extend(high, low, close, volume)

    def append(self, high: float, low: float, close: float, volume: float) -> "TwiggsMoneyFlow":
        """Append one high/low/close/volume bar and return this instance."""
        self._state.append(float(high), float(low), float(close), float(volume))
        return self

    def extend(self, high: Any, low: Any, close: Any, volume: Any) -> "TwiggsMoneyFlow":
        """Append aligned OHLCV histories and return this instance.

        Raises:
            ValueError: If the converted float64 series differ in length.
        """
        series = tuple(as_float64_series(item) for item in (high, low, close, volume))
        if len({len(item) for item in series}) != 1:
            raise ValueError("OHLCV inputs must have equal lengths")
        self._state.extend(*series)
        return self

    @property
    def value(self) -> float | None:
        """Return latest money-flow ratio, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return aligned TMF history with warm-up represented by ``NaN``."""
        return self._state.compute()

    def reset(self) -> "TwiggsMoneyFlow":
        """Clear seed and Wilder-average state, then return this instance."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of bars stored by native state."""
        return len(self._state)
