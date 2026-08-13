"""Shared Python boundary for condition/value Rust states."""

from typing import Any

import numpy as np

from ._adapter_protocol import adapter_length
from ._series import as_float64_series


class ConditionValueAdapter:
    _native_cls = None

    def __init__(self) -> None:
        self._state = self._native_cls()

    def append(self, condition: bool, _input: float) -> "ConditionValueAdapter":
        self._state.append(bool(condition), float(_input))
        return self

    def extend(self, condition: Any, _input: Any) -> "ConditionValueAdapter":
        condition_values = np.asarray(condition, dtype=bool)
        input_values = as_float64_series(_input)
        if len(condition_values) != len(input_values):
            raise ValueError("condition and input must have equal lengths")
        self._state.extend(condition_values, input_values)
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self) -> float | None:
        return self._state.value

    def reset(self) -> "ConditionValueAdapter":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return adapter_length(self)
