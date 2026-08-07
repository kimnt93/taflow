"""Persistent Money Flow Index indicator."""

from __future__ import annotations

from typing import Any

import numpy as np

from ._native import MoneyFlowIndex as _NativeMoneyFlowIndex
from ._series import as_float64_series


class MoneyFlowIndex:
    """Compute MFI history once, then append HLCV bars in O(1)."""

    def __init__(self, high: Any | None = None, low: Any | None = None, close: Any | None = None,
                 volume: Any | None = None, timeperiod: int = 14) -> None:
        self._state = _NativeMoneyFlowIndex(timeperiod)
        if high is not None or low is not None or close is not None or volume is not None:
            self.extend(high, low, close, volume)

    def append(self, high: float, low: float, close: float, volume: float) -> "MoneyFlowIndex":
        self._state.append(float(high), float(low), float(close), float(volume))
        return self

    def extend(self, high: Any, low: Any | None = None, close: Any | None = None,
               volume: Any | None = None) -> "MoneyFlowIndex":
        if low is None or close is None or volume is None:
            raise ValueError("high, low, close, and volume must be provided together")
        self._state.extend(as_float64_series(high), as_float64_series(low),
                           as_float64_series(close), as_float64_series(volume))
        return self

    def compute(self) -> np.ndarray: return self._state.compute()

    @property
    def value(self) -> float | None: return self._state.value

    @property
    def timeperiod(self) -> int: return self._state.timeperiod

    def reset(self) -> "MoneyFlowIndex":
        self._state.reset()
        return self

    def __len__(self) -> int: return len(self._state)



__all__ = ["MoneyFlowIndex"]
