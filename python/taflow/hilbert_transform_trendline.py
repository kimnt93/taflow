"""Descriptive stateful interface for the Hilbert Transform trendline."""

from taflow._native import StatefulHtTrendline
from typing import Any


class HilbertTransformTrendline:
    """Incrementally compute the instantaneous Hilbert Transform trendline."""

    def __init__(self, _input: Any | None = None):
        """Create the trendline with an optional initial price series."""
        self._state = StatefulHtTrendline()
        if _input is not None:
            self.extend(_input)

    def append(self, _input):
        return self._state.append(_input)

    def extend(self, _input):
        return self._state.extend(_input)

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
