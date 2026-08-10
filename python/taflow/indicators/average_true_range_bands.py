"""Average True Range bands adapter."""

from typing import Any

import numpy as np

from .._native import AverageTrueRangeBands as _Native
from .._series import as_float64_series


class AverageTrueRangeBands:
    """Wrap the current close with Wilder Average True Range bands.

    Rust computes ``close ± multiplier * ATR(period)`` and returns histories in
    ``(upper, middle, lower)`` order. Warm-up rows are ``NaN``. This class maps
    to Wickra ``AtrBands``.

    Args:
        high: Required chronological high-price history.
        low: Required chronological low-price history.
        close: Required chronological closing-price history.
        period: Wilder ATR period. Defaults to 14.
        multiplier: Positive ATR band multiplier. Defaults to 3.0.

    Raises:
        ValueError: If histories differ in length or configuration is invalid.
    """

    def __init__(
        self,
        high: Any,
        low: Any,
        close: Any,
        period: int = 14,
        multiplier: float = 3.0,
    ) -> None:
        """Initialize the native state and process aligned price history."""
        self._state = _Native(period, multiplier)
        self.extend(high, low, close)

    def append(self, high: float, low: float, close: float) -> "AverageTrueRangeBands":
        """Append one high/low/close bar and return ``self``."""
        self._state.append(float(high), float(low), float(close))
        return self

    def extend(self, high: Any, low: Any, close: Any) -> "AverageTrueRangeBands":
        """Append aligned high, low, and close histories and return ``self``."""
        arrays = tuple(as_float64_series(item) for item in (high, low, close))
        if len({len(item) for item in arrays}) != 1:
            raise ValueError("high, low, and close inputs must have equal lengths")
        self._state.extend(*arrays)
        return self

    @property
    def value(self) -> tuple[float, float, float] | None:
        """Return latest ``(upper, middle, lower)``, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        """Return aligned upper, middle, and lower native histories."""
        return self._state.compute()

    def reset(self) -> "AverageTrueRangeBands":
        """Clear native state and history and return ``self``."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)
