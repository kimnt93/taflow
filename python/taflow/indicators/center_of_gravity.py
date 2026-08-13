"""Public adapter for native Ehlers Center of Gravity."""

from typing import Any
import numpy as np
from .._native import CenterOfGravity as _Native
from .._series import as_float64_series


class CenterOfGravity:
    """Compute Ehlers' rolling center-of-gravity oscillator.

    Recent-to-old observations receive weights ``1..period`` and the negated
    weighted ratio is centered by ``(period + 1) / 2``. Leading warm-up is
    ``NaN``. This maps to Wickra ``CenterOfGravity``.

    Args:
        values: Chronological price history.
        period: Positive rolling window length, default 10.

    Raises:
        ValueError: If ``period`` is zero.
    """

    def __init__(self, period: int = 10) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native(period)

    def append(self, value: float) -> "CenterOfGravity":
        """Append one price and return this adapter."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "CenterOfGravity":
        """Append one converted float64 history and return this adapter."""
        self._state.extend(as_float64_series(values))
        return self

    @property
    def value(self) -> float | None:
        """Return the latest oscillator, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return aligned output with warm-up represented by ``NaN``."""
        return self._state.compute()

    def reset(self) -> "CenterOfGravity":
        """Reset native rolling state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)
