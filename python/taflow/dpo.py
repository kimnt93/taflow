"""Persistent causal Detrended Price Oscillator."""

from typing import Any

import numpy as np

from ._native import DpoOperator as _Native
from ._series import as_float64_series


class Dpo:
    """Causal DPO; pandas-ta centered/lookahead output is not exposed."""

    def __init__(self, close: Any | None = None, period=20):
        self._state = _Native(period)
        self.extend(close) if close is not None else None

    def append(self, close: float):
        self._state.append(close)
        return self

    def extend(self, close: Any):
        self._state.extend(as_float64_series(close))
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
        return self
