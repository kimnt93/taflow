"""Persistent fast stochastic oscillator adapter."""

from typing import Any

import numpy as np

from ._native import FastStochasticOscillator as _NativeFastStochasticOscillator
from ._series import as_float64_series


class FastStochasticOscillator:
    """Compute fast %K and %D from aligned high, low, and close series."""

    def __init__(self, high: Any, low: Any, close: Any, fast_k_period: int = 5,
                 fast_d_period: int = 3, fast_d_average_type: int = 0) -> None:
        self._state = _NativeFastStochasticOscillator(fast_k_period, fast_d_period, fast_d_average_type)
        self.extend(high, low, close)

    def append(self, high: float, low: float, close: float) -> "FastStochasticOscillator":
        self._state.append(float(high), float(low), float(close)); return self

    def extend(self, high: Any, low: Any, close: Any) -> "FastStochasticOscillator":
        self._state.extend(as_float64_series(high), as_float64_series(low), as_float64_series(close)); return self

    def compute(self) -> tuple[np.ndarray, np.ndarray]: return self._state.compute()
    @property
    def value(self) -> tuple[float, float] | None: return self._state.value
    def reset(self) -> "FastStochasticOscillator": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
