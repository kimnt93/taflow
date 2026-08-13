"""Native-backed Negative Volume Index adapter."""

from typing import Any

import numpy as np

from .._native import NegativeVolumeIndexOperator as _Native
from .._adapter_protocol import adapter_length
from .._series import as_float64_series


class NegativeVolumeIndex:
    """Track the Negative Volume Index from aligned close and volume bars.

    ``close`` and ``volume`` are required equal-length chronological histories;
    empty aligned arrays create a fresh stream. Rust starts at the conventional
    1000 and updates the index only when current volume is below the previous
    volume, using the percentage close change. ``compute`` returns one aligned
    float array, ``value`` is ``None`` for an empty stream, and lifecycle
    mutators return ``self``. The definition has no independent TA-Lib oracle;
    it is recorded as a native stateful variant.
    """

    def __init__(self) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native()

    def append(self, close: float, volume: float) -> "NegativeVolumeIndex":
        """Append one close/volume bar in that order."""
        self._state.append(float(close), float(volume))
        return self

    def extend(self, close: Any, volume: Any) -> "NegativeVolumeIndex":
        """Append aligned histories without mutating on length mismatch."""
        arrays = as_float64_series(close), as_float64_series(volume)
        if len(arrays[0]) != len(arrays[1]):
            raise ValueError("close and volume must have equal lengths")
        self._state.extend(*arrays)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned Negative Volume Index history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest index, or ``None`` when no bar was processed."""
        return self._state.value

    def reset(self) -> "NegativeVolumeIndex":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of close/volume bars processed by Rust."""
        return adapter_length(self)


__all__ = ["NegativeVolumeIndex"]
