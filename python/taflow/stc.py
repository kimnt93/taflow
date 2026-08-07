"""Persistent Schaff Trend Cycle (pandas-ta classic alignment)."""
from typing import Any
import numpy as np
from ._native import StcOperator as _Native
from ._series import as_float64_series


class SchaffTrendCycle:
    """Stateful SchaffTrendCycle indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """
    def __init__(
        self,
        close: Any | None = None,
        tclength: int = 10,
        fast: int = 12,
        slow: int = 26,
        factor: float = 0.5,
    ):
        self._state = _Native(tclength, fast, slow, factor)
        self.extend(close) if close is not None else None

    def append(self, close: float):
        self._state.append(close)
        return self

    def extend(self, close: Any):
        self._state.extend(as_float64_series(close))
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        return self._state.compute()

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
        return self
