"""Persistent Smoothed Trend Channel adapter."""

from typing import Any

import numpy as np

from .._native import SmoothedTrendChannel as _NativeSmoothedTrendChannel
from .._series import as_float64_series


class SmoothedTrendChannel:
    """Compute causal smoothed high/low channels in native Rust state.

    ``high``, ``low`` and ``close`` are required aligned series and may all be
    empty for a fresh stream. ``length`` defaults to 10 and must be positive.
    The first ``length - 1`` output pairs are NaN; ``compute`` returns
    ``(lower, upper)`` arrays. This is the TAFlow SSL Channel definition.
    """

    def __init__(self, length: int = 10) -> None:
        self._state = _NativeSmoothedTrendChannel(int(length))

    def append(self, high: float, low: float, close: float) -> "SmoothedTrendChannel":
        """Append one high/low/close bar and return this adapter."""
        self._state.append(float(high), float(low), float(close))
        return self

    def extend(self, high: Any, low: Any, close: Any) -> "SmoothedTrendChannel":
        """Append aligned high, low, and close histories in that order."""
        arrays = tuple(as_float64_series(series) for series in (high, low, close))
        if len({len(array) for array in arrays}) != 1:
            raise ValueError("high, low, and close must have equal lengths")
        self._state.extend(*arrays)
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray]:
        """Return aligned lower and upper channel arrays."""
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float] | None:
        """Return the latest ``(lower, upper)`` pair, or ``None`` in warm-up."""
        return self._state.value

    def reset(self) -> "SmoothedTrendChannel":
        """Restore fresh native state and return this indicator."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return len(self._state)


__all__ = ["SmoothedTrendChannel"]
