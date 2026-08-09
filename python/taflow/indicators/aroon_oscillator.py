"""Persistent Aroon Oscillator adapter."""

from typing import Any

import numpy as np

from .._native import AroonOscillator as _NativeAroonOscillator
from .._series import as_float64_series


class AroonOscillator:
    """Compute Aroon Up minus Aroon Down in persistent Rust state.

    The constructor requires aligned chronological high and low series. Pass
    two empty arrays for a fresh streaming state. ``timeperiod`` defaults to 14
    and must be at least 2. Output contains NaN for the first ``timeperiod``
    bars and maps to TA-Lib ``AROONOSC``.
    """

    def __init__(self, high: Any, low: Any, timeperiod: int = 14) -> None:
        self._state = _NativeAroonOscillator(timeperiod)
        self.extend(high, low)

    def append(self, high: float, low: float) -> "AroonOscillator":
        """Append one high/low pair and return this indicator."""
        self._state.append(float(high), float(low))
        return self

    def extend(self, high: Any, low: Any) -> "AroonOscillator":
        """Append aligned high and low histories and return this indicator."""
        self._state.extend(as_float64_series(high), as_float64_series(low))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned float64 oscillator history with NaN warm-up."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest oscillator value, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "AroonOscillator":
        """Restore fresh native state, clear history, and return this indicator."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed pairs."""
        return len(self._state)
