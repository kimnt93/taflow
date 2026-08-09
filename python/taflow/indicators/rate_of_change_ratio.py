"""Persistent Rate of Change Ratio adapter."""

from typing import Any

import numpy as np

from .._native import RateOfChangeRatio as _NativeRateOfChangeRatio
from .._series import as_float64_series


class RateOfChangeRatio:
    """Compute ``current / previous`` in persistent Rust state.

    ``values`` is required; pass an empty series to create a fresh streaming
    state. ``timeperiod`` defaults to 14 and must be positive. The first
    ``timeperiod`` outputs are NaN; a warmed zero denominator produces zero,
    matching TA-Lib ``ROCR``.
    """

    def __init__(self, values: Any, timeperiod: int = 14) -> None:
        self._state = _NativeRateOfChangeRatio(timeperiod)
        self.extend(values)

    def append(self, value: float) -> "RateOfChangeRatio":
        """Append one chronological value and return this indicator."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "RateOfChangeRatio":
        """Append a chronological series and return this indicator."""
        self._state.extend(as_float64_series(values))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned float64 history with NaN warm-up."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest rate-of-change ratio, or ``None`` in warm-up."""
        return self._state.value

    def reset(self) -> "RateOfChangeRatio":
        """Restore fresh native state, clear history, and return this indicator."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed values."""
        return len(self._state)
