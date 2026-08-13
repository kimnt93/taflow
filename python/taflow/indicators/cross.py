"""Native-backed any-direction crossing signal adapter."""

from typing import Any

import numpy as np

from .._native import CrossOperator as _Native
from .._series import as_float64_series


class Cross:
    """Emit one on either causal upward or downward crossing.

    ``left`` and ``right`` are required equal-length chronological numeric
    histories and may both be empty for a fresh stream. The first output is
    zero because no prior pair exists. ``compute`` returns one aligned float
    array, ``value`` is the latest scalar or ``None`` for an empty stream, and
    lifecycle mutators return ``self``. Input mismatches are rejected before
    native mutation.
    """

    def __init__(self) -> None:
        self._state = _Native()

    def append(self, left: float, right: float) -> "Cross":
        """Append one pair and return this adapter."""
        self._state.append(float(left), float(right))
        return self

    def extend(self, left: Any, right: Any) -> "Cross":
        """Append equal-length left/right histories."""
        arrays = as_float64_series(left), as_float64_series(right)
        if len(arrays[0]) != len(arrays[1]):
            raise ValueError("left and right must have equal lengths")
        self._state.extend(*arrays)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned any-direction crossing flags."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest flag, or ``None`` for an empty stream."""
        return self._state.value

    def reset(self) -> "Cross":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed pairs."""
        return len(self._state)


__all__ = ["Cross"]
