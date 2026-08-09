"""Canonical native-backed linear-decay adapter."""
from typing import Any
import numpy as np
from .._native import WeightedMovingAverage as _Native
from .._series import as_float64_series


class DecayLinear:
    """WorldQuant linear-decay weighted moving average."""
    def __init__(self, input: Any, timeperiod: int = 30) -> None:
        self._state = _Native(int(timeperiod)); self._length = 0; self.extend(input)
    def append(self, input: float) -> "DecayLinear":
        self._state.append(float(input)); self._length += 1; return self
    def extend(self, input: Any) -> "DecayLinear":
        values = as_float64_series(input); self._state.extend(values); self._length += len(values); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "DecayLinear": self._state.reset(); self._length = 0; return self
    def __len__(self) -> int: return self._length
