"""Python adapter for native Percent Above Moving Average breadth."""

from typing import Any
import numpy as np
from .._native import PercentAboveMovingAverage as _Native
from .._series import as_float64_series


class PercentAboveMovingAverage:
    """Return the percentage of constituents above a chosen moving average.

    Inputs are pre-aggregated counts; the caller chooses and evaluates the
    moving average per constituent. The result maps to Wickra ``PercentAboveMa``.

    Args:
        above_moving_average_count: Constituents above their reference average.
        universe_size: Total constituent count at each aligned tick.

    Raises:
        ValueError: If the two histories have different lengths.
    """

    def __init__(self) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native()

    def append(
        self, above_moving_average_count: float, universe_size: float
    ) -> "PercentAboveMovingAverage":
        """Append one market-wide count pair and return this adapter."""
        self._state.append(float(above_moving_average_count), float(universe_size))
        return self

    def extend(
        self, above_moving_average_count: Any, universe_size: Any
    ) -> "PercentAboveMovingAverage":
        """Append aligned count histories after length validation."""
        arrays = (
            as_float64_series(above_moving_average_count),
            as_float64_series(universe_size),
        )
        if len(arrays[0]) != len(arrays[1]):
            raise ValueError("above count and universe size must have equal lengths")
        self._state.extend(*arrays)
        return self

    @property
    def value(self) -> float | None:
        """Return the latest percentage, or ``None`` before input."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return the aligned percentage history."""
        return self._state.compute()

    def reset(self) -> "PercentAboveMovingAverage":
        """Reset native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-tick count delegated to native state."""
        return len(self._state)
