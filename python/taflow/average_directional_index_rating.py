"""Descriptive stateful interface for the ADX Rating."""

from taflow._native import StatefulAdxr


class AverageDirectionalIndexRating:
    """Incrementally compute the lag-averaged Average Directional Index."""

    def __init__(self, period=14):
        self._state = StatefulAdxr(period)

    def append(self, high, low, close):
        return self._state.append(high, low, close)

    def extend(self, high, low, close):
        return self._state.extend(high, low, close)

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
