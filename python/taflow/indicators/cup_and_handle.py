"""Python adapter for native cup-and-handle detection."""

from typing import Any

import numpy as np

from .._native import CupAndHandle as _Native
from .._series import as_float64_series


class CupAndHandle:
    """Detect causal cup-and-handle and inverse formations.

    A native 5% non-repainting swing tracker examines four confirmed pivots.
    Matching rims within 3% followed by a shallow handle emit ``1.0`` for a
    bullish cup or ``-1.0`` for an inverse cup; every other bar emits ``0.0``.
    Although open and close are accepted for a uniform OHLC interface, geometry
    uses high and low. The definition maps to Wickra ``CupAndHandle``.

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
    ) -> "CupAndHandle":
        """Append one OHLC bar and return this adapter."""
        self._state.append(float(open), float(high), float(low), float(close))
        return self

    def extend(self, open: Any, high: Any, low: Any, close: Any) -> "CupAndHandle":
        """Append aligned OHLC histories after validating their lengths."""
        arrays = tuple(as_float64_series(item) for item in (open, high, low, close))
        if len({len(item) for item in arrays}) != 1:
            raise ValueError("OHLC inputs must have equal lengths")
        self._state.extend(*arrays)
        return self

    @property
    def value(self) -> float | None:
        """Return the latest pattern signal, or ``None`` before the first bar."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return the aligned ``-1``, ``0``, and ``1`` signal history."""
        return self._state.compute()

    def reset(self) -> "CupAndHandle":
        """Clear confirmed pivots and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)
