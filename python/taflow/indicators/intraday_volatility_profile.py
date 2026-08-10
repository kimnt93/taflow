"""Intraday volatility profile adapter."""

from typing import Any

import numpy as np

from .._native import IntradayVolatilityProfile as _Native
from .._series import as_float64_series, as_int64_series


class IntradayVolatilityProfile:
    """Compute expanding sample return volatility by time-of-day bucket.

    Rust assigns each close-to-close return to the bucket of the newer bar and
    updates that bucket with Welford's sample standard deviation. Output shape
    is ``(bars, buckets)`` with a ``NaN`` first row. The external name mapping
    is Wickra ``IntradayVolatilityProfile``.

    Args:
        open: Required opening-price history.
        high: Required high-price history.
        low: Required low-price history.
        close: Required closing-price history used to calculate returns.
        volume: Required volume history for the common OHLCV contract.
        timestamp: Chronological Unix timestamps in nanoseconds.
        buckets: Number of equal-width local-day buckets. Defaults to 24.
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
    ) -> "IntradayVolatilityProfile":
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
    ) -> "IntradayVolatilityProfile":
        """Append aligned OHLCV and timestamp histories and return ``self``."""
        arrays = tuple(
            as_float64_series(item) for item in (open, high, low, close, volume)
        ) + (as_int64_series(timestamp),)
        if len({len(item) for item in arrays}) != 1:
            raise ValueError("OHLCV and timestamp inputs must have equal lengths")
        self._state.extend(*arrays)
        return self

    @property
    def value(self) -> list[float] | None:
        """Return current bucket volatilities, or ``None`` before one return."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return aligned ``(bars, buckets)`` native output history."""
        return self._state.compute()

    def reset(self) -> "IntradayVolatilityProfile":
        """Clear native state and history and return ``self``."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)
