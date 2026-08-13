"""Native-backed drawdown duration."""

from typing import Any
import numpy as np
from .._native import RollingDrawdownDuration as _Native
from .._series import as_float64_series


class RollingDrawdownDuration:
    """Cumulative bars since the latest running equity peak."""

    def __init__(self) -> None:
        self._state = _Native()

    def append(self, value: float) -> "RollingDrawdownDuration":
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "RollingDrawdownDuration":
        self._state.extend(as_float64_series(values))
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self) -> float | None:
        return self._state.value

    def reset(self) -> "RollingDrawdownDuration":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)
