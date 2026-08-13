"""Canonical native-backed IntradayMomentumIndex adapter."""
from typing import Any
import numpy as np
from .._native import IntradayMomentumIndex as _NativeIntradayMomentumIndex
from .._series import as_float64_series


class IntradayMomentumIndex:
    """Compute rolling intraday gains divided by total candle movement."""
    def __init__(self, timeperiod: int = 14) -> None:
        self._state = _NativeIntradayMomentumIndex(timeperiod)

    def append(self, open: float, close: float) -> "IntradayMomentumIndex":
        self._state.append(float(open), float(close)); return self

    def extend(self, open: Any, close: Any) -> "IntradayMomentumIndex":
        arrays = (as_float64_series(open), as_float64_series(close))
        if len(arrays[0]) != len(arrays[1]): raise ValueError("open and close must have equal lengths")
        self._state.extend(*arrays); return self

    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "IntradayMomentumIndex": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)


__all__ = ["IntradayMomentumIndex"]
