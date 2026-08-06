"""Descriptive stateful interface for a selectable moving average."""

from taflow._native import StatefulMa


class MovingAverage:
    """Incrementally compute any TA-Lib moving-average type."""

    def __init__(self, period=30, moving_average_type=0):
        self._state = StatefulMa(period, moving_average_type)

    def append(self, value):
        return self._state.append(value)

    def extend(self, values):
        return self._state.extend(values)

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
