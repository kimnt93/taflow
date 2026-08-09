"""Canonical native-backed OpeningRange adapter."""
from typing import Any
import numpy as np
from ._native import OpeningRange as _NativeOpeningRange
from ._series import as_float64_series, as_bool_series


class OpeningRange:
    """Compute anchored opening high/low and breakout direction."""
    def __init__(self, high: Any, low: Any, close: Any, anchor: Any, bars: int = 30) -> None:
        self._state = _NativeOpeningRange(bars)
        self.extend(high, low, close, anchor)

    def append(self, high: float, low: float, close: float, anchor: bool) -> "OpeningRange":
        self._state.append(float(high), float(low), float(close), bool(anchor)); return self

    def extend(self, high: Any, low: Any, close: Any, anchor: Any) -> "OpeningRange":
        arrays = (as_float64_series(high), as_float64_series(low), as_float64_series(close), as_bool_series(anchor))
        if len({len(array) for array in arrays}) != 1: raise ValueError("inputs must have equal lengths")
        self._state.extend(*arrays); return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]: return self._state.compute()
    @property
    def value(self) -> tuple[float, float, int] | None: return self._state.value
    def reset(self) -> "OpeningRange": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)


__all__ = ["OpeningRange"]
