"""Canonical native-backed SessionVolumeLevels adapter."""
from typing import Any
import numpy as np
from ._native import SessionVolumeLevels as _NativeSessionVolumeLevels
from ._series import as_float64_series, as_bool_series


class SessionVolumeLevels:
    """Compute fixed-bin point-of-control and session value-area levels."""
    def __init__(self, high: Any, low: Any, close: Any, volume: Any, anchor: Any, bins: int = 24, value_area: float = 0.7) -> None:
        self._state = _NativeSessionVolumeLevels(bins, value_area)
        self.extend(high, low, close, volume, anchor)

    def append(self, high: float, low: float, close: float, volume: float, anchor: bool) -> "SessionVolumeLevels":
        self._state.append(float(high), float(low), float(close), float(volume), bool(anchor)); return self

    def extend(self, high: Any, low: Any, close: Any, volume: Any, anchor: Any) -> "SessionVolumeLevels":
        arrays = (as_float64_series(high), as_float64_series(low), as_float64_series(close), as_float64_series(volume), as_bool_series(anchor))
        if len({len(array) for array in arrays}) != 1: raise ValueError("inputs must have equal lengths")
        self._state.extend(*arrays); return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]: return self._state.compute()
    @property
    def value(self) -> tuple[float, float, float] | None: return self._state.value
    def reset(self) -> "SessionVolumeLevels": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)


__all__ = ["SessionVolumeLevels"]
