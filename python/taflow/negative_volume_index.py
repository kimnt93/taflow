"""Canonical native-backed Negative Volume Index adapter."""
from typing import Any
import numpy as np
from ._native import NegativeVolumeIndexOperator as _Native
from ._series import as_float64_series


class NegativeVolumeIndex:
    """Causal index updated when volume decreases."""
    def __init__(self, close: Any, volume: Any) -> None:
        self._state = _Native(); self._length = 0; self.extend(close, volume)
    def append(self, close: float, volume: float) -> "NegativeVolumeIndex":
        self._state.append(float(close), float(volume)); self._length += 1; return self
    def extend(self, close: Any, volume: Any) -> "NegativeVolumeIndex":
        arrays = [as_float64_series(v) for v in (close, volume)]
        if arrays[0].shape != arrays[1].shape: raise ValueError("close and volume must have equal lengths")
        self._state.extend(*arrays); self._length += len(arrays[0]); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float: return self._state.value
    def reset(self) -> "NegativeVolumeIndex": self._state.reset(); self._length = 0; return self
    def __len__(self) -> int: return self._length
