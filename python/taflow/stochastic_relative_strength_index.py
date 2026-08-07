"""Descriptive stateful interface for Stochastic RSI."""

from taflow._native import StatefulStochrsi
from typing import Any


class StochasticRelativeStrengthIndex:
    """Incrementally compute aligned stochastic-RSI fast %K and fast %D."""

    def __init__(
        self,
        time_period=14,
        fast_k_period=5,
        fast_d_period=3,
        fast_d_average_type=0,
        input: Any | None = None,
    ):
        self._state = StatefulStochrsi(
            time_period,
            fast_k_period,
            fast_d_period,
            fast_d_average_type,
        )
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
