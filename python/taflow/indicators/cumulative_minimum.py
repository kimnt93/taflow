"""Persistent cumulative minimum indicator."""

from typing import Any

import numpy as np

from .._native import CumulativeMinimum as _NativeCumulativeMinimum
from .._series import as_float64_series


class CumulativeMinimum:
    """Compute the cumulative minimum in persistent Rust state.

    Supply the chronological numeric ``_input`` series through ``extend``. There is no warm-up. Correctness maps to the
    Polars ``Series.cum_min`` expression.
    """

    def __init__(self) -> None:
        self._state = _NativeCumulativeMinimum()

    def append(self, _input: float) -> "CumulativeMinimum":
        """Append one observation and return this indicator."""
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "CumulativeMinimum":
        """Append chronological observations and return this indicator."""
        self._state.extend(as_float64_series(_input))
        return self

    def compute(self) -> np.ndarray:
        """Return the complete aligned ``float64`` history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest result, or ``None`` before the first value."""
        return self._state.value

    def reset(self) -> "CumulativeMinimum":
        """Restore fresh native state and return this indicator."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed observations."""
        return len(self._state)
