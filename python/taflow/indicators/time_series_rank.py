"""Canonical native-backed time-series-rank adapter."""

from typing import Any
import numpy as np
from .._adapter_protocol import adapter_length
from .._native import RollingRank as _Native
from .._series import as_float64_series


class TimeSeriesRank:
    """WorldQuant rank of each value within a trailing window."""

    def __init__(self, timeperiod: int = 14) -> None:
        self._state = _Native(int(timeperiod))

    def append(self, input: float) -> "TimeSeriesRank":
        self._state.append(float(input))
        return self

    def extend(self, input: Any) -> "TimeSeriesRank":
        values = as_float64_series(input)
        self._state.extend(values)
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self) -> float | None:
        return self._state.value

    def reset(self) -> "TimeSeriesRank":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return adapter_length(self)
