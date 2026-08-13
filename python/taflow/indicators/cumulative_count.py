"""Native-backed cumulative observation count adapter."""

from typing import Any

import numpy as np

from .._native import CumulativeCount as _NativeCumulativeCount
from .._series import as_float64_series


class CumulativeCount:
    """Emit the one-based count of chronological observations.

    ``_input`` is the required input series and may be empty to create a fresh
    stream. Each bar emits its one-based position, so ``compute`` returns a
    one-dimensional float NumPy array and ``value`` is ``None`` before the
    first append. ``append``, ``extend``, and ``reset`` mutate and return this
    adapter; the native Rust state performs all counting and warm-up handling.
    """

    def __init__(self) -> None:
        self._state = _NativeCumulativeCount()

    def append(self, _input: float) -> "CumulativeCount":
        """Append one chronological observation and return this adapter."""
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "CumulativeCount":
        """Append a converted chronological observation history."""
        self._state.extend(as_float64_series(_input))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned one-based observation counts."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest count, or ``None`` before the first append."""
        return self._state.value

    def reset(self) -> "CumulativeCount":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed observations."""
        return len(self._state)


__all__ = ["CumulativeCount"]
