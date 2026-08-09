"""Persistent Williams Percent R adapter."""

from typing import Any

import numpy as np

from ._native import WilliamsPercentR as _NativeWilliamsPercentR
from ._series import as_float64_series


class WilliamsPercentR:
    """Locate close within the trailing high-low range on a -100 to 0 scale.

    The constructor requires aligned chronological high, low, and close series.
    Pass three empty arrays for a fresh streaming state. ``timeperiod`` defaults
    to 14 and must be at least 2. The first ``timeperiod - 1`` aligned outputs
    are NaN; a zero trailing range produces ``0.0``. This maps to TA-Lib
    ``WILLR``.
    """

    def __init__(self, high: Any, low: Any, close: Any, timeperiod: int = 14) -> None:
        self._state = _NativeWilliamsPercentR(timeperiod)
        self.extend(high, low, close)

    def append(self, high: float, low: float, close: float) -> "WilliamsPercentR":
        """Append one high/low/close tuple and return this indicator."""
        self._state.append(float(high), float(low), float(close))
        return self

    def extend(self, high: Any, low: Any, close: Any) -> "WilliamsPercentR":
        """Append aligned high, low, and close histories and return this indicator."""
        self._state.extend(
            as_float64_series(high), as_float64_series(low), as_float64_series(close)
        )
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned ``float64`` Williams Percent R history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest value, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "WilliamsPercentR":
        """Restore fresh native state, clear history, and return this indicator."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed tuples."""
        return len(self._state)
