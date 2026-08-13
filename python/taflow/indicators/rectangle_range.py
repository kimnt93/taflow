"""Python adapter for native rectangle-range detection."""

from typing import Any

import numpy as np

from .._native import RectangleRange as _Native
from .._series import as_float64_series


class RectangleRange:
    """Detect horizontal support-and-resistance rectangles from swing pivots.

    The native state confirms 5% swings and checks whether the last two highs
    and lows are each flat within 3%. A support touch emits ``1.0``, a resistance
    touch emits ``-1.0``, and all other bars emit ``0.0``. Open and close are
    accepted for the common OHLC contract but geometry uses high and low. This
    maps to Wickra ``RectangleRange``.

    Args:
        open: Required opening-price history.
        high: Required high-price history.
        low: Required low-price history.
        close: Required closing-price history.

    Raises:
        ValueError: If the four OHLC histories have different lengths.
    """

    def __init__(self) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native()

    def append(
        self, open: float, high: float, low: float, close: float
    ) -> "RectangleRange":
        """Append one OHLC bar and return this adapter."""
        self._state.append(float(open), float(high), float(low), float(close))
        return self

    def extend(self, open: Any, high: Any, low: Any, close: Any) -> "RectangleRange":
        """Append aligned OHLC histories after validating their lengths."""
        arrays = tuple(as_float64_series(item) for item in (open, high, low, close))
        if len({len(item) for item in arrays}) != 1:
            raise ValueError("OHLC inputs must have equal lengths")
        self._state.extend(*arrays)
        return self

    @property
    def value(self) -> float | None:
        """Return the latest range signal, or ``None`` before the first bar."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return the aligned ``-1``, ``0``, and ``1`` signal history."""
        return self._state.compute()

    def reset(self) -> "RectangleRange":
        """Clear confirmed pivots and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)
