"""Public adapter for the native volume Relative Strength Index state."""

from typing import Any

import numpy as np

from .._native import VolumeRelativeStrengthIndex as _Native
from .._series import as_float64_series


class VolumeRelativeStrengthIndex:
    """Apply Wilder's Relative Strength Index to volume changes.

    The first observation establishes previous volume. Rust then seeds Wilder
    average gains and losses from ``period`` changes, so output is ``NaN`` for
    the first ``period`` bars. A flat volume stream returns neutral 50. This
    maps to Wickra ``VolumeRsi`` 0.9.9; TA-Lib has no direct equivalent.

    Args:
        volume: Initial chronological volume series. An empty series creates a
            fresh state for later streaming.
        period: Wilder smoothing period. Defaults to 14.

    Raises:
        ValueError: If ``period`` is zero.
    """

    def __init__(self, volume: Any, period: int = 14) -> None:
        """Initialize the index and process the supplied volume history."""
        self._state = _Native(int(period))
        self.extend(volume)

    def append(self, volume: float) -> "VolumeRelativeStrengthIndex":
        """Append one volume observation and return this instance."""
        self._state.append(float(volume))
        return self

    def extend(self, volume: Any) -> "VolumeRelativeStrengthIndex":
        """Append a chronological volume series and return this instance."""
        self._state.extend(as_float64_series(volume))
        return self

    @property
    def value(self) -> float | None:
        """Return the latest volume RSI, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return aligned RSI values, including warm-up ``NaN``."""
        return self._state.compute()

    def reset(self) -> "VolumeRelativeStrengthIndex":
        """Clear Wilder smoothing state and return this instance."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of volume observations processed by Rust."""
        return len(self._state)


__all__ = ["VolumeRelativeStrengthIndex"]
