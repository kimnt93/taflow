"""Shared native boundary for pointwise math classes."""

from typing import Any

import numpy as np

from ._series import as_float64_series


class MathUnaryState:
    _native_cls = None

    def __init__(self) -> None:
        self._state = self._native_cls()

    def append(self, _input: float) -> "Self":
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "Self":
        self._state.extend(as_float64_series(_input))
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self) -> float | None:
        return self._state.value

    def reset(self) -> "Self":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)


class MathBinaryState:
    _native_cls = None

    def __init__(self) -> None:
        self._state = self._native_cls()

    def append(self, left: float, right: float) -> "Self":
        self._state.append(float(left), float(right))
        return self

    def extend(self, left: Any, right: Any) -> "Self":
        left_values = as_float64_series(left)
        right_values = as_float64_series(right)
        if len(left_values) != len(right_values):
            raise ValueError("inputs must have equal lengths")
        self._state.extend(left_values, right_values)
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self) -> float | None:
        return self._state.value

    def reset(self) -> "Self":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)
