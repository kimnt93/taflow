"""Native-backed volume-weighted moving-average adapter."""

from typing import Any

import numpy as np

from ._native import VwmaOperator as _Native
from ._adapter_protocol import adapter_length
from ._series import as_float64_series


class VolumeWeightedMovingAverage:
    """Compute a trailing volume-weighted average of price.

    ``price`` and ``volume`` are required equal-length chronological series in
    that order and may both be empty for a fresh stream. ``timeperiod``
    defaults to 10. Rust owns the weighted rolling sums and NaN warm-up;
    ``compute`` returns one aligned float array, ``value`` is the latest scalar
    or ``None`` before warm-up, and lifecycle mutators return ``self``. The
    oracle is pandas rolling ``sum(price * volume) / sum(volume)``.
    """

    def __init__(
        self, price: Any, volume: Any, timeperiod: int = 10
    ) -> None:
        self._state = _Native(int(timeperiod))
        self.extend(price, volume)

    def append(self, price: float, volume: float) -> "VolumeWeightedMovingAverage":
        """Append one price/volume pair and return this adapter."""
        self._state.append(float(price), float(volume))
        return self

    def extend(
        self, price: Any, volume: Any
    ) -> "VolumeWeightedMovingAverage":
        """Append equal-length price and volume histories."""
        arrays = as_float64_series(price), as_float64_series(volume)
        if len(arrays[0]) != len(arrays[1]):
            raise ValueError("price and volume must have equal lengths")
        self._state.extend(*arrays)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned volume-weighted average history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest average, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "VolumeWeightedMovingAverage":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed pairs."""
        return adapter_length(self)


__all__ = ["VolumeWeightedMovingAverage"]
