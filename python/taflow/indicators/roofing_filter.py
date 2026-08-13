"""Public adapter for the native Ehlers Roofing Filter."""

from typing import Any
import numpy as np
from .._native import RoofingFilter as _Native
from .._series import as_float64_series


class RoofingFilter:
    """Band-limit a signal with an Ehlers high-pass and SuperSmoother.

    ``high_period`` removes slower trend while ``low_period`` removes faster
    noise. Native output begins on the first bar. The oracle mapping is Wickra
    ``RoofingFilter`` with ``lp_period`` and ``hp_period``.

    Args:
        values: Chronological price or signal history.
        low_period: Positive low-pass period, default 10.
        high_period: High-pass period, default 48 and greater than low_period.

    Raises:
        ValueError: If periods are zero or incorrectly ordered.
    """

    def __init__(self, low_period: int = 10, high_period: int = 48) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native(low_period, high_period)

    def append(self, value: float) -> "RoofingFilter":
        """Append one signal value and return this adapter."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "RoofingFilter":
        """Append one converted float64 history and return this adapter."""
        self._state.extend(as_float64_series(values))
        return self

    @property
    def value(self) -> float | None:
        """Return the latest band-limited value, or ``None`` when empty."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return the aligned native filter history."""
        return self._state.compute()

    def reset(self) -> "RoofingFilter":
        """Reset high-pass and smoothing state, then return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)
