"""Public adapter for native Time Segmented Volume."""

from typing import Any

import numpy as np

from .._native import TimeSegmentedVolume as _Native
from .._series import as_float64_series


class TimeSegmentedVolume:
    """Compute Worden's rolling close-change-weighted volume sum.

    Each flow is ``(close[t] - close[t-1]) * volume[t]`` and the output is
    the sum of the latest ``period`` flows. One seed bar plus ``period`` flows
    are required. This contract maps to Wickra ``TSV``.

    Args:
        close: Initial chronological closes.
        volume: Initial chronological volumes.
        period: Rolling number of flows, default 18.

    Raises:
        ValueError: If inputs are misaligned or ``period`` is zero.
    """

    def __init__(self, close: Any, volume: Any, period: int = 18) -> None:
        """Initialize the native state and process the supplied history."""
        self._state = _Native(period)
        self.extend(close, volume)

    def append(self, close: float, volume: float) -> "TimeSegmentedVolume":
        """Append one close/volume sample and return this instance."""
        self._state.append(float(close), float(volume))
        return self

    def extend(self, close: Any, volume: Any) -> "TimeSegmentedVolume":
        """Append aligned close and volume histories and return this instance."""
        close_series = as_float64_series(close)
        volume_series = as_float64_series(volume)
        if len(close_series) != len(volume_series):
            raise ValueError("close and volume must have equal lengths")
        self._state.extend(close_series, volume_series)
        return self

    @property
    def value(self) -> float | None:
        """Return the latest rolling TSV, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return the aligned TSV history with warm-up represented by ``NaN``."""
        return self._state.compute()

    def reset(self) -> "TimeSegmentedVolume":
        """Clear close and rolling-flow state, then return this instance."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of samples stored by the native state."""
        return len(self._state)
