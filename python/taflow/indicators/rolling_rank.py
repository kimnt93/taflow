"""Native-backed causal rolling-rank adapter."""

from typing import Any

import numpy as np

from .._native import RollingRank as _Native
from .._adapter_protocol import adapter_length
from .._series import as_float64_series


class RollingRank:
    """Compute the percentile rank of each value in a trailing window.

    ``_input`` is the required chronological series and may be empty for a
    fresh stream. ``timeperiod`` defaults to 14 and must be positive. The
    native state emits NaN until the window is full and uses the causal
    latest-value rank definition. ``compute`` returns one aligned float array;
    ``value`` is the latest scalar or ``None`` during warm-up; lifecycle
    mutators return ``self``. The independent oracle is pandas rolling apply.
    """

    def __init__(self, _input: Any, timeperiod: int = 14) -> None:
        self._state = _Native(int(timeperiod))
        self.extend(_input)

    def append(self, _input: float) -> "RollingRank":
        """Append one observation and return this adapter."""
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "RollingRank":
        """Append a chronological observation series and return this adapter."""
        values = as_float64_series(_input)
        self._state.extend(values)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned trailing-rank history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest rank, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "RollingRank":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed observations."""
        return adapter_length(self)


__all__ = ["RollingRank"]
