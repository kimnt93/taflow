from typing import Any
import numpy as np
from .._native import ProjectionBands as _Native
from .._series import as_float64_series


class ProjectionBands:
    """Rolling projected central value."""

    def __init__(self, period: int = 20) -> None:
        self._state = _Native(period)

    def append(self, value: float) -> "ProjectionBands":
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "ProjectionBands":
        self._state.extend(as_float64_series(values))
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self) -> float | None:
        return self._state.value

    def reset(self) -> "ProjectionBands":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)
