"""Canonical native-backed Rolling Average Deviation adapter."""

from typing import Any
import numpy as np
from .._native import RollingAverageDeviation as _NativeRollingAverageDeviation
from .._series import as_float64_series


class RollingAverageDeviation:
    """Compute AVGDEV from required ``values`` with period 14 by default."""

    def __init__(self, values: Any, timeperiod: int = 14) -> None:
        self._state = _NativeRollingAverageDeviation(timeperiod)
        self.extend(values)

    def append(self, value: float) -> "RollingAverageDeviation":
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "RollingAverageDeviation":
        self._state.extend(as_float64_series(values))
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self) -> float | None:
        return self._state.value

    def reset(self) -> "RollingAverageDeviation":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)


__all__ = ["RollingAverageDeviation"]
