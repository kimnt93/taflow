"""Canonical native-backed Triangular Moving Average adapter."""

from typing import Any
import numpy as np
from .._native import TriangularMovingAverage as _NativeTriangularMovingAverage
from .._series import as_float64_series


class TriangularMovingAverage:
    """Compute TRIMA from required ``values`` through the Rust state."""

    def __init__(self, values: Any, timeperiod: int = 30) -> None:
        self._state = _NativeTriangularMovingAverage(timeperiod)
        self.extend(values)

    def append(self, value: float) -> "TriangularMovingAverage":
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "TriangularMovingAverage":
        self._state.extend(as_float64_series(values))
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self) -> float | None:
        return self._state.value

    def reset(self) -> "TriangularMovingAverage":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)


__all__ = ["TriangularMovingAverage"]
