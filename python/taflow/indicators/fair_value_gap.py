"""Canonical causal fair-value-gap adapter."""

from typing import Any

import numpy as np

from .._native import FairValueGapOperator as _Native
from .._series import as_float64_series


class FairValueGap:
    """Detect causal fair-value gaps and subsequent mitigation events."""

    def __init__(self) -> None:
        self._state = _Native()

    def append(self, _open: float, high: float, low: float, close: float) -> "FairValueGap":
        self._state.append(float(_open), float(high), float(low), float(close))
        return self

    def extend(self, _open: Any, high: Any, low: Any, close: Any) -> "FairValueGap":
        values = tuple(as_float64_series(series) for series in (_open, high, low, close))
        if len({len(series) for series in values}) != 1:
            raise ValueError("_open, high, low, and close must have equal lengths")
        self._state.extend(*values)
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray, np.ndarray]:
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float, float, float] | None:
        return self._state.value

    def reset(self) -> "FairValueGap":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)
