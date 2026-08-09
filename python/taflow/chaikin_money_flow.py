"""Canonical native-backed Chaikin Money Flow adapter."""
from typing import Any
import numpy as np
from ._native import ChaikinMoneyFlowOperator as _Native
from ._series import as_float64_series


class ChaikinMoneyFlow:
    """Chaikin Money Flow over high, low, close, and volume."""
    def __init__(self, high: Any, low: Any, close: Any, volume: Any, period: int = 20) -> None:
        self._state = _Native(int(period)); self._length = 0; self.extend(high, low, close, volume)
    def append(self, high: float, low: float, close: float, volume: float) -> "ChaikinMoneyFlow":
        self._state.append(float(high), float(low), float(close), float(volume)); self._length += 1; return self
    def extend(self, high: Any, low: Any, close: Any, volume: Any) -> "ChaikinMoneyFlow":
        arrays = [as_float64_series(v) for v in (high, low, close, volume)]
        if not all(a.shape == arrays[0].shape for a in arrays): raise ValueError("OHLCV series must have equal lengths")
        self._state.extend(*arrays); self._length += len(arrays[0]); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "ChaikinMoneyFlow": self._state.reset(); self._length = 0; return self
    def __len__(self) -> int: return self._length
