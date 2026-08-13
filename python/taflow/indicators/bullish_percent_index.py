"""Python adapter for the native Bullish Percent Index."""

from typing import Any
import numpy as np
from .._native import BullishPercentIndex as _Native
from .._series import as_float64_series


class BullishPercentIndex:
    """Return the percentage of a universe on point-and-figure buy signals.

    The result is ``100 * on_buy_signal_count / universe_size`` and maps to
    Wickra ``BullishPercentIndex``.

    Args:
        on_buy_signal_count: Number of constituent buy signals at each tick.
        universe_size: Number of constituents at each aligned tick.

    Raises:
        ValueError: If the input histories have different lengths.
    """

    def __init__(self) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native()

    def append(
        self, on_buy_signal_count: float, universe_size: float
    ) -> "BullishPercentIndex":
        """Append one market-wide count pair and return this adapter."""
        self._state.append(float(on_buy_signal_count), float(universe_size))
        return self

    def extend(
        self, on_buy_signal_count: Any, universe_size: Any
    ) -> "BullishPercentIndex":
        """Append aligned count histories after validating their lengths."""
        arrays = as_float64_series(on_buy_signal_count), as_float64_series(universe_size)
        if len(arrays[0]) != len(arrays[1]):
            raise ValueError("signal count and universe size must have equal lengths")
        self._state.extend(*arrays)
        return self

    @property
    def value(self) -> float | None:
        """Return the latest percentage, or ``None`` before input."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return the aligned percentage history."""
        return self._state.compute()

    def reset(self) -> "BullishPercentIndex":
        """Reset native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-tick count delegated to native state."""
        return len(self._state)
