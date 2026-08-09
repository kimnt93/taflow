"""Persistent On-Balance Volume adapter."""

from typing import Any

import numpy as np

from .._native import OnBalanceVolume as _NativeOnBalanceVolume
from .._series import as_float64_series


class OnBalanceVolume:
    """Accumulate volume according to consecutive closing-price direction.

    The constructor requires aligned chronological ``close`` and ``volume``
    series. Pass two empty arrays for a fresh streaming state. The first output
    equals the first volume; later volume is added, subtracted, or retained when
    close rises, falls, or is unchanged. There is no warm-up. This definition
    maps to TA-Lib ``OBV``.

    Parameters
    ----------
    close : Any
        Chronological closing-price series.
    volume : Any
        Chronological volume series aligned with ``close``.
    """

    def __init__(self, close: Any, volume: Any) -> None:
        self._state = _NativeOnBalanceVolume()
        self.extend(close, volume)

    def append(self, close: float, volume: float) -> "OnBalanceVolume":
        """Append one close/volume pair and return this indicator."""
        self._state.append(float(close), float(volume))
        return self

    def extend(self, close: Any, volume: Any) -> "OnBalanceVolume":
        """Append aligned close and volume histories and return this indicator."""
        self._state.extend(as_float64_series(close), as_float64_series(volume))
        return self

    def compute(self) -> np.ndarray:
        """Return the complete aligned ``float64`` On-Balance Volume history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest value, or ``None`` before the first pair."""
        return self._state.value

    def reset(self) -> "OnBalanceVolume":
        """Restore fresh native state, clear history, and return this indicator."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed pairs."""
        return len(self._state)
