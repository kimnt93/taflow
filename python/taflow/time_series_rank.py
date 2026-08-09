"""Canonical native-backed time-series-rank adapter."""
from typing import Any
import numpy as np
from ._native import RollingRankOperator as _Native
from ._series import as_float64_series


class TimeSeriesRank:
    """WorldQuant rank of each value within a trailing window."""
    def __init__(self, input: Any, timeperiod: int = 14) -> None:
        self._state = _Native(int(timeperiod)); self._length = 0; self.extend(input)
    def append(self, input: float) -> "TimeSeriesRank":
        self._state.append(float(input)); self._length += 1; return self
    def extend(self, input: Any) -> "TimeSeriesRank":
        values = as_float64_series(input); self._state.extend(values); self._length += len(values); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "TimeSeriesRank": self._state.reset(); self._length = 0; return self
    def __len__(self) -> int: return self._length
