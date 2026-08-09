"""Canonical native-backed Jurik Moving Average adapter."""

from typing import Any

import numpy as np

from .._native import JurikMovingAverage as _NativeJurikMovingAverage
from .._series import as_float64_series


class JurikMovingAverage:
    """Compute the adaptive Jurik-like moving average from a close series.

    ``values`` is required and may be empty for a fresh stream. ``length``
    defaults to 7 and ``phase`` to 0.0. Rust owns the recurrence and warm-up;
    ``compute`` returns aligned NaN values and ``value`` is the latest scalar
    or ``None``. Lifecycle mutators are fluent. Oracle mapping is the
    pandas-ta-classic ``jma`` definition used by TAFlow.
    """

    def __init__(self, values: Any, length: int = 7, phase: float = 0.0) -> None:
        self._state = _NativeJurikMovingAverage(length, phase)
        self.extend(values)

    def append(self, value: float) -> "JurikMovingAverage":
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "JurikMovingAverage":
        self._state.extend(as_float64_series(values))
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self) -> float | None:
        return self._state.value

    def reset(self) -> "JurikMovingAverage":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)


__all__ = ["JurikMovingAverage"]
