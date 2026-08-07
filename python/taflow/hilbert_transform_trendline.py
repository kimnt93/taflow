"""Descriptive stateful interface for the Hilbert Transform trendline."""

from taflow._native import StatefulHtTrendline
from typing import Any


class HilbertTransformTrendline:
    """Incrementally compute the instantaneous Hilbert Transform trendline."""

    def __init__(self, input: Any | None = None):
        """Create the trendline with an optional initial price series."""
        self._state = StatefulHtTrendline()
        if input is not None:
            self.extend(input)

    def append(self, input):
        return self._state.append(input)

    def extend(self, input):
        return self._state.extend(input)

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
