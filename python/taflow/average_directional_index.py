"""Descriptive stateful interface for the Average Directional Index."""

from taflow._native import StatefulAdx
from typing import Any


class AverageDirectionalIndex:
    """Incrementally compute Wilder's Average Directional Index."""

    def __init__(self, period: int = 14, high: Any | None = None,
                 low: Any | None = None, close: Any | None = None):
        """Create the indicator and optionally process an initial history.

        Parameters are ``period`` (Wilder lookback), ``high``, ``low``, and
        ``close`` (aligned OHLC series).  The constructor returns no value;
        use ``extend`` for later history.
        """
        self._state = StatefulAdx(period)
        if any(value is not None for value in (high, low, close)):
            self.extend(high, low, close)

    def append(self, high, low, close):
        return self._state.append(high, low, close)

    def extend(self, high, low, close):
        return self._state.extend(high, low, close)

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
