"""Persistent Aroon adapter."""

from typing import Any

import numpy as np

from ._native import Aroon as _NativeAroon
from ._series import as_float64_series


class Aroon:
    """Compute named Aroon Down and Up histories in persistent Rust state.

    The constructor requires aligned chronological high and low series. Pass
    two empty arrays for a fresh streaming state. ``timeperiod`` defaults to 14
    and must be at least 2. The state uses a ``timeperiod + 1`` bar extrema
    window with latest-equal extrema winning. Both outputs contain NaN for the
    first ``timeperiod`` bars and are returned in ``(down, up)`` order, matching
    TA-Lib ``AROON``.
    """

    def __init__(self, high: Any, low: Any, timeperiod: int = 14) -> None:
        self._state = _NativeAroon(int(timeperiod))
        self.extend(high, low)

    def append(self, high: float, low: float) -> "Aroon":
        """Append one high/low pair and return this indicator."""
        self._state.append(float(high), float(low))
        return self

    def extend(self, high: Any, low: Any) -> "Aroon":
        """Append aligned high and low histories and return this indicator."""
        arrays = as_float64_series(high), as_float64_series(low)
        if len(arrays[0]) != len(arrays[1]):
            raise ValueError("high and low must have equal lengths")
        self._state.extend(*arrays)
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray]:
        """Return aligned ``(down, up)`` float64 histories with NaN warm-up."""
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float] | None:
        """Return latest ``(down, up)``, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "Aroon":
        """Restore fresh native state, clear histories, and return this indicator."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed pairs."""
        return len(self._state)
