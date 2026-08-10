"""Public adapter for the native Ehlers Decycler Oscillator."""

from typing import Any
import numpy as np
from .._native import DecyclerOscillator as _Native
from .._series import as_float64_series


class DecyclerOscillator:
    """Subtract a slow Ehlers decycler from a fast Ehlers decycler.

    Both filters emit from the first bar; the aligned oscillator therefore has
    no ``NaN`` warm-up. It maps to Wickra ``DecyclerOscillator``.

    Args:
        values: Initial chronological price or signal history.
        fast: Positive fast critical period, default 10.
        slow: Slow critical period, default 20 and strictly greater than fast.

    Raises:
        ValueError: If periods are zero or ``fast >= slow``.
    """

    def __init__(self, values: Any, fast: int = 10, slow: int = 20) -> None:
        """Initialize both native filters and process the initial history."""
        self._state = _Native(fast, slow)
        self.extend(values)

    def append(self, value: float) -> "DecyclerOscillator":
        """Append one signal value and return this adapter."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "DecyclerOscillator":
        """Append one converted float64 history and return this adapter."""
        self._state.extend(as_float64_series(values))
        return self

    @property
    def value(self) -> float | None:
        """Return the latest oscillator, or ``None`` when empty."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return the aligned native oscillator history."""
        return self._state.compute()

    def reset(self) -> "DecyclerOscillator":
        """Reset both filters and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)
