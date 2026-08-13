"""Python adapter for native Up/Down Volume Ratio breadth."""

from typing import Any
import numpy as np
from .._native import UpDownVolumeRatio as _Native
from .._series import as_float64_series


class UpDownVolumeRatio:
    """Divide aggregate advancing volume by aggregate declining volume.

    A zero declining-volume denominator is floored to one, matching Wickra
    ``UpDownVolumeRatio``. Values are available from the first market tick.

    Args:
        advancing_volume: Market-wide volume in advancing issues.
        declining_volume: Market-wide volume in declining issues.

    Raises:
        ValueError: If the two histories have different lengths.
    """

    def __init__(self) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native()

    def append(self, advancing_volume: float, declining_volume: float) -> "UpDownVolumeRatio":
        """Append one aggregate-volume tick and return this adapter."""
        self._state.append(float(advancing_volume), float(declining_volume))
        return self

    def extend(self, advancing_volume: Any, declining_volume: Any) -> "UpDownVolumeRatio":
        """Append aligned aggregate-volume histories after length validation."""
        arrays = as_float64_series(advancing_volume), as_float64_series(declining_volume)
        if len(arrays[0]) != len(arrays[1]):
            raise ValueError("advancing and declining volume must have equal lengths")
        self._state.extend(*arrays)
        return self

    @property
    def value(self) -> float | None:
        """Return the latest ratio, or ``None`` before input."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return the aligned ratio history."""
        return self._state.compute()

    def reset(self) -> "UpDownVolumeRatio":
        """Reset native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-tick count delegated to native state."""
        return len(self._state)
