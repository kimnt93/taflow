"""Public adapter for native Williams Accumulation/Distribution."""

from typing import Any

import numpy as np

from .._native import WilliamsAccumulationDistribution as _Native
from .._series import as_float64_series


class WilliamsAccumulationDistribution:
    """Accumulate price movement relative to true highs and true lows.

    Up-closes add ``close - min(low, previous_close)`` and down-closes add
    ``close - max(high, previous_close)``. This indicator uses no volume. The
    first bar seeds the previous close and returns ``NaN``. It maps to Wickra
    ``Wad``.

    Args:
        high: Chronological high prices.
        low: Chronological low prices.
        close: Chronological closing prices.

    Raises:
        ValueError: If the three series differ in length.
    """

    def __init__(self) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native()

    def append(self, high: float, low: float, close: float) -> "WilliamsAccumulationDistribution":
        """Append one high/low/close bar and return this instance."""
        self._state.append(float(high), float(low), float(close))
        return self

    def extend(self, high: Any, low: Any, close: Any) -> "WilliamsAccumulationDistribution":
        """Append aligned high, low, and close histories and return self."""
        series = tuple(as_float64_series(item) for item in (high, low, close))
        if len({len(item) for item in series}) != 1:
            raise ValueError("high, low, and close must have equal lengths")
        self._state.extend(*series)
        return self

    @property
    def value(self) -> float | None:
        """Return latest cumulative WAD, or ``None`` before the second bar."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return aligned WAD history with the seed position as ``NaN``."""
        return self._state.compute()

    def reset(self) -> "WilliamsAccumulationDistribution":
        """Clear previous-close and cumulative state, then return self."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of bars stored by native state."""
        return len(self._state)
