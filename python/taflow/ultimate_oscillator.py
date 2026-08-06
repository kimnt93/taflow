"""Persistent Ultimate Oscillator."""
from typing import Any
import numpy as np
from ._native import UltimateOscillator as _Native
from ._series import as_float64_series
class UltimateOscillator:
    def __init__(self, high: Any | None = None, low: Any | None = None, close: Any | None = None, timeperiod1: int = 7, timeperiod2: int = 14, timeperiod3: int = 28) -> None:
        self._state = _Native(timeperiod1, timeperiod2, timeperiod3)
        if high is not None or low is not None or close is not None: self.extend(high, low, close)
    def append(self, high: float, low: float, close: float) -> "UltimateOscillator": self._state.append(high, low, close); return self
    def extend(self, high: Any, low: Any | None = None, close: Any | None = None) -> "UltimateOscillator":
        if low is None or close is None: raise ValueError("high, low, and close must be provided together")
        self._state.extend(as_float64_series(high), as_float64_series(low), as_float64_series(close)); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "UltimateOscillator": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
ULTOSC = UltimateOscillator
