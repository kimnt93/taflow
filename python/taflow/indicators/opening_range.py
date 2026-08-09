"""Native-backed anchored opening-range adapter."""

from typing import Any

import numpy as np

from .._native import OpeningRange as _NativeOpeningRange
from .._series import as_bool_series, as_float64_series


class OpeningRange:
    """Compute anchored opening high, low, and breakout direction.

    Parameters
    ----------
    high, low, close : array-like
        Required aligned OHLC histories in that order.
    anchor : array-like of bool
        Required session-anchor flags aligned with the OHLC histories.
    bars : int, default 30
        Positive maximum number of bars used to form each opening range.

    ``compute`` returns ``(high, low, direction)`` arrays in that order, with
    warm-up and session behavior owned by Rust. ``value`` returns the latest
    tuple or ``None`` before output is available. All lifecycle mutators return
    ``self``; unequal input lengths are rejected before native mutation.
    """

    def __init__(
        self, high: Any, low: Any, close: Any, anchor: Any, bars: int = 30
    ) -> None:
        self._state = _NativeOpeningRange(int(bars))
        self.extend(high, low, close, anchor)

    def append(
        self, high: float, low: float, close: float, anchor: bool
    ) -> "OpeningRange":
        """Append one high/low/close/anchor bar in that order."""
        self._state.append(float(high), float(low), float(close), bool(anchor))
        return self

    def extend(
        self, high: Any, low: Any, close: Any, anchor: Any
    ) -> "OpeningRange":
        """Append aligned high/low/close/anchor histories in that order."""
        arrays = (
            as_float64_series(high),
            as_float64_series(low),
            as_float64_series(close),
            as_bool_series(anchor),
        )
        if len({len(array) for array in arrays}) != 1:
            raise ValueError("high, low, close, and anchor must have equal lengths")
        self._state.extend(*arrays)
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        """Return aligned opening-high, opening-low, and direction arrays."""
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float, int] | None:
        """Return the latest range tuple, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "OpeningRange":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return len(self._state)


__all__ = ["OpeningRange"]
