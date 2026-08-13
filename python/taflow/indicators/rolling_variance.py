"""Canonical native-backed Rolling Variance adapter."""

from typing import Any
import numpy as np
from .._native import RollingVariance as _NativeRollingVariance
from .._series import as_float64_series


class RollingVariance:
    """Compute population variance from required ``values``; ``nbdev`` is accepted for API parity."""

    def __init__(self, timeperiod: int = 14, nbdev: float = 1.0) -> None:
        self._state = _NativeRollingVariance(timeperiod, nbdev)

    def append(self, value: float) -> "RollingVariance":
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "RollingVariance":
        self._state.extend(as_float64_series(values))
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self) -> float | None:
        return self._state.value

    def reset(self) -> "RollingVariance":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)


__all__ = ["RollingVariance"]
