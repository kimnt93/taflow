"""Persistent rolling midpoint adapter."""

from typing import Any

import numpy as np

from .._native import RollingMidpoint as _NativeRollingMidpoint
from .._series import as_float64_series


class RollingMidpoint:
    """Compute the midpoint of rolling highs and lows in native Rust state.

    ``values`` is required and may be empty for a fresh stream. ``timeperiod``
    defaults to 14 and must be positive. The first ``timeperiod - 1`` outputs
    are NaN; ``compute`` returns one aligned float64 array. This maps to
    TA-Lib ``MIDPOINT``.
    """

    def __init__(self, values: Any, timeperiod: int = 14) -> None:
        self._state = _NativeRollingMidpoint(timeperiod)
        self.extend(values)

    def append(self, value: float) -> "RollingMidpoint":
        """Append one value and return this indicator."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "RollingMidpoint":
        """Append a chronological series and return this indicator."""
        self._state.extend(as_float64_series(values))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned rolling midpoint history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest midpoint, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "RollingMidpoint":
        """Restore fresh native state and return this indicator."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed values."""
        return len(self._state)
