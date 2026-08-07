"""Descriptive stateful interface for extended MACD."""

from taflow._native import StatefulMacdExt
from typing import Any


class MovingAverageConvergenceDivergenceExtended:
    """Incrementally compute MACDEXT with independently selected MA types."""

    def __init__(
        self,
        fast_period=12,
        fast_average_type=1,
        slow_period=26,
        slow_average_type=1,
        signal_period=9,
        signal_average_type=1,
        _input: Any | None = None,
    ):
        self._state = StatefulMacdExt(
            fast_period,
            fast_average_type,
            slow_period,
            slow_average_type,
            signal_period,
            signal_average_type,
        )
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
