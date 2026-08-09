"""Shared Python boundary for condition/value Rust states."""

from typing import Any

import numpy as np

from ._adapter_protocol import adapter_length
from ._series import as_float64_series


class ConditionValueAdapter:
    _native_cls = None

    def __init__(self, condition: Any, _input: Any) -> None:
        self._state = self._native_cls()
        self.extend(condition, _input)

    def append(self, condition: bool, _input: float) -> "ConditionValueAdapter":
        self._state.append(bool(condition), float(_input))
        return self

    def extend(self, condition: Any, _input: Any) -> "ConditionValueAdapter":
        self._state.extend(
            np.asarray(condition, dtype=bool), as_float64_series(_input)
        )
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
