"""Causal equal-high/equal-low detection."""
from typing import Any
import numpy as np
from ._native import EqualHighsLowsOperator as _Native
from ._series import as_float64_series


class EqualHighsLows:
    """Stateful EqualHighsLows indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """
    def __init__(
        self,
        high: Any | None = None,
        low: Any | None = None,
        close: Any | None = None,
        eq_len: int = 3,
        atr_period: int = 200,
        eq_threshold: float = 0.1,
    ):
        self._state = _Native(eq_len, atr_period, eq_threshold)
        self.extend(high, low, close) if any(value is not None for value in (high, low, close)) else None

    def append(self, high: float, low: float, close: float):
        self._state.append(high, low, close)
        return self

    def extend(self, high: Any, low: Any, close: Any):
        self._state.extend(as_float64_series(high), as_float64_series(low), as_float64_series(close))
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        return self._state.compute()

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
        return self
