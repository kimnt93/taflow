"""Descriptive stateful interface for fixed-parameter MACD."""

from taflow._native import StatefulMacdFix


class MovingAverageConvergenceDivergenceFixed:
    """Incrementally compute TA-Lib's fixed 12/26 MACD variant."""

    def __init__(self, signal_period=9):
        self._state = StatefulMacdFix(signal_period)

    def append(self, value):
        return self._state.append(value)

    def extend(self, values):
        return self._state.extend(values)

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
