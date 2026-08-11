"""Canonical native-backed liquidity adapter."""

from typing import Any
import numpy as np
from .._adapter_protocol import adapter_length
from .._native import LiquidityOperator as _Native
from .._series import as_float64_series


class Liquidity:
    """Causal liquidity-pool clustering over required high and low series.
    swing_length confirms swings and range_percent controls price clustering.
    Outputs are liquidity, level, swept with NaN before an event.
    """

    def __init__(
        self, high: Any, low: Any, swing_length: int = 50, range_percent: float = 0.01
    ) -> None:
        self._state = _Native(int(swing_length), float(range_percent))
        self.extend(high, low)

    def append(self, high: float, low: float) -> "Liquidity":
        self._state.append(float(high), float(low))
        return self

    def extend(self, high: Any, low: Any) -> "Liquidity":
        high_values = as_float64_series(high)
        low_values = as_float64_series(low)
        if high_values.shape != low_values.shape:
            raise ValueError("high and low must have equal lengths")
        self._state.extend(high_values, low_values)
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float, float] | None:
        return self._state.value

    def reset(self) -> "Liquidity":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return adapter_length(self)
