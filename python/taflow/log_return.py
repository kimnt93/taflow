"""Persistent causal logarithmic-return indicator."""

from typing import Any

import numpy as np

from ._native import LogReturn as _NativeLogReturn
from ._series import as_float64_series


class LogReturn:
    """Compute ``ln(x[t] / x[t-timeperiod])`` using persistent Rust state.

    ``_input`` is required; pass an empty series for a fresh streaming state.
    ``timeperiod`` defaults to 1 and must be positive. The first
    ``timeperiod`` history positions are ``NaN``. Correctness maps to pandas
    shifted division followed by ``numpy.log``.
    """

    def __init__(self, _input: Any, timeperiod: int = 1) -> None:
        self._state = _NativeLogReturn(timeperiod)
        self.extend(_input)

    def append(self, _input: float) -> "LogReturn":
        """Append one observation and return this indicator."""
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "LogReturn":
        """Append chronological observations and return this indicator."""
        self._state.extend(as_float64_series(_input))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned history, including ``NaN`` warm-up."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest logarithmic return, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "LogReturn":
        """Restore fresh native state and return this indicator."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed observations."""
        return len(self._state)
