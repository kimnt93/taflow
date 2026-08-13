"""Native-backed causal rolling-interquartile-range adapter."""

from typing import Any

import numpy as np

from .._native import RollingInterquartileRange as _Native
from .._adapter_protocol import adapter_length
from .._series import as_float64_series


class RollingInterquartileRange:
    """Compute Q3 minus Q1 over a trailing window.

    ``_input`` is the required chronological series and may be empty for a
    fresh stream. ``timeperiod`` defaults to 14 and must be positive. Rust owns
    the sorted window, percentile interpolation, NaN warm-up, and aligned
    history. ``compute`` returns one float array; ``value`` is the latest scalar
    or ``None`` during warm-up; lifecycle mutators return ``self``. The
    independent oracle is pandas rolling quantile.
    """

    def __init__(self, timeperiod: int = 14) -> None:
        self._state = _Native(int(timeperiod))

    def append(self, _input: float) -> "RollingInterquartileRange":
        """Append one observation and return this adapter."""
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "RollingInterquartileRange":
        """Append a chronological observation series and return this adapter."""
        values = as_float64_series(_input)
        self._state.extend(values)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned trailing interquartile-range history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest interquartile range, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "RollingInterquartileRange":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed observations."""
        return adapter_length(self)


__all__ = ["RollingInterquartileRange"]
