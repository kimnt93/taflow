"""Descriptive stateful interface for the ADX Rating."""

from taflow._native import StatefulAdxr
from typing import Any


class AverageDirectionalIndexRating:
    """Incrementally compute the lag-averaged Average Directional Index."""

    def __init__(self, period: int = 14, high: Any | None = None,
                 low: Any | None = None, close: Any | None = None):
        """Create ADXR with an optional aligned high/low/close history."""
        self._state = StatefulAdxr(period)
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
