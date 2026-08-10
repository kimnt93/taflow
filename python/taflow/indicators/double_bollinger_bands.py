"""Double Bollinger Bands adapter."""

from typing import Any

import numpy as np

from .._native import DoubleBollingerBands as _Native
from .._series import as_float64_series


class DoubleBollingerBands:
    """Compute inner and outer population-deviation Bollinger envelopes.

    Both envelopes share a simple moving average and population standard
    deviation. Native output order is ``(upper_outer, upper_inner, middle,
    lower_inner, lower_outer)`` with ``NaN`` during warm-up. This maps to
    Wickra ``DoubleBollinger``.

    Args:
        values: Required chronological price history.
        period: Rolling window. Defaults to 20.
        inner_multiplier: Positive inner deviation multiplier. Defaults to 1.0.
        outer_multiplier: Outer multiplier, greater than the inner. Defaults to 2.0.

    Raises:
        ValueError: If the period or ordered multipliers are invalid.
    """

    def __init__(
        self,
        values: Any,
        period: int = 20,
        inner_multiplier: float = 1.0,
        outer_multiplier: float = 2.0,
    ) -> None:
        """Initialize the native state and process the supplied history."""
        self._state = _Native(period, inner_multiplier, outer_multiplier)
        self.extend(values)

    def append(self, value: float) -> "DoubleBollingerBands":
        """Append one price and return ``self``."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "DoubleBollingerBands":
        """Append a contiguous float64 price history and return ``self``."""
        self._state.extend(as_float64_series(values))
        return self

    @property
    def value(self) -> tuple[float, float, float, float, float] | None:
        """Return the five latest ordered bands, or ``None`` during warm-up."""
        return self._state.value

    def compute(
        self,
    ) -> tuple[np.ndarray, np.ndarray, np.ndarray, np.ndarray, np.ndarray]:
        """Return the five aligned native band histories in documented order."""
        return self._state.compute()

    def reset(self) -> "DoubleBollingerBands":
        """Clear native state and history and return ``self``."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-value count delegated to native state."""
        return len(self._state)
