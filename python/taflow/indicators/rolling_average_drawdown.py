"""Public adapter for native rolling average drawdown."""

from typing import Any

import numpy as np

from .._native import RollingAverageDrawdown as _Native
from .._series import as_float64_series


class RollingAverageDrawdown:
    """Average maximum depth of distinct drawdown episodes in each window.

    An episode begins below a running peak and ends on recovery. Its depth is
    measured at its trough; the result averages episode depths rather than
    every underwater bar. ``period - 1`` leading outputs are ``NaN``. The
    independent name mapping is Wickra ``AverageDrawdown``.

    Args:
        values: Chronological equity-curve observations.
        period: Rolling window length, default 14.

    Raises:
        ValueError: If ``period`` is zero.
    """

    def __init__(self, period: int = 14) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native(period)

    def append(self, value: float) -> "RollingAverageDrawdown":
        """Append one equity observation and return this instance."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "RollingAverageDrawdown":
        """Append a chronological float64-compatible series and return self."""
        self._state.extend(as_float64_series(values))
        return self

    @property
    def value(self) -> float | None:
        """Return the latest average drawdown, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return aligned history with warm-up represented by ``NaN``."""
        return self._state.compute()

    def reset(self) -> "RollingAverageDrawdown":
        """Clear the rolling state and return this instance."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of observations stored by native state."""
        return len(self._state)
