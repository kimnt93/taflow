"""Persistent Exponential Moving Average indicator."""

from __future__ import annotations

from typing import Any

import numpy as np

from ._native import ExponentialMovingAverage as _NativeExponentialMovingAverage
from ._series import as_float64_series


class ExponentialMovingAverage:
    """Compute EMA history once, then continue it with new observations."""

    def __init__(
        self,
        input: Any | None = None,
        timeperiod: int = 30,
        *,
        column: str | None = None,
    ) -> None:
        self._state = _NativeExponentialMovingAverage(timeperiod)
        if input is not None:
            self.extend(input, column=column)

    def append(self, value: float) -> "ExponentialMovingAverage":
        self._state.append(float(value))
        return self

    def extend(
        self, values: Any, *, column: str | None = None
    ) -> "ExponentialMovingAverage":
        self._state.extend(as_float64_series(values, column=column))
        return self

    def compute(self) -> np.ndarray:
        """Return every aligned result accumulated by this object."""

        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest warm value without materializing history."""

        return self._state.value

    @property
    def timeperiod(self) -> int:
        return self._state.timeperiod

    def reset(self) -> "ExponentialMovingAverage":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)


EMA = ExponentialMovingAverage

__all__ = ["ExponentialMovingAverage", "EMA"]
