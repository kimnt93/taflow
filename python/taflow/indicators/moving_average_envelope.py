"""Moving-average envelope adapter."""

from typing import Any

import numpy as np

from .._native import MovingAverageEnvelope as _Native
from .._series import as_float64_series


class MovingAverageEnvelope:
    """Wrap a simple moving average with fixed-percentage bands.

    Rust computes ``middle = SMA(period)`` and ``middle * (1 ± percent)``.
    Histories are returned as ``(upper, middle, lower)`` arrays with ``NaN``
    during the ``period - 1`` warm-up bars. The external mapping is Wickra
    ``MaEnvelope``.

    Args:
        values: Required chronological price history.
        period: Simple moving-average window. Defaults to 20.
        percent: Fractional envelope distance; ``0.025`` means 2.5 percent.

    Raises:
        ValueError: If ``period`` or ``percent`` is not positive.
    """

    def __init__(self, period: int = 20, percent: float = 0.025) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native(period, percent)

    def append(self, value: float) -> "MovingAverageEnvelope":
        """Append one price and return ``self``."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "MovingAverageEnvelope":
        """Append a contiguous float64 price series and return ``self``."""
        self._state.extend(as_float64_series(values))
        return self

    @property
    def value(self) -> tuple[float, float, float] | None:
        """Return latest ``(upper, middle, lower)``, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        """Return aligned upper, middle, and lower native histories."""
        return self._state.compute()

    def reset(self) -> "MovingAverageEnvelope":
        """Clear native state and history and return ``self``."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-value count delegated to native state."""
        return len(self._state)
