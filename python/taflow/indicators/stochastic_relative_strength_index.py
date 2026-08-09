"""Persistent stochastic RSI adapter."""

from typing import Any
import numpy as np
from .._native import StochasticRelativeStrengthIndex as _NativeStochasticRelativeStrengthIndex
from .._series import as_float64_series


class StochasticRelativeStrengthIndex:
    """Compute stochastic RSI fast %K and %D in native Rust state."""

    def __init__(self, _input: Any, time_period: int = 14, fast_k_period: int = 5,
                 fast_d_period: int = 3, fast_d_average_type: int = 0) -> None:
        self._state = _NativeStochasticRelativeStrengthIndex(time_period, fast_k_period, fast_d_period, fast_d_average_type)
        self.extend(_input)

    def append(self, _input: float) -> "StochasticRelativeStrengthIndex": self._state.append(float(_input)); return self
    def extend(self, _input: Any) -> "StochasticRelativeStrengthIndex": self._state.extend(as_float64_series(_input)); return self
    def compute(self) -> tuple[np.ndarray, np.ndarray]: return self._state.compute()
    @property
    def value(self) -> tuple[float, float] | None: return self._state.value
    def reset(self) -> "StochasticRelativeStrengthIndex": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
