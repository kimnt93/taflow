"""Python adapter for the native Cumulative Volume Index."""

from typing import Any
import numpy as np
from .._native import CumulativeVolumeIndex as _Native
from .._series import as_float64_series


class CumulativeVolumeIndex:
    """Accumulate volume-normalized net advancing volume.

    Each increment is ``(advancing_volume - declining_volume) / total_volume``.
    A zero-volume tick contributes zero. Inputs are pre-aggregated market-wide
    volumes. This maps to Wickra ``CumulativeVolumeIndex``.

    Args:
        advancing_volume: Total volume in advancing issues at each tick.
        declining_volume: Total volume in declining issues at each tick.

    Raises:
        ValueError: If the input histories have different lengths.
    """

    def __init__(self, advancing_volume: Any, declining_volume: Any) -> None:
        """Initialize native state and process aligned volume histories."""
        self._state = _Native()
        self.extend(advancing_volume, declining_volume)

    def append(
        self, advancing_volume: float, declining_volume: float
    ) -> "CumulativeVolumeIndex":
        """Append one aggregate-volume tick and return this adapter."""
        self._state.append(float(advancing_volume), float(declining_volume))
        return self

    def extend(
        self, advancing_volume: Any, declining_volume: Any
    ) -> "CumulativeVolumeIndex":
        """Append aligned aggregate-volume histories after length validation."""
        arrays = (
            as_float64_series(advancing_volume),
            as_float64_series(declining_volume),
        )
        if len(arrays[0]) != len(arrays[1]):
            raise ValueError("advancing and declining volume must have equal lengths")
        self._state.extend(*arrays)
        return self

    @property
    def value(self) -> float | None:
        """Return the latest cumulative index, or ``None`` before input."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return the aligned cumulative index history."""
        return self._state.compute()

    def reset(self) -> "CumulativeVolumeIndex":
        """Reset the cumulative total and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-tick count delegated to native state."""
        return len(self._state)
