"""Descriptive stateful interface for the Directional Movement Index."""

from taflow._native import StatefulDx


class DirectionalMovementIndex:
    """Incrementally compute Wilder's Directional Movement Index."""

    def __init__(self, period=14):
        self._state = StatefulDx(period)

    def append(self, high, low, close):
        return self._state.append(high, low, close)

    def extend(self, high, low, close):
        return self._state.extend(high, low, close)

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
