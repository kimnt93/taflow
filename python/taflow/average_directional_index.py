"""Descriptive stateful interface for the Average Directional Index."""

from taflow._native import StatefulAdx


class AverageDirectionalIndex:
    """Incrementally compute Wilder's Average Directional Index."""

    def __init__(self, period=14):
        self._state = StatefulAdx(period)

    def append(self, high, low, close):
        return self._state.append(high, low, close)

    def extend(self, high, low, close):
        return self._state.extend(high, low, close)

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
