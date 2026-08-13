"""Public adapter for native rolling median absolute deviation."""

from typing import Any

import numpy as np

from .._native import RollingMedianAbsoluteDeviation as _Native
from .._series import as_float64_series


class RollingMedianAbsoluteDeviation:
    """Compute raw median absolute deviation over a rolling window.

    The result is ``median(abs(x - median(x)))`` without Gaussian scaling.
    The first ``period - 1`` outputs are ``NaN``. This definition maps to
    Wickra ``MedianAbsoluteDeviation``.

    Args:
        values: Chronological observations.
        period: Rolling window length, default 20.

    Raises:
        ValueError: If ``period`` is zero.
    """

    def __init__(self, period: int = 20) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native(period)

    def append(self, value: float) -> "RollingMedianAbsoluteDeviation":
        """Append one observation and return this instance."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "RollingMedianAbsoluteDeviation":
        """Append a float64-compatible series and return this instance."""
        self._state.extend(as_float64_series(values))
        return self

    @property
    def value(self) -> float | None:
        """Return latest raw MAD, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return aligned MAD history with warm-up represented by ``NaN``."""
        return self._state.compute()

    def reset(self) -> "RollingMedianAbsoluteDeviation":
        """Clear rolling and scratch state, then return this instance."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of observations stored by native state."""
        return len(self._state)
