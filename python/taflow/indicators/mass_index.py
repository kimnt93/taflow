"""Canonical native-backed Mass Index adapter."""

from typing import Any
import numpy as np
from .._adapter_protocol import adapter_length
from .._native import MassIndexOperator as _Native
from .._series import as_float64_series


class MassIndex:
    """Mass Index from EMA-smoothed high-low ranges."""

    def __init__(
        self, ema_period: int = 9, sum_period: int = 25
    ) -> None:
        self._state = _Native(int(ema_period), int(sum_period))

    def append(self, high: float, low: float) -> "MassIndex":
        self._state.append(float(high), float(low))
        return self

    def extend(self, high: Any, low: Any) -> "MassIndex":
        high_values = as_float64_series(high)
        low_values = as_float64_series(low)
        if high_values.shape != low_values.shape:
            raise ValueError("high and low must have equal lengths")
        self._state.extend(high_values, low_values)
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self) -> float | None:
        return self._state.value

    def reset(self) -> "MassIndex":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return adapter_length(self)
