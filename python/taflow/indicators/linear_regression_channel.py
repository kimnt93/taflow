"""Linear regression channel adapter."""

from typing import Any

import numpy as np

from .._native import LinearRegressionChannel as _Native
from .._series import as_float64_series


class LinearRegressionChannel:
    """Compute a rolling OLS endpoint and residual-deviation channel.

    Rust fits price against positions ``0..period-1``. The middle output is the
    fitted endpoint and the bands are ``multiplier`` times the population
    residual deviation away. Output order is ``(upper, middle, lower)`` with
    ``NaN`` during warm-up. This maps to Wickra ``LinRegChannel``.

    Args:
        values: Required chronological price history.
        period: Regression window, at least two. Defaults to 20.
        multiplier: Positive residual-deviation multiplier. Defaults to 2.0.

    Raises:
        ValueError: If the configuration is invalid.
    """

    def __init__(self, period: int = 20, multiplier: float = 2.0) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native(period, multiplier)

    def append(self, value: float) -> "LinearRegressionChannel":
        """Append one price and return ``self``."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "LinearRegressionChannel":
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

    def reset(self) -> "LinearRegressionChannel":
        """Clear native state and history and return ``self``."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-value count delegated to native state."""
        return len(self._state)
