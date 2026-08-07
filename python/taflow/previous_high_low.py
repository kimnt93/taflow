"""Causal prior-higher-timeframe high/low tracking with break flags."""
from typing import Any
import numpy as np
from ._native import PreviousHighLowOperator as _Native
from ._series import as_float64_series


class PreviousHighLow:
    def __init__(self, new_session: Any | None = None, high: Any | None = None, low: Any | None = None):
        self._state = _Native()
        self.extend(new_session, high, low) if new_session is not None or high is not None or low is not None else None

    def append(self, new_session: bool, high: float, low: float):
        self._state.append(new_session, high, low)
        return self

    def extend(self, new_session: Any, high: Any, low: Any):
        self._state.extend(np.asarray(new_session, dtype=bool), as_float64_series(high), as_float64_series(low))
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray, np.ndarray]:
        return self._state.compute()

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
        return self
