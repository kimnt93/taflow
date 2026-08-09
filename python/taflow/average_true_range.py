"""Persistent Average True Range adapter."""

from typing import Any

import numpy as np

from ._native import AverageTrueRange as _NativeAverageTrueRange
from ._series import as_float64_series


class AverageTrueRange:
    """Compute Wilder-smoothed True Range in persistent Rust state.

    High, low, and close histories are required; pass three empty arrays for a
    fresh state. ``timeperiod`` defaults to 14 and must be positive. The first
    ``timeperiod`` outputs are NaN. This maps to TA-Lib ``ATR``.
    """

    def __init__(self, high: Any, low: Any, close: Any, timeperiod: int = 14) -> None:
        self._state = _NativeAverageTrueRange(timeperiod)
        self.extend(high, low, close)

    def append(self, high: float, low: float, close: float) -> "AverageTrueRange":
        """Append one high/low/close tuple and return this indicator."""
        self._state.append(float(high), float(low), float(close))
        return self

    def extend(self, high: Any, low: Any, close: Any) -> "AverageTrueRange":
        """Append aligned high, low, and close histories and return this indicator."""
        self._state.extend(
            as_float64_series(high), as_float64_series(low), as_float64_series(close)
        )
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned float64 history with NaN warm-up."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest value, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "AverageTrueRange":
        """Restore fresh native state, clear history, and return this indicator."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed tuples."""
        return len(self._state)
