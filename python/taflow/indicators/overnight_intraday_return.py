"""Public adapter for native overnight and intraday returns."""

from typing import Any
import numpy as np
from .._native import OvernightIntradayReturn as _Native
from .._series import as_float64_series, as_int64_series


class OvernightIntradayReturn:
    """Decompose each local day into overnight and intraday return components.

    At a local day boundary, overnight return is ``open / prior_close - 1``;
    during that day intraday return is ``latest_close / day_open - 1``. The
    first day emits ``NaN`` because no prior close exists. Output order is
    ``(overnight, intraday)``. Timestamps are Unix nanoseconds. This maps to
    Wickra ``OvernightIntradayReturn`` after timestamp-unit conversion.

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

    def append(self, open: float, high: float, low: float, close: float, volume: float, timestamp: int) -> "OvernightIntradayReturn":
        """Append one timestamped OHLCV bar and return this adapter."""
        self._state.append(float(open), float(high), float(low), float(close), float(volume), int(timestamp))
        return self

    def extend(self, open: Any, high: Any, low: Any, close: Any, volume: Any, timestamp: Any) -> "OvernightIntradayReturn":
        """Append aligned timestamped OHLCV histories and return this adapter."""
        series = tuple(as_float64_series(item) for item in (open, high, low, close, volume)) + (as_int64_series(timestamp),)
        if len({len(item) for item in series}) != 1:
            raise ValueError("OHLCV and timestamp must have equal lengths")
        self._state.extend(*series)
        return self

    @property
    def value(self) -> tuple[float, float] | None:
        """Return latest overnight/intraday pair, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> tuple[np.ndarray, np.ndarray]:
        """Return aligned overnight and intraday return histories."""
        return self._state.compute()

    def reset(self) -> "OvernightIntradayReturn":
        """Reset day anchors and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)
