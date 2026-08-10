"""Time-of-day return profile adapter."""

from typing import Any

import numpy as np

from .._native import TimeOfDayReturnProfile as _Native
from .._series import as_float64_series, as_int64_series


class TimeOfDayReturnProfile:
    """Compute expanding mean close returns by intraday time bucket.

    Each close-to-close return is assigned to the bucket containing the newer
    bar. Rust owns the running sums, counts, aligned history, and warm-up. The
    result has shape ``(bars, buckets)``; the first row is ``NaN`` because no
    return exists yet. This class maps to Wickra ``TimeOfDayReturnProfile``.

    Args:
        open: Required opening-price history.
        high: Required high-price history.
        low: Required low-price history.
        close: Required closing-price history.
        volume: Required volume history; accepted for the common OHLCV contract.
        timestamp: Unix timestamps in nanoseconds, ordered chronologically.
        buckets: Number of equal-width buckets in each local day. Defaults to 24.
        utc_offset_minutes: Fixed offset applied before bucketing. Defaults to 0.

    Raises:
        ValueError: If inputs differ in length or ``buckets`` is zero.
    """

    def __init__(
        self,
        open: Any,
        high: Any,
        low: Any,
        close: Any,
        volume: Any,
        timestamp: Any,
        buckets: int = 24,
        utc_offset_minutes: int = 0,
    ) -> None:
        """Initialize the native profile and process the supplied history."""
        self._state = _Native(buckets, utc_offset_minutes)
        self.extend(open, high, low, close, volume, timestamp)

    def append(
        self,
        open: float,
        high: float,
        low: float,
        close: float,
        volume: float,
        timestamp: int,
    ) -> "TimeOfDayReturnProfile":
        """Append one OHLCV bar and Unix-nanosecond timestamp; return ``self``."""
        self._state.append(
            float(open), float(high), float(low), float(close),
            float(volume), int(timestamp),
        )
        return self

    def extend(
        self,
        open: Any,
        high: Any,
        low: Any,
        close: Any,
        volume: Any,
        timestamp: Any,
    ) -> "TimeOfDayReturnProfile":
        """Append aligned OHLCV and timestamp series and return ``self``.

        Raises:
            ValueError: If the six input series have different lengths.
        """
        arrays = tuple(
            as_float64_series(item) for item in (open, high, low, close, volume)
        ) + (as_int64_series(timestamp),)
        if len({len(item) for item in arrays}) != 1:
            raise ValueError("OHLCV and timestamp inputs must have equal lengths")
        self._state.extend(*arrays)
        return self

    @property
    def value(self) -> list[float] | None:
        """Return current bucket means, or ``None`` before the first return."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return the aligned ``(bars, buckets)`` native output history."""
        return self._state.compute()

    def reset(self) -> "TimeOfDayReturnProfile":
        """Clear native state and history and return ``self``."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)
