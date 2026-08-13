"""Persistent hundred-scaled Rate of Change Ratio adapter."""

from typing import Any

import numpy as np

from .._native import RateOfChangeRatioPercent as _NativeRateOfChangeRatioPercent
from .._series import as_float64_series


class RateOfChangeRatioPercent:
    """Compute ``100 * current / previous`` in persistent Rust state.

    ``values`` is required; supply the series through ``extend`` after construction. ``timeperiod`` defaults to 14 and must be positive. The first
    ``timeperiod`` outputs are NaN; a warmed zero denominator produces zero,
    matching TA-Lib ``ROCR100``.
    """

    def __init__(self, timeperiod: int = 14) -> None:
        self._state = _NativeRateOfChangeRatioPercent(timeperiod)

    def append(self, value: float) -> "RateOfChangeRatioPercent":
        """Append one chronological value and return this indicator."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "RateOfChangeRatioPercent":
        """Append a chronological series and return this indicator."""
        self._state.extend(as_float64_series(values))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned float64 history with NaN warm-up."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest hundred-scaled ratio, or ``None`` in warm-up."""
        return self._state.value

    def reset(self) -> "RateOfChangeRatioPercent":
        """Restore fresh native state, clear history, and return this indicator."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed values."""
        return len(self._state)
