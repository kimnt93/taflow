"""Canonical native-backed Positive Volume Index adapter."""
from typing import Any
import numpy as np
from .._native import PositiveVolumeIndexOperator as _Native
from .._series import as_float64_series


class PositiveVolumeIndex:
    """Causal index updated when volume increases."""
    def __init__(self, close: Any, volume: Any) -> None:
        self._state = _Native(); self.extend(close, volume)
    def append(self, close: float, volume: float) -> "PositiveVolumeIndex":
        self._state.append(float(close), float(volume)); return self
    def extend(self, close: Any, volume: Any) -> "PositiveVolumeIndex":
        arrays = [as_float64_series(v) for v in (close, volume)]
        if arrays[0].shape != arrays[1].shape: raise ValueError("close and volume must have equal lengths")
        self._state.extend(*arrays); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float: return self._state.value
    def reset(self) -> "PositiveVolumeIndex": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)
