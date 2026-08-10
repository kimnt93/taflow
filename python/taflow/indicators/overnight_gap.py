"""Overnight gap adapter."""

from typing import Any

import numpy as np

from .._native import OvernightGap as _Native
from .._series import as_float64_series, as_int64_series


class OvernightGap:
    """Measure each session open relative to the prior session close.

    On the first bar of a new local calendar day, Rust computes
    ``open / previous_close - 1`` and holds that gap through the session. The
    first observed session is warm-up and emits ``NaN``. Timestamps are Unix
    nanoseconds. This class maps to Wickra ``OvernightGap``.

    Args:
        open: Required opening-price history used at session boundaries.
        high: Required high-price history for the common OHLCV contract.
        low: Required low-price history for the common OHLCV contract.
        close: Required closing-price history used as prior-session close.
        volume: Required volume history for the common OHLCV contract.
        timestamp: Chronological Unix timestamps in nanoseconds.
        utc_offset_minutes: Fixed offset used to identify local days. Defaults to 0.

    Raises:
        ValueError: If the six histories have different lengths.
    """

    def __init__(
        self,
        open: Any,
        high: Any,
        low: Any,
        close: Any,
        volume: Any,
        timestamp: Any,
        utc_offset_minutes: int = 0,
    ) -> None:
        """Initialize the native state and process the supplied history."""
        self._state = _Native(utc_offset_minutes)
        self.extend(open, high, low, close, volume, timestamp)

    def append(
        self,
        open: float,
        high: float,
        low: float,
        close: float,
        volume: float,
        timestamp: int,
    ) -> "OvernightGap":
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
    ) -> "OvernightGap":
        """Append aligned OHLCV and timestamp histories and return ``self``."""
        arrays = tuple(
            as_float64_series(item) for item in (open, high, low, close, volume)
        ) + (as_int64_series(timestamp),)
        if len({len(item) for item in arrays}) != 1:
            raise ValueError("OHLCV and timestamp inputs must have equal lengths")
        self._state.extend(*arrays)
        return self

    @property
    def value(self) -> float | None:
        """Return the current held overnight gap, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return the aligned native overnight-gap history."""
        return self._state.compute()

    def reset(self) -> "OvernightGap":
        """Clear native state and history and return ``self``."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)
