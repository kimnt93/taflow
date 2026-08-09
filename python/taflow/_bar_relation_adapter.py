"""Shared Python boundary for two-bar high/low relation states."""

from typing import Any

import numpy as np

from ._series import as_float64_series
from ._adapter_protocol import adapter_length


class BarRelationAdapter:
    """Delegate aligned high/low relation processing to a native Rust state."""

    _native_cls = None

    def __init__(self, high: Any, low: Any) -> None:
        self._state = self._native_cls()
        self.extend(high, low)

    def append(self, high: float, low: float) -> "BarRelationAdapter":
        self._state.append(float(high), float(low))
        return self

    def extend(self, high: Any, low: Any) -> "BarRelationAdapter":
        self._state.extend(as_float64_series(high), as_float64_series(low))
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
