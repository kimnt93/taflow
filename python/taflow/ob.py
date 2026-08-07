"""Causal order-block detection with volatile-bar exclusion and mitigation."""
from typing import Any
import numpy as np
from ._native import ObOperator as _Native
from ._series import as_float64_series


class Ob:
    def __init__(
        self,
        high: Any | None = None,
        low: Any | None = None,
        close: Any | None = None,
        volume: Any | None = None,
        swing_length: int = 50,
        internal_length: int = 5,
        atr_period: int = 200,
        threshold: float = 2.0,
    ):
        self._state = _Native(swing_length, internal_length, atr_period, threshold)
        self.extend(high, low, close, volume) if any(value is not None for value in (high, low, close, volume)) else None

    def append(self, high: float, low: float, close: float, volume: float):
        self._state.append(high, low, close, volume)
        return self

    def extend(self, high: Any, low: Any, close: Any, volume: Any):
        self._state.extend(
            as_float64_series(high),
            as_float64_series(low),
            as_float64_series(close),
            as_float64_series(volume),
        )
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray, np.ndarray, np.ndarray]:
        return self._state.compute()

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
        return self
