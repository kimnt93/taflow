"""Descriptive stateful interface for the Hilbert Transform trendline."""

from taflow._native import StatefulHtTrendline


class HilbertTransformTrendline:
    """Incrementally compute the instantaneous Hilbert Transform trendline."""

    def __init__(self):
        self._state = StatefulHtTrendline()

    def append(self, input):
        return self._state.append(input)

    def extend(self, input):
        return self._state.extend(input)

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
