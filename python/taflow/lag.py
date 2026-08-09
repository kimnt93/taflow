"""Persistent causal lag indicator."""

from typing import Any

import numpy as np

from ._native import Lag as _NativeLag
from ._series import as_float64_series


class Lag:
    """Return the value from ``timeperiod`` bars earlier using Rust state.

    ``_input`` is required; pass an empty series for a fresh streaming state.
    ``timeperiod`` defaults to 1 and must be positive. The first
    ``timeperiod`` history positions are ``NaN``. Correctness maps to pandas
    ``Series.shift``.
    """

    def __init__(self, _input: Any, timeperiod: int = 1) -> None:
        self._state = _NativeLag(timeperiod)
        self.extend(_input)

    def append(self, _input: float) -> "Lag":
        """Append one observation and return this indicator."""
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "Lag":
        """Append chronological observations and return this indicator."""
        self._state.extend(as_float64_series(_input))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned history, including ``NaN`` warm-up."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest delayed value, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "Lag":
        """Restore fresh native state and return this indicator."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed observations."""
        return len(self._state)
