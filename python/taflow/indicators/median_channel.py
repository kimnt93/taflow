"""Public adapter for the native robust Median Channel."""

from typing import Any
import numpy as np
from .._native import MedianChannel as _Native
from .._series import as_float64_series


class MedianChannel:
    """Build robust rolling bands from the median and median deviation.

    ``middle`` is the rolling median, and the upper/lower bands are
    ``middle ± multiplier * median(abs(price - middle))``. Output order is
    ``(upper, middle, lower)``; warm-up is ``NaN``. This maps to Wickra
    ``MedianChannel``.

    Args:
        prices: Chronological price history.
        period: Positive rolling window length, default 20.
        multiplier: Positive MAD multiplier, default 2.0.

    Raises:
        ValueError: If period or multiplier is not positive.
    """

    def __init__(self, period: int = 20, multiplier: float = 2.0) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native(period, multiplier)

    def append(self, price: float) -> "MedianChannel":
        """Append one price and return this adapter."""
        self._state.append(float(price))
        return self

    def extend(self, prices: Any) -> "MedianChannel":
        """Append one converted float64 price history and return this adapter."""
        self._state.extend(as_float64_series(prices))
        return self

    @property
    def value(self) -> tuple[float, float, float] | None:
        """Return latest upper/middle/lower tuple, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        """Return aligned upper, middle, and lower histories."""
        return self._state.compute()

    def reset(self) -> "MedianChannel":
        """Reset native window and scratch state, then return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)
