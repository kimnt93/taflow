"""Persistent TTM Squeeze (pandas-ta classic alignment)."""
from typing import Any
import numpy as np
from ._native import SqueezeOperator as _Native
from ._series import as_float64_series


class Squeeze:
    def __init__(
        self,
        high: Any | None = None,
        low: Any | None = None,
        close: Any | None = None,
        bb_length: int = 20,
        bb_std: float = 2.0,
        kc_length: int = 20,
        kc_scalar: float = 1.5,
        mom_length: int = 12,
        mom_smooth: int = 6,
    ):
        self._state = _Native(bb_length, bb_std, kc_length, kc_scalar, mom_length, mom_smooth)
        self.extend(high, low, close) if any(value is not None for value in (high, low, close)) else None

    def append(self, high: float, low: float, close: float):
        self._state.append(high, low, close)
        return self

    def extend(self, high: Any, low: Any, close: Any):
        self._state.extend(
            as_float64_series(high), as_float64_series(low), as_float64_series(close)
        )
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray, np.ndarray]:
        return self._state.compute()

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
        return self
