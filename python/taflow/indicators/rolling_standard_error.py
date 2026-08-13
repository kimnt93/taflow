"""Public adapter for native rolling regression standard error."""

from typing import Any

import numpy as np

from .._native import RollingStandardError as _Native
from .._series import as_float64_series


class RollingStandardError:
    """Measure residual error around a rolling least-squares trend.

    Each full window fits ``y = intercept + slope * index`` and returns
    ``sqrt(RSS / (period - 2))``. This is regression standard error, not the
    standard error of a sample mean. It maps to Wickra ``StandardError``.

    Args:
        values: Chronological dependent observations.
        period: Regression window length, default 20 and minimum 3.

    Raises:
        ValueError: If ``period`` is below three.
    """

    def __init__(self, period: int = 20) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native(period)

    def append(self, value: float) -> "RollingStandardError":
        """Append one observation and return this instance."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "RollingStandardError":
        """Append a chronological float64-compatible series and return self."""
        self._state.extend(as_float64_series(values))
        return self

    @property
    def value(self) -> float | None:
        """Return latest regression error, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return aligned errors with leading warm-up ``NaN`` values."""
        return self._state.compute()

    def reset(self) -> "RollingStandardError":
        """Clear rolling regression state and return this instance."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of observations stored by native state."""
        return len(self._state)
