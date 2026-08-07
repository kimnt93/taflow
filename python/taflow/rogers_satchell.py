"""Rolling mean of ``ln(H/C)ln(H/O) + ln(L/C)ln(L/O)`` (Rogers-Satchell)."""
from typing import Any
import numpy as np
from ._native import RogersSatchellOperator as _Native
from ._series import as_float64_series


class RogersSatchell:
    def __init__(
        self,
        open: Any | None = None,
        high: Any | None = None,
        low: Any | None = None,
        close: Any | None = None,
        timeperiod: int = 20,
    ):
        self._state = _Native(timeperiod)
        self.extend(open, high, low, close) if any(value is not None for value in (open, high, low, close)) else None

    def append(self, open: float, high: float, low: float, close: float):
        self._state.append(open, high, low, close)
        return self

    def extend(self, open: Any, high: Any, low: Any, close: Any):
        self._state.extend(
            as_float64_series(open),
            as_float64_series(high),
            as_float64_series(low),
            as_float64_series(close),
        )
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
        return self
