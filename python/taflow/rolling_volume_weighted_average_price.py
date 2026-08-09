"""Persistent rolling volume-weighted average price."""

from typing import Any

import numpy as np

from ._native import VwapOperator as _Native
from ._series import as_float64_series


class RollingVolumeWeightedAveragePrice:
    """Compute typical-price VWAP over a causal trailing window."""

    def __init__(self, high: Any, low: Any, close: Any, volume: Any, timeperiod: int = 20) -> None:
        self._state = _Native(timeperiod)
        self._length = 0
        self.extend(high, low, close, volume)

    def append(self, high: float, low: float, close: float, volume: float) -> "RollingVolumeWeightedAveragePrice":
        self._state.append(float(high), float(low), float(close), float(volume))
        self._length += 1
        return self

    def extend(self, high: Any, low: Any, close: Any, volume: Any) -> "RollingVolumeWeightedAveragePrice":
        values = tuple(as_float64_series(series) for series in (high, low, close, volume))
        if len({len(series) for series in values}) != 1:
            raise ValueError("high, low, close, and volume must have equal lengths")
        self._state.extend(*values)
        self._length += len(values[0])
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self) -> float | None:
        return self._state.value

    def reset(self) -> "RollingVolumeWeightedAveragePrice":
        self._state.reset()
        self._length = 0
        return self

    def __len__(self) -> int:
        return self._length
