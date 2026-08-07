"""Descriptive stateful interface for Parabolic SAR."""

from taflow._native import StatefulSar
from typing import Any


class ParabolicSar:
    """Incrementally compute Parabolic SAR from high/low bars."""

    def __init__(self, acceleration: float = 0.02, maximum: float = 0.2,
                 high: Any | None = None, low: Any | None = None):
        """Create Parabolic SAR with optional aligned high/low history."""
        self._state = StatefulSar(acceleration, maximum)
        if high is not None or low is not None:
            self.extend(high, low)

    def append(self, high, low):
        return self._state.append(high, low)

    def extend(self, high, low):
        return self._state.extend(high, low)

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
