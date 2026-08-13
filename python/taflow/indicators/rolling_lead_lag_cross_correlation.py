from typing import Any

import numpy as np

from .._native import RollingLeadLagCrossCorrelation as _Native
from .._series import as_float64_series


class RollingLeadLagCrossCorrelation:
    """Find the strongest signed correlation across bounded lead/lag offsets.

    Output is ``(lag, correlation)``; positive lag means ``left`` leads
    ``right``. Zero lag is checked first so ties prefer the smallest absolute
    offset, matching Wickra ``LeadLagCrossCorrelation``.

    Args:
        left: Required first chronological series.
        right: Required second series aligned with ``left``.
        window: Number of overlapping pairs used at each lag. Defaults to 20.
        max_lag: Largest positive and negative lag searched. Defaults to 10.

    Raises:
        ValueError: If histories differ in length or configuration is invalid.
    """

    def __init__(self, window: int = 20, max_lag: int = 10) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native(window, max_lag)

    def append(
        self, left: float, right: float
    ) -> "RollingLeadLagCrossCorrelation":
        """Append one aligned pair and return this adapter."""
        self._state.append(float(left), float(right))
        return self

    def extend(self, left: Any, right: Any) -> "RollingLeadLagCrossCorrelation":
        """Append aligned histories after validating their lengths."""
        left_array, right_array = as_float64_series(left), as_float64_series(right)
        if len(left_array) != len(right_array):
            raise ValueError("left and right inputs must have equal lengths")
        self._state.extend(left_array, right_array)
        return self

    @property
    def value(self) -> tuple[float, float] | None:
        """Return the latest ``(lag, correlation)``, or ``None`` in warm-up."""
        return self._state.value

    def compute(self) -> tuple[np.ndarray, np.ndarray]:
        """Return aligned lag and correlation histories."""
        return self._state.compute()

    def reset(self) -> "RollingLeadLagCrossCorrelation":
        """Clear all rolling pairs and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-pair count delegated to native state."""
        return len(self._state)
