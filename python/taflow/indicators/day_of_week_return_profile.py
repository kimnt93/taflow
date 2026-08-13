"""Day-of-week return profile adapter."""

from typing import Any

import numpy as np

from .._native import DayOfWeekReturnProfile as _Native
from .._series import as_float64_series, as_int64_series


class DayOfWeekReturnProfile:
    """Compute expanding mean close returns for Monday through Sunday.

    Returns are assigned to the local weekday of the newer bar. Rust owns all
    arithmetic and returns a ``(bars, 7)`` history ordered Monday through
    Sunday; the first row is ``NaN``. This maps to Wickra ``DayOfWeekProfile``.

    Args:
        open: Required opening-price history.
        high: Required high-price history.
        low: Required low-price history.
        close: Required closing-price history.
        volume: Required volume history for the common OHLCV contract.
        timestamp: Chronological Unix timestamps in nanoseconds.
        utc_offset_minutes: Fixed offset applied before weekday selection.

    Raises:
        ValueError: If the input histories have different lengths.
    """

    def __init__(
        self,
        utc_offset_minutes: int = 0,
    ) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native(utc_offset_minutes)

    def append(
        self,
        open: float,
        high: float,
        low: float,
        close: float,
        volume: float,
        timestamp: int,
    ) -> "DayOfWeekReturnProfile":
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
    ) -> "DayOfWeekReturnProfile":
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
        """Return the seven current weekday means, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return aligned native output with Monday-to-Sunday columns."""
        return self._state.compute()

    def reset(self) -> "DayOfWeekReturnProfile":
        """Clear native state and history and return ``self``."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)
