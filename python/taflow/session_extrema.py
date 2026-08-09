"""Canonical session-extrema adapter."""

from typing import Any

import numpy as np

from ._native import SessionExtremaOperator as _Native
from ._series import as_float64_series


class SessionExtrema:
    """Track high and low values within explicit causal sessions."""

    def __init__(self, new_session: Any, high: Any, low: Any) -> None:
        self._state = _Native()
        self._length = 0
        self.extend(new_session, high, low)

    def append(self, new_session: bool, high: float, low: float) -> "SessionExtrema":
        self._state.append(bool(new_session), float(high), float(low))
        self._length += 1
        return self

    def extend(self, new_session: Any, high: Any, low: Any) -> "SessionExtrema":
        session_values = np.asarray(new_session, dtype=bool)
        high_values = as_float64_series(high)
        low_values = as_float64_series(low)
        if len({len(session_values), len(high_values), len(low_values)}) != 1:
            raise ValueError("new_session, high, and low must have equal lengths")
        self._state.extend(session_values, high_values, low_values)
        self._length += len(session_values)
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray]:
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float] | None:
        return self._state.value

    def reset(self) -> "SessionExtrema":
        self._state.reset()
        self._length = 0
        return self

    def __len__(self) -> int:
        return self._length
