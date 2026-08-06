"""Descriptive stateful interface for a variable-period moving average."""

from taflow._native import StatefulMavp


class VariablePeriodMovingAverage:
    """Incrementally compute MAVP from values and per-bar periods."""

    def __init__(self, min_period=2, max_period=30, average_type=0):
        self._state = StatefulMavp(min_period, max_period, average_type)

    def append(self, input, period):
        return self._state.append(input, period)

    def extend(self, input, periods):
        return self._state.extend(input, periods)

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
