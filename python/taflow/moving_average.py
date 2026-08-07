"""Descriptive stateful interface for a selectable moving average."""

from taflow._native import StatefulMa
from typing import Any


class MovingAverage:
    """Incrementally compute any TA-Lib moving-average type."""

    def __init__(self, period: int = 30, moving_average_type: int = 0,
                 values: Any | None = None):
        """Create a selectable moving average with optional initial values."""
        self._state = StatefulMa(period, moving_average_type)
        if values is not None:
            self.extend(values)

    def append(self, value):
        return self._state.append(value)

    def extend(self, values):
        return self._state.extend(values)

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
