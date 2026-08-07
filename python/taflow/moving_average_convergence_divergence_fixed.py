"""Descriptive stateful interface for fixed-parameter MACD."""

from taflow._native import StatefulMacdFix
from typing import Any


class MovingAverageConvergenceDivergenceFixed:
    """Incrementally compute TA-Lib's fixed 12/26 MACD variant."""

    def __init__(self, signal_period: int = 9, value: Any | None = None):
        """Create fixed MACD with an optional initial price series."""
        self._state = StatefulMacdFix(signal_period)
        if value is not None:
            self.extend(value)

    def append(self, value):
        return self._state.append(value)

    def extend(self, values):
        return self._state.extend(values)

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
