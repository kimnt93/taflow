"""Native-backed causal swing-confirmation adapter."""

from typing import Any

import numpy as np

from .._native import SwingHighLowOperator as _Native
from .._series import as_float64_series


class SwingHighLow:
    """Confirm swing highs and lows after a centered causal window.

    ``high`` and ``low`` are required equal-length chronological histories and
    may both be empty for a fresh stream. ``swing_length`` defaults to 5 and
    must be positive. ``compute`` returns ``(swing_high, swing_low,
    bars_since)`` arrays; Rust owns confirmation delay and NaN warm-up, while
    lifecycle mutators return ``self`` and ``value`` exposes the latest tuple.
    """

    def __init__(self, swing_length: int = 5) -> None:
        self._state = _Native(int(swing_length))

    def append(self, high: float, low: float) -> "SwingHighLow":
        """Append one high/low bar and return this adapter."""
        self._state.append(float(high), float(low))
        return self

    def extend(self, high: Any, low: Any) -> "SwingHighLow":
        """Append equal-length high and low histories."""
        arrays = as_float64_series(high), as_float64_series(low)
        if len(arrays[0]) != len(arrays[1]):
            raise ValueError("high and low must have equal lengths")
        self._state.extend(*arrays)
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        """Return aligned swing-high, swing-low, and bars-since arrays."""
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float, float] | None:
        """Return the latest swing tuple, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "SwingHighLow":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return len(self._state)


__all__ = ["SwingHighLow"]
