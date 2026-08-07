"""Descriptive stateful interface for a variable-period moving average."""

from taflow._native import StatefulMavp
from typing import Any


class VariablePeriodMovingAverage:
    """Incrementally compute MAVP from values and per-bar periods."""

    def __init__(self, min_period: int = 2, max_period: int = 30,
                 average_type: int = 0, _input: Any | None = None,
                 periods: Any | None = None):
        """Create MAVP with optional values and per-bar periods."""
        self._state = StatefulMavp(min_period, max_period, average_type)
        if _input is not None or periods is not None:
            self.extend(_input, periods)

    def append(self, _input, period):
        return self._state.append(_input, period)

    def extend(self, _input, periods):
        return self._state.extend(_input, periods)

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
