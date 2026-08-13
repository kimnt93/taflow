"""Canonical native-backed Exponential Moving Average adapter."""

from typing import Any
import numpy as np
from .._native import ExponentialMovingAverage as _NativeExponentialMovingAverage
from .._series import as_float64_series


class ExponentialMovingAverage:
    """Compute EMA from required ``values``; Rust owns seed and warm-up."""

    def __init__(self, timeperiod: int = 30) -> None:
        self._state = _NativeExponentialMovingAverage(timeperiod)

    def append(self, value: float) -> "ExponentialMovingAverage":
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "ExponentialMovingAverage":
        self._state.extend(as_float64_series(values))
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self) -> float | None:
        return self._state.value

    def reset(self) -> "ExponentialMovingAverage":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)


__all__ = ["ExponentialMovingAverage"]
