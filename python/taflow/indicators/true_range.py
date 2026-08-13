"""Persistent True Range adapter."""

from typing import Any

import numpy as np

from .._native import TrueRange as _NativeTrueRange
from .._series import as_float64_series


class TrueRange:
    """Compute max(high-low, |high-prev close|, |low-prev close|) in Rust.

    Supply aligned high, low, and close histories through ``extend``. The first output is NaN because no previous close exists. This
    maps to TA-Lib ``TRANGE``.
    """

    def __init__(self) -> None:
        self._state = _NativeTrueRange()

    def append(self, high: float, low: float, close: float) -> "TrueRange":
        """Append one high/low/close tuple and return this indicator."""
        self._state.append(float(high), float(low), float(close))
        return self

    def extend(self, high: Any, low: Any, close: Any) -> "TrueRange":
        """Append aligned high, low, and close histories and return this indicator."""
        self._state.extend(
            as_float64_series(high), as_float64_series(low), as_float64_series(close)
        )
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned float64 history with first-bar NaN warm-up."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest value, or ``None`` before two bars are present."""
        return self._state.value

    def reset(self) -> "TrueRange":
        """Restore fresh native state, clear history, and return this indicator."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed tuples."""
        return len(self._state)
