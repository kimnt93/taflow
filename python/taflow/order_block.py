"""Canonical native-backed order-block adapter."""
from typing import Any
import numpy as np
from ._native import OrderBlockOperator as _Native
from ._series import as_float64_series


class OrderBlock:
    """Causal order-block detection over high, low, close, and volume.
    swing_length and internal_length confirm structure, atr_period normalizes
    volatility, and threshold excludes volatile bars. Outputs are ob, top,
    bottom, ob_volume, mitigated.
    """
    def __init__(self, high: Any, low: Any, close: Any, volume: Any,
                 swing_length: int = 50, internal_length: int = 5,
                 atr_period: int = 200, threshold: float = 2.0) -> None:
        self._state = _Native(int(swing_length), int(internal_length), int(atr_period), float(threshold)); self._length = 0
        self.extend(high, low, close, volume)
    def append(self, high: float, low: float, close: float, volume: float) -> "OrderBlock":
        self._state.append(float(high), float(low), float(close), float(volume)); self._length += 1; return self
    def extend(self, high: Any, low: Any, close: Any, volume: Any) -> "OrderBlock":
        arrays = [as_float64_series(value) for value in (high, low, close, volume)]
        if not all(array.shape == arrays[0].shape for array in arrays): raise ValueError("OHLCV series must have equal lengths")
        self._state.extend(*arrays); self._length += len(arrays[0]); return self
    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray, np.ndarray, np.ndarray]:
        return self._state.compute()
    @property
    def value(self) -> tuple[float, float, float, float, float] | None:
        return self._state.value
    def reset(self) -> "OrderBlock":
        self._state.reset(); self._length = 0; return self
    def __len__(self) -> int:
        return self._length

