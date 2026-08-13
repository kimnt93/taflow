"""Persistent absolute price oscillator adapter."""

from typing import Any
import numpy as np
from .._native import AbsolutePriceOscillator as _NativeAbsolutePriceOscillator
from .._series import as_float64_series


class AbsolutePriceOscillator:
    """Compute fast moving average minus slow moving average in Rust."""
    def __init__(self, fastperiod: int = 12, slowperiod: int = 26, moving_average_type: int = 0) -> None:
        self._state = _NativeAbsolutePriceOscillator(fastperiod, slowperiod, moving_average_type)
    def append(self, _input: float) -> "AbsolutePriceOscillator": self._state.append(float(_input)); return self
    def extend(self, _input: Any) -> "AbsolutePriceOscillator": self._state.extend(as_float64_series(_input)); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "AbsolutePriceOscillator": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
