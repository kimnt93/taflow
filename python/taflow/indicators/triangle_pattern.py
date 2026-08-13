"""Python adapter for native triangle-pattern detection."""

from typing import Any
import numpy as np
from .._native import TrianglePattern as _Native
from .._series import as_float64_series


class TrianglePattern:
    """Detect ascending, descending, and symmetrical triangles.

    Four confirmed 5% pivots define two highs and lows. Flat highs with rising
    lows emit ``1.0``; falling highs with flat lows emit ``-1.0``; a symmetrical
    triangle follows the latest pivot direction. Flat levels use a 3% relative
    tolerance. Other bars emit ``0.0``. This maps to Wickra ``Triangle``.

    Args:
        open: Required opening-price history.
        high: Required high-price history.
        low: Required low-price history.
        close: Required closing-price history.

    Raises:
        ValueError: If OHLC histories have different lengths.
    """

    def __init__(self) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native()

    def append(
        self, open: float, high: float, low: float, close: float
    ) -> "TrianglePattern":
        """Append one OHLC bar and return this adapter."""
        self._state.append(float(open), float(high), float(low), float(close))
        return self

    def extend(
        self, open: Any, high: Any, low: Any, close: Any
    ) -> "TrianglePattern":
        """Append aligned OHLC histories after length validation."""
        arrays = tuple(as_float64_series(item) for item in (open, high, low, close))
        if len({len(item) for item in arrays}) != 1:
            raise ValueError("OHLC inputs must have equal lengths")
        self._state.extend(*arrays)
        return self

    @property
    def value(self) -> float | None:
        """Return the latest signal, or ``None`` before the first bar."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return aligned ``-1``, ``0``, and ``1`` signals."""
        return self._state.compute()

    def reset(self) -> "TrianglePattern":
        """Clear confirmed pivots and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)
