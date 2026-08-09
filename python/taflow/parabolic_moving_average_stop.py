"""Canonical native-backed Parabolic Moving Average Stop adapter."""

from typing import Any

import numpy as np

from ._native import ParabolicMovingAverageStop as _NativeParabolicMovingAverageStop
from ._series import as_float64_series


class ParabolicMovingAverageStop:
    """Compute pandas-ta-compatible EMA/ATR PMAX bands and trend direction.

    ``high``, ``low``, and ``close`` are required aligned chronological
    series; empty arrays create a fresh stream. ``length`` defaults to 10 and
    ``multiplier`` to 3.0. Rust owns Wilder ATR/EMA seeding, PMAX band
    transitions, warm-up (``value`` is ``None`` until ``length - 1`` bars),
    and aligned output. ``compute`` returns ``(stop, trend)`` arrays, while
    lifecycle mutators return ``self``. The independent oracle is
    ``pandas-ta-classic.pmax``.
    """

    def __init__(
        self,
        high: Any,
        low: Any,
        close: Any,
        length: int = 10,
        multiplier: float = 3.0,
    ) -> None:
        self._state = _NativeParabolicMovingAverageStop(length, multiplier)
        self.extend(high, low, close)

    def append(self, high: float, low: float, close: float) -> "ParabolicMovingAverageStop":
        """Append one OHLC bar and return this adapter."""
        self._state.append(float(high), float(low), float(close))
        return self

    def extend(self, high: Any, low: Any, close: Any) -> "ParabolicMovingAverageStop":
        """Append aligned OHLC histories and return this adapter."""
        high_array = as_float64_series(high)
        low_array = as_float64_series(low)
        close_array = as_float64_series(close)
        if len(high_array) != len(low_array) or len(high_array) != len(close_array):
            raise ValueError("high, low, and close must have equal lengths")
        self._state.extend(high_array, low_array, close_array)
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray]:
        """Return aligned stop and trend arrays."""
        return self._state.compute()

    @property
    def value(self) -> tuple[float, int] | None:
        """Return the latest ``(stop, trend)`` pair, or ``None`` in warm-up."""
        return self._state.value

    def reset(self) -> "ParabolicMovingAverageStop":
        """Reset the state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)


__all__ = ["ParabolicMovingAverageStop"]
