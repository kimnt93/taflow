"""Native-backed Force Index adapter."""
from typing import Any

import numpy as np

from .._native import ForceIndexOperator as _Native
from .._series import as_float64_series


class ForceIndex:
    """Compute EMA-smoothed signed price force weighted by volume.

    ``close`` and ``volume`` are required equal-length chronological series
    and may both be empty for a fresh stream. Rust computes
    ``EMA((close_t - close_(t-1)) * volume_t, period)``. The first ``period``
    bars are warm-up NaN because one bar seeds the prior close and ``period``
    force observations seed the EMA. Wickra ``ForceIndex`` is the oracle.
    ``compute`` returns one aligned float array, ``value`` is the latest scalar
    or ``None`` during warm-up, and lifecycle mutators return ``self``.
    """

    def __init__(self, period: int = 13) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native(period)

    def append(self, close: float, volume: float) -> "ForceIndex":
        """Append one close/volume pair and return this adapter."""
        self._state.append(float(close), float(volume))
        return self

    def extend(self, close: Any, volume: Any) -> "ForceIndex":
        """Append equal-length close and volume histories."""
        arrays = as_float64_series(close), as_float64_series(volume)
        if len(arrays[0]) != len(arrays[1]):
            raise ValueError("close and volume must have equal lengths")
        self._state.extend(*arrays)
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
        return self

    def __len__(self) -> int:
        """Return the number of processed pairs."""
        return len(self._state)
