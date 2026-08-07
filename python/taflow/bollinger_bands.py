"""Descriptive stateful interface for Bollinger Bands."""

from taflow._native import StatefulBbands
from typing import Any


class BollingerBands:
    """Incrementally compute upper, middle, and lower Bollinger Bands."""

    def __init__(
        self,
        period=5,
        deviations_up=2.0,
        deviations_down=2.0,
        moving_average_type=0,
        values: Any | None = None,
    ):
        self._state = StatefulBbands(
            period, deviations_up, deviations_down, moving_average_type
        )
        if values is not None:
            self.extend(values)

    def append(self, value):
        return self._state.append(value)

    def extend(self, values):
        return self._state.extend(values)

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
