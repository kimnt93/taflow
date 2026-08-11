"""Canonical native-backed Simple Moving Average adapter."""

from typing import Any
import numpy as np
from .._native import SimpleMovingAverage as _NativeSimpleMovingAverage
from .._series import as_float64_series


class SimpleMovingAverage:
    """Compute SMA from required ``values`` using the Rust rolling state."""

    def __init__(self, values: Any, timeperiod: int = 30) -> None:
        self._state = _NativeSimpleMovingAverage(timeperiod)
        self.extend(values)

    def append(self, value: float) -> "SimpleMovingAverage":
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "SimpleMovingAverage":
        self._state.extend(as_float64_series(values))
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self) -> float | None:
        return self._state.value

    def reset(self) -> "SimpleMovingAverage":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)


__all__ = ["SimpleMovingAverage"]
