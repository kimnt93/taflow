"""Public adapter for native three-session range tracking."""

from typing import Any
import numpy as np
from .._native import SessionRange as _Native
from .._series import as_float64_series, as_int64_series


class SessionRange:
    """Track Asia, Europe, and US high-low ranges for each local day.

    Sessions are local hours 00–08, 08–16, and 16–24 after applying
    ``utc_offset_minutes``. All ranges reset at a day boundary; an unseen
    session reports zero. Output order is ``(asia, europe, united_states)``.
    Timestamps are Unix nanoseconds. The oracle mapping is Wickra
    ``SessionRange`` whose timestamps are converted to milliseconds.

    Args:
        open: Chronological opening prices.
        high: Chronological high prices.
        low: Chronological low prices.
        close: Chronological closing prices.
        volume: Chronological volumes.
        timestamp: Initial Unix-nanosecond timestamps.
        utc_offset_minutes: Signed local offset from UTC, default 0.

    Raises:
        ValueError: If the six input histories differ in length.
    """

    def __init__(self, utc_offset_minutes: int = 0) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native(utc_offset_minutes)

    def append(self, open: float, high: float, low: float, close: float, volume: float, timestamp: int) -> "SessionRange":
        """Append one timestamped OHLCV bar and return this adapter."""
        self._state.append(float(open), float(high), float(low), float(close), float(volume), int(timestamp))
        return self

    def extend(self, open: Any, high: Any, low: Any, close: Any, volume: Any, timestamp: Any) -> "SessionRange":
        """Append aligned timestamped OHLCV histories and return this adapter."""
        series = tuple(as_float64_series(item) for item in (open, high, low, close, volume)) + (as_int64_series(timestamp),)
        if len({len(item) for item in series}) != 1:
            raise ValueError("OHLCV and timestamp must have equal lengths")
        self._state.extend(*series)
        return self

    @property
    def value(self) -> tuple[float, float, float] | None:
        """Return the latest Asia/Europe/US ranges, or ``None`` when empty."""
        return self._state.value

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        """Return aligned Asia, Europe, and US range histories."""
        return self._state.compute()

    def reset(self) -> "SessionRange":
        """Reset calendar and session extents, then return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)
