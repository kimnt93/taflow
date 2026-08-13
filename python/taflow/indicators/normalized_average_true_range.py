"""Persistent Normalized Average True Range adapter."""

from typing import Any

import numpy as np

from .._native import NormalizedAverageTrueRange as _NativeNormalizedAverageTrueRange
from .._series import as_float64_series


class NormalizedAverageTrueRange:
    """Compute ``100 * AverageTrueRange / close`` in persistent Rust state.

    Supply aligned high, low, and close histories through ``extend``. ``timeperiod`` defaults to 14 and must be positive. Warm-up is
    NaN through index ``timeperiod - 1`` and a warmed zero close returns zero.
    TA-Lib's historical period-1 contract returns raw True Range without
    normalization; this adapter preserves that special case. This maps to
    TA-Lib ``NATR``.
    """

    def __init__(self, timeperiod: int = 14) -> None:
        self._state = _NativeNormalizedAverageTrueRange(timeperiod)

    def append(
        self, high: float, low: float, close: float
    ) -> "NormalizedAverageTrueRange":
        """Append one high/low/close tuple and return this indicator."""
        self._state.append(float(high), float(low), float(close))
        return self

    def extend(self, high: Any, low: Any, close: Any) -> "NormalizedAverageTrueRange":
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

    def reset(self) -> "NormalizedAverageTrueRange":
        """Restore fresh native state, clear history, and return this indicator."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed tuples."""
        return len(self._state)
