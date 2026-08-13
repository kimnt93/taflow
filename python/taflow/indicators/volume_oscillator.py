"""Public adapter for the native volume oscillator state."""

from typing import Any

import numpy as np

from .._native import VolumeOscillator as _Native
from .._series import as_float64_series


class VolumeOscillator:
    """Measure the percentage spread between fast and slow volume averages.

    Rust computes ``100 * (fast SMA - slow SMA) / slow SMA`` and emits
    ``NaN`` until the slow average is ready. This definition maps to Wickra
    ``VolumeOscillator`` 0.9.9; TA-Lib has no direct equivalent.

    Args:
        volume: Chronological volume series supplied through ``extend``.
        fast: Fast simple-moving-average period. Defaults to 5.
        slow: Slow simple-moving-average period. Defaults to 10 and must be
            greater than ``fast``.

    Raises:
        ValueError: If the periods do not satisfy ``1 <= fast < slow``.
    """

    def __init__(self, fast: int = 5, slow: int = 10) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native(int(fast), int(slow))

    def append(self, volume: float) -> "VolumeOscillator":
        """Append one volume observation and return this instance."""
        self._state.append(float(volume))
        return self

    def extend(self, volume: Any) -> "VolumeOscillator":
        """Append a chronological volume series and return this instance."""
        self._state.extend(as_float64_series(volume))
        return self

    @property
    def value(self) -> float | None:
        """Return the latest oscillator value, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return the aligned oscillator history, including warm-up ``NaN``."""
        return self._state.compute()

    def reset(self) -> "VolumeOscillator":
        """Restore fresh-state behavior and return this instance."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of volume observations processed by Rust."""
        return len(self._state)


__all__ = ["VolumeOscillator"]
