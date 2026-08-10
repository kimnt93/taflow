"""Public adapter for the native Volume Zone Oscillator state."""

from typing import Any

import numpy as np

from .._native import VolumeZoneOscillator as _Native
from .._series import as_float64_series


class VolumeZoneOscillator:
    """Measure EMA-smoothed signed volume as a percentage of total volume.

    Volume is signed from each close change, then the signed and unsigned
    streams are independently EMA-smoothed. The first close establishes
    direction, so output remains ``NaN`` for ``timeperiod`` additional bars.
    This definition maps to Wickra ``VZO`` 0.9.9; TA-Lib has no equivalent.

    Args:
        close: Initial chronological closing-price series.
        volume: Initial chronological volume series aligned with ``close``.
        timeperiod: EMA smoothing period. Defaults to 14.

    Raises:
        ValueError: If the input lengths differ or ``timeperiod`` is zero.
    """

    def __init__(self, close: Any, volume: Any, timeperiod: int = 14) -> None:
        """Initialize the oscillator and process aligned close/volume history."""
        self._state = _Native(int(timeperiod))
        self.extend(close, volume)

    def append(self, close: float, volume: float) -> "VolumeZoneOscillator":
        """Append one close/volume bar and return this instance."""
        self._state.append(float(close), float(volume))
        return self

    def extend(self, close: Any, volume: Any) -> "VolumeZoneOscillator":
        """Append aligned close and volume series.

        Returns:
            This instance, allowing method chaining.

        Raises:
            ValueError: If ``close`` and ``volume`` have different lengths.
        """
        close_values = as_float64_series(close)
        volume_values = as_float64_series(volume)
        if len(close_values) != len(volume_values):
            raise ValueError("close and volume must have equal lengths")
        self._state.extend(close_values, volume_values)
        return self

    @property
    def value(self) -> float | None:
        """Return the latest oscillator value, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return aligned oscillator values, including warm-up ``NaN``."""
        return self._state.compute()

    def reset(self) -> "VolumeZoneOscillator":
        """Clear close direction and EMA state, then return this instance."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of aligned bars processed by Rust."""
        return len(self._state)


__all__ = ["VolumeZoneOscillator"]
