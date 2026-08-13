"""Shared Python boundary for two-bar high/low relation states."""

from typing import Any

import numpy as np

from ._series import as_float64_series
from ._adapter_protocol import adapter_length


class BarRelationAdapter:
    """Delegate aligned high/low relation processing to a native Rust state."""

    _native_cls = None

    def __init__(self) -> None:
        self._state = self._native_cls()

    def append(self, high: float, low: float) -> "BarRelationAdapter":
        self._state.append(float(high), float(low))
        return self

    def extend(self, high: Any, low: Any) -> "BarRelationAdapter":
        high_values = as_float64_series(high)
        low_values = as_float64_series(low)
        if len(high_values) != len(low_values):
            raise ValueError("high and low must have equal lengths")
        self._state.extend(high_values, low_values)
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self) -> float | None:
        return self._state.value

    def reset(self) -> "BarRelationAdapter":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return adapter_length(self)
