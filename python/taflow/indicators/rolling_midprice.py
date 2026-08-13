"""Persistent rolling midprice adapter."""

from typing import Any

import numpy as np

from .._native import RollingMidprice as _NativeRollingMidprice
from .._series import as_float64_series


class RollingMidprice:
    """Compute midpoint of rolling high maxima and low minima in Rust.

    ``high`` and ``low`` are required aligned series and may both be empty for
    a fresh stream. ``timeperiod`` defaults to 14 and must be positive. The
    first ``timeperiod - 1`` outputs are NaN; ``compute`` returns one aligned
    float64 array. This maps to TA-Lib ``MIDPRICE``.
    """

    def __init__(self, timeperiod: int = 14) -> None:
        self._state = _NativeRollingMidprice(timeperiod)

    def append(self, high: float, low: float) -> "RollingMidprice":
        """Append one aligned high/low pair and return this indicator."""
        self._state.append(float(high), float(low))
        return self

    def extend(self, high: Any, low: Any) -> "RollingMidprice":
        """Append aligned high and low series and return this indicator."""
        self._state.extend(as_float64_series(high), as_float64_series(low))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned rolling midprice history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest midprice, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "RollingMidprice":
        """Restore fresh native state and return this indicator."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed pairs."""
        return len(self._state)
