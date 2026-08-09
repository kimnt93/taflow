"""Native-backed causal rolling-mode adapter."""

from typing import Any

import numpy as np

from ._native import RollingModeOperator as _Native
from ._series import as_float64_series


class RollingMode:
    """Return the most frequent value in a trailing window.

    ``_input`` is the required chronological series and may be empty for a
    fresh stream. ``timeperiod`` defaults to 14 and must be positive. The
    native state emits NaN until the window is full, then chooses the earliest
    maximal-count value in window order. ``compute`` returns one aligned float
    array; lifecycle mutators return ``self`` and ``value`` is the latest
    scalar or ``None``. The independent oracle is pandas rolling/value-counts.
    """

    def __init__(self, _input: Any, timeperiod: int = 14) -> None:
        self._state = _Native(int(timeperiod))
        self._length = 0
        self.extend(_input)

    def append(self, _input: float) -> "RollingMode":
        """Append one observation and return this adapter."""
        self._state.append(float(_input))
        self._length += 1
        return self

    def extend(self, _input: Any) -> "RollingMode":
        """Append a chronological observation series and return this adapter."""
        values = as_float64_series(_input)
        self._state.extend(values)
        self._length += len(values)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned trailing-mode history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest mode, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "RollingMode":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        self._length = 0
        return self

    def __len__(self) -> int:
        """Return the number of processed observations."""
        return self._length


__all__ = ["RollingMode"]
