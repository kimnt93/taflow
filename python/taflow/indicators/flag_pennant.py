"""Python adapter for native Flag/Pennant detection."""

from typing import Any
import numpy as np
from .._native import FlagPennant as _Native
from .._series import as_float64_series


class FlagPennant:
    """Detect a shallow consolidation after a directional pole.

    Three confirmed 5% swing pivots define the pole and pullback. A pullback
    smaller than half the pole emits ``1.0`` after an up-pole or ``-1.0`` after
    a down-pole; every other bar emits ``0.0``. Geometry uses high and low while
    open and close preserve the common OHLC contract. This maps to Wickra
    ``FlagPennant``.

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

    def append(self, open: float, high: float, low: float, close: float) -> "FlagPennant":
        """Append one OHLC bar and return this adapter."""
        self._state.append(float(open), float(high), float(low), float(close))
        return self

    def extend(self, open: Any, high: Any, low: Any, close: Any) -> "FlagPennant":
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

    def reset(self) -> "FlagPennant":
        """Clear confirmed pivots and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)
