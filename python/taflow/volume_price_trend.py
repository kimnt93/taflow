"""Canonical native-backed Volume Price Trend adapter."""
from typing import Any
import numpy as np
from ._native import VolumePriceTrendOperator as _Native
from ._series import as_float64_series


class VolumePriceTrend:
    """Causal cumulative volume-price trend over close and volume."""
    def __init__(self, close: Any, volume: Any) -> None:
        self._state = _Native(); self._length = 0; self.extend(close, volume)
    def append(self, close: float, volume: float) -> "VolumePriceTrend":
        self._state.append(float(close), float(volume)); self._length += 1; return self
    def extend(self, close: Any, volume: Any) -> "VolumePriceTrend":
        arrays = [as_float64_series(v) for v in (close, volume)]
        if arrays[0].shape != arrays[1].shape: raise ValueError("close and volume must have equal lengths")
        self._state.extend(*arrays); self._length += len(arrays[0]); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "VolumePriceTrend": self._state.reset(); self._length = 0; return self
    def __len__(self) -> int: return self._length
