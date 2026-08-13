"""Shared native boundary for two-input rolling states."""

from typing import Any

import numpy as np

from ._series import as_float64_series


class BivariateState:
    _native_cls = None

    def __init__(self, timeperiod: int = 5) -> None:
        self._state = self._native_cls(timeperiod)

    def append(self, input0: float, input1: float) -> "Self":
        self._state.append(float(input0), float(input1))
        return self

    def extend(self, input0: Any, input1: Any) -> "Self":
        first = as_float64_series(input0)
        second = as_float64_series(input1)
        if len(first) != len(second):
            raise ValueError("inputs must have equal lengths")
        self._state.extend(first, second)
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self) -> object:
        return self._state.value

    def reset(self) -> "Self":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)
