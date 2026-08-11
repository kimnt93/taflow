"""Native-backed rolling volume-weighted-average-price adapter."""

from typing import Any

import numpy as np

from .._adapter_protocol import adapter_length
from .._native import RollingVolumeWeightedAveragePriceOperator as _Native
from .._series import as_float64_series


class RollingVolumeWeightedAveragePrice:
    """Compute trailing VWAP from typical price and volume.

    ``high``, ``low``, ``close``, and ``volume`` are required equal-length
    chronological histories in that order and may all be empty for a fresh
    stream. ``timeperiod`` defaults to 20. Rust owns typical-price weighting,
    warm-up, and aligned output; ``compute`` returns one float array, ``value``
    is the latest scalar or ``None`` during warm-up, and lifecycle mutators
    return ``self``. The oracle is pandas rolling
    ``sum(typical_price * volume) / sum(volume)``.
    """

    def __init__(
        self,
        high: Any,
        low: Any,
        close: Any,
        volume: Any,
        timeperiod: int = 20,
    ) -> None:
        self._state = _Native(int(timeperiod))
        self.extend(high, low, close, volume)

    def append(
        self, high: float, low: float, close: float, volume: float
    ) -> "RollingVolumeWeightedAveragePrice":
        """Append one OHLCV bar and return this adapter."""
        self._state.append(float(high), float(low), float(close), float(volume))
        return self

    def extend(
        self, high: Any, low: Any, close: Any, volume: Any
    ) -> "RollingVolumeWeightedAveragePrice":
        """Append equal-length OHLCV histories."""
        arrays = tuple(
            as_float64_series(series) for series in (high, low, close, volume)
        )
        if len({len(array) for array in arrays}) != 1:
            raise ValueError("high, low, close, and volume must have equal lengths")
        self._state.extend(*arrays)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned rolling VWAP history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest VWAP, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "RollingVolumeWeightedAveragePrice":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return adapter_length(self)


__all__ = ["RollingVolumeWeightedAveragePrice"]
