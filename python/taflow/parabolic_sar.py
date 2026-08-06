"""Descriptive stateful interface for Parabolic SAR."""

from taflow._native import StatefulSar


class ParabolicSar:
    """Incrementally compute Parabolic SAR from high/low bars."""

    def __init__(self, acceleration=0.02, maximum=0.2):
        self._state = StatefulSar(acceleration, maximum)

    def append(self, high, low):
        return self._state.append(high, low)

    def extend(self, high, low):
        return self._state.extend(high, low)

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
