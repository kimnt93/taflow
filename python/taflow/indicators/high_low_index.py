"""Python adapter for the native High-Low Index."""

from typing import Any
import numpy as np
from .._native import HighLowIndex as _Native
from .._series import as_float64_series


class HighLowIndex:
    """Smooth the percentage of new extremes that are new highs.

    Each tick contributes ``100 * new_highs / max(new_highs + new_lows, 1)``;
    native Rust returns its simple mean over ``period`` ticks. Warm-up is
    represented by ``NaN``. This maps to Wickra ``HighLowIndex``.

    Args:
        new_highs: Number of constituents making a new high at each tick.
        new_lows: Number making a new low at each aligned tick.
        period: Smoothing window. Defaults to 10.

    Raises:
        ValueError: If histories differ in length or ``period`` is zero.
    """

    def __init__(self, period: int = 10) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native(period)

    def append(self, new_highs: float, new_lows: float) -> "HighLowIndex":
        """Append one aggregate-extremes tick and return this adapter."""
        self._state.append(float(new_highs), float(new_lows))
        return self

    def extend(self, new_highs: Any, new_lows: Any) -> "HighLowIndex":
        """Append aligned extreme-count histories after length validation."""
        arrays = as_float64_series(new_highs), as_float64_series(new_lows)
        if len(arrays[0]) != len(arrays[1]):
            raise ValueError("new-high and new-low counts must have equal lengths")
        self._state.extend(*arrays)
        return self

    @property
    def value(self) -> float | None:
        """Return the latest smoothed percentage, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return aligned High-Low Index history with warm-up as ``NaN``."""
        return self._state.compute()

    def reset(self) -> "HighLowIndex":
        """Clear the rolling window and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-tick count delegated to native state."""
        return len(self._state)
