"""Persistent percentage price oscillator adapter."""

from typing import Any
import numpy as np
from ._native import PercentagePriceOscillator as _NativePercentagePriceOscillator
from ._series import as_float64_series


class PercentagePriceOscillator:
    """Compute the percentage difference between fast and slow averages."""
    def __init__(self, _input: Any, fastperiod: int = 12, slowperiod: int = 26, moving_average_type: int = 0) -> None:
        self._state = _NativePercentagePriceOscillator(fastperiod, slowperiod, moving_average_type)
        self.extend(_input)
    def append(self, _input: float) -> "PercentagePriceOscillator": self._state.append(float(_input)); return self
    def extend(self, _input: Any) -> "PercentagePriceOscillator": self._state.extend(as_float64_series(_input)); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "PercentagePriceOscillator": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
