"""Canonical native-backed Rolling Standard Deviation adapter."""

from typing import Any
import numpy as np
from .._native import RollingStandardDeviation as _NativeRollingStandardDeviation
from .._series import as_float64_series


class RollingStandardDeviation:
    """Compute STDDEV from required ``values`` with configurable ``nbdev``."""

    def __init__(self, values: Any, timeperiod: int = 14, nbdev: float = 1.0) -> None:
        self._state = _NativeRollingStandardDeviation(timeperiod, nbdev)
        self.extend(values)

    def append(self, value: float) -> "RollingStandardDeviation":
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "RollingStandardDeviation":
        self._state.extend(as_float64_series(values))
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self) -> float | None:
        return self._state.value

    def reset(self) -> "RollingStandardDeviation":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)


__all__ = ["RollingStandardDeviation"]
