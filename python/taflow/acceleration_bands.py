"""Descriptive stateful interface for Acceleration Bands."""

from taflow._native import StatefulAccbands


class AccelerationBands:
    """Incrementally compute upper, middle, and lower Acceleration Bands."""

    def __init__(self, period=20):
        self._state = StatefulAccbands(period)

    def append(self, high, low, close):
        return self._state.append(high, low, close)

    def extend(self, high, low, close):
        return self._state.extend(high, low, close)

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
