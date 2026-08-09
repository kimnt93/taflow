"""Persistent cumulative maximum indicator."""

from typing import Any

import numpy as np

from ._native import CumulativeMaximum as _NativeCumulativeMaximum
from ._series import as_float64_series


class CumulativeMaximum:
    """Compute the cumulative maximum in persistent Rust state.

    ``_input`` is a required chronological numeric series; pass an empty array
    for a fresh streaming state. There is no warm-up. Correctness maps to the
    Polars ``Series.cum_max`` expression.
    """

    def __init__(self, _input: Any) -> None:
        self._state = _NativeCumulativeMaximum()
        self.extend(_input)

    def append(self, _input: float) -> "CumulativeMaximum":
        """Append one observation and return this indicator."""
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "CumulativeMaximum":
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

    def reset(self) -> "CumulativeMaximum":
        """Restore fresh native state and return this indicator."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of observations recorded by Rust."""
        return len(self._state)


__all__ = ["CumulativeMaximum"]
