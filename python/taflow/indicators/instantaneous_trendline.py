"""Public adapter for native Ehlers Instantaneous Trendline."""

from typing import Any
import numpy as np
from .._native import InstantaneousTrendline as _Native
from .._series import as_float64_series


class InstantaneousTrendline:
    """Compute Ehlers' initialized two-pole instantaneous trendline.

    The first six bars use the standard weighted initial condition; later bars
    use the full recursive trendline with ``alpha = 2 / (period + 1)``. Output
    begins immediately. This maps to Wickra ``InstantaneousTrendline``.

    Args:
        values: Initial chronological price or signal history.
        period: Positive dominant-cycle period, default 10.

    Raises:
        ValueError: If ``period`` is zero.
    """

    def __init__(self, values: Any, period: int = 10) -> None:
        """Initialize native recursion and process the initial history."""
        self._state = _Native(period)
        self.extend(values)

    def append(self, value: float) -> "InstantaneousTrendline":
        """Append one signal value and return this adapter."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "InstantaneousTrendline":
        """Append one converted float64 history and return this adapter."""
        self._state.extend(as_float64_series(values))
        return self

    @property
    def value(self) -> float | None:
        """Return the latest trendline, or ``None`` when empty."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return the aligned native trendline history."""
        return self._state.compute()

    def reset(self) -> "InstantaneousTrendline":
        """Reset native recursion and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)
