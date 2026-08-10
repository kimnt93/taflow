"""Public adapter for the native Ehlers SuperSmoother."""

from typing import Any
import numpy as np
from .._native import SuperSmoother as _Native
from .._series import as_float64_series


class SuperSmoother:
    """Apply Ehlers' two-pole low-pass filter to a chronological series.

    The native recursion uses Butterworth-style coefficients derived from
    ``period`` and emits from the first bar using the input as its initial
    condition. The oracle/name mapping is Wickra ``SuperSmoother``.

    Args:
        values: Initial chronological price or signal history.
        period: Positive critical period, default 10.

    Raises:
        ValueError: If ``period`` is zero.
    """

    def __init__(self, values: Any, period: int = 10) -> None:
        """Initialize native filter state and process the initial history."""
        self._state = _Native(period)
        self.extend(values)

    def append(self, value: float) -> "SuperSmoother":
        """Append one signal value and return this adapter."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "SuperSmoother":
        """Append one converted float64 history and return this adapter."""
        self._state.extend(as_float64_series(values))
        return self

    @property
    def value(self) -> float | None:
        """Return the latest filtered value, or ``None`` when empty."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return the aligned native output history."""
        return self._state.compute()

    def reset(self) -> "SuperSmoother":
        """Reset native recursion and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)
