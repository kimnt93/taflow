from typing import Any
import numpy as np
from .._native import RollingCoefficientOfDetermination as _Native
from .._series import as_float64_series


class RollingCoefficientOfDetermination:
    """Rolling coefficient of determination for two aligned series."""

    def __init__(self, x: Any, y: Any, period: int = 20) -> None:
        self._state = _Native(period)
        self.extend(x, y)

    def append(self, x: float, y: float) -> "RollingCoefficientOfDetermination":
        self._state.append(float(x), float(y))
        return self

    def extend(self, x: Any, y: Any) -> "RollingCoefficientOfDetermination":
        a, b = as_float64_series(x), as_float64_series(y)
        if len(a) != len(b):
            raise ValueError("x and y must have equal lengths")
        self._state.extend(a, b)
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self) -> float | None:
        return self._state.value

    def reset(self) -> "RollingCoefficientOfDetermination":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)
