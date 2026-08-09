"""Canonical native-backed Ichimoku adapter."""
from typing import Any
import numpy as np
from .._native import IchimokuOperator as _Native
from .._series import as_float64_series


class Ichimoku:
    """Causal Ichimoku Kinko Hyo over required high, low, and close series.
    tenkan, kijun, and senkou are rolling midpoint periods.
    """
    def __init__(self, high: Any, low: Any, close: Any, tenkan: int = 9,
                 kijun: int = 26, senkou: int = 52) -> None:
        self._state = _Native(int(tenkan), int(kijun), int(senkou)); self._length = 0
        self.extend(high, low, close)
    def append(self, high: float, low: float, close: float) -> "Ichimoku":
        self._state.append(float(high), float(low), float(close)); self._length += 1; return self
    def extend(self, high: Any, low: Any, close: Any) -> "Ichimoku":
        high_values = as_float64_series(high); low_values = as_float64_series(low); close_values = as_float64_series(close)
        if not (high_values.shape == low_values.shape == close_values.shape): raise ValueError("high, low, and close must have equal lengths")
        self._state.extend(high_values, low_values, close_values); self._length += len(high_values); return self
    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray, np.ndarray, np.ndarray]:
        return self._state.compute()
    @property
    def value(self) -> tuple[float, float, float, float, float] | None:
        return self._state.value
    def reset(self) -> "Ichimoku":
        self._state.reset(); self._length = 0; return self
    def __len__(self) -> int:
        return self._length
