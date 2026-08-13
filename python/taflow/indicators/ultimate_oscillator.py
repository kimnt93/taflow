"""Native-backed Ultimate Oscillator adapter."""

from typing import Any

import numpy as np

from .._native import UltimateOscillator as _NativeUltimateOscillator
from .._series import as_float64_series


class UltimateOscillator:
    """Compute TA-Lib ``ULTOSC`` from aligned high, low, and close series.

    The three trailing periods default to 7, 14, and 28.  The Rust state owns
    all warm-up and recurrence arithmetic; history contains ``NaN`` until the
    longest period is available.  ``append``, ``extend``, and ``reset`` mutate
    and return this instance, while ``value`` exposes the latest scalar.
    """

    def __init__(
        self,
        timeperiod1: int = 7,
        timeperiod2: int = 14,
        timeperiod3: int = 28,
    ) -> None:
        """Create a state and consume the supplied aligned OHLC histories."""
        self._state = _NativeUltimateOscillator(
            timeperiod1, timeperiod2, timeperiod3
        )

    def append(self, high: float, low: float, close: float) -> "UltimateOscillator":
        """Append one high, low, close bar and return ``self``."""
        self._state.append(float(high), float(low), float(close))
        return self

    def extend(
        self, high: Any, low: Any, close: Any
    ) -> "UltimateOscillator":
        """Consume aligned high, low, close series and return ``self``."""
        high_series = as_float64_series(high)
        low_series = as_float64_series(low)
        close_series = as_float64_series(close)
        if not (len(high_series) == len(low_series) == len(close_series)):
            raise ValueError("high, low, and close must have equal lengths")
        self._state.extend(high_series, low_series, close_series)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned ULTOSC history as a NumPy array."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest oscillator value, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "UltimateOscillator":
        """Reset native state and history, returning ``self``."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of observations retained in output history."""
        return len(self._state)
