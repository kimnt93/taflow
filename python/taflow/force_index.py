"""Native-backed Force Index adapter."""

from typing import Any

import numpy as np

from ._native import ForceIndexOperator as _Native
from ._series import as_float64_series


class ForceIndex:
    """Compute signed price change multiplied by volume.

    ``close`` and ``volume`` are required equal-length chronological series
    and may both be empty for a fresh stream. Rust emits the first value as
    warm-up NaN, then computes ``(close_t - close_(t-1)) * volume_t``.
    ``compute`` returns one aligned float array, ``value`` is the latest scalar
    or ``None`` during warm-up, and lifecycle mutators return ``self``.
    """

    def __init__(self, close: Any, volume: Any) -> None:
        self._state = _Native()
        self._length = 0
        self.extend(close, volume)

    def append(self, close: float, volume: float) -> "ForceIndex":
        """Append one close/volume pair and return this adapter."""
        self._state.append(float(close), float(volume))
        self._length += 1
        return self

    def extend(self, close: Any, volume: Any) -> "ForceIndex":
        """Append equal-length close and volume histories."""
        arrays = as_float64_series(close), as_float64_series(volume)
        if len(arrays[0]) != len(arrays[1]):
            raise ValueError("close and volume must have equal lengths")
        self._state.extend(*arrays)
        self._length += len(arrays[0])
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned Force Index history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest force value, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "ForceIndex":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        self._length = 0
        return self

    def __len__(self) -> int:
        """Return the number of processed pairs."""
        return self._length


__all__ = ["ForceIndex"]
