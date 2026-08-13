"""Regression standard-error bands adapter."""

from typing import Any

import numpy as np

from .._native import StandardErrorBands as _Native
from .._series import as_float64_series


class StandardErrorBands:
    """Compute rolling OLS endpoint bands using residual standard error.

    The native kernel fits a line over each window and offsets its endpoint by
    ``multiplier * sqrt(RSS / (period - 2))``. Histories are ordered
    ``(upper, middle, lower)`` and use ``NaN`` during warm-up. The oracle/name
    mapping is Wickra ``StandardErrorBands``.

    Args:
        values: Required chronological price history.
        period: Regression window, at least three. Defaults to 21.
        multiplier: Positive standard-error multiplier. Defaults to 2.0.

    Raises:
        ValueError: If the configuration is invalid.
    """

    def __init__(self, period: int = 21, multiplier: float = 2.0) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native(period, multiplier)

    def append(self, value: float) -> "StandardErrorBands":
        """Append one price and return ``self``."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "StandardErrorBands":
        """Append a contiguous float64 price history and return ``self``."""
        self._state.extend(as_float64_series(values))
        return self

    @property
    def value(self) -> tuple[float, float, float] | None:
        """Return latest ``(upper, middle, lower)``, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        """Return aligned upper, middle, and lower native histories."""
        return self._state.compute()

    def reset(self) -> "StandardErrorBands":
        """Clear native state and history and return ``self``."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-value count delegated to native state."""
        return len(self._state)
